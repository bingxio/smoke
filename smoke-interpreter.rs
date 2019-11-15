use std::io::{Write, Read, stdin, stdout};
use std::fs::File;
use std::fmt::Debug;
use std::str::Chars;
use std::mem::discriminant;

use crate::TypeDef::*;

#[derive(Debug, Clone)]
enum TypeDef { L, R, A, M, LB, RB, D, S, C, T }

fn typedef_eq(a: &TypeDef, b: &TypeDef) -> bool {
  discriminant(a) == discriminant(b)
}

#[derive(Debug)]
struct Token { typedef: TypeDef, line: i32 }

impl Token {
  fn new(typedef: TypeDef, line: i32) -> Self {
    Token { typedef, line }
  }
}

fn tokenizer(mut chars: Chars) -> (bool, Vec<Token>) {
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
      '<' => tokens.push(Token::new(L, line)),
      '>' => tokens.push(Token::new(R, line)),
      '+' => tokens.push(Token::new(A, line)),
      '-' => tokens.push(Token::new(M, line)),
      '[' => tokens.push(Token::new(LB, line)),
      ']' => tokens.push(Token::new(RB, line)),
      '.' => tokens.push(Token::new(D, line)),
      ',' => tokens.push(Token::new(C, line)),
      '!' => tokens.push(Token::new(T, line)),
      '*' => tokens.push(Token::new(S, line)),
      ch @ _ => {
        eprintln!("SyntaxErr: unknown char: {}", ch);
        return (false, tokens);
      }
    }
  }

  (true, tokens)
}

struct Chunk {
  tokens: Vec<Token>,
  // memory -> 10000 * i32
  memory: [i32; 10000],
  // The position of every layer in memory.
  // 0 -> 10000
  p: i32,
  // Index of the token list range.
  pos: i32,
  // If running in repl mode to do something.
  repl_mode: bool,
  // If the program has some error to handle repl.
  has_err: bool
}

impl Chunk {
  fn execute(&mut self) {
    while let Some(token) = self.tokens.get(self.pos as usize) {
      let cloned = token.typedef.clone();

      self.statement(cloned);
      self.pos += 1;
    }
  }

  fn handle_err(&mut self, message: &str) {
    eprintln!("{}", message);
    self.has_err = true;
  }

  fn statement(&mut self, typedef: TypeDef) {
    match typedef {
      L =>  self.l_stmt(),
      R =>  self.r_stmt(),
      A =>  self.a_stmt(),
      M =>  self.m_stmt(),
      LB => self.lb_stmt(),
      RB => self.rb_stmt(),
      D =>  self.d_stmt(),
      S =>  self.s_stmt(),
      C =>  self.c_stmt(),
      T =>  self.t_stmt(),
    }
  }

  fn get_memory_value(&self, pos: i32) -> i32 {
    self.memory[pos as usize]
  }

  fn l_stmt(&mut self) {
    if self.p != 0 {
      self.p -= 1;
    }
  }

  fn r_stmt(&mut self) {
    self.p += 1;
  }

  fn a_stmt(&mut self) {
    self.memory[self.p as usize] += 1;
  }

  fn m_stmt(&mut self) {
    self.memory[self.p as usize] -= 1;
  }

  fn lb_stmt(&mut self) {
    self.pos += 1;

    let p_backup = self.p;
    let pos_backup = self.pos;

    while self.get_memory_value(p_backup) != 0 {
      if self.pos as usize >= self.tokens.len() {
        self.handle_err("SyntaxErr: expect right bracket that program was end.");
        break;
      }

      let tok = self.tokens.get(self.pos as usize).unwrap();
      let cloned = tok.typedef.clone();

      if typedef_eq(&tok.typedef, &RB) {
        self.pos = pos_backup;
        continue;
      }

      self.statement(cloned);
      self.pos += 1;
    }
  }

  fn rb_stmt(&mut self) {
    // TODO: The right bracket was skiped that ignore it.
  }

  fn d_stmt(&mut self) {
    print!("{}{}", self.memory[self.p as usize] as u8 as char, if self.repl_mode { '\n' } else { ' ' });
  }

  fn s_stmt(&mut self) {
    print!("{}{}", self.memory[self.p as usize], if self.repl_mode { '\n' } else { ' ' });
  }

  fn c_stmt(&mut self) {
    let mut line = String::new();

    stdin().read_line(&mut line).expect("Failed to read line !");

    match line.trim().parse::<i32>() {
      Ok(val) => {
        self.memory[self.p as usize] = val;
      }
      Err(_) => self.handle_err("Please input a number.")
    }
  }

  fn t_stmt(&mut self) {
    let mut skip = 50;

    for (i, v) in self.memory.iter().enumerate() {
      print!("{} ", v);

      if i + 1 % 50 == 0 {
        println!();
      }

      if i == skip {
        println!("\nPress \"n\" to next and press other to quit.");

        let mut line = String::new();

        stdin().read_line(&mut line).expect("Failed to read line !");

        if line.trim_end().eq(&"n".to_string()) {
          skip += 50;
          continue;
        } else {
          break;
        }
      }
    }
  }
}

fn run_file(path: String) {
  if path.ends_with(".sk") == false {
    eprintln!("You should use .sk file suffix only !");
  } else {
    let mut file = File::open(path).
      expect("Failed to read file !");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Cannot read buffer to String.");

    Chunk {
      tokens: tokenizer(contents.chars()).1,
      memory: [0; 10000],
      p: 0,
      pos: 0,
      repl_mode: false,
      has_err: false
    }.execute();
  }
}

fn run_repl() {
  println!("Smoke 1.0.0 [dev, Nov 10 2019, 17:18] Hello, nice to meet you >_ !");
  println!("Type \"help\", \"copyright\", \"license\" for more information.");

  let mut memory: [i32; 10000] = [0; 10000];
  let mut p: i32 = 0;

  fn input_help() {
    let help = "
      Welcome to Smoke 1.0's help utility !
    ";
    println!("{}", help);
  }

  fn input_copyright() {
    let license = "
      Copyright [2019] [Turaiiao]

      Licensed under the Apache License, Version 2.0 (the \"License\");
      you may not use this file except in compliance with the License.
      You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

      Unless required by applicable law or agreed to in writing, software
      distributed under the License is distributed on an \"AS IS\" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
      See the License for the specific language governing permissions and
      limitations under the License.
      ";
    println!("{}", license);
  }

  fn input_license() {
    let license = "
      The Smoke is open source ! More information to type \"copyright\".

      You can star this project on GitHub: https://github.com/turaiiao/smoke
    ";
    println!("{}", license);
  }

  loop {
    print!(">>> ");

    let mut line = String::new();

    stdout().flush().expect("Failed to flush the screen !");
    stdin().read_line(&mut line).expect("Failed to read line !");

    if line.trim_end().len() > 0 {
      match line.as_str().trim_end() {
        "help"      => input_help(),
        "copyright" => input_copyright(),
        "license"   => input_license(),
        "exit"      => break,
        _=> {
          let tokens = tokenizer(line.chars());

          if tokens.0 {
            let mut chunk = Chunk { tokens: tokens.1, memory, p, pos: 0, repl_mode: true, has_err: false };

            chunk.execute();

            if !chunk.has_err {
              memory = chunk.memory;
              p = chunk.p;
            }
          }
        }
      }
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
