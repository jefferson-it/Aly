#[cfg(feature = "mqtt")]
mod mqtt_mod {
    use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Incoming};
    use std::sync::Arc;
    use std::time::Duration;
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    use crate::native::std::{arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    type SharedClient = Arc<Mutex<Option<AsyncClient>>>;
    type CallbackMap = Arc<Mutex<HashMap<String, String>>>;

    thread_local! {
        static MQTT_CLIENT: std::cell::RefCell<SharedClient> = std::cell::RefCell::new(Arc::new(Mutex::new(None)));
        static MQTT_CALLBACKS: std::cell::RefCell<CallbackMap> = std::cell::RefCell::new(Arc::new(Mutex::new(HashMap::new())));
    }

    fn ensure_client() -> Option<AsyncClient> {
        MQTT_CLIENT.with(|c| {
            let guard = c.borrow();
            let mutex = guard.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { mutex.lock().await.clone() })
        })
    }

    fn set_client(client: AsyncClient) {
        MQTT_CLIENT.with(|c| {
            let guard = c.borrow();
            let mutex = guard.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { 
                let mut client_guard = mutex.lock().await;
                *client_guard = Some(client);
            });
        });
    }

    fn clear_client() {
        MQTT_CLIENT.with(|c| {
            let guard = c.borrow();
            let mutex = guard.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { 
                let mut client_guard = mutex.lock().await;
                *client_guard = None;
            });
        });
    }

    fn register_callback(topic: String, func_name: String) {
        MQTT_CALLBACKS.with(|c| {
            let guard = c.borrow();
            let mutex = guard.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { 
                let mut cb = mutex.lock().await;
                cb.insert(topic, func_name);
            });
        });
    }

    fn get_callback(topic: &str) -> Option<String> {
        MQTT_CALLBACKS.with(|c| {
            let guard = c.borrow();
            let mutex = guard.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { 
                let cb = mutex.lock().await;
                cb.get(topic).cloned()
            })
        })
    }

    fn run_event_loop(mut eventloop: rumqttc::EventLoop) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                match eventloop.poll().await {
                    Ok(notification) => {
                        if let Event::Incoming(Incoming::Publish(p)) = notification {
                            let topic = p.topic.clone();
                            let _payload = String::from_utf8_lossy(&p.payload).to_string();
                            let callback_name = get_callback(&topic);
                            if let Some(func_name) = callback_name {
                                let run = crate::aly::get_runtime();
                                let fake_lexer = vec![crate::lexer::Lexer::new(crate::tokens::Tokens::Identifier, func_name, 0)];
                                let _ = run.function_run(fake_lexer);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("MQTT event loop error: {}", e);
                        break;
                    }
                }
            }
        });
    }

    pub fn mqtt_connect(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 5);
        let host = arg(&args, 0);
        let port: u16 = arg(&args, 1).parse().unwrap_or(1883);
        let client_id = arg(&args, 2);
        let username = arg(&args, 3);
        let password = arg(&args, 4);

        let mut mqttoptions = MqttOptions::new(client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(30));
        
        if !username.is_empty() {
            mqttoptions.set_credentials(username, password);
        }

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
        
        // Spawn the event loop in a separate thread
        std::thread::spawn(move || {
            run_event_loop(eventloop);
        });
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            set_client(client.clone()).await;
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn mqtt_publish(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let topic = arg(&args, 0);
        let payload = arg(&args, 1);
        let qos: u8 = arg(&args, 2).parse().unwrap_or(0);

        let client_opt = {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { ensure_client().await })
        };

        if let Some(client) = client_opt {
            let qos = match qos {
                1 => QoS::AtLeastOnce,
                2 => QoS::ExactlyOnce,
                _ => QoS::AtMostOnce,
            };
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = client.publish(topic, qos, false, payload).await;
            });
            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: cliente MQTT não conectado".to_string()))
        }
    }

    pub fn mqtt_subscribe(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let topic = arg(&args, 0);
        let qos: u8 = arg(&args, 1).parse().unwrap_or(0);
        let callback = arg(&args, 2);

        let client_opt = {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { ensure_client() })
        };

        if let Some(client) = client_opt {
            let qos = match qos {
                1 => QoS::AtLeastOnce,
                2 => QoS::ExactlyOnce,
                _ => QoS::AtMostOnce,
            };
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = client.subscribe(topic.clone(), qos).await;
                register_callback(topic, callback);
            });
            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: cliente MQTT não conectado".to_string()))
        }
    }

    pub fn mqtt_unsubscribe(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let topic = arg(&args, 0);

        let client_opt = {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { ensure_client() })
        };

        if let Some(client) = client_opt {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = client.unsubscribe(topic).await;
            });
            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: cliente MQTT não conectado".to_string()))
        }
    }

    pub fn mqtt_disconnect(params: String) -> Box<dyn Validator> {
        let client_opt = {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { ensure_client() })
        };

        if let Some(client) = client_opt {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = client.disconnect().await;
                clear_client();
            });
            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: cliente MQTT não conectado".to_string()))
        }
    }
}

#[cfg(feature = "mqtt")]
pub use mqtt_mod::*;

#[cfg(not(feature = "mqtt"))]
mod mqtt_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn mqtt_connect(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'mqtt' não habilitada".to_string()))
    }
    pub fn mqtt_publish(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'mqtt' não habilitada".to_string()))
    }
    pub fn mqtt_subscribe(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'mqtt' não habilitada".to_string()))
    }
    pub fn mqtt_unsubscribe(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'mqtt' não habilitada".to_string()))
    }
    pub fn mqtt_disconnect(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'mqtt' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "mqtt"))]
pub use mqtt_stub::*;