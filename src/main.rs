use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear},
    ExecutableCommand, QueueableCommand,
};

struct World {
    maxc: u16,
    maxl: u16,
    pc: u16,
    pl: u16,
    map: Vec<(u16, u16)>,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    sc.queue(MoveTo(world.pc, world.pl))?;
    sc.queue(Print("P"))?;
    sc.flush()?;

    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("'".repeat(world.map[l].0.into())))?;
        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("'".repeat((world.maxc - world.map[l].1).into())))?;
    }
    sc.flush()?;

    Ok(())
}

pub fn main() -> std::io::Result<()> {
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();

    let _ = sc.execute(Hide);
    let _ = enable_raw_mode();

    let mut world = World {
        maxc,
        maxl,
        pc: maxc / 2,
        pl: maxl - 1,
        map: vec![(maxc/2-5, maxc/2+5); maxl as usize]
    };

    loop {
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            match key {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('w') => {
                        if world.pl > 1 { world.pl -= 1; }
                    }
                    KeyCode::Char('s') => {
                        if world.pl < maxl { world.pl += 1; }
                    }
                    KeyCode::Char('a') => {
                        if world.pc > 1 { world.pc -= 1; }
                    }
                    KeyCode::Char('d') => {
                        if world.pc < maxc { world.pc += 1; }
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
        }

        let _ = draw(&sc, &world);
    }

    let _ = sc.execute(Show);
    let _ = disable_raw_mode();
    Ok(())
}
