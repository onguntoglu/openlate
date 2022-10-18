#![feature(c_variadic)]
use once_cell::unsync::Lazy;
pub mod nidevice;
// pub mod attributes;
pub mod nisystem;
// pub mod utils;
pub mod nichannel;
pub mod nicommon;
pub mod nitask;

pub static mut GLOBAL_U8BUFFER: Lazy<Vec<u8>> =
  Lazy::new(|| Vec::from(vec![0; 20000]));
pub static mut GLOBAL_I32ARRAY: Lazy<Vec<i32>> =
  Lazy::new(|| Vec::from(vec![0; 20000]));

pub mod ividance;
