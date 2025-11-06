mod cli;
mod model;
mod config;
mod sshc;
mod update;

use anyhow::Result;
use clap::{Parser, CommandFactory};
use cli::{Cli, Command, AuthType};
use model::{Auth, HostEntry};
use config as cfg;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.print_config_path {
        println!("{}", cfg::config_path()?.display());
        return Ok(());
    }

    match cli.cmd {
        Some(Command::Version) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Some(Command::List) => {
            let c = cfg::load_or_init()?;
            if c.hosts.is_empty() {
                println!("no aliases configured");
            } else {
                for h in c.hosts {
                    println!("{:<16} {:<20} {:<6} {:?}", h.name, format!("{}@{}", h.user, h.host), h.port, h.auth);
                }
            }
        }
        Some(Command::Export { alias }) => {
            let c = cfg::load_or_init()?;
            let h = cfg::find_host(&c, &alias)?;
            let cmd = sshc::export_command(&h, &[]);
            println!("{cmd}");
        }
        Some(Command::Remove { alias }) => {
            let c = cfg::load_or_init()?;
            let c = cfg::remove_host(c, &alias)?;
            cfg::save(&c)?;
            println!("removed '{}'", alias);
        }
        Some(Command::Add(args)) => {
            let entry = match args.auth {
                AuthType::Cert => {
                    let id = args.identity_file
                        .expect("--identity-file is required for --auth cert");
                    HostEntry {
                        name: args.name, user: args.user, host: args.host,
                        port: args.port, auth: Auth::Cert{ identity_file: id }
                    }
                }
                AuthType::Password => {
                    HostEntry {
                        name: args.name, user: args.user, host: args.host,
                        port: args.port, auth: Auth::Password{ password: args.password }
                    }
                }
            };
            let c = cfg::load_or_init()?;
            let c = cfg::upsert_host(c, entry)?;
            cfg::save(&c)?;
            println!("alias saved");
        }
        Some(Command::Connect { alias, ssh_args }) => {
            let c = cfg::load_or_init()?;
            let h = cfg::find_host(&c, &alias)?;
            let code = sshc::connect(&h, &ssh_args)?;
            std::process::exit(code);
        }
        Some(Command::CheckUpdates) => {
            match update::check_updates()? {
                Some((cur, latest)) => println!("update available: {cur} -> {latest}"),
                None => println!("up to date"),
            }
        }
        None => {
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}
