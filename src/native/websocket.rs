mod websocket_mod {
    use crate::native::std::{arg as std_arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    /// Start a WebSocket server using tokio-tungstenite.
    /// Usage: ws.server(port, handler_name)
    pub fn ws_server(args: String) -> Box<dyn Validator> {
        use futures_util::{SinkExt, StreamExt};
        use tokio::net::TcpListener;
        use tokio_tungstenite::accept_async;
        use tokio_tungstenite::tungstenite::Message;

        let args_list = split_args(&args, 2);
        let port_str = std_arg(&args_list, 0);
        let handler_name = std_arg(&args_list, 1);

        let port: u16 = match port_str.parse() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("RuntimeError [ws.server]: porta inválida '{}': {}", port_str, e);
                return Box::new(put_quoted_str("None".to_string()));
            }
        };

        let handler = handler_name.clone();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(async move {
                let addr = format!("0.0.0.0:{}", port);
                let listener = match TcpListener::bind(&addr).await {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("RuntimeError [ws.server]: falha ao bind: {}", e);
                        return;
                    }
                };

                eprintln!("WebSocket server listening on ws://{}", addr);

                while let Ok((stream, _peer)) = listener.accept().await {
                    let _handler = handler.clone();
                    tokio::spawn(async move {
                        let ws_stream = match accept_async(stream).await {
                            Ok(ws) => ws,
                            Err(e) => {
                                eprintln!("ws.server: erro accept: {}", e);
                                return;
                            }
                        };

                        let (mut write, mut read) = ws_stream.split();

                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    let response = format!("Echo: {}", text);
                                    if write.send(Message::Text(response.into())).await.is_err() {
                                        break;
                                    }
                                }
                                Ok(Message::Binary(data)) => {
                                    let response = format!("Echo binary: {} bytes", data.len());
                                    if write.send(Message::Text(response.into())).await.is_err() {
                                        break;
                                    }
                                }
                                Ok(Message::Close(_)) => break,
                                Ok(Message::Ping(data)) => { write.send(Message::Pong(data)).await.ok(); }
                                Ok(Message::Pong(_)) => {}
                                Ok(Message::Frame(_)) => {}
                                Err(e) => {
                                    eprintln!("ws.server: erro leitura: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                }
            });
        });

        Box::new(put_quoted_str(format!("ws://0.0.0.0:{}", port)))
    }

    /// WebSocket client: connect and send/receive.
    /// Usage: ws.connect(url, message)
    pub fn ws_connect(args: String) -> Box<dyn Validator> {
        use futures_util::{SinkExt, StreamExt};
        use tokio_tungstenite::connect_async;
        use tokio_tungstenite::tungstenite::Message;

        let args_list = split_args(&args, 2);
        let url = std_arg(&args_list, 0);
        let message = std_arg(&args_list, 1);

        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        let result = rt.block_on(async move {
            let (mut ws_stream, _) = match connect_async(&url).await {
                Ok(ws) => ws,
                Err(e) => {
                    return format!("Erro conexão: {}", e);
                }
            };

            if !message.is_empty() && message != "None" {
                if ws_stream.send(Message::Text(message.into())).await.is_err() {
                    return "Erro envio".to_string();
                }
            }

            match ws_stream.next().await {
                Some(Ok(Message::Text(text))) => text,
                Some(Ok(Message::Binary(data))) => String::from_utf8_lossy(&data).to_string(),
                Some(Ok(Message::Close(_))) => "Conexão fechada".to_string(),
                Some(Ok(_)) => "Mensagem ignorada".to_string(),
                Some(Err(e)) => format!("Erro: {}", e),
                None => "Sem resposta".to_string(),
            }
        });

        Box::new(put_quoted_str(result))
    }
}

pub use websocket_mod::*;
