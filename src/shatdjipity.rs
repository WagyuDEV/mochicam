use opencv::{
    core::{Mat, Vector},
    imgcodecs, prelude::*, videoio,
};

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on http://0.0.0.0:8080");

    let cam = Arc::new(Mutex::new(
        videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Failed to open camera"),
    ));

    if !videoio::VideoCapture::is_opened(&cam.lock().unwrap()).unwrap() {
        panic!("Camera not available");
    }

    for stream in listener.incoming() {
        let cam = cam.clone();
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    handle_client(&mut stream, &cam);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(stream: &mut TcpStream, cam: &Arc<Mutex<videoio::VideoCapture>>) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        return;
    }

    let request = String::from_utf8_lossy(&buffer);

    if request.starts_with("GET /stream") {
        if !check_auth(&request) {
            send_unauthorized(stream);
            return;
        }
        serve_stream(stream, cam);
    } else {
        serve_html(stream);
    }
}

// Checks if Authorization header matches "user:pass"
fn check_auth(request: &str) -> bool {
    // Replace "user:pass" base64 with your desired credentials
    const EXPECTED_AUTH: &str = "Basic dXNlcjpwYXNz"; // base64("user:pass")

    for line in request.lines() {
        if line.starts_with("Authorization:") {
            // Header looks like: Authorization: Basic dXNlcjpwYXNz
            let auth_val = line.trim_start_matches("Authorization:").trim();
            if auth_val == EXPECTED_AUTH {
                return true;
            }
        }
    }
    false
}

fn send_unauthorized(stream: &mut TcpStream) {
    let response = "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"Restricted\"\r\nContent-Length: 12\r\n\r\nUnauthorized";
    let _ = stream.write_all(response.as_bytes());
}

fn serve_html(stream: &mut TcpStream) {
    let contents = r#"<!DOCTYPE html>
<html>
<head>
    <title>Rust Webcam Stream</title>
    <style>
        body { font-family: sans-serif; background: #111; color: white; text-align: center; }
        img { margin-top: 20px; max-width: 100%; }
    </style>
</head>
<body>
    <h1>Live Webcam Stream (Protected)</h1>
    <p>This stream is password protected. Enter <code>user</code> / <code>pass</code> when prompted.</p>
    <img src="/stream" />
</body>
</html>"#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    let _ = stream.write_all(response.as_bytes());
}

fn serve_stream(stream: &mut TcpStream, cam: &Arc<Mutex<videoio::VideoCapture>>) {
    let mut frame = Mat::default();
    let mut buf = Vector::new();

    let header =
        "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n";
    if stream.write_all(header.as_bytes()).is_err() {
        return;
    }

    loop {
        {
            let mut cam = match cam.lock() {
                Ok(lock) => lock,
                Err(_) => break,
            };

            if cam.read(&mut frame).is_err() {
                break;
            }

            buf.clear();
            if imgcodecs::imencode(".jpg", &frame, &mut buf, &Vector::new()).is_err() {
                break;
            }
        }

        let image_header = format!(
            "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
            buf.len()
        );

        if stream.write_all(image_header.as_bytes()).is_err()
            || stream.write_all(buf.as_slice()).is_err()
            || stream.write_all(b"\r\n").is_err()
        {
            break;
        }

        let _ = stream.flush();
        thread::sleep(Duration::from_millis(33));
    }
}
