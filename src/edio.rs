use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{thread::sleep, time::Duration};

use crate::cmd;

pub struct EDIO {
    port: Option<Box<dyn serialport::SerialPort>>,
}

static IDENTIFIER: &str = "EverDrive N8";
const ADDR_FIFO: u32 = 0x1810000;

#[allow(dead_code)]
impl EDIO {
    pub fn new() -> Self {
        Self {
            port: Some(EDIO::open_port()),
        }
    }

    fn seek() -> String {
        for port in serialport::available_ports().expect("No serial ports available") {
            match port.port_type {
                serialport::SerialPortType::UsbPort(port_type) => {
                    if cfg!(windows) {
                        // on Windows, names aren't provided by the library yet
                        return port.port_name;
                    }

                    if port_type.product.unwrap() == IDENTIFIER {
                        return port.port_name;
                    }
                }
                _ => {} // Ignore Bluetooth, etc.
            }
        }
        panic!("Failed to find a suitable port");
    }

    #[inline]
    fn open_port() -> Box<dyn serialport::SerialPort> {
        serialport::new(EDIO::seek(), 115_200)
            .timeout(Duration::from_millis(300))
            .open()
            .expect("Failed to open port")
    }
    #[inline]
    fn get_port(&mut self) -> &mut Box<dyn serialport::SerialPort> {
        if let Some(port) = &mut self.port {
            return port;
        }
        unreachable!("Port not connected");
    }
    pub fn connect(&mut self) {
        if !self.get_status() {
            panic!("Failed to connect");
        }
        self.get_port()
            .set_timeout(Duration::from_millis(2000))
            .expect("Failed to set timeout");
    }

    fn tx_str(&mut self, buff: &str) {
        self.get_port()
            .write_u16::<LittleEndian>(buff.len().try_into().unwrap())
            .expect("Failed to write string length");
        self.get_port()
            .write(buff.as_bytes())
            .expect("Failed to write string");
    }
    fn tx_data(&mut self, buff: &[u8]) {
        self.get_port()
            .write(&buff)
            .expect("Failed to write message");
    }
    fn tx8(&mut self, data: u8) {
        self.get_port().write_u8(data).expect("Failed to write u8")
    }
    fn tx16(&mut self, data: u16) {
        self.get_port()
            .write_u16::<LittleEndian>(data)
            .expect("Failed to write u16");
    }
    fn tx32(&mut self, data: u32) {
        self.get_port()
            .write_u32::<LittleEndian>(data)
            .expect("Failed to write u32");
    }

    fn rx_data(&mut self, len: u32) -> Vec<u8> {
        let mut data = vec![0; len as usize].into_boxed_slice();
        self.get_port()
            .read_exact(data.as_mut())
            .expect("Failed to read data");

        data.to_vec()
    }

    fn rx_string(&mut self) -> String {
        let len = self.rx16();
        if len == 0 {
            return "".to_string();
        }

        let length: usize = len.into();
        let mut data = vec![0; length].into_boxed_slice();

        self.get_port()
            .read_exact(data.as_mut())
            .expect("Failed to read string");

        String::from_utf8((*data).to_vec())
            .expect("Failed to parse string")
            .clone()
    }

    fn rx32(&mut self) -> u32 {
        self.get_port()
            .read_u32::<LittleEndian>()
            .expect("Failed to read u32")
    }

    fn rx16(&mut self) -> u16 {
        self.get_port()
            .read_u16::<LittleEndian>()
            .expect("Failed to read u16")
    }

    fn rx8(&mut self) -> u8 {
        self.get_port().read_u8().expect("Failed to read u8")
    }

    fn tx_data_ack(&mut self, buff: &[u8]) {
        let mut len = buff.len();
        let mut offset = 0;

        let pb = indicatif::ProgressBar::new(len.try_into().unwrap());

        while len > 0 {
            let resp = self.rx8();
            if resp != 0 {
                panic!("tx ack: {}", resp);
            }

            let mut block = 1024;
            if block > len {
                block = len;
            }

            pb.set_position(offset.try_into().unwrap());

            self.tx_data(&buff[offset..(offset + block)]);
            len -= block;
            offset += block;
        }

        pb.finish_and_clear();
    }

    fn tx_cmd(&mut self, cmd_code: u8) {
        let cmd = [b'+', b'+' ^ 0xff, cmd_code, cmd_code ^ 0xff];
        self.tx_data(&cmd);
    }

    pub fn get_status(&mut self) -> bool {
        self.tx_cmd(cmd::CMD_STATUS);
        let resp = self.rx16();

        (resp & 0xff00) == 0xA500
    }

    pub fn dir_read(&mut self) -> FileInfo {
        self.tx_cmd(cmd::CMD_F_DIR_RD);
        self.tx16(0xff);

        let response = self.rx8();
        if response != 0 {
            panic!("Dir Read Error: {}", response);
        }

        self.rx_file_info()
    }

    pub fn dir_get_size(&mut self) -> u16 {
        self.tx_cmd(cmd::CMD_F_DIR_SIZE);
        self.rx16()
    }

    pub fn dir_open(&mut self, path: &str) {
        self.tx_cmd(cmd::CMD_F_DIR_OPN);
        self.tx_str(path);
        if !self.get_status() {
            panic!("Failed to open dir {}", path);
        }
    }

