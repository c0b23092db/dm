mod core;
use core::{
    show_files_in_directory,
    move_files,
    move_specified_file,
    copy_files,
    copy_specified_file,
    remove_files,
    remove_specified_file,
    back_specified_file,
};

use std::env::current_dir;
use clap::{Parser, Subcommand};
use dirs::download_dir;

#[derive(Parser, Debug)]
#[command(version,about,
    arg_required_else_help = false,
    allow_negative_numbers = true
)]
struct Args {
    /// Number of files being moved
    #[arg(value_name = "count")]
    count:Option<i32>,
    /// Move files at the specified number
    #[arg(short, long)]
    specify:bool,
    /// Subcommands for additional functionalities
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Check files in the download directory
    #[command(alias = "list", alias = "dir")]
    Ls,
    /// Remove files in the download directory
    #[command(
        alias = "rm",
        arg_required_else_help = false,
        allow_negative_numbers = true
    )]
    Remove {
        /// Number of files being removed
        #[arg(value_name = "count")]
        count:Option<i32>,
        /// Remove files at the specified number
        #[arg(short, long)]
        specify:bool,
    },
    /// Copy files in the download directory
    #[command(
        alias = "cp",
        arg_required_else_help = false,
        allow_negative_numbers = true
    )]
    Copy {
        /// Number of files being copied
        #[arg(value_name = "count")]
        count:Option<i32>,
        /// Copy files at the specified number
        #[arg(short, long)]
        specify:bool,
    },
    /// Move files in the download directory
    #[command(alias = "bk")]
    Back {
        /// Files name
        #[arg(value_name = "file")]
        files: Vec<String>,
    },
    /// Get Path of the download directory
    Path,
    /// Open Download Directory
    Open,
}

enum ExecutionMode {
    ByIndex(usize),
    ByCount(i32),
}

fn resolve_execution_mode(specify: bool, count: Option<i32>) -> Result<ExecutionMode, &'static str> {
    if specify {
        match count {
            Some(index) if index > 0 => Ok(ExecutionMode::ByIndex(index as usize)),
            Some(_) => Err("Index must be a positive number"),
            None => Err("Please specify a file number with -s option"),
        }
    } else {
        Ok(ExecutionMode::ByCount(count.unwrap_or(1)))
    }
}

fn main(){
    let Ok(current_dir) = current_dir() else {
        eprintln!("Failed to get path of the Current Directory");
        return;
    };
    let Some(download_path) = download_dir() else {
        eprintln!("Failed to get path of the Downloads Directory");
        return;
    };
    let args = Args::parse();
    match args.command {
        Some(Command::Ls) => show_files_in_directory(download_path),
        Some(Command::Path) => println!("{}", download_path.display()),
        Some(Command::Open) => if let Err(err) = open::that(&download_path) {
            eprintln!("Failed to open download directory: {}", err);
        },
        Some(Command::Back { files }) => {
            if files.is_empty() {
                eprintln!("Please specify file names to move with the back command");
            } else {
                back_specified_file(current_dir, download_path, files);
            }
        },
        Some(Command::Copy { count, specify }) => {
            match resolve_execution_mode(specify, count) {
                Ok(ExecutionMode::ByIndex(index)) => copy_specified_file(download_path, current_dir, index),
                Ok(ExecutionMode::ByCount(count)) => copy_files(download_path, current_dir, count),
                Err(message) => eprintln!("{}", message),
            }
        },
        Some(Command::Remove { count, specify }) => {
            match resolve_execution_mode(specify, count) {
                Ok(ExecutionMode::ByIndex(index)) => remove_specified_file(download_path, index),
                Ok(ExecutionMode::ByCount(count)) => remove_files(download_path, count),
                Err(message) => eprintln!("{}", message),
            }
        },
        None => {
            match resolve_execution_mode(args.specify, args.count) {
                Ok(ExecutionMode::ByIndex(index)) => move_specified_file(download_path, current_dir, index),
                Ok(ExecutionMode::ByCount(count)) => move_files(download_path, current_dir, count),
                Err(message) => eprintln!("{}", message),
            }
        },
    };
}
