use std::collections::HashMap;

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
        match action {
            None => {
                header_string = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                response_string = "".to_string();
            }  
            Some(f) => {
                header_string = "HTTP/1.1 200 OK\r\n\r\n".to_string();
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
