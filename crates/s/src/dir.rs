use crate::*;

pub fn main(m: &mut Main) -> Res {
    let Main { sh, sys, args } = m;

    let Some(Arg::Value(path)) = args.next()?
    else {bail!("")};

    let Some(Arg::Value(max)) = args.next()?
    else {bail!("")};

    let path = path.to_str().unwrap();
    let max = max.to_str().unwrap().parse::<u64>()?;

    let mut list = vec![];

    for tmp in WalkDir::new(path).max_depth(1).into_iter() {
        let entry = tmp?;

        if entry.file_type().is_file() {
            continue;
        }

        let path = entry.into_path();
        let dir_size = fs_dir::get_size(&path)?;

        list.push((path, dir_size, true));
    }

    let mut tmp = 0;

    for (path, size, need_mv) in list.iter_mut() {
        tmp += *size;

        if tmp <= max {
        } else {
            *need_mv = false;
            tmp = 0;
        }
    }

    assert_eq!(true, list.len() >= 2);
    list[1].2 = false;

    let options = fs_dir::CopyOptions::new();
    let mut to = None;

    for (path, size, need_mv) in list.iter().skip(1) {
        if *need_mv {
            fs_dir::move_dir(path, to.unwrap(), &options)?;
        } else {
            to = Some(path);
        }
    }

    Ok(())
}
