
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use structopt::StructOpt;
use ignore::WalkBuilder;
use humantime::format_duration;

/// Calm down and check your code again.
///
/// If timeout is not specified,
/// it only prints the last modification time and doesn't fail.
#[derive(Debug, StructOpt)]
#[structopt(global_setting(structopt::clap::AppSettings::TrailingVarArg))]
struct Opt {
    #[structopt(short, long, default_value = "0")]
    hours: u32,
    #[structopt(short, long, default_value = "0")]
    minutes: u32,
    #[structopt(short, long, default_value = "0")]
    seconds: u32,
    /// If missed, current directory is used instead.
    #[structopt(index(1))]
    directories: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let timeout = opt.hours as u64 * 3600 + opt.minutes as u64 * 60 + opt.seconds as u64;
    let timeout = Duration::from_secs(timeout);
    let now = SystemTime::now();
    let deadline = now - timeout;

    let mut dirs = opt.directories.iter();
    let builder = match dirs.next() {
        None => WalkBuilder::new("."),
        Some(dir) => {
            let mut builder = WalkBuilder::new(dir);
            for dir in dirs {
                builder.add(dir);
            }
            builder
        }
    };

    let latest = builder.build()
        .filter_map(|entry| {
            entry.ok()?
                .metadata().ok()?
                .modified().ok()
        })
        .max();
    let latest = match latest {
        None => return,
        Some(time) => time,
    };

    if timeout == Duration::from_secs(0) {
        println!("{} passed since the last modification.",
            format_duration(now.duration_since(latest).unwrap()));
        return
    }

    if latest >= deadline {
        eprintln!("Calm down! {} passed since the last modification.",
            format_duration(now.duration_since(latest).unwrap()));
        eprintln!("Try again after {}",
            format_duration(latest.duration_since(deadline).unwrap()));
        std::process::exit(1)
    }
}
