use std::collections::HashMap;

use crate::html_parser;

pub struct Router {
    route_actions: HashMap<String, Box<dyn Fn() -> String>>

}

impl Router {
    pub fn add_route(&mut self, route: String, route_handler:Box<dyn Fn() -> String>){
       self.route_actions.insert(route, route_handler); 
    }

    pub fn handle_route(&mut self, route: String) -> (String, String){
        let action = self.route_actions.get(&route);
        let header_string: String;
        let response_string: String;
        let request_path: Vec<&str> = route.split('/').collect();
        if request_path[1] == "static" {
            header_string = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n".to_string();
            response_string = html_parser::parse_html(request_path[2]);
            return (header_string, response_string);
        }
        match action {
            None => {
                header_string = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                response_string = "".to_string();
            }  
            Some(f) => {
                header_string = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n".to_string();
                let fun = &(f);
                response_string = fun();
            }

        }
        (header_string, response_string)
    }
}




impl Default for Router {
    fn default() -> Self {
        Self {route_actions: HashMap::default()}
    }
}
