use crate::structs::tokens::TOK::{Block, Ccode, NamePattern};
use crate::structs::tokens::{Token};

use std::default::Default;
use std::iter::Peekable;
use std::fs;
use std::process;
use std::str::Chars;

fn is_space(c: Option<&char>) -> bool {
    if c == Some(&' ') || c == Some(&'\t') {
        return true;
    }
    false
}

fn get_lex_def(path: &str, tok_list: &mut Vec<Token>, mut lines: &mut Peekable<std::str::Lines<'_>>) -> i32
{
    //let (mut err, mut  line, mut s_line, mut e_line, mut column):
    //    (i32, i32, i32, i32, i32) = (0, 1, 1, 0, 0);

    let mut y: i32 = 0;
    let mut in_block: bool = false;
    let mut res: i32 = 0;
    let mut block_str: String = Default::default();
    let mut key: String = Default::default();
    let mut value: String = Default::default();

    while let Some(line) = lines.next() {
        y += 1;

        if line.trim().is_empty() {
            continue;
        }
        
        let l = line.chars().nth(0);
        if l == Some(' ') {
            let mut tok: Token = Token::default();
            tok.update(Ccode, line.to_string(), "".to_string(), y, 0);
            tok_list.push(tok);
            continue;
        }

        let temp_line: &str = line.get(..2).unwrap_or("");
        match temp_line {
            "%}" => {
                in_block = false;
                if !block_str.is_empty() {
                    let mut tok: Token = Token::default();
                    tok.update(Block, block_str.to_string(), "".to_string(), y, 0);
                    tok_list.push(tok);
                    continue;
                }
            }
            "%{" => {
                in_block = true;
                continue;
            }
            "%%" => {
                return res;
            }
            _=> {
                if in_block {
                    block_str.push_str(line);
                    if block_str.len() == line.len() {
                        block_str.push_str("\n");
                    }
                    continue;
                }
            }
        }

        let mut state_c = line.chars().peekable();
        if state_c.next() == Some('%') {
            match state_c.next() {
                Some('s') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('x') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('p') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('n') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('a') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('e') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('k') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }
                Some('o') => {
                    if is_space(state_c.peek()) {
                    }
                    continue;
                }    
                _=> {
                    eprintln!("{}:{}: unrecognized '%' directive", path, y);
                    res += 1;
                    continue;
                }
            }
        }

        let v: Vec<&str> = line.split(' ').collect();
        if v.is_empty() {
            continue;
        }
        if v.len() == 1 {
            let c = line.chars().next().unwrap();
            if !char::is_alphabetic(c) { eprintln!("{}:{}: bad character: {}", path, y, c); }
            else { eprintln!("{}:{}: incomplete name definition", path, y); }
            res += 1;
            continue;
        } 
        if v.len() > 2 {
            eprintln!("{}:{}: unrecognized rule", path, y);
            res += 1;
            continue;
        }    
        key = v.first().unwrap().to_string();
        value = v.last().unwrap().to_string();
        if !key.is_empty() && !value.is_empty() {
            let mut tok: Token = Token::default();
            tok.update(NamePattern, key, value, y, 0);
            tok_list.push(tok);
        }
    }
    res
}

pub fn lexer_loop(path: &str)
{
    let mut should_die: i32 = 0;
    let mut tok_def: Vec<Token> = vec![];
    let content: String = fs::read_to_string(path).unwrap();
    let mut lines: Peekable<std::str::Lines<'_>>  = content.lines().peekable();
    
    should_die += get_lex_def(path, &mut tok_def, &mut lines);
    for (i, tok) in tok_def.iter().enumerate() {
        println!("Token[{}]:\n Type: {:?}\n Key: [START]{}[END]\n Value: {}\n Line: {}\n Column: {}", 
            i, tok.gettype(), tok.getkey(), tok.getvalue(), tok.getline(), tok.getcolumn()
        );
    }

    if should_die > 0 {
        process::exit(1);
    }
}