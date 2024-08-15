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

    let binding = http_request[0].clone();
    let parts: Vec<&str> = binding
	.split(" ")
	.collect();

    let (status_line, filename) =
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
		/*"/sh" => {
		    let tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.show();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/st" => {
		    rust_to_do_web_app::tasks_and_banks::tasks::show_tasks();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/pr" => {
		    let parts: Vec<&str> = parts[1][3..]
			.split("*")
			.collect();
		    let mut tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.withdraw(0, parts[0].parse::<f32>().unwrap(), parts[1]);
		    tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.show();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/bi" => {
		    let parts: Vec<&str> = parts[1][3..]
			.split("*")
			.collect();
		    let mut tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.withdraw(1, parts[0].parse::<f32>().unwrap(), parts[1]);
		    tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.show();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/bf" => {
		    let parts: Vec<&str> = parts[1][3..]
			.split("*")
			.collect();
		    let mut tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.withdraw(2, parts[0].parse::<f32>().unwrap(), parts[1]);
		    tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.show();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/ef" => {
		    let parts: Vec<&str> = parts[1][3..]
			.split("*")
			.collect();
		    let mut tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.withdraw(3, parts[0].parse::<f32>().unwrap(), parts[1]);
		    tmp = rust_to_do_web_app::tasks_and_banks::banks::Total::new_from_files();
		    tmp.show();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/de" => {
		    let number = parts[0][3..].parse::<usize>().unwrap();
		    rust_to_do_web_app::tasks_and_banks::tasks::delete_task(number).unwrap();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/ad" => {
		    let new_task = parts[1][3..].replace("%20", " ");
		    rust_to_do_web_app::tasks_and_banks::tasks::add_task(&new_task);
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},
		"/mo" => {
		    let parts: Vec<&str> = parts[1][3..]
			.split("*")
			.collect();
		    let from = parts[0].parse::<usize>().unwrap();
		    let to = parts[1].parse::<usize>().unwrap();
		    rust_to_do_web_app::tasks_and_banks::tasks::interchange_tasks(from, to).unwrap();
		    ("HTTP/1.1 200 OK", "german.xhtml")
		},*/
		_ => ("HTTP/1.1 404 NOT FOUND", "404.xhtml"),
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
