use crate::gamee::game::Rps;
use crate::gamee::input::Input;

use std::collections::HashMap;


pub struct UserData {
    pub user_name: String,
    pub user_move: Option<Rps>,
}

pub struct User {
    user_data: Option<UserData>,
}

impl User {
    pub fn new() -> Self {
      let user_name = Input::new()
                                    .inp("Write your username: ".to_string())
                                    .val.take().unwrap();
      let user_move = None;
      User { user_data: Some(UserData{ user_name, user_move }) }
    }

    pub fn user_mv(&mut self) -> &mut Self {
      let fmt_text = format!("1 - Rock;\n2 - Scissors;\n3 - Paper;\nChange your move(write number):");
      let user_input_move = Input::new()
                                    .inp(fmt_text)
                                    .to_i32().unwrap();
      if [1, 2, 3].contains(&user_input_move) {
        let user_name = self.user_data.as_ref().unwrap().user_name.clone();
        let user_move = Some(Input::get_move(&user_input_move));
        self.user_data = Some(UserData { user_name, user_move });
      }
      self
    }
}