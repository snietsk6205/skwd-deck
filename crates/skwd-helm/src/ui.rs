use super::error::CliError;

pub(super) fn run(args: &[String]) -> Result<(), CliError> {
    let command = args.join(" ");
    let command = command.trim();
    if command.is_empty() {
        return Err(CliError::BadArgs(String::from(
            "usage: ui <toggle|show|hide|stash|close|dismiss|mode M|filter TEXT|clear|open PANEL|playlist ID|playlist-demo ACTION|schedule-demo ACTION|scroll-demo RATE|settings-search TEXT|settings-result N|tab NAME|section N|nav DIR|pick|select KEY|flip|kind K|orient O|resolution R|bar ACTION|badges ACTION|demo ACTION|tune PATH VALUE|motion SCALE|recolour PALETTE|sort MODE|colour NAME|stage-blur RADIUS|apply OUTPUT [SHADER] [MS]|set PATH VALUE|state|perf [on|off]|bench [--gpu high|low]>",
        )));
    }
    if command == "perf" {
        return print_query("perf-query");
    }
    if command == "bench" || command.starts_with("bench ") {
        let options: Vec<&str> = command.split_whitespace().skip(1).collect();
        return super::bench::run(&options);
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
