#![allow(dead_code)]

pub const CMD_STATUS: u8 = 0x10;
pub const CMD_GET_MODE: u8 = 0x11;
pub const CMD_HARD_RESET: u8 = 0x12;
pub const CMD_GET_VDC: u8 = 0x13;
pub const CMD_RTC_GET: u8 = 0x14;
pub const CMD_RTC_SET: u8 = 0x15;
pub const CMD_FLA_RD: u8 = 0x16;
pub const CMD_FLA_WR: u8 = 0x17;
pub const CMD_FLA_WR_SDC: u8 = 0x18;
pub const CMD_MEM_RD: u8 = 0x19;
pub const CMD_MEM_WR: u8 = 0x1A;
pub const CMD_MEM_SET: u8 = 0x1B;
pub const CMD_MEM_TST: u8 = 0x1C;
pub const CMD_MEM_CRC: u8 = 0x1D;
pub const CMD_FPG_USB: u8 = 0x1E;
pub const CMD_FPG_SDC: u8 = 0x1F;
pub const CMD_FPG_FLA: u8 = 0x20;
pub const CMD_FPG_CFG: u8 = 0x21;
pub const CMD_USB_WR: u8 = 0x22;
pub const CMD_FIFO_WR: u8 = 0x23;
pub const CMD_UART_WR: u8 = 0x24;
pub const CMD_REINIT: u8 = 0x25;
pub const CMD_SYS_INF: u8 = 0x26;
pub const CMD_GAME_CTR: u8 = 0x27;
pub const CMD_UPD_EXEC: u8 = 0x28;

pub const CMD_DISK_INIT: u8 = 0xC0;
pub const CMD_DISK_RD: u8 = 0xC1;
pub const CMD_DISK_WR: u8 = 0xC2;
pub const CMD_F_DIR_OPN: u8 = 0xC3;
pub const CMD_F_DIR_RD: u8 = 0xC4;
pub const CMD_F_DIR_LD: u8 = 0xC5;
pub const CMD_F_DIR_SIZE: u8 = 0xC6;
pub const CMD_F_DIR_PATH: u8 = 0xC7;
pub const CMD_F_DIR_GET: u8 = 0xC8;
pub const CMD_F_FOPN: u8 = 0xC9;
pub const CMD_F_FRD: u8 = 0xCA;
pub const CMD_F_FRD_MEM: u8 = 0xCB;
pub const CMD_F_FWR: u8 = 0xCC;
pub const CMD_F_FWR_MEM: u8 = 0xCD;
pub const CMD_F_FCLOSE: u8 = 0xCE;
pub const CMD_F_FPTR: u8 = 0xCF;
pub const CMD_F_FINFO: u8 = 0xD0;
pub const CMD_F_FCRC: u8 = 0xD1;
pub const CMD_F_DIR_MK: u8 = 0xD2;
pub const CMD_F_DEL: u8 = 0xD3;

pub const CMD_USB_RECOV: u8 = 0xF0;
pub const CMD_RUN_APP: u8 = 0xF1;
