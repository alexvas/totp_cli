use clap::Parser;
use totp_cli::{Cli, run};

fn main() {
    let cli = Cli::parse();
    run(cli).expect("выполнилось неуспешно");
}
