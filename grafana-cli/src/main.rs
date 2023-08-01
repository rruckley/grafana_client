//! Grafana CLI using the Grafana LIB crate
//! 
//! 
use grafana_lib::{client::Client, community::dashboard::DashboardBuilder};
use clap::{Parser,Subcommand};
use log::{info,error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional host, overrides GRAFANA_HOST environment 
    #[arg(long)]
    host: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

// CLI commands
#[derive(Subcommand,Debug)]
pub enum Commands {
    Dashboard {
        #[command(subcommand)]
        cmd : DashboardCommands,
    },
    Folder {
        #[arg(short, long)]
        list: bool,
    }
}

#[derive(Subcommand,Debug)]
pub enum DashboardCommands {
    Create {
        #[arg(short, long)]
        name : String,
    },
    List {

    }
}

fn main() {
    let args = Args::parse();
    env_logger::init();

    // Create a client to use for cli
    let client = Client::new(String::from("http://localhost:3000"));

    match args.command {
        Some(Commands::Dashboard { cmd }) => {
            info!("Executing Dashboard");
            match cmd {
                DashboardCommands::Create { name } => {
                    info!("Creating new dashboard: {name}");
                    let model = DashboardBuilder::new(name).build();
                    let _output = client.dashboard()
                        .create(model)
                        .with_message(String::from("New Dashboard via CLI"))
                        .with_folder_id(6)
                        .send();
                },
                List => {
                    info!("Listing dashboards");
                    let _output = client.search().dashboard(None);
                },
            }
        },
        Some(Commands::Folder { list }) => {
            info!("Executing Folder Search");
            if list {
                let mut output = String::from("");
                let results = client.search().folder(None);
                match results {
                    Ok(r) => {
                        r.into_iter().for_each(|_fr| {
                            output.push_str("test")
                        })
                    },
                    Err(e) => {
                        error!("Folder Search: error {}",e.message);
                    },
                }
                println!("Folder results: {}",output);
            }
        },
        None => {},
    }
}