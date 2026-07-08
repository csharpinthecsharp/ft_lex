use std::env; // standard library
use std::process;
use std::fs::File;

struct Opt
{
    v: bool,
    n: bool,
    t: bool,
    stdin: bool,
}

impl Opt
{
    fn setv(&mut self) {
        self.v = true;
    }
    fn setn(&mut self) {
        self.n = true;
    }
    fn sett(&mut self) {
        self.t = true;
    }
    fn setstdin(&mut self) {
        self.stdin = true;
    }
}

impl Default for Opt
{
    fn default() -> Self {
        Self { v: false, n: false, t: false, stdin: false }
    }
}

fn ft_error(content: &str, attachement: &str) -> ! // fn never come back
{
    eprintln!("ft_lex: {} {}", content, attachement);
    process::exit(1);
}

fn parse_opt(args: &mut Vec<String>, s_opt: &mut Opt) -> usize
{ 
    let mut index = 0;
    for (i, arg) in args.iter().enumerate() { 
        index = i;
        match arg.as_str() {
            "-v" => s_opt.setv(),
            "-n" => s_opt.setn(),
            "-t" => s_opt.sett(),
            "-"  => {
                s_opt.setstdin();
                return i + 1;
            }
            _ => return i,
        }
    }
    index
}

fn main()
{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Using stdin");
        process::exit(0);
    }
    
    args.remove(0);
    let mut s_opt = Opt::default();
    
    let res_opt = parse_opt(&mut args, &mut s_opt);
    if res_opt == 0 {
        println!("No opt");
    }
         
    let mut _file = match File::open(&args[0]) {
        Ok (file) => file,
        Err(_) => {
            ft_error("can't open", &args[0]);
        }
    };
}
