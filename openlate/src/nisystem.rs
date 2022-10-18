use crate::nichannel::niphyschan::PhysicalChannel;
use crate::nicommon::nierr::daqmx_check;
use crate::nidevice::Device;
use crate::nitask::Task;
use anyhow::Result;
use nidaqmx_sys::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::{addr_of_mut, null_mut};
use std::rc::Rc;

#[derive(Default, Debug)]
struct DriverVersion(u32, u32, u32);

#[repr(C)]
#[derive(Debug, Default)]
pub struct System {
  global_channel: u32,
  scales: u32,
  tasks: HashMap<String, Task>,
  devices: HashMap<String, Device>,
  driver_version: DriverVersion,
}

impl System {
  pub fn new() -> Result<System> {
    Ok(
      System::default()
        .driver_version()?
        .devices()?
        .system_tasks()?, // .physical_channel()?
    )
  }

  fn driver_version(mut self) -> Result<Self> {
    daqmx_check(unsafe {
      DAQmxGetSysNIDAQMajorVersion(addr_of_mut!(self.driver_version.0))
    })?;
    daqmx_check(unsafe {
      DAQmxGetSysNIDAQMinorVersion(addr_of_mut!(self.driver_version.1))
    })?;
    daqmx_check(unsafe {
      DAQmxGetSysNIDAQUpdateVersion(addr_of_mut!(self.driver_version.2))
    })?;
    Ok(self)
  }

  fn devices(mut self) -> Result<Self> {
    let buffersize =
      unsafe { DAQmxGetSysDevNames(std::ptr::null_mut(), 0_u32) };
    let mut buf = vec![0_u8; buffersize as usize];
    daqmx_check(unsafe {
      DAQmxGetSysDevNames(buf.as_mut_ptr() as *mut i8, buffersize as u32)
    })?;
    let devices = Rc::new(
      CStr::from_bytes_with_nul(buf.as_ref())?
        .to_str()?
        .to_string(), // .replace(",", "")
                      // .split_whitespace(),
    );

    for dev in devices.replace(",", "").split_whitespace() {
      self
        .devices
        .insert(dev.to_string(), Device::new(dev).unwrap());
    }
    Ok(self)
  }

  fn system_tasks(mut self) -> Result<Self> {
    let buffersize = unsafe { DAQmxGetSysTasks(null_mut(), 0_u32) };
    let mut buf = vec![0_u8; buffersize as usize];
    daqmx_check(unsafe {
      DAQmxGetSysTasks(buf.as_mut_ptr() as *mut i8, buffersize as u32)
    })?;
    CStr::from_bytes_with_nul(buf.as_ref())?
      .to_str()?
      .to_string()
      .replace(",", "")
      .split_whitespace()
      .for_each(|task| {
        self
          .tasks
          .insert(task.to_string(), Task::new(task).unwrap());
      });
    Ok(self)
  }

  // fn physical_channel(mut self) -> Result<Self> {}
}

#[test]
fn test_system() {
  let system = System::new().unwrap();

  test_driver_version(&system);
  test_sysget_devicenames(&system);
  test_sysget_task_names(&system);
}

#[cfg(test)]
fn test_driver_version(system: &System) {
  println!("Driver version: {:#?}", system.driver_version);
}

#[cfg(test)]
fn test_sysget_devicenames(system: &System) {
  println!("Device names: {:#?}", system.devices)
}

#[cfg(test)]
fn test_sysget_task_names(system: &System) {
  println!("Task names: {:#?}", &system.tasks)
}
