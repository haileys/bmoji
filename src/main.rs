use std::io::{Write, BufWriter, BufRead, BufReader};
use std::process::{Command, Stdio};

fn prompt() -> Result<Option<&'static emojis::Emoji>, anyhow::Error> {
    let bemenu = Command::new("bemenu")
        .args(&["--prompt", "bmoji"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = BufWriter::new(bemenu.stdin.expect("bemenu stdin is always Some"));

        for emoji in emojis::iter() {
            write!(stdin, "{} {}", emoji.as_str(), emoji.name())?;

            if let Some(shortcode) = emoji.shortcode() {
                write!(stdin, " :{}:", shortcode)?;
            }

            writeln!(stdin)?;
        }
    }

    let mut stdout = BufReader::new(bemenu.stdout.expect("bemenu stdout is always Some"));

    let mut selection = Vec::new();
    stdout.read_until(b' ', &mut selection)?;

    let selection = std::str::from_utf8(&selection)?.trim();
    Ok(emojis::lookup(selection))
}

fn copy(s: &str) -> Result<(), anyhow::Error> {
    let wl_copy = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = wl_copy.stdin.expect("wl-copy stdin is always Some");
    stdin.write_all(s.as_bytes())?;
    stdin.flush()?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    if let Some(emoji) = prompt()? {
        copy(emoji.as_str())?;
    }

    Ok(())
}
