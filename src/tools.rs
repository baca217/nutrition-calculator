use std::io::{self,stdin, stdout, Read, Write};

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).unwrap();
    return buffer;
}
