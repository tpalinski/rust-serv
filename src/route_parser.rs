pub fn parse_route(request_buffer: &[u8]) -> String{
    let mut write_flag: u8 = 0;
    let mut route_vec: Vec<u8> = vec![];
    for byte in request_buffer {
        if write_flag == 1 {
            route_vec.push(*byte);
        } else if write_flag == 2 {
           break; 
        }
        if *byte == 0x20{ // Check for space in request header
            write_flag += 1;
        }
    }
    route_vec.pop();
    String::from_utf8(route_vec).unwrap()
}
