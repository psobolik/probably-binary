/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-22
 */
use probably_binary::{entry_type, EntryType, FileType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("specify a file on the command line")
    }
    let path = std::path::Path::new(&args[1]);
    match entry_type(path) {
        Ok(entry_type) => match entry_type {
            EntryType::Directory => {
                println!("{path:?} is a directory");
            }
            EntryType::File(file_type) => match file_type {
                FileType::Text => println!("{path:?} is probably text"),
                FileType::Binary => println!("{path:?} is probably binary"),
            },
        },
        Err(error) => {
            dbg!(error);
        }
    };
}
