use crate::expr::Literal;

pub fn clock(_args: Vec<Literal>) -> Result<Literal, String> {
    use std::time::SystemTime;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => Ok(Literal::Number(d.as_secs_f64())),
        Err(msg) => Err(msg.to_string())
    }
}

pub fn clear(_args: Vec<Literal>) -> Result<Literal, String> {
    use std::{process::Command, env::consts::OS};

    match Command::new(match OS {
        "windows" => "powershell",
        "macos" => "terminal",
        "linux" => "sh",
        _ => return Err("Commands only implemented for windows, macos and linux".to_string()),
    })
    .arg("clear")
    .output() {
        Ok(_) => Ok(Literal::Nil),
        Err(msg) => Err(msg.to_string()),
    }
}
