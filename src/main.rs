use std::{path::Path, str};

mod bluetooth;
mod projects;

use bluetooth::{connect_to_device, disconnect_to_device};
use clap::{command, Parser, Subcommand};
use projects::folders::get_git_repositories;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Bluetooth {
        #[command(subcommand)]
        command: Option<BluetoothCommands>,
    },
    Projects {
        #[command(subcommand)]
        command: Option<ProjectCommands>,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    List {},
}

#[derive(Subcommand)]
enum BluetoothCommands {
    Connect {},
    Disconnect {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Bluetooth { command }) => match &command {
            Some(BluetoothCommands::Connect {}) => {
                connect_to_device();
            }
            Some(BluetoothCommands::Disconnect {}) => {
                disconnect_to_device();
            }
            None => {}
        },
        Some(Commands::Projects { command }) => match &command {
            Some(ProjectCommands::List {}) => {
                let repos = get_git_repositories(Path::new("/home/andreas/git/github.com"));
                for repo in repos {
                    println!("{}", repo)
                }
            }
            None => {}
        },
        None => {}
    }
}
