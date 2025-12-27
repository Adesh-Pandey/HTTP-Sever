use std::{
    fs::{self},
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

use http::ThreadPool;

const MAX_BUFFER_LENGTH: usize = 10 * 1024 * 1024;

fn get_error_data_bytes() -> String {
    let path = Path::new("content/404.html");
    let v = fs::read_to_string(path).unwrap();
    return v;
}

fn find_request_header_end(buf: &Vec<u8>) -> Option<usize> {
    let pattern = b"\r\n\r\n";
    return buf.windows(pattern.len()).position(|w| w == pattern);
}

fn handle_stream(stream: &mut TcpStream) -> Result<(), io::Error> {
    let mut buf: Vec<u8> = vec![];
    let req_string: String;
    let mut content_length_to_read = 0;
    const BYTES_TO_READ: usize = 512;
    let mut chunk = [0; BYTES_TO_READ];

    loop {
        if buf.len() > MAX_BUFFER_LENGTH {
            let status = "HTTP/1.1 400 Bad Request";
            println!("Too large request");
            stream.write(status.as_bytes()).unwrap();
            stream.flush().unwrap();
            return Ok(());
        }

        println!("Reading {} bytes chunck", BYTES_TO_READ);
        let read = stream.read(&mut chunk)?;

        if read == 0 {
            // connection was closed;
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "connection closed",
            ));
        }

        buf.extend_from_slice(&chunk);

        // try to find \r\n\r\n to parse header;
        if let Some(pos) = find_request_header_end(&buf) {
            // find content length to read rest of the request.
            let current_string = String::from_utf8_lossy(&buf[..pos]).to_string();

            current_string.lines().for_each(|line| {
                if let Some(v) = line.strip_prefix("Content-Length:") {
                    let val: Option<usize> = v.trim().parse().ok();
                    // make sure we read only things we do not
                    // have in the buffer
                    if let Some(value) = val
                        && value + pos + 4 > buf.len()
                    {
                        content_length_to_read = value;
                    }
                };
            });
            break;
        }
    }

    while content_length_to_read > 0 {
        if buf.len() > MAX_BUFFER_LENGTH {
            let status = "HTTP/1.1 400 Bad Request";
            println!("Too large request");
            stream.write(status.as_bytes()).unwrap();
            stream.flush().unwrap();
            return Ok(());
        }
        println!("Reading {} bytes chunck", BYTES_TO_READ);
        let read = stream.read(&mut chunk)?;

        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "connection closed",
            ));
        }
        buf.extend_from_slice(&chunk);
        content_length_to_read = if content_length_to_read < BYTES_TO_READ {
            0
        } else {
            content_length_to_read - BYTES_TO_READ
        };
    }

    req_string = String::from_utf8_lossy(&buf).to_string();

    println!("Complete Request \n\n {}", req_string);

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
        return Ok(());
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
    return Ok(());
}

fn read_conn_2() -> io::Result<()> {
    let thread_pool = ThreadPool::new(10);

    let tcp_listener = TcpListener::bind("127.0.0.1:3001")?;

    for stream in tcp_listener.incoming() {
        println!("Stream came in ");
        thread_pool.execute(move || {
            let _ = handle_stream(&mut stream.unwrap());
        });
    }

    Ok(())
}

fn main() {
    let _ = read_conn_2();
}
