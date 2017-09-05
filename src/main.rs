use std::thread;
use std::process;
use std::sync::mpsc;

mod commands;
mod parser;
mod keys;
mod error;

fn main() {
    // Create channel
    let (tx,rx) = mpsc::channel();

    // Start background thread to parse stdin
    thread::spawn(move || {
        parser::stdin_parser(tx);
    });

    // Loop until it reads kill command
    loop {
        match rx.recv().unwrap() {
            commands::Command::Kill => {
                println!("Program killed");
                process::exit(0);
            },
            commands::Command::CreateKey(id) => {
                println!("Create key: {}", id);
                if let Err(err) = keys::generate(id) {
                    println!("Err: {:?}", err);
                }
            },
            commands::Command::CreateTransaction => {
                println!("Create transaction");
            }
        }
    }
}
