// TODO:
// + mv file
// + rm file
//   pipe: fd -t f . | vidir -
// ? undo

use crate::*;

const NULL: char = '\u{0000}';

pub fn main(Main { sh, sys, args }: &mut Main) -> Res {
    let mut conf = Conf {
        path: PathBuf::from("./"),
        depth: 100,
        force: false,
        tmp: tempfile::NamedTempFile::new()?,
        is_from_pipe: false,
    };

    if let Some(v) = args.next()? {
        match v {
            Short('f') => conf.force = true,
            Short('d') => {
                let Arg::Value(v) = args.next()?.unwrap()
            else {bail!("")};
                conf.depth = v.parse()?;
            }

            Value(v) if v == OsString::from("-") => {
                conf.is_from_pipe = true;
            }

            Value(v) => conf.path = PathBuf::from(v.to_str().unwrap()),

            _ => {}
        }
    }

    let editor = std::env::var("VISUAL").unwrap_or(std::env::var("EDITOR")?);
    let fs_options = fs_dir::CopyOptions::new();
    // Vec<(idx, path)>
    let mut walk: Vec<(usize, PathBuf)> = Vec::new();
    let mut walk_new: Vec<(usize, &str)> = Vec::new();

    if conf.is_from_pipe {
        for (idx, path) in io::stdin().lines().into_iter().enumerate() {
            let path = path?;
            let s = format!("{idx}{NULL}{path}\n");

            walk.push((idx, PathBuf::from(&path)));
            conf.tmp.write_all(s.as_bytes())?;
        }
    } else {
        for (idx, path) in WalkDir::new(conf.path)
            .max_depth(conf.depth)
            .into_iter()
            .enumerate()
        {
            let path = path?.into_path();
            let path = path.display().to_string();

            let s = format!("{idx}{NULL}{path}\n");

            walk.push((idx, PathBuf::from(&path)));
            conf.tmp.write_all(s.as_bytes())?;
        }
    }

    //
    std::process::Command::new(editor)
        .arg(conf.tmp.path())
        .status()?;

    //
    let mut new = "".to_string();
    conf.tmp.reopen()?.read_to_string(&mut new)?;
    for line in new.lines() {
        let line = line.split(NULL).into_iter().collect::<Vec<&str>>();

        if !line[0].is_empty() {
            walk_new.push((line[0].parse::<usize>()?, line[1]));
        }
    }

    let mut n: usize = 0;
    for (idx, path) in walk.iter() {
        let src = path.as_path();

        if *idx == walk_new[n].0 {
            let dst = Path::new(walk_new[n].1);

            if src != dst {
                println!("mv: {} -> {}", src.display(), dst.display());

                if src.is_dir() && dst.is_dir() {
                    // move
                    fs_extra::move_items(&[src], dst, &fs_options)?;
                } else {
                    // rename
                    fs::rename(src, dst)?;
                }
            } else {
                // do nothing
            }

            n += 1;
        } else {
            println!("rm: {} ", src.display());

            if src.is_file() {
                fs::remove_file(src)?;
            } else if src.is_dir() {
                if conf.force {
                    fs::remove_dir_all(src)?;
                } else {
                    fs::remove_dir(src)?;
                }
            }
        }
    }

    Ok(())
}

pub struct Conf {
    depth: usize,
    path: PathBuf,
    force: bool,
    tmp: tempfile::NamedTempFile,
    is_from_pipe: bool,
}
