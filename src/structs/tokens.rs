#[derive(Clone, Debug)]
pub enum TOK {
    Default,
    Ccode,
    Block, // %{...%}
    NamePattern, // DIGIT // [0-9] or {DIGIT}+
    StartCond, // %s / %x decla
}

pub struct Token {
   typ: TOK,
   key: String,
   value: String,
   line: i32,
   column: i32,
}

impl Default for Token
{
    fn default() -> Self {
        Self { typ: TOK::Default, key: String::new(), value: String::new(), line: 1, column: 0 }
    }
}
 
impl Token
{
    pub fn update(&mut self, typ: TOK, key: String, value: String, line: i32, column: i32) {
        self.typ = typ;
        self.key = key;
        self.value = value;
        self.line = line;
        self.column = column
    }
    pub fn gettype(&self) -> TOK {
        self.typ.clone()
    }
    pub fn getkey(&self) -> String {
        self.key.clone()
    }
    pub fn getvalue(&self) -> String {
        self.value.clone()
    }
    pub fn getline(&self) -> i32 {
        self.line
    }
    pub fn getcolumn(&self) -> i32 {
        self.column
    }
}
