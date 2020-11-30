pub mod clipboard;
pub mod decrypt;
pub mod encrypt;
pub mod notification;
pub mod qrcode;
pub mod sync;
pub mod tree;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "pass for the windows platform")]
struct CliArgs {
    password: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "ls")]
    List,
    Find,
    Show,
    Insert,
    Edit,
    Generate,
    #[structopt(name = "rm")]
    Remove,
    #[structopt(name = "cp")]
    Copy,
    Git,
}

fn main() {
    let cli_args = CliArgs::from_args();
    println!("{:?}", cli_args);
}
