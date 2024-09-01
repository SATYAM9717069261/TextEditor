/**
* By default terminal start in Canionical mode -> In this mode keyboard input diretly go to running Program
* start typing, all words go to b , and when ENTER Press then
* go to c
*   Go to Raw mode  add  { cargo add crossterm }
*/
use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        if c.is_control() {
            println!("{}", b);
        } else {
            println!("Char => {} , Binary =>{}", c, b);
        }

        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