    // Populates size of dir
    pub fn dir_load(&mut self, path: &str, sorted: u8) {
        self.tx_cmd(cmd::CMD_F_DIR_LD);
        self.tx8(sorted);
        self.tx_str(path);
        if !self.get_status() {
            panic!("Failed to load dir {}", path);
        }
    }

    pub fn file_info(&mut self, path: &str) -> FileInfo {
        self.tx_cmd(cmd::CMD_F_FINFO);
        self.tx_str(path);
        let response = self.rx8();
        if response != 0 {
            panic!("File access error: {}", response);
        }

        self.rx_file_info()
    }

    pub fn file_read(&mut self, length: u32) -> Vec<u8> {
        let mut len = length;
        let mut offset = 0;

        self.tx_cmd(cmd::CMD_F_FRD);
        self.tx32(len);

        let mut buff = vec![];

        while len > 0 {
            let mut block = 4096;

            if block > len {
                block = len;
            }
            let response = self.rx8();
            if response != 0 {
                panic!("File read error: {}", response);
            }

            buff.append(&mut self.rx_data(block));

            offset += block;
            len -= block;
        }

        return buff;
    }

    fn rx_file_info(&mut self) -> FileInfo {
        let size = self.rx32();
        let date = self.rx16();
        let time = self.rx16();
        let attrib = self.rx8();

        let day = date & 31;
        let month = (date >> 5) & 15;
        let year = (date >> 9) + 1980;
        let hour = time >> 11;
        let min = (time >> 5) & 0x3F;
        let sec = (time & 0x1F) * 2;

        let datetime = chrono::NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
            .expect("Failed to create date")
            .and_hms_opt(hour.into(), min.into(), sec.into())
            .expect("Failed to create time");

        let name = self.rx_string();

        FileInfo {
            size,
            date: datetime,
            attrib,
            name,
        }
    }

    pub fn is_service_mode(&mut self) -> bool {
        self.tx_cmd(cmd::CMD_GET_MODE);
        self.rx8() == 0xA1
    }

    pub fn exit_service_mode(&mut self) {
        if !self.is_service_mode() {
            return;
        }

        self.tx_cmd(cmd::CMD_RUN_APP);
        self.boot_wait();
        if self.is_service_mode() {
            panic!("Device stuck in service mode");
        }
    }

    fn boot_wait(&mut self) {
        sleep(Duration::from_millis(1000));
        self.port = None;
        sleep(Duration::from_millis(1000));
        self.port = Some(EDIO::open_port());

        self.connect();
    }

    pub fn file_open(&mut self, path: &str, mode: u8) {
        self.tx_cmd(cmd::CMD_F_FOPN);
        self.tx8(mode);
        self.tx_str(path);
        if !self.get_status() {
            panic!("Failed to open file {}", path);
        }
    }

    pub fn file_write(&mut self, data: &[u8]) {
        self.tx_cmd(cmd::CMD_F_FWR);
        self.tx32(data.len().try_into().expect("Failed to convert length"));
        self.tx_data_ack(data);
        if !self.get_status() {
            panic!("Failed to write file");
        }
    }

    pub fn file_close(&mut self) {
        self.tx_cmd(cmd::CMD_F_FCLOSE);
        if !self.get_status() {
            panic!("Failed to close file");
        }
    }

    fn mem_wr(&mut self, addr: u32, buff: &[u8]) {
        self.tx_cmd(cmd::CMD_MEM_WR);
        self.tx32(addr);
        self.tx32(buff.len().try_into().expect("Failed to convert length"));
        self.tx8(0);
        self.tx_data(buff);
    }

    pub fn rtc_get(&mut self) {
        self.tx_cmd(cmd::CMD_RTC_GET);
        let buff = [
            self.rx8(),
            self.rx8(),
            self.rx8(),
            self.rx8(),
            self.rx8(),
            self.rx8(),
        ];

        println!(
            "{}/{}/{} {:02}:{:02}:{:02}",
            hex2dec(buff[0]) as u16 + 2000,
            hex2dec(buff[1]),
            hex2dec(buff[2]),
            hex2dec(buff[3]),
            hex2dec(buff[4]),
            hex2dec(buff[5]),
        );
    }

    fn fifo_wr(&mut self, buff: &[u8]) {
        self.mem_wr(ADDR_FIFO, buff);
    }

    pub fn fifo_cmd(&mut self, cmd: u8) {
        let buff = [b'*', cmd];
        self.fifo_wr(&buff);
    }

    fn fifo_tx_str(&mut self, path: &str) {
        let mut len = vec![];
        len.write_u16::<LittleEndian>(path.len() as u16)
            .expect("Failed to write length");
        self.fifo_wr(&len);
        self.fifo_wr(path.as_bytes());
    }

    pub fn run_game(&mut self) {
        self.fifo_cmd(b's');
    }

    pub fn sel_game(&mut self, path: &str) -> u16 {
        self.fifo_cmd(b'n');
        self.fifo_tx_str(path);
        let response = self.rx8();
        if response != 0 {
            panic!("Failed to select game");
        }

        self.rx16() // Mapper
    }
}

#[inline]
fn hex2dec(val: u8) -> u8 {
    ((val >> 4) & 0xF) * 10 + (val & 0xF)
}

#[derive(Debug)]
pub struct FileInfo {
    pub size: u32,
    pub date: chrono::NaiveDateTime,
    pub attrib: u8,
    pub name: String,
}
