use crate::get_phychan;
use crate::nicommon::niattr;
use crate::nicommon::nierr::daqmx_check;
use crate::nicommon::niprop::PhysicalChannelAttr;
use anyhow::Result;
use bumpalo::Bump;
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use nidaqmx_sys::*;
use once_cell::unsync::{Lazy, OnceCell};
use regex::{Regex, RegexSet};
use std::ffi::{c_char, c_void, CString};
use std::marker::PhantomData;
use std::ptr::{addr_of_mut, null_mut};
use std::rc::Rc;

struct Pch<S> {
  id: CString,
  state: S,
}

impl From<Pch<Proto>> for Pch<AI> {
  fn from(val: Pch<Proto>) -> Pch<AI> {
    Pch {
      id: val.id,
      state: AI(val.id),
    }
  }
}

pub enum PhysicalChannel {
  Prototype(Pch<Proto>),
  AnalogInput(Pch<AI>),
  AnalogOutput(Pch<AO>),
  DigitalInput(Pch<DI>),
  DigitalOutput(Pch<DO>),
  CounterInput(Pch<CI>),
  CounterOutput(Pch<CO>),
}

pub struct Proto {
  id: CString,
}

impl Proto {
  fn new(id: &str) -> Self {
    Proto {
      id: CString::new(id)
        .expect("Invalid CString, is there a NULL in the slice?"),
    }
  }
}

impl Pch<Proto> {
  fn new(id: &str) -> Self {
    Pch {
      id: CString::new(id)
        .expect("Invalid CString, is there a NULL in the slice?"),
      state: Proto::new(id),
    }
  }
}

impl AI {
  fn get(&self, attr: PhysicalChannelAttr -> Vec) {
    let mut value = Vec::new();
    unsafe {
      DAQmxGetPhysicalChanAttribute(
        self.0.as_ptr(),
        PhysicalChannelAttr::into(attr),
        value.as_mut_slice().as_mut_ptr(),
      );
    }
  }
}

#[derive(Debug, Default)]
pub struct AI(CString);

#[derive(Default)]
pub struct DI {
  data: [u8; 0],
}
#[derive(Default)]
struct DO {
  data: [u8; 0],
}

#[derive(Debug, Default)]
pub struct CI {
  supported_meastypes: Vec<niattr::CIMeasurementType>,
  trigger_usage: i32,
}
#[derive(Debug)]
pub struct CO {}

#[derive(Default)]
pub struct AO {
  supported_output_types: Vec<niattr::AOOutputChannelType>,
  supported_powerup_output_types: Vec<u32>,
}

// impl<T: PhysicalChan> PhysicalChannel<T> {
//   fn new(ch: ChannelType) -> PhysicalChannel<T> {
//     let channel = match ch {
//       ChannelType::AnalogInput(id) => PhysicalChannel::<AnalogInput> {
//         id: CString::new(id).unwrap(),
//         state: Box::new(AnalogInput::default()),
//       },
//       ChannelType::AnalogOutput(id) => PhysicalChannel::<T> {
//         id: CString::new(id).unwrap(),
//         state: AnalogOutput::default(),
//       },
//       ChannelType::DigitalOutput(id) => PhysicalChannel::<T> {
//         id: CString::new(id).unwrap(),
//         state: DigitalOutput::default(),
//       },
//       ChannelType::DigitalInput(id) => PhysicalChannel::<T> {
//         id: CString::new(id).unwrap(),
//         state: DigitalInput::default(),
//       },
//       ChannelType::CounterInput(id) => PhysicalChannel::<T> {
//         id: CString::new(id).unwrap(),
//         state: CounterInput::default(),
//       },
//       ChannelType::CounterOutput(id) => PhysicalChannel::<T> {
//         id: CString::new(id).unwrap(),
//         state: CounterOutput::default(),
//       },
//     };
//   }
// }

#[test]
fn test_physical_ao() {
  let _name = CString::new("PXI1Slot4/ai0").unwrap();
  let test = AI(_name);

  let bool323 = vec![0_i8; 1000];

  test.get(
    PhysicalChannelAttr::AI_SensorPower_Overcurrent,
    bool323.as_mut_ptr() as *mut c_void,
  )
}
