use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

const server_URI: &str = "127.0.0.1:7777";
const no_of_threads: usize = 4;

fn limited_thread_pool() {
    let listener = TcpListener::bind(server_URI).unwrap();
    let pool = ThreadPool::new(no_of_threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn thread_per_request() {
    // a thread pool is a group of threads that are spawned
    // and ready to use

    // fork-join model
    // single-thread async I/O model
    // multi-thread async I/O model

    // compiler driven development

    // spawning a thread for every request is a bad idea
    // because it can cause the whole server machine to
    // be blocked by a DoS attach

    // with a limited number of threads, the machine could also
    // run something else
    let listener = TcpListener::bind(server_URI).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let thunk = || {
            handle_connection(stream);
        };
        thread::spawn(thunk);
    }
}

fn single_threaded_server() {
    let listener = TcpListener::bind(server_URI).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // full slice of request Line  &request_line[..]
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // single_threaded_server();

    // thread_per_request();

    limited_thread_pool();
}
