use std::process::Command;

pub fn open_in_file_explorer(path: &str) {
    // 手动拼接 explorer 命令
    let mut command = String::from("explorer /select,");
    command.push_str(path);

    println!("com: {}", command);

    let status = Command::new("cmd").args(["/C", &command]).status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("Successfully opened Explorer for: {}", path);
        }
        Ok(exit_status) => {
            eprintln!("Explorer exited with error: {:?}", exit_status.code());
        }
        Err(e) => {
            eprintln!("Failed to execute Explorer command: {}", e);
        }
    }
}
