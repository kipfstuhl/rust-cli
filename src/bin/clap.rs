#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::process::Command;
use std::time::Duration;
use versuch::{print_with_delay, print_with_random_delay};

fn main() {
    let matches = App::new("rust-cli with clap")
        .version(crate_version!())
        .about("Prints characaters separately")
        .arg(
            Arg::with_name("delay")
                .short("d")
                .long("delay")
                .help("The dealy between every character")
                .takes_value(true)
                .default_value("100"),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT STRING")
                .help("The string to output character by character"),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .help("Output color and Ansi escape sequences"),
        )
        .get_matches();

    let output = matches.value_of("output").unwrap_or("Hello World!");
    let out = &mut std::io::stdout();
    println!("{}", output);
    println!("Now some default output, not respecting user defined delay.");

    print_with_delay(&output, Duration::from_millis(100), out).expect("panicking");
    print_with_random_delay(&output, Duration::from_millis(200), None, out).expect("panicking");
    print_with_random_delay(&output, Duration::from_millis(200), Some(100), out)
        .expect("panicking");

    println!("Now respect the user specified delay, or use the default value.");
    let duration_millis = value_t!(matches, "delay", u64).unwrap();
    let duration = Duration::from_millis(duration_millis);
    print_with_delay(&output, duration, out).expect("panicking");

    if matches.is_present("color") {
        println!("this may mess up in your terminal");

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
}
