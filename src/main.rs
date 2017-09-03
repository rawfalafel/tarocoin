use std::thread;
use std::io::{self, BufRead};
use std::process;
use std::sync::mpsc;

enum Command {
    Kill
}

fn stdin_parser(tx: mpsc::Sender<Command>) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.eq("kill") {
            tx.send(Command::Kill).unwrap();
        }
    }
}

fn main() {
    // Create channel
    let (tx,rx) = mpsc::channel();

    // Start background thread to parse stdin
    println!("Starting thread");
    thread::spawn(move || {
        stdin_parser(tx);
    });

    // Loop until stdin reads kill command
    loop {
        match rx.recv().unwrap() {
            Command::Kill => {
                println!("Program killed");
                process::exit(0);
            }
        }
    }
}
