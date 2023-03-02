s_macro::script_main!({
    use fs_extra::dir;
    use std::path::PathBuf;
    use walkdir::WalkDir;

    let Some(Arg::Value(path)) = args.next()? else {anyhow::bail!("")};

    let Some(Arg::Value(max)) = args.next()? else {anyhow::bail!("")};

    let path = path.to_str().unwrap();
    let max = max.to_str().unwrap().parse::<u64>()?;

    let mut list = vec![];

    for tmp in WalkDir::new(path).max_depth(1).into_iter() {
        let entry = tmp?;

        if entry.file_type().is_file() {
            continue;
        }

        let path = entry.into_path();
        let dir_size = dir::get_size(&path)?;

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

    let options = fs_extra::dir::CopyOptions::new();
    let mut to = None;

    for (path, size, need_mv) in list.iter().skip(1) {
        if *need_mv {
            dir::move_dir(path, to.unwrap(), &options)?;
        } else {
            to = Some(path);
        }
    }
});
