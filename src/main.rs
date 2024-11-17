use std::{fs::File, io::Read};

mod cmd;
mod edio;

const FAT_CREATE_ALWAYS: u8 = 0x08;
const FAT_WRITE: u8 = 0x02;

fn main() {
    // let ports = serialport::available_ports().expect("No ports found!");
    // for p in ports {
    //     match p.port_type {
    //         serialport::SerialPortType::UsbPort(info) => {
    //             println!("{} {}", p.port_name, info.pid);
    //         }
    //         _ => {}
    //     }
    // }
    let mut edio = edio::EDIO::new();
    if !edio.get_status() {
        panic!("Failed to connect to EverDrive");
    }
    edio.connect();
    if edio.is_service_mode() {
        println!("Restarting");
        edio.exit_service_mode();
    }

    // let metadata = std::fs::metadata("from_below.nes").expect("unable to read rom metadata");
    // let mut buffer = vec![0; metadata.len() as usize];
    // let mut file = File::open("from_below.nes").expect("Failed to open ROM");
    // file.read(&mut buffer).expect("buffer overflow");
    //
    // edio.file_open("from_below.nes", FAT_CREATE_ALWAYS | FAT_WRITE);
    // edio.file_write(buffer.as_slice());
    // edio.file_close();

    // edio.dir_load("", 1);

    edio.dir_open("");
    for _ in 0..edio.dir_get_size() {
        println!("{:?}", edio.dir_read());
    }
}
