use std::fs;
use super::Todo;

pub fn load_from_file() -> Vec<Todo> {
  let file_content = fs::read_to_string("todo.txt")
      .expect("Something went wrong reading the file storage");

  let contents = file_content.lines();
  let mut todos: Vec<Todo> = Vec::new();
  for c in contents {
      if let [status, text, created_at] = c.split("#").collect::<Vec<&str>>()[..] {
          let todo = Todo {
              content: text.to_owned(),
              is_done: match status {
                  "1" => true,
                  _ => false,
              },
              created_at: created_at.parse::<u64>().unwrap(),
          };
          todos.push(todo);
      }
  }
  todos
}

pub fn save_to_file(todos: Vec<Todo>) {
  let mut content = String::from("");
  for todo in todos {
    content.push_str("\n");
    let status = if todo.is_done { "1" } else { "0" };
    let todo_str = format!("{}#{}#{}", status, todo.content, todo.created_at);
    content.push_str(&todo_str);
  };
  fs::write("todo.txt", content).expect("Unable to write file");
}