use lexopt::Parser;
use sysinfo::{System, SystemExt};
use xshell::Shell;

const LIST: &[(
    &'static str,
    (fn(sh: &Shell, sys: &System, args: &mut Parser) -> anyhow::Result<()>),
)] = &[
    ("dir", s::dir::main),
    ("img", s::img::main),
    ("catmd", s::catmd::main),
];

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let sys = System::new();
    let mut args = lexopt::Parser::from_env();

    let Some(lexopt::Arg::Value(cmd)) = args.next()? else {print_help();};
    let cmd = cmd.to_str().unwrap();

    for (tmp, f) in LIST.iter() {
        if &cmd == tmp {
            println!("RUN: {}()", cmd);
            return f(&sh, &sys, &mut args);
        }
    }

    print_help();
}

fn print_help() -> ! {
    println!(
        r#"
COMMAND:
    dir  [ sp <u64> ]
    img  [ to_heic <dir> ]
    catmd <file>

"#
    );

    std::process::exit(0);
}
