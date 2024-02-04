use std::{error::Error, fs, io, sync::mpsc, thread, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::Frame,
    render::{self, render},
};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    get_audio_files("resources/")?
        .iter()
        .for_each(|f| audio.add(file_name(f), String::from("resources/") + f));

    audio.play("explode");
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    audio.wait();

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = Frame::new();
        let mut stdout = io::stdout();
        let _ = render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            let _ = render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    'gameloop: loop {
        let curr_frame = Frame::new();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    _ => {}
                }
            }
        }

        // Draw and render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    drop(render_tx);
    render_handle.join();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn file_name(str: &String) -> String {
    str[..str.find(".").unwrap_or(str.len())].to_string()
}

fn get_audio_files(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let res: Vec<String> = fs::read_dir(path)?
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|f| f.unwrap().file_name().into_string())
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .filter(|f| f.ends_with(".wav"))
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::{file_name, get_audio_files};

    #[test]
    fn test_get_audio_files() {
        let _files = get_audio_files("resources/");
        assert!(_files.is_ok());
        let files = _files.unwrap();
        for f in files {
            assert!(f.ends_with(".wav"))
        }
    }

    #[test]
    fn test_file_name() {
        assert_eq!(file_name(&String::from("test.file")), String::from("test"));
        assert_eq!(file_name(&String::from("test")), String::from("test"));
    }
}
