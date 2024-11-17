use std::{fs::File, io::Read};

use clap::{arg, command, Command};

mod cmd;
mod edio;

const FAT_CREATE_ALWAYS: u8 = 0x08;
const FAT_WRITE: u8 = 0x02;

fn main() {
    let matches = command!()
        .subcommands([
            Command::new("ls").arg(arg!([PATH]).default_value("")),
            Command::new("cp")
                .arg(arg!(<SRC> "Source file"))
                .arg(arg!(<DEST> "Destination file")),
        ])
        .subcommand_required(true)
        .get_matches();

    let mut edio = edio::EDIO::new();
    if !edio.get_status() {
        panic!("Failed to connect to EverDrive");
    }
    edio.connect();
    if edio.is_service_mode() {
        println!("Restarting");
        edio.exit_service_mode();
    }

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("Required");
            edio.dir_open(path);
            edio.dir_load(path, 1);
            for _ in 0..edio.dir_get_size() {
                println!("{:?}", edio.dir_read());
            }
        }
        Some(("cp", sub_matches)) => {
            let src = sub_matches.get_one::<String>("SRC").expect("SRC Required");
            let dest = sub_matches
                .get_one::<String>("DEST")
                .expect("DEST Required");

            let metadata = std::fs::metadata(src).expect("unable to read rom metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            let mut file = File::open(src).expect("Failed to open ROM");
            file.read(&mut buffer).expect("buffer overflow");

            edio.file_open(dest, FAT_CREATE_ALWAYS | FAT_WRITE);
            edio.file_write(buffer.as_slice());
            edio.file_close();
        }
        _ => {}
    }
}

fn run() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        match p.port_type {
            serialport::SerialPortType::UsbPort(info) => {
                println!("{} {}", p.port_name, info.product.unwrap());
            }
            _ => {}
        }
    }

    // let metadata = std::fs::metadata("from_below.nes").expect("unable to read rom metadata");
    // let mut buffer = vec![0; metadata.len() as usize];
    // let mut file = File::open("from_below.nes").expect("Failed to open ROM");
    // file.read(&mut buffer).expect("buffer overflow");
    //
    // edio.file_open("from_below.nes", FAT_CREATE_ALWAYS | FAT_WRITE);
    // edio.file_write(buffer.as_slice());
    // edio.file_close();
}
