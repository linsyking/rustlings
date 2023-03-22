// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data); // Wht data is not mutable but can be passed to a (mut data) argument?
    // println!("{}", data); NOT POSSIBLE! data has been MOVED
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) { // mut data doesn't require input data be mutable. Since
    // data here is owned by this function, it can change the modifier
    data = data.to_uppercase();

    println!("{}", data);
}
