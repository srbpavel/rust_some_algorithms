use rust_some_algorithms::strings;

//
fn main() {
    println!("i am: {:?}",
             strings::who_am_i(),
    );

    let input = "carponizer";
    let output = strings::string_to_chars(input);

    println!("{} -> {:?}",
             input,
             output,
    );

    let output_upper = strings::string_to_chars_uppercase(input);

    println!("{} -> {:?}",
             input,
             output_upper,
    );
}
