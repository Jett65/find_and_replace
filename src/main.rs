// This project is a simple find and replace algorithm.
// The project will read a file find a specified string of
// characters, and replace it with a specified string.
use std::any::type_name;
use std::fs;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn find(path: &str, search: &str) /*  ->  Result<(), Error>  */
{
    let file = fs::read_to_string(path);

    println!("The type is {}", type_of(file))
}

fn main() {
    find("test.txt", "The");
}
