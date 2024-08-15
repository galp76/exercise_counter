use std::{ io::Write, fs };
use chrono::{Datelike, Utc};

pub fn help() {
    println!("\nSolicitud no válida. Opciones:\npeso: p [número]\nflexiones: f [número] [cantidad de segundos para la alarma]\ncaminata: c [número]\nwalking: w\nbackup: b [fecha: AAAA-MM-DD]\nsearch: s [fecha: AAAA-MM-DD]\n");
}

fn clean_file(file_name: String) {
    std::fs::OpenOptions::new()
	.create(true)
	.write(true)
	.truncate(true)
	.open(file_name)
	.unwrap();
}

fn appendToFile(fileName: String, data: String) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(fileName)
        .unwrap();
    write!(file, "{}\n", &data).expect("Unable to write data");
}

fn createFile(data: String) {
    fs::File::create(format!("dates/{}.txt", data)).expect("Unable to create file");
}

fn getDate() -> String {
    let now = Utc::now();
    let result = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());

    result
}

fn split76(text: &str) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    let mut tmp: String = String::new();
    for ch in text.chars() {
        match ch {
            ' ' => {
                answer.push(String::from(&tmp));
                tmp.clear();
            },
            _ => tmp.push(ch)
                
        }
    }
    answer.push(tmp);

    answer
}

pub fn set_alarm(seconds: usize) {
    let stop_flag = std::sync::atomic::AtomicBool::new(false);
    let counter = std::sync::atomic::AtomicUsize::new(0);
    let limit = std::sync::atomic::AtomicUsize::new(seconds);
    std::thread::scope(|s| {
        s.spawn(|| {
            while !stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                spin_sleep::sleep(std::time::Duration::new(1, 0));
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if counter.load(std::sync::atomic::Ordering::Relaxed) >= limit.load(std::sync::atomic::Ordering::Relaxed) { break; }     
            }
	    // Para que no suene la alarma cuando se usa "stop":
	    if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
		std::process::exit(0);
	    }
            let mut child = std::process::Command::new("mplayer")
                .arg("bell.mp3")
                .spawn()
                .expect("Falla al ejecutar mplayer");   
            child.wait().expect("Failed to wait child process mplayer");
            println!("Recordatorio para flexiones");
            std::process::exit(0);
        });

        println!("Iniciando {seconds} segundos para recordatorio.");
        println!("Comandos disponibles: stop / show / f [cantidad] [segundos para recordatorio]");
        println!("Indique un comando:");
        for line in std::io::stdin().lines() {
	    let string = line.unwrap();
	    if &string == "" {
		println!("No se recibio ningun comando.");
		println!("Comandos disponibles: stop / show / f [cantidad] [segundos para recordatorio]");
		println!("Indique un comando:");
		continue;
	    }
	    let commands: Vec<&str> = string.split(' ').collect();
	    match commands[0] {
		"stop" => break,
		"show" => println!("Counter: {}", counter.load(std::sync::atomic::Ordering::Relaxed)),
		"f" if commands.len() == 3 => {
		    data::newData(commands[0], commands[1]);
		    counter.store(0, std::sync::atomic::Ordering::Relaxed);
		    limit.store(commands[2].parse::<usize>().unwrap(), std::sync::atomic::Ordering::Relaxed);
		    println!("Reiniciando contador para recordatorio en {} segundos", limit.load(std::sync::atomic::Ordering::Relaxed));
		},
		cmd => println!("{cmd:?} no es un comando valido.\n"),
	    }
            println!("Comandos disponibles: stop / show / f [cantidad] [segundos para recordatorio]");
            println!("Indique un comando:");
        }

        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);

    });
}

pub mod data {

use std::{ fs, process, error::Error };

    fn changeChar(file: &str, actual: char, new: char) -> Result<String, Box<dyn Error>> {
        let mut answer: String = format!("{};", &file[..file.len()-4]);
        let entries = fs::read_to_string(file)?;
        for ch in entries.chars() {
            if ch != actual {
                answer.push(ch);
            } else {
                answer.push(new);
            }
        }

        Ok(answer)
    }

