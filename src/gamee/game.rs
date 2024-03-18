use crate::gamee::*;
use self::user::UserData;



pub enum Rps {
  Rock,
  Paper,
  Scissor,
}

pub struct Gamers {
  gamer_one: UserData,
  gamer_two: UserData,
}

pub struct Game {
  pub game_run: bool,
  pub gamers: Gamers,
}

impl Game {
  pub fn new(gamer_one: UserData, gamer_two: UserData) -> Self {
    Game { 
        game_run: true, 
        gamers: Gamers {
               gamer_one,
               gamer_two 
              },
        }
  }

  pub fn check_winner(&self) -> Result<&UserData, &'static str> {
    let gamer_one_move = self.gamers.gamer_one.user_move.as_ref().unwrap_or(&Rps::Rock);
    let gamer_two_move = self.gamers.gamer_two.user_move.as_ref().unwrap_or(&Rps::Rock);

    match (gamer_one_move, gamer_two_move) {
        (Rps::Rock, Rps::Scissor) | (Rps::Paper, Rps::Rock) | (Rps::Scissor, Rps::Paper) => {
            Ok(&self.gamers.gamer_one)
        }
        (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissor) | (Rps::Scissor, Rps::Rock) => {
            Ok(&self.gamers.gamer_two)
        }
        _ => Err("It's a tie!"),
    }
}
}