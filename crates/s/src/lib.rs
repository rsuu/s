//!

pub use {
    fs_extra::{dir as fs_dir, file as fs_file},
    lexopt::{prelude::*, Arg, Parser},
    std::{
        ffi::OsString,
        fmt::Write as _,
        io::Write as _,
        path::{Path, PathBuf},
        process::exit,
        sync::{Arc, RwLock},
    },
    sysinfo::{System, SystemExt as _},
    threadpool::ThreadPool,
    walkdir::WalkDir,
    xshell::{cmd, Shell},
};
// macro
pub use anyhow::bail;

pub mod dir;
pub mod file;
pub mod img;
pub mod utils {
    pub mod types;
}

/// clone Arc values in closure
#[macro_export]
macro_rules! arc_clone {
    (
      $($x:ident),*
    ) => {
        $(
        let $x = $x.clone();
        )*
    };
}

pub type Res = anyhow::Result<()>;

#[derive(Debug)]
pub struct Main<'a> {
    pub sh: &'a Shell,
    pub sys: &'a System,
    pub args: &'a mut Parser,
}

pub fn sleep_ms(v: u64) {
    std::thread::sleep(std::time::Duration::from_millis(v));
}
