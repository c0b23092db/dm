mod core;
use core::{show_files_in_directory,move_files,move_specified_file};

use std::env::current_dir;
use clap::Parser;
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
    /// Check files in the download directory
    #[arg(short,long,
        visible_aliases = ["list","dir"],
    )]
    ls:bool,
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
    if args.ls { // `dm -l`のように`-l`オプションが指定された場合の処理 //
        show_files_in_directory(download_path);
    } else if args.specify { // `dm -s 5`のように数値とともに`-s`オプションが指定された場合の処理 //
        if let Some(index) = args.count {
            if 0 < index {
                move_specified_file(download_path, current_dir, index as usize);
            } else {
                eprintln!("Index must be a positive number");
            }
        } else {
            eprintln!("Please specify a file number with -s option");
        }
    } else if args.count.is_none() { // `dm`コマンドのみで実行された場合の処理 //
        move_files(download_path,current_dir,1);
    }  else if args.count == Some(0) { // `dm 0`で指定された場合の処理 //
        move_files(download_path,current_dir,0);
    } else if let Some(count) = args.count { // `dm 5`のように数値が指定された場合の処理 //
        move_files(download_path,current_dir,count);
    }
}
