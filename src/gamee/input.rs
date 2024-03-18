use std::io;


pub struct Input {
  pub val: Option<String>
}

impl Input {
  pub fn new() -> Self {
    Input { val: None }
  }

  pub fn inp(&mut self) -> &mut Self {
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
}