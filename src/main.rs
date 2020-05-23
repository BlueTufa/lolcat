extern crate ansi_term;
extern crate rand;

use ansi_term::Colour;
use rand::Rng;
use std::env::args;
use std::fs;
use std::io::stderr;
use std::io::Write;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let entropy = rand::thread_rng().gen();
    match args.len() {
        1 => {
            match fs::read_to_string(String::from(&args[0])) {
                Ok(f) => {
                    let my_chars: Vec<_> = f.chars().collect();
                    cat(my_chars, entropy);
                }
                Err(e) => {
                    writeln!(stderr(), "Invalid input. {}", e).unwrap();
                    std::process::exit(1);
                }
            };
        }
        _ => match read_stdin(entropy) {
            _ => {}
        },
    }
}

fn rainbow(seed: f64) -> ansi_term::Colour {
    // TODO: freq needs to be variable based on input content size.
    // approaching 0.02 for larger content to 1.2 for multiple pages.
    let freq = 1.13;
    let factor = seed / 16.0;

    let red_work = freq * factor as f64;
    let red = red_work.sin().mul_add(127.0, 128.0) as u8;

    let green_work = freq * factor + 2.0 * std::f64::consts::PI / 3.0 as f64;
    let green = green_work.sin().mul_add(127.0, 128.0) as u8;

    let blue_work = freq * factor + 4.0 * std::f64::consts::PI / 3.0 as f64;
    let blue = blue_work.sin().mul_add(127.0, 128.0) as u8;

    Colour::RGB(red, green, blue)
}

fn cat(chars: Vec<char>, entropy: i32) {
    let mut index = 1;
    for c in chars {
        let seed = f64::from(index + entropy);
        let colour = rainbow(seed);
        let rainbow_char = colour.paint(c.to_string());
        print!("{}", rainbow_char);
        index += 1;
    }
}

fn read_stdin(entropy: i32) -> std::io::Result<()> {
    // every time we recurse, add one offset
    let index = entropy + 1;
    let mut buffer = String::new();
    let bytes = std::io::stdin().read_line(&mut buffer)?;
    if bytes == 0 {
        Ok(())
    } else {
        let my_chars: Vec<_> = buffer.chars().collect();
        cat(my_chars, index);
        read_stdin(index)
    }
}
