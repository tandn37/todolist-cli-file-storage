use std::io;
use std::num::ParseIntError;

use super::GetIdError;

pub fn show_main_menu() {
  println!("\n============================
Please select your command:
      1, List
      2, Add
      3, Update
      4, Remove
      5, Exit
============================
  ");
}

pub fn get_text_input() -> String {
  let mut content = String::new();
      io::stdin()
          .read_line(&mut content)
          .expect("Failed to read todo");
  content
}

pub fn get_input_todo_id(max_id: usize) -> Result<usize, GetIdError> {
  println!("Please input todo id:");
  let id = get_text_input();
  let index: Result<usize, ParseIntError> = id.trim().parse();
  match index {
      Err(_) => {
          return Err(GetIdError)
      },
      Ok(value) => {
          if value < 1 || value > max_id {
              return Err(GetIdError)
          }
          let ind = value - 1;
          return Ok(ind);
      }
  }
}