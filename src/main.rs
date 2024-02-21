pub mod rope;

use std::{
    io::{stdout, Read, Write},
    thread,
    time::Duration,
};
use termion::{async_stdin, raw::IntoRawMode};
fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut stdin = async_stdin().bytes();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let mut commands: Vec<u8> = Vec::new();
    let mut command_mode = false;
    thread::sleep(Duration::from_millis(500));
    let mut textPos: Cursor = Cursor { x: 1, y: 1 };
    loop {
        let b = stdin.next();
        let b = match b {
            Some(b) => b.unwrap_or(0),
            None => 0,
        };

        if b != 0 {
            if b == b':' {
                command_mode = true;
                let size = termion::terminal_size().unwrap_or_default();
                write!(stdout, "{}{}", termion::cursor::Goto(1, size.1), b as char).unwrap();
            } else if command_mode {
                if b == b'\r' {
                    command_mode = false;
                    let command = parse_command(commands.as_ref());
                    match command {
                        CommandList::Quit => break,
                        CommandList::Clear => {
                            write!(
                                stdout,
                                "{}{}",
                                termion::clear::All,
                                termion::cursor::Goto(1, 1)
                            )
                            .unwrap();
                            textPos = Cursor { x: 1, y: 1 };
                        }
                        CommandList::NewFile => {
                            write!(
                                stdout,
                                "{}NewFile{}",
                                termion::clear::CurrentLine,
                                termion::cursor::Goto(textPos.x, textPos.y)
                            )
                            .unwrap();
                        }
                        CommandList::Invalid => write!(
                            stdout,
                            "{}Invalid Command{}",
                            termion::clear::CurrentLine,
                            termion::cursor::Goto(textPos.x, textPos.y)
                        )
                        .unwrap(),
                        CommandList::WriteFile => write!(
                            stdout,
                            "{}File Written{}",
                            termion::clear::CurrentLine,
                            termion::cursor::Goto(textPos.x, textPos.y)
                        )
                        .unwrap(),
                        _ => write!(stdout, "Unimplimented").unwrap(),
                    };
                    commands.clear();
                } else {
                    commands.push(b);
                    write!(stdout, "{}", b as char).unwrap();
                }
            } else if b == b'\r' {
                textPos = Cursor {
                    x: 1,
                    y: textPos.y + 1,
                };
                write!(stdout, "{}", termion::cursor::Goto(textPos.x, textPos.y)).unwrap();
            } else if b == 127 {
                if textPos.x > 1 {
                    textPos.x -= 1;
                } else if textPos.y > 1 {
                    textPos.y -= 1;
                    // textPos.x
                }
            } else if !command_mode {
                write!(stdout, "{}", b as char).unwrap();
                textPos.x += 1;
            };
        }

        stdout.flush().unwrap();
    }
}

fn parse_command(command_list: &[u8]) -> CommandList {
    match command_list {
        b"q" => CommandList::Quit,
        b"c" => CommandList::Clear,
        b"n" => CommandList::NewFile,
        b"w" => CommandList::WriteFile,
        _ => CommandList::Invalid,
    }
}

enum CommandList {
    Quit,
    NewFile,
    Invalid,
    Clear,
    WriteFile,
}

struct Cursor {
    x: u16,
    y: u16,
}
