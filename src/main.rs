#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    nonce: u32,
    timestamp: f32,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    nonce: u32,
    gas_price: u32,
    gas_limit: u32,
    to: TransactionTo,
    value: u32,
    // TODO: This should actually be v, r, s
    // Yellow Paper: Page 4 and Appendix F
    sender: u32,
}

#[derive(Serialize, Deserialize, Debug)]
enum TransactionTo {
    Address(String),
    Empty,
}

fn listen_transactions(host: &str, port: &str) {
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    for stream in listener.incoming() {
        let mut buffer = String::new();
        stream.unwrap().read_to_string(&mut buffer).unwrap();
        let transaction: Transaction = serde_json::from_str(&buffer).unwrap();
        spread_transaction(&transaction);
        process_transaction(transaction);
    }
}

fn spread_transaction(transaction: &Transaction) {}

fn process_transaction(transaction: Transaction) {
    println!("Processing transaction {:?}", transaction);
}

fn main() {
    let chain: Vec<Block>;
    let transactions_thread = thread::spawn(|| listen_transactions("0.0.0.0", "8000"));
    transactions_thread.join().unwrap();
}
