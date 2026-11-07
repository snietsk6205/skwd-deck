use super::error::CliError;

pub(super) fn run(args: &[String]) -> Result<(), CliError> {
    let command = args.join(" ");
    let command = command.trim();
    if command.is_empty() {
        return Err(CliError::BadArgs(String::from(
            "usage: ui <toggle|show|hide|stash|close|dismiss|mode M|filter TEXT|clear|open PANEL|playlist ID|playlist-demo ACTION|schedule-demo ACTION|scroll-demo RATE|settings-search TEXT|settings-result N|tab NAME|section N|nav DIR|pick|select KEY|flip|kind K|orient O|resolution R|bar ACTION|badges ACTION|demo ACTION|tune PATH VALUE|motion SCALE|recolour PALETTE|sort MODE|colour NAME|stage-blur RADIUS|apply OUTPUT [SHADER] [MS]|set PATH VALUE|state|perf [on|off]|bench>",
        )));
    }
    if command == "perf" {
        return print_query("perf-query");
    }
    if command == "bench" {
        return run_bench();
    }
    if command == "state" || command.starts_with("state ") {
        let response = wall_proto::picker_control::send_query(command)
            .map_err(|_| CliError::local("picker is not running (launch skwd-wall first)"))?;
        if response.is_empty() {
            return Err(CliError::local("picker gave no state reply"));
        }
        println!("{response}");
        return Ok(());
    }
    wall_proto::picker_control::send_command(command)
        .map_err(|_| CliError::local("picker is not running (launch skwd-wall first)"))?;
    println!("sent: {command}");
    Ok(())
}

fn print_query(query: &str) -> Result<(), CliError> {
    let response = wall_proto::picker_control::send_query(query)
        .map_err(|_| CliError::local("picker is not running (launch skwd-wall first)"))?;
    if response.is_empty() {
        return Err(CliError::local("picker gave no reply"));
    }
    println!("{response}");
    Ok(())
}

/// Starts the picker's built-in benchmark, waits for it, then prints the report.
fn run_bench() -> Result<(), CliError> {
    wall_proto::picker_control::send_command("bench")
        .map_err(|_| CliError::local("picker is not running (launch skwd-wall first)"))?;
    eprintln!("benchmark started: the picker opens and moves by itself for about 30 seconds...");
    let started = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_secs(2));
    while started.elapsed() < std::time::Duration::from_secs(120) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let Ok(response) = wall_proto::picker_control::send_query("perf-query") else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&response) else {
            continue;
        };
        if value["bench_running"].as_bool() == Some(false)
            && let Some(report) = value["bench_report"].as_str()
        {
            println!("{report}");
            if let Some(path) = value["bench_report_path"].as_str() {
                println!("\nSaved to {path} (and latest.json next to it).");
            }
            return Ok(());
        }
    }
    Err(CliError::local("the benchmark did not finish within 2 minutes"))
}
