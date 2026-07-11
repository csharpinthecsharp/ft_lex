use crate::structs::tokens::TOK::FirstBlock;
use crate::structs::tokens::{self, Token};

use std::iter::Peekable;
use std::str::Chars;
use std::fs;

fn skip_line(chars: &mut Peekable<Chars>, line: &mut i32) {
    while let Some(&c) = chars.peek() {
        if c == '\n' {
            *line+=1;
            break;
        }
        chars.next();
    }
}

pub fn lexer_loop(path: &str)
{
    let mut open_block = false;
    let mut line = 1;
    let mut column = 0;
    let content = fs::read_to_string(path).unwrap();
    let mut chars: Peekable<Chars> = content.chars().peekable();
    let mut strbuf: String = Default::default();
    let mut tokens: Vec<Token> = vec![];
    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                if open_block {
                    let mut token = Token::default();
                    token.update(FirstBlock, strbuf.clone(), line, column);
                    tokens.push(token);
                    strbuf.clear();
                }
                line+=1;
            }
            '%' => {
                if chars.peek() == Some(&'{') {
                    open_block = true;
                    skip_line(&mut chars, &mut line);
                }
                if chars.peek() == Some(&'}') {
                    open_block = false;
                    skip_line(&mut chars, &mut line);
                }
                if chars.peek() == Some(&'%')  {
                    // Exit First PART
                }
            }
            _ => {
                if open_block {
                    strbuf.push(c);
                }
                if c.is_alphabetic() {
                    eprintln!("{}:{}: incomplete name definition", path, line);
                    skip_line(&mut chars, &mut line);
                }
                else {
                    eprintln!("{}:{}: bad character: {}", path, line, c)
                }
            }
        }
    }
    if open_block {
        eprintln!("{}:{}: premature EOF", path, line); 
    }
}
