use std::process::Command;

pub fn main() {
    let shuf_command = "shuf";
    let tldr_command = "tldr";

    // check for shuf command
    let result = Command::new("bash")
        .arg("-c")
        .arg(format!("which {}", shuf_command))
        .output()
        .expect("failed to execute bash command");

    if !result.status.success() {
        println!(
            r#"cargo:warning={} is not installed. To install it (on osx) run "brew install coreutils""#,
            shuf_command
        );
        panic!("Failed to build executable due to missing tldr command");
    }

    // check for tldr command
    let result = Command::new("bash")
        .arg("-c")
        .arg(format!("which {}", tldr_command))
        .output()
        .expect("failed to execute bash command");

    if !result.status.success() {
        println!(
            r#"cargo:warning={} is not installed. To install it run "cargo install tealdeer""#,
            tldr_command
        );
        panic!("Failed to build executable due to missing tldr command");
    }
}
