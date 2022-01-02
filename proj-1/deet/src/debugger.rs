use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::{Inferior, Status};
use nix::sys::signal;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: HashMap<usize, Option<Breakpoint>>, // mem_addr -> written byte, orig_byte
}

#[derive(Clone, Debug)]
struct Breakpoint {
    cur_byte: usize,
    orig_byte: u8,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("PWD").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if let Some(inferior) = Inferior::new(&self.target, &args) {
                        self.clear_inferior();
                        // Create the inferior
                        self.inferior = Some(inferior);
                        for addr in self.breakpoints.clone().keys() {
                            let bp = self.insert_breakpoint(*addr);
                            self.breakpoints.insert(*addr, bp);
                        }
                        self.go();
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    self.clear_inferior();
                    return;
                }
                DebuggerCommand::Continue => match self.inferior {
                    None => {
                        println!("Run the process first!");
                    }
                    Some(_) => {
                        self.go();
                    }
                },
                DebuggerCommand::Backtrace => match &self.inferior {
                    Some(process) => {
                        process.print_backtrace(&self.debug_data).unwrap();
                    }
                    None => {}
                },
                DebuggerCommand::Breakpoint(addr) => {
                    let num_addr = parse_address(&addr).unwrap();
                    println!("Set breakpoint {} at {}", self.breakpoints.len(), num_addr);
                    let bp = self.insert_breakpoint(num_addr);
                    self.breakpoints.insert(num_addr, bp);
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }

    fn go(&mut self) {
        let process = self.inferior.as_mut().unwrap();
        loop {
            match process.go_on(None) {
                Err(_) => {
                    format!("Inferior (pid:{}) couldn't continue", process.pid()).as_str();
                    break;
                }
                Ok(Status::Continued()) => match process.wait(None) {
                    Err(_) => {
                        format!("Inferior (pid:{}) couldn't wait", process.pid()).as_str();
                        break;
                    }
                    Ok(Status::Stopped(sig, rip)) => {
                        println!("child stopped (signal: {}, rip: {})", sig, rip);
                        if let Some(line) = self.debug_data.get_line_from_addr(rip) {
                            println!("Stopped at {}:{}", line.file, line.number);
                        }
                        if sig == signal::Signal::SIGTRAP {
                            match self.restore_breakpoint(rip - 1) {
                                None => {}
                                bp => {
                                    self.breakpoints.insert(rip - 1, bp);
                                    dbg!(&self.breakpoints);
                                    // TODO: rewind instruction pointer
                                }
                            }
                        }
                        break;
                    }
                    Ok(Status::Exited(exit_code)) => {
                        println!("child exited (status {})", exit_code);
                        self.inferior = None;
                        break;
                    }
                    Ok(Status::Killed(exit_code)) => {
                        println!("child killed (status {})", exit_code);
                        self.inferior = None;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn clear_inferior(&mut self) {
        match &self.inferior {
            None => {}
            Some(process) => match process.kill() {
                Err(e) => {
                    panic!("{}", e);
                }
                Ok(Status::Killed(exit_code)) => {
                    println!(
                        "killed running inferior (pid: {}, code: {})",
                        process.pid(),
                        exit_code,
                    );
                    self.inferior = None;
                }
                _ => {}
            },
        }
    }

    fn insert_breakpoint(&mut self, mem_addr: usize) -> Option<Breakpoint> {
        let cur_byte = parse_address("0xcc")?;
        if let Some(inferior) = self.inferior.as_mut() {
            let orig_byte = inferior.write_byte(mem_addr, cur_byte as u8).unwrap();
            dbg!(format!(
                "insert {:?}: {:?} => {:?}",
                mem_addr, cur_byte, orig_byte
            ));
            return Some(Breakpoint {
                cur_byte,
                orig_byte,
            });
        }
        None
    }

    fn restore_breakpoint(&mut self, mem_addr: usize) -> Option<Breakpoint> {
        match self.breakpoints.clone().get(&mem_addr) {
            Some(Some(bp)) => {
                if let Some(inferior) = self.inferior.as_mut() {
                    let orig_byte = inferior.write_byte(mem_addr, bp.orig_byte as u8).unwrap();
                    dbg!(format!(
                        "restore {:?}: {:?} => {:?}",
                        mem_addr, bp.orig_byte, orig_byte
                    ));
                    return Some(Breakpoint {
                        cur_byte: bp.orig_byte as usize,
                        orig_byte,
                    });
                }
                None
            }
            _ => None,
        }
    }
}

pub fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}
