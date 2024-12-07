use std::io::{stdout, IsTerminal, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, terminal, ExecutableCommand};
use crossterm::terminal::ClearType;

#[allow(dead_code)]
pub fn draw_frame(string: &str, delay: Option<u64>) {
    let mut stdout = stdout();
    stdout.is_terminal();

    // Clear the terminal
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // Move the cursor to the top-left corner
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    println!("{string}");


    // Flush to ensure the output is displayed
    stdout.flush().unwrap();

    sleep(Duration::from_millis(delay.unwrap_or(9)));

}