//! deps: ImageMagick

use crate::*;

pub fn main(Main { sh, sys, args }: &mut Main) -> Res {
    let Some(Arg::Value(path)) = args.next()?
    else { bail!("") };

    let cpu_nums = sys.physical_core_count().unwrap();
    let path: &Path = path.as_ref();

    if path.is_dir() {
        let vec = {
            let mut tmp = vec![];
            for t in WalkDir::new(&path).into_iter() {
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
                let dst = format!("{src}.heif");

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
    } else if path.is_file() {
        let sh = Shell::new().unwrap();
        let fname = path.file_name().unwrap().to_str().unwrap();
        let dst = format!("{fname}.heif");

        cmd!(sh, "convert -quality 30 {path} {dst}")
            .quiet()
            .run()
            .unwrap();
    } else {
        bail!("")
    }

    Ok(())
}
