//! image

use crate::*;

pub fn main(m: &mut Main) -> Res {
    let Main { sh, sys, args } = m;
    let Some(Arg::Value(cmd)) = args.next()?
    else {bail!("")};

    match cmd.to_str().unwrap() {
        "to_heif" => {
            let Some(Arg::Value(path)) = args.next()?
            else {bail!("")};

            to_heif(sh, path, sys.physical_core_count().unwrap())?;
        }
        _ => bail!(""),
    }

    Ok(())
}

/// e.g. s to_heif ./img
fn to_heif(sh: &Shell, path: impl AsRef<Path>, cpu_nums: usize) -> Res {
    let mut vec = {
        let mut tmp = vec![];
        for t in WalkDir::new(path.as_ref()).into_iter() {
            let Ok(entry) = t
            else {continue;};

            if entry.file_type().is_dir() {
                continue;
            }

            let Some(ty) = infer::get_from_path(entry.path())?
            else {continue;};

            match ty.extension() {
                "jpg" | "jpeg" | "png" => {
                    tmp.push(entry.into_path());
                }

                _ => {}
            }
        }

        tmp
    };
    let n_jobs = vec.len();
    let pool = ThreadPool::new(cpu_nums);
    let arc_nums = Arc::new(RwLock::new(n_jobs));

    // threads
    for n in 0..n_jobs {
        arc_clone!(arc_nums);
        let entry = vec[n].clone();

        pool.execute(move || {
            let sh = Shell::new().unwrap();
            let path = entry.parent().unwrap().to_str().unwrap();
            let src = entry.file_name().unwrap().to_str().unwrap();
            let dst = format!("{}.heif", src);

            cmd!(sh, "convert -quality 30 {path}/{src} {path}/{dst}")
                .quiet()
                .run()
                .unwrap();
            fs_file::remove(format!("{path}/{src}")).unwrap();

            // if done
            while let Ok(ref mut nums) = arc_nums.try_write() {
                *(*nums) -= 1;
                break;
            }
        });
    }

    loop {
        sleep_ms(100);

        let Ok(nums) = arc_nums.try_read() else {continue;};

        if *nums == 0 {
            break;
        }
    }

    Ok(())
}
