mod proto;
mod cli;
mod core;
mod crypto;
mod types;

use std::io::Read;

use cli::start_cli;
use crypto::keys::{self, get_private_key_from_mnemonic};

use prost;
use prost::{Enumeration, Message};

fn main() {
    block_serialization().unwrap();

    let matches = start_cli().get_matches();

    match matches.subcommand() {
        Some(("address", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", _)) => {
                println!("Generating new address...");
                let entropy = crypto::keys::new_entropy();
                let mnemonic = crypto::keys::get_mnemonic_from_entropy(&entropy);
                let private_key = crypto::keys::get_private_key_from_mnemonic(&mnemonic);
                let public_key = private_key.public_key();
                let address = public_key.address();

                println!("mnemonic: {}", mnemonic);
                println!("address: {}", address.to_string());
            }
            Some(("restore", restore_matches)) => {
                let mnemonic = restore_matches.get_one::<String>("mnemonic").unwrap();
                let private_key = get_private_key_from_mnemonic(mnemonic);
                let public_key = private_key.public_key();
                let address = public_key.address();
                println!("address: {}", address.to_string());
            }
            _ => {
                println!("No address subcommand was used");
            }
        },
        _ => {
            println!("No subcommand was used");
        }
    }

    
}


fn block_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("Block Serialization");

    let mnemonic_to = "all wild paddle pride wheat menu task funny sign profit blouse hockey";
    let private_key_to = crypto::keys::get_private_key_from_mnemonic(&mnemonic_to);
    let public_key_to = private_key_to.public_key();
    let address_to = public_key_to.address();
    println!("address to: {}", address_to.to_string());

    let mnemonic_from = "hello wild paddle pride wheat menu task funny sign profit blouse hockey";
    let private_key_from = crypto::keys::get_private_key_from_mnemonic(&mnemonic_from);
    let public_key_from = private_key_from.public_key();
    let address_from = public_key_from.address();
    println!("address from: {}", address_from.to_string());



    // Create an instance of Header
    let header = proto::Header {
        prev_block_hash: [0; 32].to_vec(),
        tx_hash: [0; 32].to_vec(),
        version: 1,
        height: 1,
        timestamp: 1627483623,
        nonce: 12345,
        difficulty: 10,
    };
    let mut buf = Vec::new();
    header.encode(&mut buf)?;
    println!("Encoded Header: {:?}", buf);
    println!("Header hex: {:?}", hex::encode(&buf));

    let tx = proto::Transaction {
        from: public_key_from.to_bytes().to_vec(),
        to: public_key_to.to_bytes().to_vec(),
        value: 1000,
        data: b"Transaction data".to_vec(),
        signature: [0; 64].to_vec(),
        nonce: 123,
        hash: [0; 32].to_vec(),
    };
    
    let public_key_from_bytes = keys::PublicKey::from_bytes(&tx.from);
    println!("public_key from bytes address: {}", public_key_from_bytes.address().to_string());

    // println!("Transaction: {:?}", tx);
    let mut buf = Vec::new();
    tx.encode(&mut buf)?;
    println!("Encoded tx: {:?}", buf);
    println!("tx hex: {:?}", hex::encode(&buf));

    // Create an instance of Block
    let block = proto::Block {
        header: Some(header),
        transactions: vec![tx],
        public_key: public_key_from.to_bytes().to_vec(),
        signature: vec![],
        hash: vec![],
    };

    // Serialize the block to a byte array
    let mut buf = Vec::new();
    block.encode(&mut buf)?;
    println!("Encoded Block: {:?}", buf);
    println!("Block hex: {:?}", hex::encode(&buf));

    // Deserialize the byte array back into a Block
    let decoded_block = proto::Block::decode(&buf[..])?;

    let decoded_header = decoded_block.header.unwrap();
    println!("Decoded Header: {:?}", decoded_header);
    println!("Decoded Header Version: {:?}", decoded_header.version);
    println!("Decoded Header Height: {:?}", decoded_header.height);
    println!("Decoded Header Timestamp: {:?}", decoded_header.timestamp);
    println!("Decoded Header Nonce: {:?}", decoded_header.nonce);
    println!("Decoded Header Difficulty: {:?}", decoded_header.difficulty);
    println!("Decoded Header Previous Block Hash: {:?}", hex::encode(&decoded_header.prev_block_hash));
    println!("Decoded Header Tx Hash: {:?}", hex::encode(&decoded_header.tx_hash));

    Ok(())
}