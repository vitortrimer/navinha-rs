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
    map: Vec<(u16, u16)>,
    player: Player
}

struct Player {
    pc: u16,
    pl: u16,
    dead: bool,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;

    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("'".repeat(world.map[l].0.into())))?;
        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("'".repeat((world.maxc - world.map[l].1).into())))?;
    }
    sc.flush()?;
    
    sc.queue(MoveTo(world.player.pc, world.player.pl))?;
    sc.queue(Print("P"))?;
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
        map: vec![(maxc/2-5, maxc/2+5); maxl as usize],
        player: Player { pc: maxc/2, pl: maxl -1, dead: false },
    };

   while !world.player.dead {
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            while poll(Duration::from_millis(0)).unwrap() {
                let _ = read();
            }
            match key {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('w') => {
                        if world.player.pl > 1 { world.player.pl -= 1; }
                    }
                    KeyCode::Char('s') => {
                        if world.player.pl < maxl { world.player.pl += 1; }
                    }
                    KeyCode::Char('a') => {
                        if world.player.pc > 1 { world.player.pc -= 1; }
                    }
                    KeyCode::Char('d') => {
                        if world.player.pc < maxc { world.player.pc += 1; }
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
