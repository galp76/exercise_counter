use std::net::{ TcpListener, TcpStream };
use std::io::{ prelude::*, BufReader };
use std::fs;

fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let listener = TcpListener::bind("0.0.0.0:10000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
            handle_connection(stream);
        }

        println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
	.lines()
	.map(|result| result.unwrap())
	.take_while(|line| !line.is_empty())
	.collect();

    // Imprimiendo HTTP_REQUEST para debugging
    println!("Contenido de HTTP_REQUEST: {:?}", http_request);
    let (status_line, filename) =
        if http_request.len() == 0 {
	    ("HTTP/1.1 200 OK", "hello.xhtml")
        } else {
            let binding = http_request[0].clone();
            let parts: Vec<&str> = binding
                .split(" ")
                .collect();

            if parts[1] == "/" || parts[1] == "/?" {
                ("HTTP/1.1 200 OK", "hello.xhtml")
            } else {
                match &parts[1][..3] {
                    "/fl" => {
                        let quantity = &parts[1][3..];
                        exercise_app_version_4::data::newData("f", quantity);

                        ("HTTP/1.1 200 OK", "hello.xhtml")
                    },
                    "/re" => {
                        let quantity = &parts[1][3..];
                        exercise_app_version_4::data::newData("r", quantity);

                        ("HTTP/1.1 200 OK", "hello.xhtml")
                    },
                    "/pe" => {
                        let quantity = &parts[1][3..];
                        exercise_app_version_4::data::newData("p", quantity);

                        ("HTTP/1.1 200 OK", "hello.xhtml")
                    },
                    _ => ("HTTP/1.1 404 NOT FOUND", "404.xhtml"),
                }
            }
	};
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{}\r\nContent length: {}\r\n\r\n{}",
		       status_line,
		       length,
		       contents);
    
    stream.write_all(response.as_bytes()).unwrap();
}
