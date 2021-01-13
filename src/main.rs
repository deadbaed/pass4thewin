pub mod cmd;
mod constants;
mod decrypt;
mod encrypt;
mod password;
pub mod settings;
mod sync;
mod tree;

use crate::settings::Settings;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
/// pass for the windows platform
struct CliArgs {
    // List of subcommands
    #[structopt(subcommand)]
    cmd: Option<Command>,

    // Replica of subcommand `show`
    password: Option<String>,
    /// Display only specific line of password file
    #[structopt(long = "line")]
    line: Option<usize>,
    /// Copy password to clipboard
    #[structopt(short = "c", long = "clipboard")]
    clipboard: bool,
    /// Display password as a qr-code
    #[structopt(short = "q", long = "qrcode")]
    qr_code: bool,
}

#[derive(StructOpt)]
enum Command {
    /// Initiate password store
    Init {
        /// Location of PGP key to use
        pgp_key: PathBuf,
        #[structopt(short = "p", long = "path")]
        /// Location of password store
        path: Option<PathBuf>,
    },
    #[structopt(name = "ls")]
    /// List passwords
    List { password: Option<String> },
    /// Find passwords matching parameter
    Find { search: String },
    /// Show existing password
    Show {
        password: Option<String>,
        /// Display only specific line of password file
        #[structopt(long = "line")]
        line: Option<usize>,
        /// Copy password to clipboard
        #[structopt(short = "c", long = "clipboard")]
        clipboard: bool,
        /// Display password as a qr-code
        #[structopt(short = "q", long = "qrcode")]
        qr_code: bool,
    },
    /// Insert new password
    Insert {
        password: String,
        /// Insert multiple lines in password file
        #[structopt(short = "m", long = "multiline")]
        multi_line: bool,
        /// Display password to terminal after inserting it
        #[structopt(short = "e", long = "echo")]
        echo: bool,
        /// Force insertion of password
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
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
    Remove {
        path: String,
        /// Recursively delete contents of path
        #[structopt(short = "r", long = "recursive")]
        recursive: bool,
        /// Force deletion of path
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
    #[structopt(name = "mv")]
    /// Move or rename existing password or directory
    Move {
        old_path: String,
        new_path: String,
        /// Force move of path
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
    #[structopt(name = "cp")]
    /// Copy existing password or directory
    Copy {
        old_path: String,
        new_path: String,
        /// Force copy of path
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
    /// If the password store is a git repository, execute some git commands
    Git(GitCommands),
    /// Dump current settings
    Settings,
}

#[derive(StructOpt)]
enum GitCommands {
    /// Initiate git repository
    Init,
}

fn main() -> anyhow::Result<()> {
    let cli_args = CliArgs::from_args();

    let mut settings = Settings::try_load();

    // If a password is passed, pass it to show command
    if let Some(password) = cli_args.password {
        cmd::show(
            Some(password),
            cli_args.line,
            cli_args.clipboard,
            cli_args.qr_code,
            &settings,
        )?
    }

    // Run command
    match cli_args.cmd {
        Some(cmd) => match cmd {
            Command::Init { pgp_key, path } => cmd::init(&pgp_key, path, &mut settings)?,
            Command::List { password } => cmd::list(password, &settings)?,
            Command::Find { search } => cmd::find(&search),
            Command::Show {
                password,
                line,
                clipboard,
                qr_code,
            } => cmd::show(password, line, clipboard, qr_code, &settings)?,
            Command::Insert {
                password,
                multi_line,
                echo,
                force,
            } => cmd::insert(&password, multi_line, echo, force, &settings)?,
            Command::Edit { password } => cmd::edit(&password),
            Command::Generate { password, length } => cmd::generate(&password, length),
            Command::Remove {
                path,
                recursive,
                force,
            } => cmd::remove(&path, recursive, force),
            Command::Move {
                old_path,
                new_path,
                force,
            } => cmd::m0ve(&old_path, &new_path, force),
            Command::Copy {
                old_path,
                new_path,
                force,
            } => cmd::copy(&old_path, &new_path, force),
            Command::Git(git_cmd) => match git_cmd {
                GitCommands::Init => cmd::git::init(settings.get_password_store_path()?)?,
            },
            Command::Settings => settings.dump()?,
        },
        None => cmd::list(None, &settings)?,
    }

    Ok(())
}
