use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

// struct Client {
//     Client_ID: String,
//     UUID: String,
// }

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    let mqtt_packet_type = buffer[0]>>4;
    // let _mqtt_packet_flag = buffer[0] & 0x0f;
    // println!("{}", mqtt_packet_type);
    match mqtt_packet_type {
        1 => {
            println!("CONNECT : {}",mqtt_packet_type);
            println!("{}",mqtt_connect(buffer))
        },
        2 =>{

        }
        _ => {
            println!("undefine type ! {}",mqtt_packet_type)
        }
    }
}

fn mqtt_connect(buffer: [u8; 512]) -> String{
    // "NAN".to_string()
    let mut _msg = String::new();
    _msg = "Done".to_string();
    let mqtt_packet_flag = buffer[0] & 0x0f;
    if mqtt_packet_flag != 0 {
        _msg = "Miss flag for CONNECT message".to_string();
    }else{
        let packet_lenght = buffer[1];
        if packet_lenght >= 10 {
            let _conv_mqtt_protocol_name: String = String::from_utf8(buffer[4..8].to_vec()).unwrap();
            if _conv_mqtt_protocol_name == "MQTT"{
                println!("{}",_conv_mqtt_protocol_name);
                if buffer[8] <= 4{
                    // println!("{}",String::from_utf8(buffer[9..20].to_vec()).unwrap());      
                    println!("{:b}",buffer[9]);
                    if buffer[9]>>7&1 == 0{
                        
                    }else{
                        _msg = "ERROR Connect Flags".to_string();
                    }
                }else{
                    _msg = "ERROR MQTT Protocol Level".to_string();
                }
            }else{
                _msg = "ERROR MQTT Protocol Name".to_string();
            }
        }else{
            _msg = "ERROR lenght for packet".to_string();
        }
    }
    
    
    _msg
}