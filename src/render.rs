use std::{error::Error, io::{Stdout, Write}};

use crossterm::{cursor::MoveTo, style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}, QueueableCommand};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) -> Result<(), Box<dyn Error>>{

    if force {
        stdout.queue(SetBackgroundColor(Color::Blue))?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(SetBackgroundColor(Color::Black))?;
    }

    for (x, y, val) in curr_frame.into_iter() {
        if val != last_frame[x][y] || force {
            stdout.queue(MoveTo(x as u16, y as u16))?;
            print!("{}", val);
        }
    }

    stdout.flush()?;

    Ok(())
}