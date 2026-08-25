use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum Command {
    //all commands that the shell can handle
    Exit,
    Echo(String),
    Ls,
    Pwd,
    Cd(String),
    Touch(String),
    Rm(String),
    Cat(String),
    Mkdir(String),
    Rmdir(String),
    //fallback for any other command that is not recognized by the shell, will be executed as a program by the OS
    Exec { program: String, args: Vec<String> },
}

// Implement TryFrom<&str> for Command to parse a string into a Command enum variant
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_value: Vec<&str> = value.split_whitespace().collect();
        //extract the first word as command name
        let command_name = match split_value.first() {
            Some(cmd) => cmd,
            None => return Err(anyhow!("Empty input!")),
        };

        match command_name.to_lowercase().as_str() {
            "exit" => Ok(Command::Exit),
            "echo" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("Echo command requires an argument"));
                } else {
                    Ok(Command::Echo(split_value[1..].join(" ")))
                }
            }
            "ls" => Ok(Command::Ls),
            "pwd" => Ok(Command::Pwd),
            "cd" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("Cd command requires an argument"));
                } else {
                    Ok(Command::Cd(split_value[1..].join(" ")))
                }
            }
            "touch" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("Touch command requires an argument"));
                } else {
                    Ok(Command::Touch(split_value[1..].join(" ")))
                }
            }
            "rm" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("Rm command requires an argument"));
                } else {
                    Ok(Command::Rm(split_value[1..].join(" ")))
                }
            }
            "cat" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("Cat command requires an argument"));
                } else {
                    Ok(Command::Cat(split_value[1..].join(" ")))
                }
            }
            "mkdir" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("mkdir command requires an argument"));
                } else {
                    Ok(Command::Mkdir(split_value[1..].join(" ")))
                }
            }
            "rmdir" => {
                if split_value.len() < 2 {
                    return Err(anyhow!("rmdir command requires an argument"));
                } else {
                    Ok(Command::Rmdir(split_value[1..].join(" ")))
                }
            }
            program => Ok(Command::Exec {
                program: program.to_string(),
                args: split_value[1..].iter().map(|s| s.to_string()).collect(),
            }),
        }
    }
}
