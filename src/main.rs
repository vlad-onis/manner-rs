use colored::Colorize;
use colored::CustomColor;
use std::io;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let tldr_list_output = Command::new("sh")
        .arg("-c")
        .arg("tldr --quiet --list | shuf -n1")
        .output()?;

    if !tldr_list_output.status.success() {
        eprintln!("Failed to get list of tldr commands.");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Command execution failed",
        ));
    }

    let random_command = String::from_utf8_lossy(&tldr_list_output.stdout)
        .trim()
        .to_string();

    let command_title_color_bright_orange = CustomColor {
        r: 255,
        g: 192,
        b: 0,
    };
    let command_title = random_command
        .to_uppercase()
        .bold()
        .custom_color(command_title_color_bright_orange);

    println!("\n\t\t{}", command_title);

    // Output the tldr page for the random command
    // By not capturing the output, the ANSI color codes are preserved
    let mut child = Command::new("tldr")
        .arg("--quiet")
        .arg(random_command)
        .stdout(Stdio::inherit()) // Inherit the stdout to preserve color formatting
        .stderr(Stdio::inherit()) // Inherit the stderr as well
        .spawn()?;

    // Wait for the tldr command to complete
    let _ = child.wait();

    Ok(())
}
