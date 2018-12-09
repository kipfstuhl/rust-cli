use console::Term;
use indicatif::ProgressBar;
use std::time::Duration;
use structopt::StructOpt;
use versuch::{print_with_delay, print_with_random_delay};

macro_rules! at_line {
    ($writer:ident, $n:expr ,$x:stmt) => {
        $writer.move_cursor_up($n).unwrap();
        $x;
        $writer.move_cursor_down($n).unwrap();
    };
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "",
    version = "",
    about = "Print character for character with a delay and show progress",
    author = "",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
struct Cli {
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
    // use Term from crate console for easier handling, e.g. can go up
    // and down like needed. This is important for handling the
    // ProgressBar when printing output.
    let out = &mut Term::stdout();

    println!("{}", output);
    println!("Now some default output, not respecting user defined delay.");
    let bar = ProgressBar::new(4);
    bar.tick();

    print_with_delay(&output, Duration::from_millis(100), out).expect("panicking");

    at_line!(out, 1, bar.inc(1));

    print_with_random_delay(&output, Duration::from_millis(200), None, out).expect("panicking");

    at_line!(out, 2, bar.inc(1));

    print_with_random_delay(&output, Duration::from_millis(200), Some(100), out)
        .expect("panicking");

    at_line!(out, 3, bar.inc(1));

    println!("Now respect the user specified delay, or use the default value.");
    let duration = Duration::from_millis(args.delay);
    print_with_delay(&output, duration, out).expect("panicking");

    at_line!(out, 5, {
        bar.inc(1);
        bar.finish();
    });
}