    pub fn fileToString(fileName: &str) {
        let data = changeChar(fileName, '\n', ';');
        match data {
            Ok(x) => super::appendToFile("resumen.txt".to_string(), x),
            _ => println!("Error procesando el archivo {}", fileName),
        }
    }

    fn dayInfo(letter: &str, date: String) -> (&str, u8) { 
        let item = match letter {
            "f" => "flexiones",
            "c" => "caminata",
            "r" => "remo",
            _ => "peso",
        };

        let mut counter: u8 = 0;
        let entries = fs::read_to_string(format!("dates/{}", date)).unwrap_or_else(|error| {
            eprintln!("Se encontró un error al tratar de leer el archivo {}: {}", date, error);
            process::exit(1);
        });

        for line in entries.lines() {
            let elements = super::split76(line);
            if elements[0] == letter {
                counter += elements[1].parse::<u8>().unwrap();
            }
        }

        (item, counter)
    }
    
    pub fn newData(exercise: &str, quantity: &str) {
        let number = quantity.parse::<u8>().unwrap_or_else(|error| {
            eprintln!("El segundo argumento no es válido: {}", error);
            process::exit(1);
        });
        let cldata = format!("{} {}", exercise, number);
        let dateStr = super::getDate();
        let mut created = false;
        let dates = fs::read_to_string("dates.txt").unwrap_or_else(|err| {
            eprintln!("No fue posible leer el archivo dates.txt: {err}");
            process::exit(1);
        });

        for line in dates.lines() {
            if line == format!("{}", dateStr) { 
                created = true;
                break;
            }
        }

        if !created {
            super::createFile(dateStr.clone());
            super::appendToFile("dates.txt".to_string(), dateStr.clone());
        }

        super::appendToFile(format!("dates/{}.txt", dateStr), cldata);
        let (item, counter) = dayInfo(exercise, format!("{}.txt", dateStr));
        println!("Registro sobre {} guardado.", item);
        if item != "peso" {
            println!("Acumulado del día: {}", counter);
            if item == "flexiones" {
                use chrono::{ Utc, Timelike };
                let now = Utc::now();
                println!("Hora: {:02}:{:02}", now.hour()-4, now.minute());
            }
        }

        // Actualizamos hello.xhtml con los nuevos datos de ejercicios
        super::clean_file("tmp.xhtml".to_string());
        let first_half = std::fs::read_to_string("hello_first_half.html").unwrap();
        super::appendToFile("tmp.xhtml".to_string(), first_half);
        let (_, counter) = dayInfo("p", format!("{}.txt", dateStr));
        super::appendToFile("tmp.xhtml".to_string(), format!(r"<h3>Peso: {}", counter));
        let (_, counter) = dayInfo("f", format!("{}.txt", dateStr));
        super::appendToFile("tmp.xhtml".to_string(), format!(r"<h3>Flexiones: {}", counter));
        let (_, counter) = dayInfo("r", format!("{}.txt", dateStr));
        super::appendToFile("tmp.xhtml".to_string(), format!(r"<h3>Remos: {}", counter));
        let second_half = std::fs::read_to_string("hello_second_half.html").unwrap();
        super::appendToFile("tmp.xhtml".to_string(), second_half);
        std::fs::rename("tmp.xhtml", "hello.xhtml");
    }
}
/*
pub mod gps {
    use std::{ str, process::{ Command, Output }, time};

    struct Location {
        latitude: f64,
        longitude: f64,
    }

    impl Location {
        fn getLocation() -> String {
            let output =
                match 
                Command::new("termux-location")
                .output()
                .expect("Failed to execute command") {
                    Output { status: _, stdout, stderr: _ } => stdout,
                };

            let output2 = match str::from_utf8(&output) {
                Ok(v) => v,
                Err(_e) => panic!("Invalid UTF-8 sequence"),
            };

            output2.to_string()
        }

        fn build() -> Location {
            let mut location_string = "".to_string();
            while location_string == "" {
                location_string = Self::getLocation();
            }
/*            let locationString = Self::getLocation();
            let locationsItems = match &locationString[..] {
                "" => {
                    eprintln!("El resultado de termux-location es un String vacío");
                    std::process::exit(1);
                },
                _ => super::split76(&locationString),
            };  */
            let locationsItems = super::split76(&location_string);
            let lat = &locationsItems[3];
            let lat = &lat[..lat.len()-2];
            let lat = lat.parse::<f64>().unwrap();
            println!("Latitude: {}", lat);
            let long = &locationsItems[6];
            let long = &long[..long.len()-2];
            let long = long.parse::<f64>().unwrap();
            println!("Longitude: {}", long);

            Location {
                latitude: lat,
                longitude: long,
            }
        }

        fn calculateDistance(origin: &Location, destination: &Location) -> f64 {
            let earth_radius_kilometer = 6371.0_f64;
            let (paris_latitude_degrees, paris_longitude_degrees) = (origin.latitude, origin.longitude);
            let (london_latitude_degrees, london_longitude_degrees) = (destination.latitude, destination.longitude);

            let paris_latitude = paris_latitude_degrees.to_radians();
            let london_latitude = london_latitude_degrees.to_radians();

            let delta_latitude = (paris_latitude_degrees - london_latitude_degrees).to_radians();
            let delta_longitude = (paris_longitude_degrees - london_longitude_degrees).to_radians();

            let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
            + paris_latitude.cos() * london_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
            let central_angle = 2.0 * central_angle_inner.sqrt().asin();

            let distance = earth_radius_kilometer * central_angle;

            println!(
                "Distance between origin and destination is {:.2} kilometers",
                distance
            );

            distance
        }
    }

    pub fn gpsDistances() {
        use std::thread;
        use std::sync::atomic::{AtomicBool, AtomicI32, Ordering::Relaxed};

        let seconds = 300; 
        let mut origin = Location::build();

        let total_distance = AtomicI32::new(0);
        let calories_burned = AtomicI32::new(0);
        let stop_flag = AtomicBool::new(false);
        thread::scope(|s| {
            let t = s.spawn(|| {
                while !stop_flag.load(Relaxed) {
                    thread::park_timeout(time::Duration::new(seconds, 0));
		    if !stop_flag.load(Relaxed) { continue; }
                    let destination = Location::build();
                    let distance = Location::calculateDistance(&origin, &destination);
                    super::appendToFile("gpsDistances.txt".to_string(), distance.to_string());
                    total_distance.fetch_add((distance * 1000.0) as i32, Relaxed);
                    let caloriesBurned = 
                        match (distance * 12.0) as i8 {
                            3 => 18,
                            4 => 19,
                            5 => 22,
                            _ => 13,
                        };
                    calories_burned.fetch_add(caloriesBurned as i32, Relaxed);
// QUITAR ESTOS DOS println! QUE SIGUEN:
                    println!("Distancia recorrida: {} metros.", total_distance.load(Relaxed));
                    println!("Calorías quemadas: {}.", calories_burned.load(Relaxed));
                    super::appendToFile("gpsDistances.txt".to_string(), format!("Total distance: {} metros\nCalorías quemadas: {}", total_distance.load(Relaxed).to_string(), calories_burned.load(Relaxed).to_string()));
                    origin = destination;
                }
            });

            println!("Mediciones cada {} segundos.", seconds);
            println!("Comandos disponibles: show / stop");
            println!("Indique un comando: ");
            for line in std::io::stdin().lines() {
                match line.unwrap().as_str() {
                    "stop" => break,
                    "show" => {
                        println!("\nDistancia recorrida: {}", total_distance.load(Relaxed));
                        println!("Calorías quemadas: {}", calories_burned.load(Relaxed));
                    },
                    cmd => println!("{cmd:?} no es un comando valido.\n"),
                }
                println!("Comandos disponibles: show / stop");
                println!("Indique un comando:");
            }

            stop_flag.store(true, Relaxed);
            println!("Deteniendo mediciones...");
	    t.thread().unpark();
        });
    }
}

pub mod search;*/
