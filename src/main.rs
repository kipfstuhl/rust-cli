extern crate rand;

use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let output = String::from("Hello, world!");
    println!("{}", output);
    print_string_with_delay(&output, time::Duration::from_millis(100)).expect("panicking");
    print_slice_with_delay(&output[0..5], time::Duration::from_millis(100)).expect("panicking");
    print_string_with_random_delay(&output, time::Duration::from_millis(200), None)
        .expect("panicking");
    print_string_with_random_delay(&output, time::Duration::from_millis(200), Some(50))
        .expect("panicking");
    print_slice_with_random_delay(&output[0..5], time::Duration::from_millis(300), Some(30))
        .expect("panicking");
}

pub fn print_string_with_delay(word: &String, delay: time::Duration) -> io::Result<()> {
    for line in word.lines() {
        for letter in line.chars() {
            let mut string = String::new();
            string.push(letter);
            io::stdout().write(string.as_bytes())?;
            io::stdout().flush()?;
            thread::sleep(delay);
        }
        io::stdout().write(b"\n")?;
        io::stdout().flush()?;
    }

    Ok(())
}

pub fn print_slice_with_delay(word: &str, delay: time::Duration) -> io::Result<()> {
    print_string_with_delay(&String::from(word), delay)
}

pub fn print_string_with_random_delay(
    word: &String,
    delay: time::Duration,
    fraction: Option<u32>,
) -> io::Result<()> {
    let upper_bound = delay.subsec_millis() * (100 + fraction.unwrap_or(10)) / 100;
    let lower_bound = delay.subsec_millis() * (100 - fraction.unwrap_or(10)) / 100;

    let mut rng = thread_rng();
    for line in word.lines() {
        for letter in line.chars() {
            let mut string = String::new();
            string.push(letter);
            io::stdout().write(string.as_bytes())?;
            io::stdout().flush()?;

            let rand_delay =
                time::Duration::from_millis(rng.gen_range(lower_bound as u64, upper_bound as u64));
            thread::sleep(rand_delay);
        }
        io::stdout().write(b"\n")?;
        io::stdout().flush()?;
    }
    Ok(())
}

pub fn print_slice_with_random_delay(
    word: &str,
    delay: time::Duration,
    fraction: Option<u32>,
) -> io::Result<()> {
    print_string_with_random_delay(&String::from(word), delay, fraction)
}
