pub fn check_type(c_type: &str) -> String {
    match c_type {
        "echo" | "exit" | "type" | "pwd" | "cd" => format!("{} is a shell builtin", c_type),
        _ => format!("{}: not found", c_type)
    }
}