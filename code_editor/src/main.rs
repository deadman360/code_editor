use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("Binario: {0:00b} ASCII: {0:#03}" ,b );
        } else {
            println!("Binario: {0:00b} ASCII: {0:#03} Caracter: {1}\r", b, c);
        }
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
