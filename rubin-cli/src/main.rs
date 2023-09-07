mod cli;
use cli::*;

use std::io::{self, Write};

use rubin::net::parser::Operation;
use rubin::net::{client::RubinClient, server::start};

#[derive(Debug, PartialEq)]
enum Comparitor {
    Eq,
    Gte,
}

fn validate_cmd_length(cmds: &Vec<&str>, size: usize, comparitor: Comparitor) -> bool {
    let command_length = cmds.len();
    match comparitor {
        Comparitor::Eq => {
            if command_length == size {
                return true;
            }
        }
        Comparitor::Gte => {
            if command_length >= size {
                return true;
            }
        }
    }

    println!("incorrect argument length for operation.\n");
    return false;
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

            loop {
                // The overheads of creating a new string each loop are insignificant
                let mut cmd = String::new();

                print!("rubin-cli > ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut cmd)?;

                let trimmed_cmd = cmd.trim();
                if trimmed_cmd == "exit" {
                    println!("Quitting.");
                    break;
                } else if trimmed_cmd.is_empty() {
                    continue;
                }

                let mut cmd_split: Vec<&str> = cmd.split(' ').collect();
                let raw_op = &cmd_split.remove(0).trim();
                let op = Operation::from_string(raw_op);

                let response = match op {
                    Operation::StringGet => {
                        if !validate_cmd_length(&cmd_split, 1, Comparitor::Eq) {
                            continue;
                        }
                        let key = &cmd_split[0];
                        client.get_string(key).await
                    }
                    Operation::StringSet => {
                        if !validate_cmd_length(&cmd_split, 2, Comparitor::Gte) {
                            continue;
                        }
                        let key = &cmd_split[0];
                        let value = &cmd_split[1..].join(" ");
                        client.insert_string(key, value).await
                    }
                    Operation::StringRemove => {
                        if !validate_cmd_length(&cmd_split, 1, Comparitor::Eq) {
                            continue;
                        }
                        let key = &cmd_split[0];
                        client.remove_string(key).await
                    }
                    Operation::StringClear => client.clear_strings().await,
                    Operation::Dump => {
                        if !validate_cmd_length(&cmd_split, 1, Comparitor::Eq) {
                            continue;
                        }

                        let path = &cmd_split[0];
                        client.dump_store(path).await
                    }
                    Operation::Error => {
                        println!("invalid operation: {}\n", raw_op);
                        continue;
                    }
                    _ => continue,
                };

                let msg = match response {
                    Ok(msg) => msg,
                    Err(e) => {
                        println!("Unable to connect to the Rubin server: {}", e);
                        break;
                    }
                };
                println!("{}\n", msg);
            }
        }
    }

    Ok(())
}
