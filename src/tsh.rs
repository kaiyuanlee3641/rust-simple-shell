use std::process::Command;
use std::process::Stdio;
use std::fs::File;
use std::io::Write;

// SimpleShell struct that stores parsed commands and arguments
pub struct SimpleShell {
    firstcmd: String,
    cmd: CommandType
}

// CommandType enum for differentiating various commands
enum CommandType {
    Empty,
    ShellCommand(Vec<String>),
    PipeCommand(Vec<String>, Vec<String>),
    RedirCommand(Vec<String>, Vec<String>, String)
}

impl SimpleShell {
    pub fn new() -> Self {
        Self{
            firstcmd: String::from(""),
            cmd: CommandType::Empty
        }
    }
    fn reset_command(&mut self) -> () {
        self.firstcmd = String::from("");
        self.cmd = CommandType::Empty;
    }
    pub fn parse_command(&mut self, cmd: String) ->  () {
        self.reset_command();

        // Split commands into separate Vec, delimited by pipe/redir characters
        let tokens: Vec<String> = cmd.split([' ','\r','\n']).map(String::from).filter(|s| s != "").collect();
        let tok = tokens.clone();
        let tok_split: Vec<Vec<String> > = tok.split(|s| s == "|" || is_redir_character(s)).fold(Vec::new(), |mut acc, s| {
            acc.push(s.to_vec());
            acc
        });

        // Store the first string of the first command
        self.firstcmd = tok_split[0][0].clone();

        // Store the command based on command type
        if is_pipe_command(&tokens) {
            let (c1, c2) = (tok_split[0].clone(), tok_split[1].clone());
            self.cmd = CommandType::PipeCommand(c1, c2);
        }
        else if let Some(redir_char) = is_redir_command(&tokens) {
            let (c1, c2) = (tok_split[0].clone(), tok_split[1].clone());
            self.cmd = CommandType::RedirCommand(c1, c2, redir_char);
        }
        else {
            self.cmd = CommandType::ShellCommand(tokens);
        }
    }
    pub fn exec_command(&mut self) -> Result<(), std::io::Error> {
        // Execute command based on type
        match &self.cmd {
            // If normal shell command, execute like normal
            CommandType::ShellCommand(c1_tokens) => {
                let mut c1_tok = c1_tokens.clone();
                let mut c1 = Command::new(c1_tok.remove(0));
                c1.args(c1_tok);
                match c1.status() {
                    Ok(_c) => {
                        // println!("{}", c);
                    },
                    Err(e) => {
                        println!("Failed to execute process - {}", e);
                        return Err(e);
                    }
                }
            },
            // If pipe command, take the stdout and pipe it into the second command
            CommandType::PipeCommand(c1_tokens, c2_tokens) => {
                let (mut c1_tok, mut c2_tok) = (c1_tokens.clone(), c2_tokens.clone());
                let c1 = Command::new(c1_tok.remove(0))
                            .args(c1_tok)
                            .stdout(Stdio::piped())
                            .spawn()
                            .unwrap();
                let mut c2 = Command::new(c2_tok.remove(0));
                c2.args(c2_tok);
                c2.stdin(Stdio::from(c1.stdout.unwrap())).stdout(Stdio::inherit());
                match c2.status() {
                    Ok(_c) => {
                        // println!("{}", c);
                    },
                    Err(e) => {
                        println!("Failed to execute process - {}", e);
                        return Err(e);
                    }
                }
            },
            // If redir command, check which character
            CommandType::RedirCommand(c1_tokens, c2_tokens, redir) => {
                let (mut c1_tok, mut c2_tok) = (c1_tokens.clone(), c2_tokens.clone());
                match redir.as_str() {
                    ">" => {
                        // Create a file
                        let file = match File::create(c2_tok.remove(0)) {
                            Ok(fd) => fd,
                            Err(e)  => {
                                println!("Failed to create file - {}", e);
                                return Err(e);
                            }
                        };

                        // Write command output to file
                        let mut c1 = Command::new(c1_tok.remove(0));
                        c1.args(c1_tok).stdout(Stdio::from(file));
                        match c1.status() {
                            Ok(_c) => {
                                // println!("{}", c);
                            },
                            Err(e) => {
                                println!("Failed to execute process - {}", e);
                                return Err(e);
                            }
                        }
                    },
                    ">>" => {
                        // Create a file if it doesn't exist, otherwise open it in append mode
                        let file_name = c2_tok.remove(0);
                        let file = match File::options().append(true).open(&file_name) {
                            Ok(fd) => fd,
                            Err(_e)  => {
                                match File::create(&file_name) {Ok(fd) => fd, Err(e) => {
                                    println!("Failed to execute process - {}", e);
                                    return Err(e);
                                }}
                            }
                        };

                        // Write command output to file
                        let mut c1 = Command::new(c1_tok.remove(0));
                        c1.args(c1_tok).stdout(Stdio::from(file));
                        match c1.status() {
                            Ok(_c) => {
                                // println!("{}", c);
                            },
                            Err(e) => {
                                println!("Failed to execute process - {}", e);
                                return Err(e);
                            }
                        } 
                    },
                    "<" => {
                        // Open file if it exists
                        let file = match File::open(c2_tok.remove(0)) {
                            Ok(fd) => fd,
                            Err(e)  => {
                                println!("Failed to open file - {}", e);
                                return Err(e);
                            }
                        };
                        // Read from file as input to command
                        let mut c1 = Command::new(c1_tok.remove(0));
                        c1.args(c1_tok).stdin(Stdio::from(file));
                        match c1.status() {
                            Ok(_c) => {
                                // println!("{}", c);
                            },
                            Err(e) => {
                                println!("Failed to execute process - {}", e);
                                return Err(e);
                            }
                        }
                    },
                    "<<" => {
                        // Read exit delimiter
                        let exit_delimiter = c2_tok.remove(0);
                                
                        print!("> ");
                        std::io::stdout().flush()?;

                        // Read input from command stdin
                        let mut input = String::new();
                        let mut here_doc = String::new();
                        while match std::io::stdin().read_line(&mut input) {
                            Ok(_) => true,
                            Err(e) => {
                                println!("error: {e}");
                                false
                            }
                        } {
                            if !input.contains(&exit_delimiter) {
                                // Concatenate input to accumulated string
                                here_doc.push_str(&input);
                                input = String::new();

                                print!("> ");
                                std::io::stdout().flush()?;
                            }
                            else {
                                break;
                            }
                        }
                        // Spawn command thread and use it as stdin
                        let mut c1_child = Command::new(c1_tok.remove(0))
                                                .args(c1_tok)
                                                .stdin(Stdio::piped())
                                                .spawn()
                                                .unwrap();

                        let c1_stdin = c1_child.stdin.as_mut().unwrap();

                        // Write accumulated string to file
                        c1_stdin.write_all(here_doc.as_bytes())?;
                        match c1_child.wait() {
                            Ok(_c) => {
                                // println!("{}", c);
                            },
                            Err(e) => {
                                println!("Failed to execute process - {}", e);
                                return Err(e);
                            }
                        }
                    },
                    _ => {}
                }
            },
            CommandType::Empty => {}
        };

        self.reset_command();
        Ok::<(), std::io::Error>(())
    }
    pub fn is_quit(&self) -> bool {
        if self.firstcmd == "quit" {
            return true;
        }
        false
    }
}

fn is_pipe_command(tokens: &Vec<String>) -> bool {
    let pipe_character = String::from("|");
    tokens.contains(&pipe_character)
}
fn is_redir_command(tokens: &Vec<String>) -> Option<String> {
    let redir_characters: Vec<String> = vec![">", ">>", "<", "<<"].into_iter().map(String::from).collect();
    for c in redir_characters{
        if tokens.contains(&c){
            return Some(c);
        }
    }
    None
}
fn is_redir_character(c: &String) -> bool {
    let redir_characters: Vec<String> = vec![">", ">>", "<", "<<"].into_iter().map(String::from).collect();
    for ch in redir_characters{
        if ch == *c {
            return true;
        }
    }
    false
}