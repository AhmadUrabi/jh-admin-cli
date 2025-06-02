use std::io::{stdin, stdout, Write};

pub fn get_input(message: &str) -> String {
    print!("{}: ", message);
    let mut buffer = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

#[inline(always)]
pub(crate) fn is_quit(input: &str) -> bool {
    input == "q" || input == "Q"
}


#[inline(always)]
pub(crate) fn select_index(input: &str, len: usize) -> Option<usize> {
    input.parse::<usize>().ok().filter(|&index| index > 0 && index <= len)
}

#[inline(always)]
pub(crate) fn clear_screen() {
    print!("{}[2J", 27 as char);
}
