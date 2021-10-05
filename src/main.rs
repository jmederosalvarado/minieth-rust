#![allow(dead_code)]

use std::time::SystemTime;

struct Block {
    index: u32,
    timestamp: f32,
    transactions: Vec<Transaction>,
}

impl Block {
    fn new(index: u32) -> Self {
        Self {
            index,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f32(),
            transactions: vec![],
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction)
    }
}

struct Transaction {
    sender: u32,
    recipient: u32,
    amount: u32,
}

fn main() {
    println!("Hello, world!");
}
