extern crate redis;
use redis::Commands;
use std::io;

struct User {
    id: u32,
    name: String,
}

impl User {
    fn new(&self) {
        let client = connection();
        let user_id: u32 = client.incr("users:id", self.id).unwrap();
        let user_key = format!("users:info:{}", user_id);

        match client.hset(&user_key, "name", &self.name) {
            Ok(value) => value,
            Err(_) => panic!("Error en crear al usuario"),
        };
    }
}

fn main() {
    loop {
        let mut input_cmd = String::new();
        println!("Agregar un nombre a Redis");
        io::stdin().read_line(&mut input_cmd).expect("No es string");

        let user_a = User {
            id: 1,
            name: input_cmd,
        };
        user_a.new();
    }
}

fn connection() -> redis::Client {
    let client = match redis::Client::open("redis://10.90.125.31:6379") {
        Ok(conection) => conection,
        Err(_) => panic!("Error en conexion con base de datos"),
    };
    client
}
