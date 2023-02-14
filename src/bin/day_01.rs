use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // fs::read_to_string not used on purpose
    for path in env::args().skip(1).collect::<Vec<String>>() {
        let mut content = [0u8; 0x100];
        let mut file = File::open(path).unwrap();
        while let Ok(n) = file.read(&mut content) {
            if n == 0 {
                break
            } else {
                let mut truncated = content.to_vec();
                truncated.truncate(n);
                print!("{}", String::from_utf8(truncated).unwrap());
            }
        }
        drop(file); // explicit drop show the use of close()
    }
}
