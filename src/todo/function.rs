use chrono::{DateTime, Utc, Local};
use std::time::{UNIX_EPOCH, Duration};

use super::{ Todo, get_text_input, get_input_todo_id, save_to_file };

pub fn list(todo_list: &Vec<Todo>) {
  if todo_list.len() == 0 {
      return println!("List is empty");
  }
  println!("List:");
  for (i, todo) in todo_list.iter().enumerate() {
      let d = UNIX_EPOCH + Duration::from_secs(todo.created_at);
      let datetime = DateTime::<Local>::from(d);
      let time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
      let status = if todo.is_done {
          "Done"
      } else {
          "NotDone"
      };
      println!("{}: ({}) {} - ({})", i + 1, status, todo.content, time_str);
  }
}

pub fn create(text: String) -> Todo {
  let now = Utc::now();
  let created_at = now.timestamp();
  Todo {
      content: text,
      is_done: false,
      created_at: created_at as u64,
  }
}

pub fn add(todo_list: &mut Vec<Todo>) {
  println!("Please input your todo:");
  let content = get_text_input();
  if content.contains("#") {
    println!("Todo can not has # character");
    return;
  }
  let todo = create(content.trim().to_string());
  todo_list.push(todo);
  save_to_file(todo_list.to_vec());
  println!("Added successful!");
  list(todo_list);
}

pub fn remove(todo_list: &mut Vec<Todo>) {
  println!("Please input todo id you want to remove:");
  let id = get_input_todo_id(todo_list.len());
  match id {
    Err(err) => {
      println!("{}", err);
    }
    Ok(value) => {
      todo_list.remove(value);
      save_to_file(todo_list.to_vec());
      println!("Removed successful!");
      list(todo_list);
    }
  }
}

pub fn update(todo_list: &mut Vec<Todo>) {
  println!("Please input action you want to update:");
  println!("
      1, Update Text
      2, Mark as done
      3, Mark as not done
      4, Back
  ");
  let action = get_text_input();
  
  match action.trim() {
    "1" => {
        println!("Please input new text:");
        let text = get_text_input();
        let id = get_input_todo_id(todo_list.len());
        match id {
          Err(err) => {
            println!("{}", err);
          }
          Ok(value) => {
            todo_list[value].content = text.trim().to_owned();
            save_to_file(todo_list.to_vec());
            println!("Updated successful!");
            list(todo_list);
          }
        }
    },
    "2" => {
        let id = get_input_todo_id(todo_list.len());
        match id {
          Err(err) => {
            println!("{}", err);
          }
          Ok(value) => {
            todo_list[value].is_done = true;
            save_to_file(todo_list.to_vec());
            println!("Updated successful!");
            list(todo_list);
          }
        }
    },
    "3" => {
        let id = get_input_todo_id(todo_list.len());
        match id {
          Err(err) => {
            println!("{}", err);
          }
          Ok(value) => {
            todo_list[value].is_done = false;
            save_to_file(todo_list.to_vec());
            println!("Updated successful!");
            list(todo_list);
          }
        }
    },
    "4" => {},
    _ => println!("Invalid option"),
  }
}