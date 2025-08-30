#[macro_use]
extern crate log;

mod actions;
mod args;
mod config;
mod deploy;
mod difference;
mod filesystem;
mod handlebars_helpers;
mod hooks;
mod init;

use std::io::{self, Write};

use anyhow::Context;
use clap::CommandFactory;
use clap_complete::{generate, generate_to};
use crossterm::style::Stylize;

macro_rules! run_cmd {
    ($cmd:expr) => {
        match $cmd {
            Ok(success) => match success {
                true => std::process::exit(0),
                false => std::process::exit(1),
            },
            Err(e) => {
                display_error(e);
                std::process::exit(1);
            }
        }
    };
}

fn main() {
    // Parse arguments
    let opt = args::get_options();

    env_logger::builder()
        .format(|buf, record| {
            //Uses similar system to the colog crate, but I removed all the trait stuff since it's internal impl
            fn prefix(level: log::Level) -> String {
                match level {
                    log::Level::Error => format!("[{}]", "E".red()),
                    log::Level::Warn => format!("[{}]", "W".yellow()),
                    log::Level::Info => format!("[{}]", "*".green()),
                    log::Level::Debug => format!("[{}]", "D".cyan()),
                    log::Level::Trace => format!("[{}]", "T".magenta()),
                }
                .bold()
                .to_string()
            }

            writeln!(
                buf,
                "{} {}",
                prefix(record.level()),
                record.args().to_string().replace("\n", "\n | ")
            )
        })
        .filter_level(match opt.verbosity {
            _ if opt.quiet => log::LevelFilter::Error,
            true => log::LevelFilter::Trace,
            false => log::LevelFilter::Info,
        })
        .init();

    debug!("Loaded options: {:#?}", opt);

    #[cfg(unix)]
    {
        use std::os::unix::prelude::MetadataExt;
        if std::env::var("USER").unwrap_or_default() == "root"
            && !std::fs::metadata(&opt.global_config).is_ok_and(|m| m.uid() == 0)
        {
            warn!("It is not recommended to run Dotter as root, since the cache files and all files not marked with an `owner` field will default to being owned by root.
If you're truly logged in as root, it is safe to ignore this message.
Otherwise, run `dotter undeploy` as root, remove cache.toml and cache/ folders, then use Dotter as a regular user.");
        }
    }

    match opt.action.clone().unwrap_or_default() {
        args::Action::Deploy => {
            debug!("Deploying...");
            run_cmd!(deploy::deploy(&opt).context("deploy"))
        }
        args::Action::Undeploy => {
            debug!("Un-Deploying...");
            run_cmd!(deploy::undeploy(&opt).context("undeploy"))
        }
        args::Action::Init => {
            debug!("Initializing repo...");
            run_cmd!(init::init(opt).context("initalize directory"))
        }
        args::Action::GenCompletions { shell, to } => {
            if let Some(to) = to {
                run_cmd!(
                    generate_to(shell, &mut args::Options::command(), "dotter", to)
                        .context("write completion to a file")
                        .map(|_| true) //We don't use the pathbuf here anyway?
                )
            } else {
                generate(
                    shell,
                    &mut args::Options::command(),
                    "dotman",
                    &mut io::stdout(),
                );
            }
        }
    }
}

pub(crate) fn display_error(error: anyhow::Error) {
    let mut chain = error.chain();
    let mut error_message = format!("Failed to {}\nCaused by:\n", chain.next().unwrap());

    //Localize scope
    use std::fmt::Write;
    for e in chain {
        writeln!(error_message, "    {e}").unwrap();
    }
    // Remove last \n
    error_message.pop();

    error!("{}", error_message);
}
