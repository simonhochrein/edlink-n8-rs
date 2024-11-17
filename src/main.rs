use std::{
    fs::File,
    io::{Read, Write},
};

use anstream::println;
use anstyle::{Ansi256Color, Style};
use clap::{arg, command, Command};
use indicatif::HumanBytes;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

mod cmd;
mod edio;

const FAT_READ: u8 = 0x01;
const FAT_WRITE: u8 = 0x02;
const FAT_OPEN_EXISTING: u8 = 0x00;
const FAT_CREATE_NEW: u8 = 0x04;
const FAT_CREATE_ALWAYS: u8 = 0x08;
const FAT_OPEN_ALWAYS: u8 = 0x10;
const FAT_OPEN_APPEND: u8 = 0x30;

fn main() {
    let matches = command!()
        .subcommands([
            Command::new("run"),
            Command::new("ls").arg(arg!([PATH]).default_value("")),
            Command::new("push")
                .arg(arg!(<SRC> "Source file"))
                .arg(arg!(<DEST> "Destination file")),
            Command::new("pull")
                .arg(arg!(<SRC> "Source file"))
                .arg(arg!(<DEST> "Destination file")),
            Command::new("rtc"),
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
        Some(("run", sub_matches)) => {
            edio.sel_game("pong.nes");
            edio.run_game();
        }
        Some(("ls", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("Required");
            edio.dir_open(path);
            edio.dir_load(path, 1);

            let mut grid = Grid::new(GridOptions {
                filling: Filling::Spaces(1),
                direction: Direction::LeftToRight,
            });

            for _ in 0..edio.dir_get_size() {
                let entry = edio.dir_read();
                let size_color = Style::new().fg_color(Some(Ansi256Color(8).into()));
                let timestamp_color = Style::new().fg_color(Some(Ansi256Color(2).into()));
                let dir_color = Style::new().bold().fg_color(Some(Ansi256Color(5).into()));

                grid.add(Cell::from(format!(
                    "{size_color}{}{size_color:#}",
                    HumanBytes(entry.size.into())
                )));
                grid.add(Cell::from(format!(
                    "{timestamp_color}{}{timestamp_color:#}",
                    entry.date.format("%d %b %Y %H:%M:%S")
                )));
                grid.add(Cell::from(if entry.attrib & 0x10 > 0 {
                    format!("{dir_color}{}/{dir_color:#}", entry.name)
                } else {
                    entry.name
                }))

                //     println!(
                //         "{size_color}{}{size_color:#}\t{timestamp_color}{}{timestamp_color:#}\t{2}",
                //         HumanBytes(entry.size.into()),
                //         entry.date.format("%d %b %Y"),
                //         if entry.attrib & 0x10 > 0 {
                //             format!("{dir_color}{}/{dir_color:#}", entry.name)
                //         } else {
                //             entry.name
                //         }
                //     );
            }
            println!("{}", grid.fit_into_columns(3));
        }
        Some(("push", sub_matches)) => {
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
        Some(("pull", sub_matches)) => {
            let src = sub_matches.get_one::<String>("SRC").expect("SRC Required");
            let dest = sub_matches
                .get_one::<String>("DEST")
                .expect("DEST Required");

            let metadata = edio.file_info(src);
            edio.file_open(src, FAT_READ | FAT_OPEN_EXISTING);
            let buffer = edio.file_read(metadata.size);
            edio.file_close();

            let mut output = File::create(dest).expect("Failed to open destination for writing");
            output
                .write(buffer.as_slice())
                .expect("Failed to write destination file");
        }
        Some(("rtc", _)) => {
            edio.rtc_get();
        }
        _ => {}
    }
}

#[allow(dead_code)]
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
