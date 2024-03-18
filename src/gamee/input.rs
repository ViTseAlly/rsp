use super::game::Rps;

use std::io::{self, Write};
use std::collections::HashMap;


pub struct Input {
  pub val: Option<String>
}

impl Input {
  pub fn new() -> Self {
    Input { val: None }
  }

  pub fn inp(&mut self, _txt: String) -> &mut Self {
    print!("{}", _txt);
    io::stdout().flush().expect("Cant flushed input.");
    let mut usr_inp: String = String::new();
    io::stdin()
        .read_line(&mut usr_inp)
        .expect("Cant read line.");
    self.val = Some(usr_inp);
    self
  }

  pub fn to_i32(&mut self) -> Result<i32, ()> {
    if let Some(el) = &self.val {
      match el.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(()), 
      }
    } else {
      Err(())
    }
  }

  pub fn get_move(input: &i32) -> Rps {
    match input {
        1 => Rps::Rock,
        2 => Rps::Scissor,
        3 => Rps::Paper,
        _ => panic!("Invalid move number"),
    }
  }
}