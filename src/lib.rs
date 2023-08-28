//!

pub mod cmd;

pub use {
    fs_extra::{dir as fs_dir, file as fs_file},
    lexopt::{prelude::*, Arg, Parser},
    std::{
        ffi::OsString,
        fmt::Write as _,
        fs::{self, File},
        io::{self, Read as _, Write as _},
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

pub type Res = anyhow::Result<()>;

#[derive(Debug)]
pub struct Main {
    pub sh: Shell,
    pub sys: System,
    pub args: Parser,
}

pub fn sleep_ms(v: u64) {
    std::thread::sleep(std::time::Duration::from_millis(v));
}

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
