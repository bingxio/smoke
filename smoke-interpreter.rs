use std::io::{Write, Read};
use std::fs::File;
use std::fmt::{Display, Formatter, Error};
use std::str::Chars;
use crate::TypeDef::*;

#[derive(Debug)]
enum TypeDef { L, R, A, M, LB, RB, D, C, T }

#[derive(Debug)]
struct Token { typedef: TypeDef, literal: char, line: i32 }

impl Token {
  fn new(typedef: TypeDef, literal: char, line: i32) -> Self {
    Token { typedef, literal, line }
  }
}

fn tokenizer(mut chars: Chars) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  let mut line = 1;

  while let Some(c) = chars.next() {
    match c {
      ' ' | '\t' | '\r' | '\n' => {
        if c == '\n' {
          line += 1;
        }
        continue;
      }
      '#' => {
        while let Some(c) = chars.next() {
          if c == '\n' {
            line += 1;
            break;
          }
        }
      }
      ch @ '<' => tokens.push(Token::new(L, ch, line)),
      ch @ '>' => tokens.push(Token::new(R, ch, line)),
      ch @ '+' => tokens.push(Token::new(A, ch, line)),
      ch @ '-' => tokens.push(Token::new(M, ch, line)),
      ch @ '[' => tokens.push(Token::new(LB, ch, line)),
      ch @ ']' => tokens.push(Token::new(RB, ch, line)),
      ch @ '.' => tokens.push(Token::new(D, ch, line)),
      ch @ ',' => tokens.push(Token::new(C, ch, line)),
      ch @ '!' => tokens.push(Token::new(T, ch, line)),
      ch @ _ => {
        eprintln!("SyntaxErr: unknown char: {}", ch);
        std::process::exit(1);
      }
    }
  }

  tokens
}

struct Chunk {
  tokens: Vec<Token>,
  // memory -> 10000 * i32
  memory: [i32; 10],
  // The position of every layer in memory.
  // 0 -> 100
  p: i32
}

impl Display for Chunk {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    write!(f, "Chunk -> [ memory size: {} ]", self.memory.len())
  }
}

impl Chunk {
  fn execute(&mut self) {
    let mut position = 0;

    loop {
      if position == self.tokens.len() {
        break;
      }

      let ins = self.tokens.get(position).unwrap();

      match ins.typedef {
        L => {
          if self.p != 0 {
            self.p -= 1;
          }
        }
        R => {
          self.p += 1;
        },
        A => self.memory[self.p as usize] += 1,
        M => self.memory[self.p as usize] -= 1,
        LB => {

        }
        RB => {

        }
        D => {
          println!("{} -> {}", self.p, self.memory[self.p as usize])
        },
        C => {

        },
        T => {
          for i in self.memory.iter() {
            print!("{} ", i);
          }
          println!();
        }
      }

      position += 1;
    }
  }
}

static MEMORY: [i32; 10] = [0; 10];
static P: i32 = 0;

fn run(source: &mut String) {
  Chunk {
    tokens: tokenizer(source.chars()),
    memory: MEMORY,
    p: P
  }.execute();

  println!("p = {}", P);
}

fn run_file(path: String) {
  if path.ends_with(".sk") == false {
    eprintln!("You should use .sk file suffix only !");
  } else {
    let mut file = File::open(path).
      expect("Failed to read file !");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Cannot read buffer to String.");

    run(&mut contents);
  }
}

fn run_repl() {
  println!("The smoke programming language interpreter [ version 1.0.0 ]");

  loop {
    print!(">>> ");

    let mut line = String::new();

    std::io::stdout().flush().expect("Failed to flush the screen !");
    std::io::stdin().read_line(&mut line).expect("Failed to read line !");

    if line.trim_end().len() > 0 {
      run(&mut line);
    }
  }
}

fn main() {
  let mut args = std::env::args();

  if args.len() == 2 {
    run_file(args.nth(1).unwrap());
  } else {
    run_repl();
  }
}