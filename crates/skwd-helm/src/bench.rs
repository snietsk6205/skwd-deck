//! `skwd-helm ui bench [--gpu high|low]`: one command for the picker's
//! benchmark. The picker covers the screen and takes the keyboard while it is
//! open, so this command does everything itself: it starts the custom picker
//! that sits next to this program (stopping an older picker without the
//! benchmark first), waits for the report without polling the picker (polling
//! would make it redraw), prints it, and closes the picker again.

use std::os::unix::fs::MetadataExt as _;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use wall_proto::picker_control::{send_command, send_query, send_query_waiting};

use super::error::CliError;

/// The whole run takes about 35 seconds; leave room for a slow first start.
const BENCH_WAIT: Duration = Duration::from_secs(240);
const START_WAIT: Duration = Duration::from_secs(40);
const STOP_WAIT: Duration = Duration::from_secs(6);
/// Installed name first, then the name `cargo build` gives it.
const PICKER_NAMES: &[&str] = &["skwd-wall-v2", "skwd-wall"];

pub(super) fn run(args: &[&str]) -> Result<(), CliError> {
    let gpu = parse_gpu(args)?;
    // A running picker that knows the benchmark can run it right away
    // (unless another GPU was asked for, which needs a fresh start).
    if gpu.is_none()
        && let Some(answer) = ask_running_picker()?
    {
        return finish(&answer);
    }
    if picker_running() {
        eprintln!(
            "stopping the running picker (it has no benchmark, or must restart on another GPU)..."
        );
        stop_pickers()?;
    }
    let picker = picker_path()?;
    eprintln!("starting {}...", picker.display());
    start_picker(&picker, gpu)?;
    wait_until_running()?;
    eprintln!(
        "benchmark running: the picker moves by itself for about 35 seconds. Do not touch the keyboard or mouse."
    );
    let answer = send_query_waiting("bench-run", BENCH_WAIT).map_err(|error| {
        CliError::local(format!("lost the picker during the benchmark: {error}"))
    })?;
    finish(&answer)
}

fn parse_gpu(args: &[&str]) -> Result<Option<&'static str>, CliError> {
    match args {
        [] => Ok(None),
        ["--gpu", "high"] => Ok(Some("high")),
        ["--gpu", "low"] => Ok(Some("low")),
        _ => Err(CliError::BadArgs(String::from(
            "usage: ui bench [--gpu high|low]   (high = the fast graphics card, low = the built-in one)",
        ))),
    }
}

/// `Some(answer)` when a running picker ran the benchmark; `None` when no
/// picker runs or the running one does not know the benchmark (it closes the
/// connection without an answer).
fn ask_running_picker() -> Result<Option<String>, CliError> {
    if !picker_running() {
        return Ok(None);
    }
    eprintln!("benchmark running in the open picker (about 35 seconds; do not touch anything)...");
    match send_query_waiting("bench-run", BENCH_WAIT) {
        Ok(answer) if !answer.is_empty() => Ok(Some(answer)),
        Ok(_) => Ok(None),
        Err(error) => Err(CliError::local(format!("the picker did not answer: {error}"))),
    }
}

fn picker_running() -> bool {
    send_query("state").is_ok_and(|state| !state.is_empty())
}

fn finish(answer: &str) -> Result<(), CliError> {
    let value: serde_json::Value = serde_json::from_str(answer)
        .map_err(|_| CliError::local("the picker sent an unreadable report"))?;
    let report = value["report"].as_str().unwrap_or_default();
    println!("{report}");
    if let Some(path) = value["path"].as_str() {
        println!("\nSaved to {path} (and latest.json next to it).");
    }
    // Give the keyboard back.
    let _ = send_command("hide");
    Ok(())
}

fn picker_path() -> Result<PathBuf, CliError> {
    let exe = std::env::current_exe()
        .map_err(|error| CliError::local(format!("cannot find this program's folder: {error}")))?;
    let folder = exe.parent().unwrap_or(Path::new("."));
    PICKER_NAMES.iter().map(|name| folder.join(name)).find(|path| path.is_file()).ok_or_else(|| {
        CliError::local(format!(
            "no picker next to this program in {} (run the skwd-helm from ~/.local/opt/skwd-custom/bin/)",
            folder.display()
        ))
    })
}

fn stop_pickers() -> Result<(), CliError> {
    for name in PICKER_NAMES {
        let _ = Command::new("pkill")
            .args(["-x", name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let started = Instant::now();
    while picker_running() {
        if started.elapsed() > STOP_WAIT {
            return Err(CliError::local(
                "the running picker did not stop; close it (Esc) and run this command again",
            ));
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

fn start_picker(picker: &Path, gpu: Option<&str>) -> Result<(), CliError> {
    let mut command = Command::new(picker);
    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).process_group(0);
    if std::env::var_os("WAYLAND_DISPLAY").is_none_or(|display| display.is_empty()) {
        // Over SSH the display's name is not set; take the desktop's socket.
        let display = wayland_display().ok_or_else(|| {
            CliError::local("no Wayland display found (is the desktop running on this computer?)")
        })?;
        eprintln!("using display {display}");
        command.env("WAYLAND_DISPLAY", display);
    }
    if let Some(gpu) = gpu {
        command.env("WGPU_POWER_PREF", gpu);
    }
    command
        .spawn()
        .map(drop)
        .map_err(|error| CliError::local(format!("cannot start {}: {error}", picker.display())))
}

fn runtime_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR").filter(|dir| !dir.is_empty()) {
        return PathBuf::from(dir);
    }
    let uid = std::fs::metadata("/proc/self").map_or(0, |meta| meta.uid());
    PathBuf::from(format!("/run/user/{uid}"))
}

/// The first `wayland-N` socket in the runtime folder.
fn wayland_display() -> Option<String> {
    let mut names: Vec<String> = std::fs::read_dir(runtime_dir())
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| is_wayland_socket_name(name))
        .collect();
    names.sort();
    names.into_iter().next()
}

pub(super) fn is_wayland_socket_name(name: &str) -> bool {
    name.strip_prefix("wayland-")
        .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|byte| byte.is_ascii_digit()))
}

fn wait_until_running() -> Result<(), CliError> {
    let started = Instant::now();
    while !picker_running() {
        if started.elapsed() > START_WAIT {
            return Err(CliError::local(
                "the picker did not start within 40 seconds; try starting it by hand once",
            ));
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    Ok(())
}
