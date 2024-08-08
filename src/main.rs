mod cli;

use cli::start_cli;

fn main() {
    let matches = start_cli().get_matches();

    match matches.subcommand() {
        Some(("test", sub_matches)) => {
            println!("testing {}", "test");
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}
