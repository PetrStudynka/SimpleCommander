/// Simple commander for basic FS operations
/// Does not work for paths with whitespaces in Win10
use simple_commander::*;
use std::io::*;

const ERROR_CMD: &'static str = "Unknown command";
fn main() {
    println!("Write your command");

    let mut buffer = String::new();

    loop {
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input.");
        if buffer.trim().len() > 0 {
            break;
        };
    }
    let command = Command::new(buffer.trim().split(" ").collect());
    let result = match command.get_name() {
        "move" => move_dir_entry(&command),
        "copy" => copy_dir_entry(&command),
        "remove" => remove_dir_entry(&command),
        _ => Err(ERROR_CMD.to_string()),
    };

    match result {
        Ok(s) => println!(
            "Command: {} executed successfully, with result: {}",
            command.get_name(),
            s
        ),
        Err(s) => println!("{}", s),
    }
}
