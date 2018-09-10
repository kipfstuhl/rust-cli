extern crate rand;

use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use std::process::Command;

fn main() {
    let output = String::from("Hello, world!");
    println!("{}", output);
    print_with_delay(&output, Duration::from_millis(100)).expect("panicking");
    print_with_delay(&output[0..5], Duration::from_millis(100)).expect("panicking");

    print_with_random_delay(&output, Duration::from_millis(200), None).expect("panicking");
    print_with_random_delay(&output, Duration::from_millis(200), Some(100)).expect("panicking");
    print_with_random_delay(&output[0..5], Duration::from_millis(300), Some(30))
        .expect("panicking");

    // try some color
    // does not work in Emacs shells, except term (it is ANSI
    // compliant), so usual execution via cargo-mode commands will not
    // fully work
    let cmd = Command::new("tput")
        .args(&["setaf", "4"])
        .output()
        .expect("failed to execute tput");
    let color_code = String::from_utf8_lossy(&cmd.stdout);

    let cmd = Command::new("tput")
        .args(&["bold"])
        .output()
        .expect("failed to execute tput");
    let bold_code = String::from_utf8_lossy(&cmd.stdout);

    let cmd = Command::new("tput")
        .args(&["sgr0"])
        .output()
        .expect("failed to execute tput");
    let reset_code = String::from_utf8_lossy(&cmd.stdout);

    println!("Status: {}", cmd.status);
    println!("Output: {}", color_code);
    println!("should be blue");
    print!("{}", reset_code);

    println!("Status: {}", cmd.status);
    println!("Output: {}", bold_code);
    print!("{}", color_code);
    println!("should be bold and blue");
    print!("{}", reset_code);

    println!("Status: {}", cmd.status);
    println!("Output: {}", reset_code);
    println!("should be normal text again");
}

pub fn print_with_random_delay<T: AsRef<str>>(
    word: T,
    delay: Duration,
    fraction: Option<u32>,
) -> io::Result<()> {
    let upper_bound = (delay.subsec_millis() * (100 + fraction.unwrap_or(10))) / 100;
    let lower_bound = (delay.subsec_millis() * (100 - fraction.unwrap_or(10))) / 100;

    let mut rng = thread_rng();
    for line in word.as_ref().lines() {
        for letter in line.chars() {
            let mut string = String::new();
            string.push(letter);
            io::stdout().write(string.as_bytes())?;
            io::stdout().flush()?;

            let rand_delay =
                Duration::from_millis(rng.gen_range(lower_bound as u64, upper_bound as u64));
            thread::sleep(rand_delay);
        }
        io::stdout().write(b"\n")?;
        io::stdout().flush()?;
    }
    Ok(())
}

pub fn print_with_delay<T: AsRef<str>>(word: T, delay: Duration) -> io::Result<()> {
    for line in word.as_ref().lines() {
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
