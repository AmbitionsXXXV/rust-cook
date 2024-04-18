use ansi_term::colorful_text;
use cmd_line::arg_parsing;

mod ansi_term;
mod cmd_line;

fn main() {
    arg_parsing();

    colorful_text();
}
