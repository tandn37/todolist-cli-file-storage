mod todo;

fn main() {
    let mut todos = todo::load_from_file();
    println!("\n***-TODO List-***");

    todo::show_main_menu();
    
    loop {
        let option = todo::get_text_input();
        match option.trim() {
            "1" => todo::list(&todos),
            "2" => todo::add(&mut todos),
            "3" => todo::update(&mut todos),
            "4" => todo::remove(&mut todos),
            "5" => break,
            _ => println!("Invalid option"),
        }
        todo::show_main_menu();
    }
}