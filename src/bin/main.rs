use std::{
    fs::{self},
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

use http::ThreadPool;

fn get_error_data_bytes() -> String {
    let path = Path::new("content/404.html");

    let v = fs::read_to_string(path).unwrap();
    return v;
}

fn handle_stream(stream: &mut TcpStream) {
    let mut buf = [0; 1024];
    let mut req_string: String = String::new();
    match stream.read(&mut buf) {
        Ok(a) => {
            println!("Read {} bytes and loosy string is ", a);

            req_string = String::from_utf8_lossy(&buf).to_string();
            println!("{}", req_string);
        }
        Err(r) => {
            println!("ERROR happened {}", r.to_string());
        }
    };

    if !req_string.starts_with("GET /something HTTP/1.1") {
        let v = get_error_data_bytes();

        let status = "HTTP/1.1 404 Not Found";
        let final_value = format!(
            "{}\r\nContent-Length:{}\r\n\r\n{}",
            status,
            v.as_bytes().len(),
            v
        );

        stream.write(final_value.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    // send content of file index.html
    let path = Path::new("content/index.html");

    let v = fs::read_to_string(path).unwrap();
    let status = "HTTP/1.1 200 OK";
    let final_value = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status,
        v.as_bytes().len(),
        v
    );

    stream.write(final_value.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn read_conn_2() -> io::Result<()> {
    let thread_pool = ThreadPool::new(10);

    let tcp_listener = TcpListener::bind("127.0.0.1:3001")?;

    for stream in tcp_listener.incoming() {
        print!("Stream came in ");
        thread_pool.execute(move || handle_stream(&mut stream.unwrap()));
    }

    Ok(())
}

fn main() {
    let _ = read_conn_2();
}
