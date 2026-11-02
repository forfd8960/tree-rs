use std::{
    fs::{self},
    io,
    path::PathBuf,
};

use crate::ScanResult;

pub fn scan_dir(root: &str) -> ScanResult<()> {
    println!("scanning dir: {:?}", root);
    let meta = fs::metadata(root)?;
    if meta.is_file() {
        println!("{}", root);
        return Ok(());
    }

    let p = PathBuf::from(root);
    let (dir_num, file_num) = scan_inner(&p, 0)?;
    println!("\n{} directories, {} files", dir_num, file_num);

    Ok(())
}

fn scan_inner(parent: &PathBuf, level: u16) -> ScanResult<(u16, u16)> {
    let name = &parent.to_string_lossy();
    let meta = fs::metadata(parent)?;

    let mut dir_num = 0;
    let mut file_num = 0;

    if meta.is_dir() {
        if level != 0 {
            dir_num += 1;
        }

        let mut entries = fs::read_dir(parent)?.collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            // println!("entry: {:?}, {:?}", ent, ent.path());
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            if name == ".git" {
                continue;
            }

            print_file(&name, level);

            if entry.file_type()?.is_dir() {
                let p = PathBuf::from(parent).join(entry.path());
                let (sub_dir_num, sub_file_num) = scan_inner(&p, level + 1)?;
                dir_num += sub_dir_num;
                file_num += sub_file_num;
            } else {
                file_num += 1;
            }
        }
    } else {
        print_file(name, level);
        file_num += 1;
    }

    Ok((dir_num, file_num))
}

fn print_file(name: &str, level: u16) {
    match level {
        0 => println!("|--{}", name),
        _ => {
            let len = "|--".len();
            let space = " ".repeat(len).repeat(level as usize);
            println!("{}|--{}", space, name)
        }
    }
}
