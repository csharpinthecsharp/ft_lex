use std::io::empty;

pub enum TOK {
    Default,
    FirstBlock, // %{...%}
    Name, // DIGIT
    Pattern, // [0-9] or {DIGIT}+
    SectionSep, // %%
    Action, // { printf(...); }
    StartCond, // %s / %x decla
    EOF
}

pub struct Token {
   typ: TOK,
   s: String,
   line: i32,
   column: i32,
}

impl Default for Token
{
    fn default() -> Self {
        Self { typ: TOK::Default, s: String::new(), line: 1, column: 0 }
    }
}
 
impl Token
{
    pub fn update(&mut self, ty: TOK, ss: String, l: i32, c: i32) -> Self {
        Self { typ: ty, s: ss, line: l, column: c }
    }
}