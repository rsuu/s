use crate::*;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn main(m: &mut Main) -> Res {
    let Main { sh, sys, args } = m;
    let Some(lexopt::Arg::Value(cmd)) = args.next()? else { SubCmd::print_help(); };
    let cmd = cmd.to_str().unwrap();

    match cmd {
        "catmd" => {
            let Some(Arg::Value(path)) = args
        .next()
        .unwrap_or(Some(Arg::Value(OsString::from("README.md")))) else {panic!()};
            let input = fs_extra::file::read_to_string(path)?;

            SubCmd::CatMd { input }.catmd()?;
        }

        _ => SubCmd::print_help(),
    }

    Ok(())
}

#[derive(Debug)]
pub enum SubCmd {
    CatMd { input: String },
}

impl SubCmd {
    fn print_help() -> ! {
        println!(
            r#"
COMMAND:
    catmd <file>
"#
        );

        std::process::exit(0);
    }

    pub fn catmd(&self) -> anyhow::Result<(), termimad::Error> {
        let SubCmd::CatMd { input } = self;

        let skin = make_skin();
        let mut view = termimad::MadView::from(input.to_string(), view_area(), skin);

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
