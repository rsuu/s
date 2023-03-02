//! image

use std::{
    fmt::Write,
    path::{Path, PathBuf},
    sync::Mutex,
    thread::sleep_ms,
};

s_macro::script_main!({
    let Some(Arg::Value(cmd)) = args.next()? else {anyhow::bail!("")};

    match cmd.to_str().unwrap() {
        "to_heic" => {
            let Some(Arg::Value(path)) = args.next()? else {anyhow::bail!("")};

            to_heic(sh, path, sys.physical_core_count().unwrap())?;
        }
        _ => anyhow::bail!(""),
    }
});

/// Convert image to HEIC format
fn to_heic(sh: &Shell, path: impl AsRef<Path>, cpu_nums: usize) -> anyhow::Result<()> {
    use infer;
    use std::sync::{Arc, RwLock};
    use threadpool::ThreadPool;
    use walkdir::WalkDir;

    let mut tmp = vec![];

    let _ = {
        for t in WalkDir::new(path.as_ref()).into_iter() {
            let Ok(entry)=t else {continue;};

            if entry.file_type().is_file() {
            } else {
                continue;
            }

            let  Some(ext) = infer::get_from_path(entry.path())?

            else {continue;};

            let ext = ext.extension();

            match ext {
                "jpg" | "jpeg" | "png" => {
                    tmp.push(entry.into_path());
                }

                _ => {}
            }
        }
    };

    let n_jobs = tmp.len();
    let pool = ThreadPool::new(cpu_nums);
    let arc_nums: Arc<RwLock<usize>> = Arc::new(RwLock::new(n_jobs));

    // threads
    for n in 0..n_jobs {
        let entry = tmp[n].clone();
        let arc_nums = arc_nums.clone();

        pool.execute(move || {
            let sh = xshell::Shell::new().unwrap();
            let path = entry.parent().unwrap().to_str().unwrap();
            let from = entry.file_name().unwrap().to_str().unwrap();
            let to = format!("{}.heic", from);

            cmd!(sh, " convert -quality 33 {path}/{from} {path}/{to}  ")
                .quiet()
                .run()
                .unwrap();
            fs_extra::file::remove(format!("{path}/{from}")).unwrap();

            // if done
            while let Ok(ref mut nums) = arc_nums.try_write() {
                *(*nums) -= 1;
                break;
            }
        });
    }

    let _ = {
        use indicatif::{ProgressBar, ProgressState, ProgressStyle};

        let arc_nums = arc_nums.clone();
        let pb = ProgressBar::new(n_jobs as u64);

        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {n_jobs} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );

        loop {
            sleep_ms(100);

            let Ok(nums) = arc_nums.read() else {continue;};

            pb.set_position((n_jobs - *nums) as u64);

            if *nums == 0 {
                pb.finish_with_message("downloaded");
                break;
            }

        }
    };

    Ok(())
}
