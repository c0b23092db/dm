mod core;
use core::{
    show_files_in_directory,
    move_files,
    move_specified_file,
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
    Remove{
        /// Number of files being removed
        #[arg(value_name = "count")]
        count:Option<i32>,
        /// Remove files at the specified number
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
        Some(Command::Remove { count, specify }) => {
            if specify {
                if let Some(index) = count {
                    if 0 < index {
                        remove_specified_file(download_path, index as usize);
                    } else {
                        eprintln!("Index must be a positive number");
                    }
                } else {
                    eprintln!("Please specify a file number with -s option");
                }
            } else if count.is_none() { // `rm`が実行された場合 //
                remove_files(download_path,1);
            } else if count == Some(0) { // `rm 0`が実行された場合 //
                remove_files(download_path,0);
            } else if let Some(count) = count { // `rm 5`のように数値が指定された場合 //
                remove_files(download_path,count);
            }
        },
        None => {
            if args.specify {
                if let Some(index) = args.count {
                    if 0 < index {
                        move_specified_file(download_path, current_dir, index as usize);
                    } else {
                        eprintln!("Index must be a positive number");
                    }
                } else {
                    eprintln!("Please specify a file number with -s option");
                }
            } else if args.count.is_none() { // `dm`が実行された場合 //
                move_files(download_path,current_dir,1);
            }  else if args.count == Some(0) { // `dm 0`が実行された場合 //
                move_files(download_path,current_dir,0);
            } else if let Some(count) = args.count { // `dm 5`のように数値が指定された場合 //
                move_files(download_path,current_dir,count);
            }
        },
    };
}
