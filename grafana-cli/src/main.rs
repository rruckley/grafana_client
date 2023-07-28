//! Grafana CLI using the Grafana LIB crate
//! 
//! 
use grafana_lib::client::{Client};
use clap::{Parser,Subcommand};
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional host, overrides GRAFANA_HOST environment 
    #[arg(long)]
    host: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

// CLI commands
#[derive(Subcommand,Debug)]
pub enum Commands {
    Dashboard {
        #[arg(short, long)]
        list: bool,
    },
    Folder {
        #[arg(short, long)]
        list: bool,
    }
}

fn main() {
    let args = Args::parse();

    // Create a client to use for cli
    let client = Client::new(String::from("http://localhost:3000"));

    dbg!(&args.command);
    match args.command {
        Commands::Dashboard { list } => {
            info!("Executing Dashboard");
            if list {
                let results = client.search_dashboards(None);
            }
        },
        Commands::Folder { list } => {
            info!("Executing Folder Search");
            if list {
                let results = client.search_folders(None);
            }
        }
    }
}