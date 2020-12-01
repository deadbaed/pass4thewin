pub mod clipboard;
pub mod decrypt;
pub mod encrypt;
pub mod notification;
pub mod qrcode;
pub mod sync;
pub mod tree;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// pass for the windows platform
struct CliArgs {
    password: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Initiate password store
    Init {
        /// Key ID of the PGP key to use
        gpg_id: String,
    },
    #[structopt(name = "ls")]
    /// List passwords
    List { password: Option<String> },
    /// Find passwords matching parameter
    Find { search: String },
    /// Show existing password
    Show { password: String },
    /// Insert new password
    Insert { password: String },
    /// Edit existing password
    Edit { password: String },
    /// Generate new password
    /// Length is xxx if unspecified FIXME: provide actual number
    Generate {
        password: String,
        length: Option<usize>,
    },
    #[structopt(name = "rm")]
    /// Delete existing password or directory
    Remove { old_path: String, new_path: String },
    #[structopt(name = "mv")]
    /// Move or rename existing password or directory
    Move { old_path: String, new_path: String },
    #[structopt(name = "cp")]
    /// Copy existing password or directory
    Copy { old_path: String, new_path: String },
    /// If the password store is a git repository, execute some git commands
    Git(GitCommands),
}

#[derive(Debug, StructOpt)]
enum GitCommands {
    /// Initiate git repository
    Init,
}

fn main() {
    let cli_args = CliArgs::from_args();
    println!("{:?}", cli_args);
}
