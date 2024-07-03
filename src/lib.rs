use std::io::{Read, Write};
use std::error::Error;
use std::net::TcpStream;
use nlo_text_editor_server::{MESSAGE_SIZE, ServerAction, ServerResponse};

pub mod application;
pub mod ui;
pub mod events;



//pub fn send_action_to_server(stream: &mut TcpStream, action: ServerAction) -> Result<(), Box<dyn Error>>{
//    let serialized_action = ron::to_string(&action)?;
//    match stream.write(serialized_action.as_bytes()){
//        Ok(bytes_written) => {
//            if bytes_written == 0{} else {}
//        }
//        Err(e) => {return Err(Box::new(e));}
//    }
//    stream.flush()?;
//
//    Ok(())
//}

//pub fn read_server_response(stream: &mut TcpStream) -> Result<ServerResponse, Box<dyn Error>>{
//    let mut response_buffer = [0u8; MESSAGE_SIZE];
//    match stream.read(&mut response_buffer){
//        Ok(size) => {
//            let my_string = String::from_utf8_lossy(&response_buffer[0..size]);
//            match ron::from_str(&my_string){
//                Ok(response) => {return Ok(response)},
//                Err(e) => {return Err(Box::new(e));}
//            };
//        }
//        Err(e) => {
//            println!("An error occurred. {}", e);
//            stream.shutdown(std::net::Shutdown::Both).unwrap();
//            return Err(Box::new(e));
//        }
//    }
//}

pub fn do_ipc_things(stream: &mut TcpStream, action: ServerAction) -> Result<ServerResponse, Box<dyn Error>>{
    let serialized_action = ron::to_string(&action)?;
    match stream.write(serialized_action.as_bytes()){
        Ok(bytes_written) => {
            if bytes_written == 0{} else {}
        }
        Err(e) => {return Err(Box::new(e));}
    }
    stream.flush()?;

    let mut response_buffer = [0u8; MESSAGE_SIZE];
    match stream.read(&mut response_buffer){
        Ok(size) => {
            let my_string = String::from_utf8_lossy(&response_buffer[0..size]);
            match ron::from_str(&my_string){
                Ok(response) => {return Ok(response)},
                Err(e) => {return Err(Box::new(e));}
            };
        }
        Err(e) => {
            println!("An error occurred. {}", e);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return Err(Box::new(e));
        }
    }
}
