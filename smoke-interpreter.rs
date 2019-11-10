use std::io::{Write, Read};
use std::fs::File;
use std::fmt::{Display, Formatter, Error, Debug};
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
  // 0 -> 100
  p: i32,
  // If running in repl mode to do something.
  repl_mode: bool
}

impl Display for Chunk {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    write!(f, "Chunk -> [ memory size: {} ]", self.memory.len())
  }
}

impl Chunk {
  fn execute(&mut self) -> bool {
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
          if self.repl_mode {
            println!("{}", self.memory[self.p as usize] as u8 as char);
          } else {
            print!("{}", self.memory[self.p as usize] as u8 as char);
          }
        },
        C => {

        },
        T => {
          let mut skip = 50;

          for (i, v) in self.memory.iter().enumerate() {
            print!("{} ", v);

            if i + 1 % 50 == 0 {
              println!();
            }

            if i == skip {
              println!("\nPress `n` to next and press other to quit.");

              let mut line = String::new();

              std::io::stdin().read_line(&mut line).expect("Failed to read line !");

              if line.eq(&"n\r\n".to_string()) {
                skip += 50;
                continue;
              } else {
                break;
              }
            }
          }
          println!();
        }
      }

      position += 1;
    }

    true
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

    Chunk { tokens: tokenizer(contents.chars()).1, memory: [0; 10000], p: 0, repl_mode: true }.execute();
  }
}

fn run_repl() {
  println!("Smoke 1.0.0 [dev, Nov 10 2019, 17:18] Nice to meet you >_ !");
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

    std::io::stdout().flush().expect("Failed to flush the screen !");
    std::io::stdin().read_line(&mut line).expect("Failed to read line !");

    if line.trim_end().len() > 0 {
      match line.as_str().trim_end() {
        "help"      => input_help(),
        "copyright" => input_copyright(),
        "license"   => input_license(),
        "exit"      => break,
        _=> {
          let tokens = tokenizer(line.chars());

          if tokens.0 {
            let mut chunk = Chunk { tokens: tokens.1, memory, p, repl_mode: true };

            if chunk.execute() {
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
