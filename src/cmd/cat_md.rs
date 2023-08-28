use crate::*;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn main(Main { sh, sys, args }: &mut Main) -> Res {
    let Some(Arg::Value(path)) = args.next()?
    else { bail!("") };
    let data = fs_file::read_to_string(path)?;

    let mut view = termimad::MadView::from(data.to_string(), view_area(), make_skin());
    let mut w = std::io::stderr();
    terminal::enable_raw_mode()?;
    queue!(w, EnterAlternateScreen)?;
    queue!(w, Hide)?; // hiding the cursor

    loop {
        view.write_on(&mut w)?;
        w.flush()?;

        let Ok(event)=event::read() else {continue;};

        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                Up | PageUp | Char('k') => view.try_scroll_lines(-1),
                Down | PageDown | Char('j') => view.try_scroll_lines(1),
                _ => break,
            },
            Event::Resize(..) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;

    Ok(())
}

fn make_skin() -> termimad::MadSkin {
    let mut skin = termimad::MadSkin::default();
    skin.table.align = termimad::Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = termimad::Alignment::Center;

    skin
}

fn view_area() -> termimad::Area {
    let mut area = termimad::Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column

    area
}
