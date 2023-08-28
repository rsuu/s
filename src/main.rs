use s::*;

type FnPtr = fn(_: &mut Main) -> Res;
const LIST: &[(&'static str, FnPtr)] = &[
    ("cat_md", s::cmd::cat_md::main),
    ("split_dir", s::cmd::split_dir::main),
    ("to_heif", s::cmd::to_heif::main),
    ("vidir", s::cmd::vidir::main),
];

fn main() -> Res {
    let mut m = Main {
        sh: Shell::new()?,
        sys: System::new(),
        args: Parser::from_env(),
    };

    let Some(Arg::Value(input)) = m.args.next()?
    else { print_help(); };
    let input = input.to_str().unwrap();

    for (cmd, f) in LIST.iter() {
        if input == *cmd {
            return f(&mut m);
        }
    }

    print_help();
}

fn print_help() -> ! {
    println!(
        r#"
COMMAND:
    catmd <file>
    split_file <file>

    to_heif <dir>
    to_opus <dir>
    split_dir <dir>

    split_video <video>
"#
    );

    exit(0);
}
