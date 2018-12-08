use rand::{thread_rng, Rng};
use std::io;
use std::thread;
use std::time::Duration;

/// pritn a word (AsRef<str>, i.e. something that behaves like a
/// reference to a string like object, charachter for character with a
/// randomly changing delay between each character.
/// The base value for the delay has to be specified.
/// For the change of the delay a value in percent can be given.
///
/// All errors that occur during output are returned as is.
pub fn print_with_random_delay<T: AsRef<str>, W: std::io::Write>(
    word: T,
    delay: Duration,
    fraction: Option<u32>,
    writer: &mut W,
) -> io::Result<()> {
    let upper_bound = (delay.subsec_millis() * (100 + fraction.unwrap_or(10))) / 100;
    let lower_bound = (delay.subsec_millis() * (100 - fraction.unwrap_or(10))) / 100;

    let mut rng = thread_rng();
    for line in word.as_ref().lines() {
        for letter in line.chars() {
            writer.write_all(&[letter as u8])?;
            writer.flush()?;
            let rand_delay = Duration::from_millis(
                rng.gen_range(u64::from(lower_bound), u64::from(upper_bound)),
            );
            thread::sleep(rand_delay);
        }
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
    Ok(())
}

/// print a string(y) word character for character with a specific delay
/// between each charachter.
///
/// Errors are returned without any handling.
pub fn print_with_delay<T: AsRef<str>, W: std::io::Write>(
    word: T,
    delay: Duration,
    writer: &mut W,
) -> io::Result<()> {
    for line in word.as_ref().lines() {
        for letter in line.chars() {
            writer.write_all(&[letter as u8])?;
            writer.flush()?;
            thread::sleep(delay);
        }
        writer.write_all(b"\n")?;
        writer.flush()?;
    }

    Ok(())
}
