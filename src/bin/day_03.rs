use std::fs::File;
use std::io::{Read, Write};
use std::ops::DerefMut;
use memmap2::{MmapOptions};

static mut COUNT: u8 = 0;

fn main() {
    setup_persistent("mmap.persistent");
}

fn setup_persistent(file_path: &str) {
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path).expect("failed to open or create the file");

    file.set_len(1).unwrap();

    let mut data = [0];
    file.read(&mut data).unwrap();

    data[0] += 1;
    unsafe {COUNT = data[0]};

    file.write_all(data.as_slice()).unwrap();

    // wrap around mmap(.., PROT_READ | PROT_WRITE, MAP_SHARED, 0)
    let mut mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .expect("failed to mmap")
            .make_mut().expect("failed to make mut")
    };

    match mmap.deref_mut().write_all(data.as_slice()) {
        Ok(r) => r,
        Err(err) => panic!("failed to write: {:?}", err),
    }

    println!("count = {:?}", unsafe{COUNT});
}