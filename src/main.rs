#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};
use serde_json;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct Block(BlockHeaders, Box<[Transaction]>); // TODO: Add Block's ommers header info: `, Box<[BlockHeaders]>`

#[derive(Serialize, Deserialize, Debug)]
struct BlockHeaders {
    #[serde(with = "SerHex::<StrictPfx>")]
    parent_hash: [u8; 32],
    // #[serde(with = "SerHex::<StrictPfx>")]
    // ommers_hash: [u8; 32],
    #[serde(with = "SerHex::<StrictPfx>")]
    beneficiary: [u8; 32],
    // #[serde(with = "SerHex::<StrictPfx>")]
    // state_root: [u8; 32],
    // #[serde(with = "SerHex::<StrictPfx>")]
    // transactions_root: [u8; 32],
    // #[serde(with = "SerHex::<StrictPfx>")]
    // receipts_root: [u8; 32],
    // #[serde(with = "SerHex::<StrictPfx>")]
    // logs_bloom: [u8; 32],
    difficulty: u32,
    number: u32,
    gas_limit: u32,
    gas_price: u32,
    timestamp: f32,
    // extra_data: [u8; 32],
    mix_hash: [u8; 32],
    nonce: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockTransactions {}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    nonce: u32,
    gas_price: u32,
    gas_limit: u32,
    to: TransactionTo,
    value: u32,
    // TODO: This should actually be v, r, s
    // Yellow Paper: Page 4 and Appendix F
    #[serde(with = "SerHex::<StrictPfx>")]
    sender: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
enum TransactionTo {
    Address(#[serde(with = "SerHex::<StrictPfx>")] [u8; 20]),
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
