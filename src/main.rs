extern crate ansi_term;
extern crate rand;

use ansi_term::Colour;
use rand::Rng;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, stderr, Write};

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let entropy = rand::thread_rng().gen_range(0, 255);

    match read(args, entropy) {
        Err(e) => {
            writeln!(stderr(), "Invalid input. {}", e).unwrap();
                std::process::exit(1);
        },
        _ => {}
    }
}

fn read(args: Vec<String>, entropy: i32) -> std::io::Result<()> {
    match args.len() {
        1 => read_file(&String::from(&args[0]), entropy),
        _ => read_stdin(entropy)
    }
}

fn rainbow(seed: f64) -> ansi_term::Colour {
    let freq = 1.13;
    let factor = seed / 16.0;

    let red = (freq * factor as f64).sin().mul_add(127.0, 128.0) as u8;
    let green = (freq * factor + 2.0 * std::f64::consts::PI / 3.0 as f64).sin().mul_add(127.0, 128.0) as u8;
    let blue = ( freq * factor + 4.0 * std::f64::consts::PI / 3.0 as f64).sin().mul_add(127.0, 128.0) as u8;

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

fn read_file(file_name: &str, entropy: i32) -> std::io::Result<()> {
    let mut index = entropy;
    let f = File::open(file_name)?;
    for line in BufReader::new(f).lines() {
        let mut stuff = line.unwrap();
        stuff.push('\n');
        cat(stuff.chars().collect(), index);
        index += 1;
    }
    Ok(())
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
