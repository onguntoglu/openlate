use crate::nicommon::niattr;
use core::ffi::c_void;
use std::marker::PhantomData;

use std::ffi::CString;

trait VirtualChannelState {}

struct AnalogInput;
struct AnalogOutput;
struct DigitalInput;
struct DigitalOutput;
struct CounterInput;
struct CounterOutput;

impl VirtualChannelState for AnalogInput {}

#[repr(C)]
#[derive(Debug)]
pub struct VirtualChannel {
  // state: Box<ChannelState>,
  // marker: PhantomData<S>,
}

#[derive(Debug)]
#[non_exhaustive]
enum ChannelType {
  AnalogInput,
  AnalogOutput,
  DigitalInput,
  DigitalOutput,
  CounterInput,
  CounterOutput,
}

// impl<AnalogInput> VirtualChannelState<AnalogInput> {}

// impl Default for VirtualChannel {
//     fn default() -> VirtualChannel {
//         VirtualChannel {
//             task_handle: *mut TaskHandle,
//             physical_channel: (),
//             label: CString::new("default"),
//             config: ChannelConfig::NoConfig,
//         }
//     }
// }
