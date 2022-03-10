use std::fmt;

#[derive(Debug, Clone)]
pub struct Todo {
    pub content: String,
    pub is_done: bool,
    pub created_at: u64,
}

pub struct GetIdError;

impl fmt::Display for GetIdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Invalid todo id")
  }
}