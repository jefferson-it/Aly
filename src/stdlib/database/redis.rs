use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::vm::value::Value;
use crate::database::driver::{DatabaseDriver, DatabaseConfig};

pub struct RedisDriver {
    data: HashMap<String, RedisValue>,
    channels: HashMap<String, Vec<Sender<String>>>,
    connected: bool,
}

enum RedisValue {
    String(String),
    Int(i64),
    List(Vec<Value>),
}

impl RedisDriver {
    pub fn new() -> Self {
        RedisDriver {
            data: HashMap::new(),
            channels: HashMap::new(),
            connected: false,
        }
    }
}

impl DatabaseDriver for RedisDriver {
    fn name(&self) -> &'static str {
        "redis"
    }

    fn connect(&mut self, _config: &DatabaseConfig) -> Result<(), String> {
        self.data.clear();
        self.channels.clear();
        self.connected = true;
        Ok(())
    }

    fn query(&mut self, cmd: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        let cmd = cmd.trim();
        let parts: Vec<&str> = cmd.splitn(3, char::is_whitespace).collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }
        match parts[0].to_uppercase().as_str() {
            "GET" => {
                let key = parts.get(1).ok_or_else(|| "GET requires a key".to_string())?;
                Ok(self.cmd_get(key))
            }
            "SET" => {
                let key = parts.get(1).ok_or_else(|| "SET requires a key".to_string())?;
                let val = parts.get(2).ok_or_else(|| "SET requires a value".to_string())?;
                Ok(self.cmd_set(key, val))
            }
            "DEL" => {
                let key = parts.get(1).ok_or_else(|| "DEL requires a key".to_string())?;
                Ok(self.cmd_del(key))
            }
            "EXISTS" => {
                let key = parts.get(1).ok_or_else(|| "EXISTS requires a key".to_string())?;
                Ok(self.cmd_exists(key))
            }
            "KEYS" => {
                let pattern = parts.get(1).unwrap_or(&"*");
                Ok(self.cmd_keys(pattern))
            }
            "INCR" => {
                let key = parts.get(1).ok_or_else(|| "INCR requires a key".to_string())?;
                Ok(self.cmd_incr(key))
            }
            "LPUSH" => {
                let key = parts.get(1).ok_or_else(|| "LPUSH requires a key".to_string())?;
                let val = parts.get(2).ok_or_else(|| "LPUSH requires a value".to_string())?;
                Ok(self.cmd_lpush(key, val))
            }
            "RPUSH" => {
                let key = parts.get(1).ok_or_else(|| "RPUSH requires a key".to_string())?;
                let val = parts.get(2).ok_or_else(|| "RPUSH requires a value".to_string())?;
                Ok(self.cmd_rpush(key, val))
            }
            "LPOP" => {
                let key = parts.get(1).ok_or_else(|| "LPOP requires a key".to_string())?;
                Ok(self.cmd_lpop(key))
            }
            "PUBLISH" => {
                let channel = parts.get(1).ok_or_else(|| "PUBLISH requires a channel".to_string())?;
                let message = parts.get(2).ok_or_else(|| "PUBLISH requires a message".to_string())?;
                Ok(self.cmd_publish(channel, message))
            }
            _ => Err(format!("Unknown Redis command: {}", parts[0])),
        }
    }

    fn execute(&mut self, cmd: &str) -> Result<Value, String> {
        self.query(cmd)
    }

    fn close(&mut self) -> Result<(), String> {
        self.data.clear();
        self.channels.clear();
        self.connected = false;
        Ok(())
    }
}

impl RedisDriver {
    fn to_value(rv: &RedisValue) -> Value {
        match rv {
            RedisValue::String(s) => Value::Str(s.clone()),
            RedisValue::Int(i) => Value::Int(*i),
            RedisValue::List(v) => Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(v.clone()))),
        }
    }

    fn cmd_set(&mut self, key: &str, val: &str) -> Value {
        self.data.insert(key.to_string(), RedisValue::String(val.to_string()));
        Value::Str("OK".to_string())
    }

    fn cmd_get(&self, key: &str) -> Value {
        match self.data.get(key) {
            Some(RedisValue::String(s)) => Value::Str(s.clone()),
            Some(RedisValue::Int(i)) => Value::Int(*i),
            Some(RedisValue::List(v)) => Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(v.clone()))),
            None => Value::Nil,
        }
    }

    fn cmd_del(&mut self, key: &str) -> Value {
        Value::Int(if self.data.remove(key).is_some() { 1 } else { 0 })
    }

    fn cmd_exists(&self, key: &str) -> Value {
        Value::Int(if self.data.contains_key(key) { 1 } else { 0 })
    }

    fn cmd_keys(&self, pattern: &str) -> Value {
        let matched: Vec<Value> = self.data.keys()
            .filter(|k| k.contains(pattern.trim_matches('*')))
            .cloned()
            .map(Value::Str)
            .collect();
        Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(matched)))
    }

    fn cmd_incr(&mut self, key: &str) -> Value {
        let n = match self.data.get(key) {
            Some(RedisValue::Int(i)) => *i + 1,
            _ => 1,
        };
        self.data.insert(key.to_string(), RedisValue::Int(n));
        Value::Int(n)
    }

    fn cmd_lpush(&mut self, key: &str, val: &str) -> Value {
        let list = self.get_list_mut(key);
        list.insert(0, Value::Str(val.to_string()));
        Value::Int(list.len() as i64)
    }

    fn cmd_rpush(&mut self, key: &str, val: &str) -> Value {
        let list = self.get_list_mut(key);
        list.push(Value::Str(val.to_string()));
        Value::Int(list.len() as i64)
    }

    fn cmd_lpop(&mut self, key: &str) -> Value {
        let list = self.get_list_mut(key);
        list.pop().unwrap_or(Value::Nil)
    }

    fn get_list_mut(&mut self, key: &str) -> &mut Vec<Value> {
        let exists_and_is_list = match self.data.get(key) {
            Some(RedisValue::List(_)) => true,
            _ => false,
        };
        if !exists_and_is_list {
            self.data.insert(key.to_string(), RedisValue::List(Vec::new()));
        }
        match self.data.get_mut(key).unwrap() {
            RedisValue::List(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    fn cmd_publish(&mut self, channel: &str, message: &str) -> Value {
        let subscribers = self.channels.entry(channel.to_string()).or_default();
        subscribers.retain(|tx| tx.send(message.to_string()).is_ok());
        Value::Int(subscribers.len() as i64)
    }

    #[allow(dead_code)]
    pub fn subscribe(&mut self, channel: &str) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();
        self.channels.entry(channel.to_string()).or_default().push(tx);
        rx
    }
}


unsafe impl Send for RedisDriver {}
unsafe impl Sync for RedisDriver {}
