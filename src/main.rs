use std::env::{args,current_dir};
use std::fs::{read_dir,rename};
use std::io::Error;
use std::process::ExitCode;
use std::path::PathBuf;
use dirs::download_dir;
use crossterm::event::{self,Event};

fn main() -> ExitCode {
    let Some(downloads_path) = download_dir() else {
        eprintln!("Failed to get path of the Downloads Directory");
        return ExitCode::FAILURE;
    };

    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        move_files(downloads_path,1);
        return ExitCode::SUCCESS;
    }
    let input = &args[1];
    if input == "help" {
        println!("Usage: dm [sum | help | dir/ls]");
        println!("    sum     : 移動するファイルの数");
        println!("    help    : ヘルプを表示");
        println!("    dir/ls  : 移動するファイルの一覧を表示");
        return ExitCode::SUCCESS;
    }
    if input == "dir" || input == "ls" {
        show_list_files(downloads_path);
        return ExitCode::SUCCESS;
    }
    if let Ok(num) = input.parse::<i32>() {
        move_files(downloads_path, num);
        return ExitCode::SUCCESS;
    }

    println!("Invalid input : Please enter help, a number or dir/ls");
    return ExitCode::SUCCESS;
}

fn get_vecfiles_modified(directory_path:PathBuf, reverse_modified:bool) -> Vec<PathBuf>{
    let mut files:Vec<PathBuf> = match read_dir(&directory_path) {
        Ok(entries) => entries.filter_map(Result::ok).map(|e| e.path()).collect(),
        Err(_) => {
            eprintln!("Failed to read directory : {}",directory_path.display());
            return vec![];
        }
    };
    files.sort_by_key(|path| path.metadata().ok().and_then(|path| path.modified().ok()));
    if reverse_modified { files.reverse() } // 最新のファイルに並び替える
    return files;
}

fn show_list_files(directory_path:PathBuf) {
    let files = get_vecfiles_modified(directory_path,true);
    let width:usize = files.len().to_string().len();
    for (index,file) in files.iter().enumerate() {
        match file.file_name() {
            Some(name) =>   println!("[{:>width$}] {}",
                            index + 1,name.to_string_lossy(),width = width),
            None => eprintln!("[Warning] Failed - {:?}", file),
        }
    }
    pause().expect("Failed to input Key");
}

fn move_files(directory_path: PathBuf, count: i32) {
    let files = get_vecfiles_modified(directory_path,0 <= count);
    if files.is_empty() {
        println!("移動するファイルがありません。");
        return;
    }
    let current_dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Failed to get Current Directory");
            return;
        }
    };
    let move_count = if count == 0 { files.len() } else { count.unsigned_abs() as usize };
    for file in files.into_iter().take(move_count) {
        if let Err(e) = rename(&file, current_dir.join(file.file_name().unwrap())) {
            eprintln!("Failed to move {:?} : {}", file, e);
        }
    }
    println!("{}個移動しました。",move_count);
}

fn pause() -> Result<(), Error> {
    // println!("続行するには何かキーを押してください...");
    loop {
        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(_) = event::read()? {
                Ok(());
            }
        }
    }
}