use ansi_term::{Colour, Style};

pub fn colorful_text() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );

    // 粗体
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );

    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
}
