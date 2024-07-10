use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand,
};

struct World {
    pc: u16,
    pl: u16,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(MoveTo(world.pc, world.pl))?;
    sc.queue(Print("P"))?;
    sc.flush()?;

    Ok(())
}

pub fn main() -> std::io::Result<()> {
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();

    sc.execute(Hide);
    enable_raw_mode();

    let mut world = World {
        pc: maxc / 2,
        pl: maxl - 1,
    };

    loop {
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            match key {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
        }

        draw(&sc, &world);
    }

    sc.execute(Show);
    disable_raw_mode();
    Ok(())
}
