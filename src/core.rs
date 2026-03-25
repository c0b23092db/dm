use std::fs::{read_dir,rename};
use std::path::PathBuf;

/// ディレクトリ内のファイルを一覧表示する関数
pub fn show_files_in_directory(directory_path:PathBuf) {
    let files = get_vecfiles_modified(directory_path,true);
    let width:usize = files.len().to_string().len();
    for (index,file) in files.iter().enumerate() {
        match file.file_name() {
            Some(name) => println!("{:>width$}. {}",index + 1,name.to_string_lossy()),
            None => eprintln!("Failed. {:?}", file),
        }
    }
}

/// ファイルを移動する関数
pub fn move_files(from: PathBuf,to: PathBuf, count: i32) {
    let files = get_vecfiles_modified(from,0 <= count);
    if files.is_empty() {
        return;
    }
    let move_count = if count == 0 { files.len() } else { count.unsigned_abs() as usize };
    for file in files.into_iter().take(move_count) {
        rename(&file, to.join(file.file_name().expect("Failed to File name"))).unwrap_or_else(|err| {
            eprintln!("Failed to move {:?} : {}", file, err);
        });
    }
}

/// 指定した番号のファイルを移動する関数
pub fn move_specified_file(from: PathBuf, to: PathBuf, index: usize) {
    let files = get_vecfiles_modified(from, true);
    if files.is_empty() {
        return;
    }
    if index == 0 || files.len() < index {
        eprintln!("Invalid index: {}. Valid range: 1-{}", index, files.len());
        return;
    }
    let file = &files[index - 1];
    rename(file, to.join(file.file_name().expect("Failed to File name"))).unwrap_or_else(|err| {
        eprintln!("Failed to move {:?} : {}", file, err);
    });
}

/// ファイルを削除する関数
pub fn remove_files(from: PathBuf, count: i32) {
    let files = get_vecfiles_modified(from,0 <= count);
    if files.is_empty() {
        return;
    }
    let remove_count = if count == 0 { files.len() } else { count.unsigned_abs() as usize };
    match trash::delete_all(files.into_iter().take(remove_count)) {
        Ok(_) => (),
        Err(err) => eprintln!("Failed to remove files: {}", err),
    };
}

/// 指定した番号のファイルを削除する関数
pub fn remove_specified_file(from: PathBuf, index: usize) {
    let files = get_vecfiles_modified(from, true);
    if files.is_empty() {
        return;
    }
    if index == 0 || files.len() < index {
        eprintln!("Invalid index: {}. Valid range: 1-{}", index, files.len());
        return;
    }
    let file = &files[index - 1];
    match trash::delete(file) {
        Ok(_) => (),
        Err(err) => eprintln!("Failed to remove {:?} : {}", file, err),
    };
}

// TODO: 指定したファイルをダウンロードディレクトリに移動する関数
pub fn back_specified_file(from: PathBuf, to: PathBuf, files: Vec<String>) {
    for file in files {
        let file_path = from.join(&file);
        if !file_path.exists() {
            eprintln!("File not found: {}", file);
            continue;
        }
        rename(&file_path, to.join(file)).unwrap_or_else(|err| {
            eprintln!("Failed to move {:?} : {}", file_path, err);
        });
    }
}

/// ファイルを更新順に取得する関数
fn get_vecfiles_modified(directory_path:PathBuf, reverse_modified:bool) -> Vec<PathBuf> {
    let mut files:Vec<PathBuf> = match read_dir(&directory_path) {
        Ok(entries) => entries.filter_map(Result::ok).map(|e| e.path()).collect(),
        Err(_) => {
            eprintln!("Failed to read directory : {}",directory_path.display());
            return Vec::new();
        }
    };
    files.sort_by_key(|path| path.metadata().ok().and_then(|path| path.modified().ok()));
    if reverse_modified { files.reverse() } // 最新のファイルに並び替える
    return files;
}