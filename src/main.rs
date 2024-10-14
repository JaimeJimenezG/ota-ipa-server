mod routes {
    pub mod build_list;
    pub mod home;
    pub mod info;
    pub mod load_plist;
    pub mod qr_page;
}

use native_tls::{Identity, TlsAcceptor};
use routes::*;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::path::Path;
use std::thread;

pub const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const PORT: u16 = 8443;
pub const ADDRESS: &str = "https://680b-90-162-43-242.ngrok-free.app"; // Asegúrate de que esta sea la dirección correcta

fn main() {
    let addr = SocketAddr::new(IP, PORT);

    let mut cert_file = File::open("cert.pem").expect("No se pudo abrir cert.pem");
    let mut key_file = File::open("key.pem").expect("No se pudo abrir key.pem");

    let mut cert = Vec::new();
    let mut key = Vec::new();

    cert_file
        .read_to_end(&mut cert)
        .expect("No se pudo leer cert.pem");
    key_file
        .read_to_end(&mut key)
        .expect("No se pudo leer key.pem");

    let identity = Identity::from_pkcs8(&cert, &key).expect("No se pudo crear la identidad");

    let acceptor = TlsAcceptor::new(identity).expect("No se pudo crear el aceptador TLS");
    let acceptor = std::sync::Arc::new(acceptor);

    let listener = TcpListener::bind(addr).expect("No se pudo vincular al puerto");
    println!("Servidor escuchando en https://{}", addr);

    for stream in listener.incoming() {
        let acceptor = acceptor.clone();
        let stream = stream.unwrap();

        thread::spawn(move || match acceptor.accept(stream) {
            Ok(tls_stream) => {
                if let Err(e) = handle_connection(tls_stream) {
                    eprintln!("Error al manejar la conexión: {}", e);
                }
            }
            Err(e) => eprintln!("Error al aceptar la conexión TLS: {}", e),
        });
    }
}

fn handle_connection<T: Read + Write>(mut stream: T) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let request = match stream.read(&mut buffer) {
        Ok(_) => String::from_utf8_lossy(&buffer[..]).to_string(),
        Err(e) => {
            eprintln!("Error al leer el stream: {}", e);
            return Ok(());
        }
    };

    let request_line = request.lines().next().unwrap_or("");

    if request_line.starts_with("GET /builds/") {
        let path = request_line.split_whitespace().nth(1).unwrap_or("");
        let file_path = Path::new(".").join(path.trim_start_matches("/"));
        if file_path.exists() && file_path.is_file() {
            let content = fs::read(&file_path)?;
            let content_type = get_content_type(file_path.extension().and_then(|s| s.to_str()).unwrap_or(""));
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                content_type,
                content.len()
            );
            stream.write_all(response.as_bytes())?;
            stream.write_all(&content)?;
        } else {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n404 - Archivo no encontrado";
            stream.write_all(response.as_bytes())?;
        }
    } else {
        let (status_line, content) = match request_line {
            "GET /builds HTTP/1.1" => ("HTTP/1.1 200 OK", build_list::build_list()),
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", home::home_page()),
            "GET /info HTTP/1.1" => ("HTTP/1.1 200 OK", info::info_page()),
            _ if request_line.starts_with("GET /qr/") => {
                let build = request_line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .trim_start_matches("/qr/");
                ("HTTP/1.1 200 OK", qr_page::qr_page(build, ADDRESS))
            }
            _ if request_line.starts_with("GET /load_plist/") => {
                let build = request_line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .trim_start_matches("/load_plist/");
                ("HTTP/1.1 200 OK", load_plist::load_plist(build, ADDRESS))
            }
            _ => (
                "HTTP/1.1 404 NOT FOUND",
                "404 - Página no encontrada".to_string(),
            ),
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            content.len(),
            content
        );

        stream.write_all(response.as_bytes())?;
    }

    stream.flush()?;
    Ok(())
}

fn get_content_type(extension: &str) -> String {
    match extension {
        "ipa" => "application/octet-stream".to_string(),
        "plist" => "application/xml".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}
