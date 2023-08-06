//! Grafana CLI using the Grafana LIB crate
//! 
//! 
use grafana_lib::{client::Client, 
    community::dashboard::DashboardBuilder, 
    community::data_source::DataSourceBuilder,
    community::folder::FolderModel,
};
use clap::{Parser,Subcommand};
use log::{info,error};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Grafana CLI using the Grafana interface crate")]
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
    Alerting {
        #[command(subcommand)]
        cmd : AlertingCommands,
    },
    Dashboard {
        #[command(subcommand)]
        cmd : DashboardCommands,
    },
    DataSource {
        #[command(subcommand)]
        cmd : DataSourceCommands,
    },
    Folder {
        #[command(subcommand)]
        cmd : FolderCommands,
    },
    Organization {
        #[command(subcommand)]
        cmd : OrganizationCommands,
    }
}

#[derive(Subcommand,Debug)]
pub enum AlertingCommands {
    Rules {
        #[command(subcommand)]
        opts : RuleOptions,
    },    
}

#[derive(Subcommand,Debug)]
pub enum RuleOptions {
    List {

    },
    Create {
        #[arg(short, long)]
        name : String,
    }
}

#[derive(Subcommand,Debug)]
pub enum DashboardCommands {
    Create {
        #[arg(short, long)]
        name : String,
    },
    List {
        #[arg(short, long)]
        query : Option<String>,
    }
}

#[derive(Subcommand,Debug)]
pub enum DataSourceCommands {
    Create {
        #[arg(short, long)]
        name : String,
    },
    List {

    }
}

#[derive(Subcommand,Debug)]
pub enum FolderCommands {
    Create {
        #[arg(short, long)]
        name : String,
    },
    List {
        #[arg(short, long)]
        query : Option<String>
    }
}

#[derive(Subcommand,Debug)]
pub enum OrganizationCommands {
    Create {
        #[arg(short, long)]
        name : String,
    },
}

fn main() {
    let args = Args::parse();
    env_logger::init();

    // Create a client to use for cli
    let client = Client::new(String::from("http://localhost:3000"));

    match args.command {
        Some(Commands::Alerting { cmd }) => {
            info!("Executing Alerting");
            match cmd {
                AlertingCommands::Rules { opts } => {
                    match opts {
                        RuleOptions::Create { name } => {
                            info!("Creating alerting rule: {name}");
                            
                        },
                        RuleOptions::List {  } => {
                            info!("Listing alerting rules");
                            let _result = client.alerting_provisioning().alert_rule().list();
                        },
                    }
                }
            }
        },
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
                DashboardCommands::List { query } => {
                    info!("Searching dashboards");
                    let results = client.search().dashboard(query);
                    match results {
                        Ok(r) => {
                            let mut output = String::from(format!("{} results.\n",r.len()));
                            r.into_iter().for_each(|dm| {
                                output.push_str(&dm.title);
                                output.push_str("\n");
                            });
                            println!("Dashboards: {}",output);
                        }
                        Err(e) => {
                            error!("Dashboard Search: error {}",e);
                        }
                       
                    }
                    
                }
            }
        },
        Some(Commands::Folder { cmd }) => {
            info!("Executing Folder API");
            match cmd {
                FolderCommands::Create { name } => {
                    let model = FolderModel::new(name);
                    let _result = client.folder().create(model);
                }
                FolderCommands::List { query } => {
                    
                    let results = client.search().folder(query);
                    match results {
                        Ok(r) => {
                            let mut output = String::from(format!("{} Results.\n",r.len()));
                            r.into_iter().for_each(|fm| {
                                output.push_str(&fm.title);
                                output.push_str("\n");
                            });
                            println!("Folders: {}",output);
                        },
                        Err(e) => {
                            error!("Folder Search: error {}",e);
                        },
                    }
                    
                }
            }
        },
        Some(Commands::DataSource { cmd }) => {
            info!("Executing Datasource commands");
            match cmd {
                DataSourceCommands::Create { name } => {
                    let model = DataSourceBuilder::new(name)
                        .build();
                    let _result = client
                        .data_source()
                        .create(model);
                },
                DataSourceCommands::List {  } => {
                    let result = client
                        .data_source()
                        .get(None);
                    match result {
                        Ok(r) => {
                            // Parse resulting vector
                            let mut output = String::from(format!("{} Results.\n",r.len()));
                            r.into_iter().for_each(|ds| {
                                output.push_str(&ds.name);
                                output.push_str("\n");    
                            });
                            println!("Folders: {}",output);
                        },
                        Err(e) => {
                            error!("List failed: {}",e.to_string())
                        }
                    } 
                },
            }
        },
        Some(Commands::Organization { cmd }) => {
            info!("Executing Organization commands");
            match cmd {
                OrganizationCommands::Create { name } => {
                    match client
                        .organization()
                        .create(name)
                        .send() {
                            Ok(r) => info!("Org created!: {r}"),
                            Err(e) => error!("Could not create org: {e}"),
                        }
                }
            }
        }
        None => {},
    }
}

mod main_test {
    #[test]
    fn verify_cli() {
        use super::*;
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}