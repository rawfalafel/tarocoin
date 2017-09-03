use std::io::{self, BufRead};
use std::sync::mpsc;

use command;

pub fn stdin_parser(tx: mpsc::Sender<command::Command>) {
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
            tx.send(command::Command::Kill).unwrap();
        } else if command.eq("create_key") {
            let id = match words.next() {
                Some(id) => id,
                None => {
                    println!("Must provided id when creatnig key");
                    continue;
                }
            };

            tx.send(command::Command::CreateKey(id.to_string())).unwrap();
        }
    }
}

