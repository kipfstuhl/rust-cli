use std::process::Command;
use std::time::Duration;

use structopt::StructOpt;

use versuch::{print_with_delay, print_with_random_delay};

#[derive(StructOpt)]
struct Cli {
    /// output color and ANSI control codes
    #[structopt(short = "c", long = "color")]
    color: bool,
    /// Delay in milliseconds for the output of strings
    #[structopt(short = "d", long = "delay", default_value = "100")]
    delay: u64,
    /// String to output
    #[structopt(default_value = "Hello World!")]
    output: String,
}

fn main() {
    let args = Cli::from_args();
    let output = args.output;
    let out = &mut std::io::stdout();
    println!("{}", output);
    println!("Now some default output, not respecting user defined delay.");

    print_with_delay(&output, Duration::from_millis(100), out).expect("panicking");
    print_with_random_delay(&output, Duration::from_millis(200), None, out).expect("panicking");
    print_with_random_delay(&output, Duration::from_millis(200), Some(100), out)
        .expect("panicking");

    // does not work, do not use such characters!!
    // print_with_random_delay(
    //     String::from("challenging: ßäöüÄÖÜ, ðÐ œ Œ"),
    //     Duration::from_millis(300),
    //     Some(100),
    //     out,
    // )
    // .expect("panicking");

    println!("Now respect the user specified delay, or use the default value.");
    let duration = Duration::from_millis(args.delay);
    print_with_delay(&output, duration, out).expect("panicking");

    if args.color {
        // try some color
        // does not work in Emacs shells, except term (it is ANSI
        // compliant), so usual execution via cargo-mode commands will not
        // fully work
        //
        // Note: this is not safe in any way to do so.  First this
        // approach needs the command tput available, second it prints
        // the escape codes directly to the terminal.  If you want
        // this use a crate made for this purpose, e.g. ansi_term

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
