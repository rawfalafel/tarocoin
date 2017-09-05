use std::io::{self, BufRead};
use std::sync::mpsc;

use commands;

pub fn stdin_parser(tx: mpsc::Sender<commands::Command>) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();

        let command = match words.next() {
            Some(str) => str,
            None => {
                println!("Command not provided");
                continue;
            }
        };

        if command.eq("kill") {
            tx.send(commands::Command::Kill).unwrap();
        } else if command.eq("key") {
            let id = match words.next() {
                Some(id) => id,
                None => {
                    println!("Must provide id when creating key");
                    continue;
                }
            };

            tx.send(commands::Command::CreateKey(id.to_string())).unwrap();
        } else if command.eq("tx") {
            tx.send(commands::Command::CreateTransaction).unwrap();
        }
    }
}

