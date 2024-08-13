mod cli;
mod crypto;

use cli::start_cli;
use crypto::keys::{self, get_private_key_from_mnemonic};

fn main() {
    let matches = start_cli().get_matches();

    match matches.subcommand() {
        Some(("address", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", create_matches)) => {
                println!("address create");
                let mnemonic = create_matches.get_one::<String>("mnemonic").unwrap();
                println!("mnemonic: {}", mnemonic);
                let private_key = get_private_key_from_mnemonic(mnemonic);
                let public_key = private_key.public_key();
                let address = public_key.address();
                println!("address: {}", address.string());
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
