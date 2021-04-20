#[macro_use]
extern crate clap;

mod models;

use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgMatches};
use models::{Configuration, WebhookPayload};
use std::vec::Vec;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let result = process();
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

const ARG_PID: &str = "arg_pid";
const ARG_DESTINATION: &str = "arg_destination";

fn process() -> Result<(), String> {
    let args = get_args();
    let configure = resolve_configuration(&args)?;
    let cmd = match get_process_cmd(configure.pid) {
        None => return Err("no exists process".to_string()),
        Some(s) => s,
    };
    let datetime_start = Utc::now();
    let message_start = create_message(configure.pid, &cmd, &datetime_start, None);
    let text_start = WebhookPayload::create(&message_start)?;
    post_message(&configure.webhook_url, &&text_start)?;
    loop {
        let duration = std::time::Duration::from_secs(configure.interval_seconds);
        std::thread::sleep(duration);

        if get_process_cmd(configure.pid).is_none() {
            break;
        }
    }
    let datetime_end = Utc::now();
    let message_end = create_message(configure.pid, &cmd, &datetime_start, Some(&datetime_end));
    let text_end = WebhookPayload::create(&message_end)?;
    post_message(&configure.webhook_url, &text_end)
}

fn get_args() -> ArgMatches<'static> {
    App::new("Process End Slack Notifier")
        .author("sinofseven")
        .version(crate_version!())
        .arg(
            Arg::with_name(ARG_PID)
                .required(true)
                .long("pid")
                .short("p")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_DESTINATION)
                .required(true)
                .long("destination")
                .short("d")
                .takes_value(true),
        )
        .get_matches()
}

fn is_destination_url(dest: &str) -> bool {
    if dest.len() <= 32 {
        return false;
    }
    return &dest[..32] == "https://hooks.slack.com/services";
}

fn resolve_configuration(args: &ArgMatches) -> Result<Configuration, String> {
    let pid = args.value_of(ARG_PID).unwrap();
    let dest = args.value_of(ARG_DESTINATION).unwrap();

    let pid: i32 = pid
        .parse()
        .map_err(|e| format!("failed to cast pid to number: {}", e))?;

    let dest = if is_destination_url(dest) {
        dest.to_string()
    } else {
        let config_file = models::ConfigFile::load()?;
        let v = config_file.webhook_urls.get(dest);
        match v {
            None => return Err("failed to resolve destination".to_string()),
            Some(v) => v.clone(),
        }
    };

    Ok(Configuration {
        webhook_url: dest,
        pid: pid,
        interval_seconds: 60,
    })
}

fn get_process_cmd(pid: i32) -> Option<String> {
    let sys = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::new().with_processes());
    sys.get_process(pid).map(|p| p.cmd().join(" "))
}

fn create_message(
    pid: i32,
    cmd: &str,
    datetime_start: &DateTime<Utc>,
    datetime_end: Option<&DateTime<Utc>>,
) -> String {
    let mut lines: Vec<String> = Vec::new();
    let datetime_format = "%Y/%m/%d %H:%M:%S %Z";

    let message = if let Some(_) = datetime_end {
        "End (Monitoring) Process"
    } else {
        "Start (Monitoring) Process"
    };

    lines.push(message.to_string());
    lines.push(format!("- Pid: `{}`", pid));
    lines.push(format!("- Cmd: `{}`", cmd));

    lines.push(format!(
        "- Start: `{}`",
        datetime_start.format(datetime_format)
    ));

    if let Some(datetime_end) = datetime_end {
        let diff = datetime_end.naive_utc() - datetime_start.naive_utc();
        lines.push(format!("- End: `{}`", datetime_end.format(datetime_format)));
        lines.push(format!(
            "- Duration: `{}d {}h {}m {}s`",
            diff.num_days(),
            diff.num_hours() % 24,
            diff.num_minutes() % 60,
            diff.num_seconds() % 60
        ));
    }

    lines.join("\n")
}

fn post_message(url: &str, text: &str) -> Result<(), String> {
    let client = reqwest::blocking::Client::new();
    client
        .post(url)
        .body(text.to_string())
        .send()
        .map(|_| ())
        .map_err(|e| format!("failed to send webhook: {}", e))
}
