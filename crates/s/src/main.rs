use s::*;

type FnPtr = fn(_: &mut Main) -> Res;
const LIST: &[(&'static str, &'static str, FnPtr)] = &[
    ("dir", "d", s::dir::main),
    ("img", "i", s::img::main),
    ("file", "f", s::file::main),
];

fn main() -> Res {
    let mut m = Main {
        sh: &Shell::new()?,
        sys: &System::new(),
        args: &mut Parser::from_env(),
    };

    let Some(Arg::Value(cmd)) = m.args.next()?
    else { print_help(); };
    let cmd = cmd.to_str().unwrap();

    for (long, short, f) in LIST.iter() {
        if cmd == *long || cmd == *short {
            //println!("RUN: {}()", long);
            return f(&mut m);
        }
    }

    print_help();
}

fn print_help() -> ! {
    println!(
        r#"
COMMAND:
    f|file
        catmd <file>

    d|dir
        sp <u64>

    i|img
        to_heif <dir>

    a|audio
        to_opus <dir>
        split <file>

    v|video
        split <file>
"#
    );

    exit(0);
}
