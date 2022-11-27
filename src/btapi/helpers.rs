use std::io::{stdin, stdout, Write};

pub fn get_user_input(input: &mut String) {
    let _ = stdout().flush();
    stdin()
        .read_line(input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}
