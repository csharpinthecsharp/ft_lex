use std::env;
use std::process;
use std::fs::File;

mod parse;
mod structs;
use parse::parse_opt;

fn ft_error(content: &str, attachement: &str) -> ! // fn never come back
{
    eprintln!("ft_lex: {} {}", content, attachement);
    process::exit(1);
}

fn main()
{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        process::exit(0);
    }
    
    let _files: Vec<String> = parse_opt::handle_args(&mut args);

    for (index, file) in files.iter().enumeration() {
        if index == 0 && structs::Opt::getstdin()
            // Lex from stdin
        // Lex from iterations
    }
    /*
    let mut _file = match File::open(&args[0]) {
        Ok (file) => file,
        Err(_) => {
            ft_error("can't open", &args[0]);
        }
    };
    */
}
