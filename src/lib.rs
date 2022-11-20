use std::error::Error;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

use clap::Parser;
use clap::Subcommand;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use totp_rs::TOTP;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    url: String,

    #[command(subcommand)]
    color: Option<Colors>,
}

#[derive(Subcommand)]
enum Colors {
    Never,
    Always,
    Ansi,
    Auto,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let totp = TOTP::<Vec<u8>>::from_url(cli.url).unwrap();
    let step = totp.step;

    let choice = match &cli.color {
        Some(Colors::Auto) | None => {
            if atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        Some(Colors::Ansi) => ColorChoice::AlwaysAnsi,
        Some(Colors::Always) => ColorChoice::Always,
        Some(Colors::Never) => ColorChoice::Never,
    };
    let writer = BufferWriter::stdout(choice);
    let mut buffer = writer.buffer();
    let default_color = &ColorSpec::default();

    let account_name = &totp.account_name;
    write!(&mut buffer, "Для аккаунта ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bg(Some(Color::White)))?;
    write!(&mut buffer, "{account_name}")?;
    buffer.set_color(default_color)?;
    if let Some(ref issuer) = totp.issuer {
        write!(&mut buffer, ", выданный ")?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bg(Some(Color::White)))?;
        write!(&mut buffer, "{issuer}")?;
        buffer.set_color(default_color)?;
    }
    writeln!(&mut buffer, ":")?;
    writer.print(&buffer)?;
    buffer.clear();

    loop {
        let ttl = totp.ttl().unwrap();
        let current_code = totp.generate_current().unwrap();
        let current_code = " ".to_owned() + &current_code + " ";

        write!(&mut buffer, "Код")?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bg(Some(Color::Black)))?;
        write!(&mut buffer, "{current_code}")?;
        buffer.set_color(default_color)?;
        write!(&mut buffer, "действителен ещё ")?;
        let need_next_code = append_ttl(ttl, step, &mut buffer)?;
        buffer.set_color(default_color)?;
        if need_next_code {
            let next_time = system_time()? + step;
            let next_code = totp.generate(next_time);
            write!(&mut buffer, ", а затем сменится на {next_code}")?;
        }
        writeln!(&mut buffer, ".")?;
        writer.print(&buffer)?;
        buffer.clear();
        let sleep_seconds = if ttl <= 0 {
            1
        } else if ttl <= 3 {
            ttl
        } else {
            ttl - 3
        };
        thread::sleep(Duration::from_secs(sleep_seconds));
    }
}

fn append_ttl(num: u64, step: u64, buffer: &mut Buffer) -> Result<bool, Box<dyn Error>> {
    if num <= 3 {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        write!(buffer, "только мгновения")?;
        Ok(true)
    } else if num <= 5 {
        write!(buffer, "только {num}")?;
        append_seconds(num, buffer)?;
        Ok(false)
    } else if step - num <= 5 {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(buffer, "{num}")?;
        append_seconds(num, buffer)?;
        Ok(false)
    } else {
        write!(buffer, "{num}")?;
        append_seconds(num, buffer)?;
        Ok(false)
    }
}

fn append_seconds(num: u64, buffer: &mut Buffer) -> Result<(), Box<dyn Error>> {
    let default_color = &ColorSpec::default();
    buffer.set_color(default_color)?;
    write!(buffer, " ")?;
    let pre_last_digit = num % 100 / 10;
    if pre_last_digit == 1 {
        write!(buffer, "секунд")?;
        return Ok(());
    }
    let last_digit = num % 10;
    if last_digit == 1 {
        write!(buffer, "секунду")?;
    } else if (2..=4).contains(&last_digit) {
        write!(buffer, "секунды")?;
    } else {
        write!(buffer, "секунд")?;
    }
    Ok(())
}

fn system_time() -> Result<u64, SystemTimeError> {
    let t = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    Ok(t)
}
