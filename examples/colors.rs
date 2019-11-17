extern crate fwdansi;
extern crate termcolor;

use fwdansi::write_ansi;
use std::io;
use termcolor::*;

fn main() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for fg in (30..=37).chain(90..=97) {
        for bg in (40..=47).chain(100..=107) {
            write_ansi(
                &mut stdout,
                format!("<\u{1b}[{fg};{bg}m{fg:2},{bg:3}\u{1b}[0m>", fg = fg, bg = bg).as_bytes(),
            )?;
        }
        write_ansi(&mut stdout, b"\n")?;
    }

    Ok(())
}
