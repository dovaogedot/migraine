use std::{io::Write, process::{Command, Stdio}};

pub fn send_to_visualizer(image: &[[f32; 3]]) {
    let bytes: Vec<u8> = image.iter().flat_map(|c| c).flat_map(|f| f.to_le_bytes()).collect();

    let mut child = Command::new("curl")
        .args([
            "-X",
            "POST",
            "http://localhost:3000/data",
            "-H",
            "Content-Type: application/octet-stream",
            "--data-binary",
            "@-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(&bytes).unwrap();
    child.wait().unwrap();
}
