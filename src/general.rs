use std::fs;
use std::path::Path;
use directories::UserDirs;

pub fn pull_file_contents(final_file_path : &Path) -> String {
    let user_dirs = UserDirs::new().unwrap();
    let download_path = user_dirs.download_dir().unwrap();
    let full_file_path = download_path.join(final_file_path);
    let contents = fs::read_to_string(full_file_path)
        .expect("Something went wrong reading the file");
    contents
}