use std::env; // standard library
use std::process;
use std::fs::File;

struct Opt {
    v: bool,
    n: bool,
    t: bool,
}

impl Opt {
    fn setv(&mut self) {
        self.v = true;
    }
    fn setn(&mut self) {
        self.n = true;
    }
    fn sett(&mut self) {
        self.t = true;
    }
}

Impl Default for Opt {
    fn default() -> Self {
        Self { v: false, n: false, t: false }
    }
}

fn ft_error(content: &str, attachement: &str) -> ! // fn never come back
{
    eprintln!("ft_lex: {} {}", content, attachement);
    process::exit(1);
}

fn parse_opt(args: &Vec<String>)> 
{
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Using stdin");
        process::exit(0);
    }
    
    let mut sOpt = Opt::default();
    // sOpt.setv();

    let mut _file = match File::open(&args[1]) {
        Ok (file) => file,
        Err(_) => {
            ft_error("can't open", &args[1]);
        }
    };
}
