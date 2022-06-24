use std::path::{Path, PathBuf};

use clap::{App, Arg};

fn main() {
    // 解析命令行参数
    let matches = App::new("tree")
        .version("0.1.0")
        .author("Tan <liangzhong.tan@outlook.com>")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .help("文件路径"))
        .arg(Arg::new("level")
            .short('l')
            .long("level")
            .help("递归文件夹层数"))
        .arg(Arg::new("all")
            .short('a')
            .long("all")
            .help("显示所有文件"))
        .get_matches();

    let p = matches.value_of("path").unwrap_or(".");
    let l = matches.value_of("level").unwrap_or("-1");
    let is_all;

    match matches.value_of("all") {
        None => { is_all = false }
        Some(_) => { is_all = true }
    }

    let mut max_level = 1;
    match l.parse() {
        Ok(n) => max_level = n,
        Err(e) => {
            eprintln!("level [{}], {}", l, e);
            return;
        }
    }
    recursion(Path::new(p).to_path_buf(), 1, max_level, is_all);
}

/// recursion Tree
fn recursion(path: PathBuf, level: i32, max_level: i32, is_all: bool) {
    match path.read_dir() {
        Ok(dirs) => {
            for it in dirs {
                let it = it.unwrap();
                if !is_all {
                    if it.file_name().to_str().unwrap().starts_with(".") {
                        continue;
                    }
                }
                let mut prefix = String::new();
                for l in 0..level {
                    if l == (level - 1) {
                        prefix.push_str("├─");
                    } else {
                        prefix.push_str("│ ");
                    }
                }
                println!("{:}{:}", prefix, it.file_name().to_str().unwrap());
                if it.path().is_dir() && (level < max_level || max_level == -1) { recursion(it.path(), level + 1, max_level, is_all) }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}