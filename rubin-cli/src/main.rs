mod cli;
use cli::*;

use std::io::{self, Write};

use rubin::net::{client::RubinClient, server::start};

fn prep_input(cmd: &str) -> String {
    let cmd_split: Vec<&str> = cmd.trim().split(' ').collect();
    if cmd_split.len() < 2 {
        return cmd.to_string();
    }

    cmd_split[0].to_owned() + "::" + &cmd_split[1..].join(" ")
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = CliParser::parse();

    match &cli.commands {
        Commands::Server(args) => {
            start(&args.address, args.port).await?;
        }
        Commands::Cli(args) => {
            let client = RubinClient::new(&args.address, args.port);
            let mut cmd = String::new();

            loop {
                print!("> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut cmd)?;
                let prepped_cmd = prep_input(&cmd);

                let response = match client.request(&prepped_cmd).await {
                    Ok(response) => response,
                    Err(e) => {
                        println!("Unable to connect to the Rubin server: {}", e);
                        break;
                    }
                };
                println!("{}\n", response);
                cmd.clear();
            }
        }
    }

    Ok(())
}
