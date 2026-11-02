use memmap2::{Mmap, MmapOptions};
use std::{fs::OpenOptions, path::PathBuf};

pub fn create_mmap(path: &PathBuf, offset: u64, length: u64) -> Mmap{

    let file = OpenOptions::new()
        .read(true)
        .open(path).expect("The file doesnt exist!");

    unsafe {
        MmapOptions::new()
            .offset(offset)
            .len(length as usize)
            .map(&file).expect("Couldnt mmap the file!")
    }
}
