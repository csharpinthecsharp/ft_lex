use std::env; // standard library
use std::process;
use std::fs::File;

fn ft_error(content: &str, attachement: &str) -> ! // fn never come back
{
    eprintln!("ft_lex: {} {}", content, attachement);
    process::exit(1);
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Using stdin");
        process::exit(0);
    }
    let mut _file = match File::open(&args[1]) {
        Ok (file) => file,
        Err(_) => {
            ft_error("can't open", &args[1]);
        }
    };
}
