use std::io::{stdin, stdout, Write};

pub fn to_usize(n1: isize) -> usize {
    if n1 >= 0 {
        n1 as usize
    } else {
        0
    }
}

pub fn max(n1: isize, n2: isize) -> usize {
    if n1 > n2 {
        to_usize(n1)
    } else {
        to_usize(n2)
    }
}

pub fn min(n1: isize, n2: isize) -> usize {
    if n1 < n2 {
        to_usize(n1)
    } else {
        to_usize(n2)
    }
}

pub fn get_input() -> String {
    let mut option: String = String::new();

    let _ = stdout().flush();
    stdin()
        .read_line(&mut option)
        .expect("Did not enter a correct string.");
    if let Some('\n') = option.chars().next_back() {
        option.pop();
    }
    if let Some('\r') = option.chars().next_back() {
        option.pop();
    }

    option
}

pub fn get_input_with_message(str: &str) -> String {
    print!("{}  ", str);

    let mut option: String = String::new();

    let _ = stdout().flush();
    stdin()
        .read_line(&mut option)
        .expect("Did not enter a correct string.");
    if let Some('\n') = option.chars().next_back() {
        option.pop();
    }
    if let Some('\r') = option.chars().next_back() {
        option.pop();
    }

    println!();

    option
}

pub fn split() {
    println!("\n\n");
}
