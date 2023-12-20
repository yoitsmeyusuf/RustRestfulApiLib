use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

use std::io::BufRead; // Import the BufRead traitImport the BufRead trait

fn main() {
     
    let mut servers=  Server::new("127.0.0.1:7878");
      
    servers.router.add_route("POST", "/", |params: &HashMap<String, String>| {
     let name = params.get("isim").cloned().unwrap_or("".to_string());
     Ok(format!("HTTP/1.1 200 OK\r\n\r\n {} POST ", name))
    });servers.router.add_route("GET", "/sa", |params: &HashMap<String, String>| {
        let name = params.get("isim").cloned().unwrap_or("".to_string());
        Ok(format!("HTTP/1.1 200 OK\r\n\r\n {} GET ", name))
       });
    servers.handle_connection();
    
}
//bundan sonra içine girilen parametrede bir değer aramaya calışacağız
//örneğin içine isim için bir değişken girmesini istiyoruz daha sonra bunu yazıp
//sadece içine isim girilen değeri alıp ekrana yazdırmaya calışacağız

struct Server {
    listener: TcpListener,
    router: Router,
}

impl Server {
    fn new(addr: &str) -> Server {
        let listener = TcpListener::bind(addr).unwrap();
        let  router = Router::new();
        
        Server { listener, router }
    }

    fn handle_connection(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = String::new();
            let mut reader = std::io::BufReader::new(&stream);
            reader.read_line(&mut buffer).unwrap();
            let method = buffer.split_whitespace().nth(0).unwrap_or("GET");
            let path = buffer.split_whitespace().nth(1).unwrap_or("/");
            let path_parts: Vec<&str> = path.split('?').collect();
            let mut params = HashMap::new();
            if path_parts.len() > 1 {
                for param in path_parts[1].split('&') {
                    let param_parts: Vec<&str> = param.split('=').collect();
                    if param_parts.len() > 1 {
                        params.insert(param_parts[0].to_string(), param_parts[1].to_string());
                    }
                }
            }
            let response = self.router.route(method, path_parts[0], &params).unwrap();
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    }

use std::collections::HashMap;

type Handler = Box<dyn Fn(&HashMap<String, String>) -> Result<String, Box<dyn Error>> + Send + Sync>;

struct Route {
    method: String,
    handler: Handler,
}

struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route<F>(&mut self, method: &str, path: &str, handler: F)
where
    F: Fn(&HashMap<String, String>) -> Result<String, Box<dyn Error>> + 'static + Send + Sync,
{
    let route = Route {
        method: method.to_uppercase(),
        handler: Box::new(handler),
    };
    self.routes.insert(path.to_string(), route);
}
fn route(&self, method: &str, path: &str, params: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    match self.routes.get(path) {
        Some(route) if route.method == method.to_uppercase() => (route.handler)(params),
        _ => Ok(format!("HTTP/1.1 404 Not Found\r\n\r\nNot Found")),
    }
}
}