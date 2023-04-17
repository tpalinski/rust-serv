pub enum ResponseCodes {
    OK,
    NotFound,
    Created,
}

impl ResponseCodes {
    pub fn as_response_header(&mut self) -> String{
        let pre = String::from("HTTP/1.1 ");
        pre + match self {
            ResponseCodes::OK => "200 OK\r\n",
            ResponseCodes::NotFound => "404 Not found\r\n",
            ResponseCodes::Created => "201 Created\r\n"
        }
    }
}
