mod pb_error;

use pb_error::PBError;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Text,
    Binary,
}

#[derive(Debug, PartialEq)]
pub enum EntryType {
    Directory,
    File(FileType),
    Other,
}

pub fn entry_type(path: &Path) -> Result<EntryType, PBError> {
    let metadata = match path.metadata() {
        Ok(metadata) => metadata,
        Err(error) => return Err(PBError::new(error.to_string())),
    };
    if metadata.is_dir() {
        Ok(EntryType::Directory)
    } else if metadata.is_file() {
        Ok(EntryType::File(guess_file_type(path, metadata.len())?))
    } else {
        Ok(EntryType::Other)
    }
}

pub const fn is_probably_binary(c: &u8) -> bool {
    matches!(c, b'\0'..=b'\x07' | b'\x0e'..=b'\x1f' | b'\x7F')
}

pub fn guess_file_type(path: &Path, len: u64) -> Result<FileType, PBError> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(error) => return Err(PBError::new(error.to_string())),
    };
    let mut bucket = [0; 256];

    // Check the first 256 bytes
    let i = match f.read(&mut bucket) {
        Ok(i) => i,
        Err(error) => return Err(PBError::new(error.to_string())),
    };
    let file_type = if bucket.iter().take(i).any(is_probably_binary) {
        FileType::Binary
    } else {
        FileType::Text
    };
    // If the first part looks to be binary, assume it is
    if file_type == FileType::Binary {
        return Ok(FileType::Binary);
    }
    // If we've checked the entire file, assume we know what it is
    if len <= 256 {
        return Ok(FileType::Text);
    }

    // If the first 256 bytes look like text, check the last 256 bytes as well
    if let Err(error) = f.seek(SeekFrom::End(-256)) {
        return Err(PBError::new(error.to_string()));
    }
    let i = match f.read(&mut bucket) {
        Ok(i) => i,
        Err(error) => return Err(PBError::new(error.to_string())),
    };
    if bucket.iter().take(i).any(is_probably_binary) {
        Ok(FileType::Binary)
    } else {
        Ok(FileType::Text)
    }
}

#[test]
fn directory_is_directory() {
    let path = Path::new(".");
    let entry_type = entry_type(path);
    assert_eq!(entry_type.unwrap(), EntryType::Directory);
}

#[test]
fn text_is_text() {
    let path = Path::new("./TestFiles/lib.rs");
    let entry_type = entry_type(path);
    assert_eq!(entry_type.unwrap(), EntryType::File(FileType::Text));
}

#[test]
fn binary_is_binary() {
    let path = Path::new("./TestFiles/probably_binary.exe");
    let entry_type = entry_type(path);
    assert_eq!(entry_type.unwrap(), EntryType::File(FileType::Binary));
}
