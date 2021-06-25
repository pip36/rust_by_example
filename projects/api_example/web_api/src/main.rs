use domain;
use persistence;

fn main() {
    println!("Welcome!");
    let todo = persistence::get_todo();
    println!("Todo - {}", todo);
    let summary = domain::format_todo_summary(&todo);
    println!("Summary - {}", summary)
}
