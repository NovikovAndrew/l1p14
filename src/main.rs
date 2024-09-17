use std::any::{Any};
fn print_type_of<T: Any>(_: &T) {
    // Получаем тип переменной в виде строки
    let type_name = std::any::type_name::<T>();
    println!("Type: {}", type_name);
}

fn main() {
    let integer = 42;
    let float = 3.14;
    let text = "Hello, Rust!";

    print_type_of(&integer);
    print_type_of(&float);
    print_type_of(&text);
}

