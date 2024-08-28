use crate::expr::LitValue;
use std::process::Command;

pub fn clock(_args: Vec<LitValue>) -> LitValue {
    LitValue::Number(
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64(),
    )
}

pub fn clear(_args: Vec<LitValue>) -> LitValue {
    Command::new(match std::env::consts::OS {
        "windows" => "powershell",
        "macos" => "terminal",
        "linux" => "sh",
        other => panic!("Not implemented for {other}"),
    })
    .arg("clear")
    .output()
    .unwrap();

    LitValue::Nil
}
