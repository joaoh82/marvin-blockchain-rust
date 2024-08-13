use clap::{arg, Arg, Command};

pub fn start_cli() -> Command {
    Command::new("marvinctl")
        .about("marvinclt is a CLI tool for the Marvin Blockchain")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .version("0.1.0")
        .subcommand(
            Command::new("test")
                .about("Test the CLI")
                .arg(arg!(-t --test ... "Test mode")),
        )
        .subcommand(
            Command::new("address")
                .about("Manage addresses")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(
                    Command::new("create").about("Create a new address").arg(
                        Arg::new("mnemonic")
                            .short('m')
                            .long("mnemonic")
                            .required(true),
                    ),
                ),
        )
}
