mod my_struct;
mod common;
mod function;
mod file;

pub use common::show_main_menu;
pub use common::get_text_input;
pub use common::get_input_todo_id;
pub use function::list;
pub use function::add;
pub use function::update;
pub use function::remove;
pub use my_struct::Todo;
pub use my_struct::GetIdError;
pub use file::load_from_file;
pub use file::save_to_file;