mod command;
mod errors;
mod helpers;
use crate::helpers::pwd;
use command::Command;
use errors::CrateResult;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};

#[tokio::main]
async fn main() -> CrateResult<()> {
    println!("Welcome to the Shell!");
    let user_input_handle = spawn_user_input_handler();
    let user_input_handle = user_input_handle.await;
    if let Ok(Err(e)) = user_input_handle {
        eprintln!("Error: {e}")
    }
    Ok(())
}

fn spawn_user_input_handler() -> JoinHandle<CrateResult<()>> {
    tokio::spawn(async {
        //initialize the stdin and stdout
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let mut reader = tokio::io::BufReader::new(stdin).lines();
        // wrap given stdout in a buffered writer to improve performance
        let mut stdout = tokio::io::BufWriter::new(stdout);

        stdout.write_all(pwd()?.as_bytes()).await?;
        stdout.write_all(b">").await?;
        stdout.flush().await?;

        while let Ok(Some(line)) = reader.next_line().await {
            let command = handle_new_line(&line).await;
            if let Ok(command) = &command {
                match command {
                    Command::Exit => {
                        println!("Exiting...");
                        break;
                    }
                    Command::Echo(message) => {
                        println!("{message}");
                    }
                    _ => {}
                }
            } else {
                eprintln!("Error parsing command: {}", command.err().unwrap());
            }
            stdout.write_all(pwd()?.as_bytes()).await?;
            stdout.write_all(b">").await?;
            stdout.flush().await?;
        }
        Ok(())
    })
}

async fn handle_new_line(line: &str) -> CrateResult<Command> {
    let command: Command = line.try_into()?;
    match &command {
        Command::Ls => {
            helpers::ls()?;
        }
        Command::Pwd => {
            println!("{}", helpers::pwd()?);
        }
        Command::Cd(s) => {
            helpers::cd(s)?;
        }
        Command::Touch(s) => {
            helpers::touch(s)?;
        }
        Command::Rm(s) => {
            helpers::rm(s)?;
        }
        Command::Cat(s) => {
            let contents = helpers::cat(s)?;
            println!("{}", contents);
        }
        Command::Mkdir(s) => {
            helpers::mkdir(s)?;
        }
        Command::Rmdir(s) => {
            helpers::rmdir(s)?;
        }
        Command::Exec { program, args } => {
            // Handle external command execution
            helpers::exec_external(program, args).await?;
        }
        _ => {}
    }
    Ok(command)
}
