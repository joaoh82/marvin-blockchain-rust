mod cli;
mod crypto;

use cli::start_cli;
use crypto::keys::get_private_key_from_mnemonic;

fn main() {
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
