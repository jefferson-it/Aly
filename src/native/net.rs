mod net_mod {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream, UdpSocket};
    use std::time::Duration;

    use crate::native::create_object::Object;
    use crate::native::std::{arg as std_arg, split_args};
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::put_quoted_str;

    // net.tcp_send(host, port, data) -> response string
    pub fn net_tcp_send(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let host = std_arg(&args, 0);
        let port = std_arg(&args, 1);
        let data = std_arg(&args, 2);

        let addr = format!("{}:{}", host, port);
        let mut stream = match TcpStream::connect(&addr) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("RuntimeError [net.tcp_send]: falha ao conectar a '{}': {}", addr, e);
                return Box::new(put_quoted_str(String::new()));
            }
        };

        stream.set_read_timeout(Some(Duration::from_secs(5))).ok();

        if !data.is_empty() {
            if let Err(e) = stream.write_all(data.as_bytes()) {
                eprintln!("RuntimeError [net.tcp_send]: erro de escrita em '{}': {}", addr, e);
                return Box::new(put_quoted_str(String::new()));
            }
        }

        let mut buf = vec![0u8; 8192];
        let mut response = String::new();
        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    response.push_str(&String::from_utf8_lossy(&buf[..n]));
                    if n < buf.len() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        Box::new(put_quoted_str(response))
    }

    // net.tcp_serve(port, response) -> serves one request with fixed response (blocking)
    pub fn net_tcp_serve(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let port = std_arg(&args, 0);
        let response = std_arg(&args, 1);

        let addr = format!("0.0.0.0:{}", port);
        let listener = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("RuntimeError [net.tcp_serve]: falha ao vincular '{}': {}", addr, e);
                return Box::new("None".to_owned());
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let mut buf = [0u8; 4096];
                    let _ = s.read(&mut buf);
                    if let Err(e) = s.write_all(response.as_bytes()) {
                        eprintln!("RuntimeError [net.tcp_serve]: erro de escrita: {}", e);
                    }
                    let _ = s.flush();
                    break;
                }
                Err(e) => {
                    eprintln!("RuntimeError [net.tcp_serve]: erro de conexão: {}", e);
                }
            }
        }

        Box::new("None".to_owned())
    }

    // net.tcp_listen(port, handler_name) -> accepts connections, calls handler function
    pub fn net_tcp_listen(x: String) -> Box<dyn Validator> {
        use crate::tokens::Tokens;

        let args = split_args(&x, 2);
        let port = std_arg(&args, 0);
        let handler_name = std_arg(&args, 1);

        let addr = format!("0.0.0.0:{}", port);
        let listener = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("RuntimeError [net.tcp_listen]: falha ao vincular '{}': {}", addr, e);
                return Box::new("None".to_owned());
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let mut buf = [0u8; 65536];
                    let n = match s.read(&mut buf) {
                        Ok(0) => continue,
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                    let client_data = String::from_utf8_lossy(&buf[..n]).to_string();

                    let run = crate::aly::get_runtime();
                    let fake = crate::lexer::Lexer::new(Tokens::Reference, handler_name.clone(), 0);
                    if let Ok(var) = run.get_var(fake) {
                        if let ValueData::Function(_, params, body) = var.get_value() {
                            let old = run.get_vars().clone();
                            if let Some(p) = params.first() {
                                run.register_var(crate::native::vars::Var::new(
                                    p.literal.clone(),
                                    ValueData::String(client_data),
                                    true,
                                ));
                            }
                            let stmts = crate::runtime::interpreter::split_statements(&body);
                            let mut v: Box<dyn Validator> =
                                Box::new(ValueData::String("None".to_owned()));
                            for stmt in stmts {
                                let mut s2 = stmt;
                                if crate::runtime::interpreter::exec(&mut s2, &mut v) {
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("RuntimeError [net.tcp_listen]: erro de conexão: {}", e);
                }
            }
        }

        Box::new("None".to_owned())
    }

    // net.udp_send(host, port, data) -> sends a UDP datagram
    pub fn net_udp_send(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let host = std_arg(&args, 0);
        let port = std_arg(&args, 1);
        let data = std_arg(&args, 2);

        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("RuntimeError [net.udp_send]: falha ao criar socket: {}", e);
                return Box::new("None".to_owned());
            }
        };

        let addr = format!("{}:{}", host, port);
        if let Err(e) = socket.send_to(data.as_bytes(), &addr) {
            eprintln!("RuntimeError [net.udp_send]: erro de envio para '{}': {}", addr, e);
        }

        Box::new("None".to_owned())
    }

    // net.udp_recv(port, size) -> receives a UDP datagram (blocking)
    // Returns an object with { data, addr, port }
    pub fn net_udp_recv(x: String) -> Box<dyn Validator> {
        use std::collections::HashMap;

        let args = split_args(&x, 2);
        let port = std_arg(&args, 0);
        let size: usize = std_arg(&args, 1).trim().parse().unwrap_or(1024);

        let addr = format!("0.0.0.0:{}", port);
        let socket = match UdpSocket::bind(&addr) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("RuntimeError [net.udp_recv]: falha ao vincular '{}': {}", addr, e);
                return Box::new("None".to_owned());
            }
        };

        socket.set_read_timeout(Some(Duration::from_secs(30))).ok();

        let mut buf = vec![0u8; size];
        match socket.recv_from(&mut buf) {
            Ok((n, src)) => {
                let data = String::from_utf8_lossy(&buf[..n]).to_string();
                let mut obj = Object::new(vec![], HashMap::new());
                obj.set_item("data".to_owned(), ValueData::String(data));
                obj.set_item("addr".to_owned(), ValueData::String(src.ip().to_string()));
                obj.set_item("port".to_owned(), ValueData::Int(src.port() as i64));
                Box::new(ValueData::Object(obj))
            }
            Err(e) => {
                eprintln!("RuntimeError [net.udp_recv]: erro de recebimento em '{}': {}", addr, e);
                Box::new("None".to_owned())
            }
        }
    }

    // net.ws_connect(url, data) -> WebSocket client: connect, send, receive
    pub fn net_ws_connect(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let url = std_arg(&args, 0);
        let data = std_arg(&args, 1);

        let (mut socket, _) = match tungstenite::connect(&url) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("RuntimeError [net.ws_connect]: falha ao conectar a '{}': {}", url, e);
                return Box::new(put_quoted_str(String::new()));
            }
        };

        if !data.is_empty() {
            if let Err(e) = socket.send(tungstenite::Message::Text(data)) {
                eprintln!("RuntimeError [net.ws_connect]: erro de envio: {}", e);
                return Box::new(put_quoted_str(String::new()));
            }
        }

        let response = match socket.read() {
            Ok(tungstenite::Message::Text(t)) => t,
            Ok(tungstenite::Message::Binary(b)) => String::from_utf8_lossy(&b).to_string(),
            Ok(_) => String::new(),
            Err(e) => {
                eprintln!("RuntimeError [net.ws_connect]: erro de leitura: {}", e);
                String::new()
            }
        };

        let _ = socket.close(None);

        Box::new(put_quoted_str(response))
    }

    // net.ws_server(port, handler_name) -> WebSocket server
    pub fn net_ws_server(x: String) -> Box<dyn Validator> {
        use crate::tokens::Tokens;
        use std::collections::HashMap;

        let args = split_args(&x, 2);
        let port = std_arg(&args, 0);
        let handler_name = std_arg(&args, 1);

        let addr = format!("0.0.0.0:{}", port);
        let listener = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("RuntimeError [net.ws_server]: falha ao vincular '{}': {}", addr, e);
                return Box::new("None".to_owned());
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(tcp) => {
                    let mut ws = match tungstenite::accept(tcp) {
                        Ok(w) => w,
                        Err(e) => {
                            eprintln!("net.ws_server: accept error: {}", e);
                            continue;
                        }
                    };

                    loop {
                        match ws.read() {
                            Ok(tungstenite::Message::Text(msg)) => {
                                let run = crate::aly::get_runtime();
                                let fake =
                                    crate::lexer::Lexer::new(Tokens::Reference, handler_name.clone(), 0);
                                if let Ok(var) = run.get_var(fake) {
                                    if let ValueData::Function(_, params, body) = var.get_value() {
                                        let old = run.get_vars().clone();
                                        if let Some(p) = params.first() {
                                            run.register_var(crate::native::vars::Var::new(
                                                p.literal.clone(),
                                                ValueData::String(msg),
                                                true,
                                            ));
                                        }
                                        let stmts =
                                            crate::runtime::interpreter::split_statements(&body);
                                        let mut v: Box<dyn Validator> =
                                            Box::new(ValueData::String("None".to_owned()));
                                        for stmt in stmts {
                                            let mut s2 = stmt;
                                            if crate::runtime::interpreter::exec(&mut s2, &mut v)
                                            {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(tungstenite::Message::Binary(b)) => {
                                let text = String::from_utf8_lossy(&b).to_string();
                                let run = crate::aly::get_runtime();
                                let fake =
                                    crate::lexer::Lexer::new(Tokens::Reference, handler_name.clone(), 0);
                                if let Ok(var) = run.get_var(fake) {
                                    if let ValueData::Function(_, params, body) = var.get_value() {
                                        let old = run.get_vars().clone();
                                        if let Some(p) = params.first() {
                                            run.register_var(crate::native::vars::Var::new(
                                                p.literal.clone(),
                                                ValueData::String(text),
                                                true,
                                            ));
                                        }
                                        let stmts =
                                            crate::runtime::interpreter::split_statements(&body);
                                        let mut v: Box<dyn Validator> =
                                            Box::new(ValueData::String("None".to_owned()));
                                        for stmt in stmts {
                                            let mut s2 = stmt;
                                            if crate::runtime::interpreter::exec(&mut s2, &mut v)
                                            {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(tungstenite::Message::Close(_)) => break,
                            Ok(_) => continue,
                            Err(e) => {
                                eprintln!("net.ws_server: read error: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("RuntimeError [net.ws_server]: erro de conexão: {}", e);
                }
            }
        }

        Box::new("None".to_owned())
    }
}

pub use net_mod::*;
