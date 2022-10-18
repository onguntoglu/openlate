#![allow(non_snake_case)]
#![allow(non_snake_case)]
use crate::nichannel::niphyschan::{PhysicalChan, PhysicalChannel};
use crate::nicommon::niattr::ProductCategory;
use crate::nicommon::nierr::daqmx_check;
use anyhow::Result;
// use itertools::izip;
use nidaqmx_sys::*;
use num_enum::FromPrimitive;
use std::collections::btree_map::Iter;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, CStr, CString};
use std::ptr::addr_of_mut;
use std::rc::Rc;

#[derive(Default, Debug)]
struct IsSimulated(u32);

#[derive(Default, Debug)]
struct Product {
  category: ProductCategory,
  ty: String,
  number: u32,
  sn: u32,
}

#[derive(Default, Debug)]
struct Accessory {
  ty: String,
  number: u32,
  sn: u32,
}

#[derive(Debug, Default)]
struct FieldDAQ {
  device_names: Vec<String>,
  bank_device_names: Vec<String>,
}

#[derive(Debug, Default)]
struct Chassis {
  module_device_names: Vec<String>,
}

#[derive(Debug, Default)]
enum Status {
  Supported,
  #[default]
  NotSupported,
}

#[derive(Default, Debug)]
struct Triggering {
  analog_trig: Status,
  digital_trig: Status,
  time_trig: Status,
}

#[derive(Default, Debug)]
struct Identification(Product, Accessory, FieldDAQ, Chassis);

#[derive(Debug)]
pub struct Device {
  devname: CString,
  is_simulated: IsSimulated,
  identification: Identification,
  triggering: Triggering,
  // pchans: HashMap<String, PhysicalChannel<dyn State>>,
}

impl Device {
  pub fn to_cchar(&self) -> *const c_char {
    self.devname.as_ptr().clone()
  }
  pub fn new(name: &str) -> Result<Device> {
    Ok(
      Device {
        devname: CString::new(name)?,
        is_simulated: IsSimulated::default(),
        identification: Identification::default(),
        triggering: Triggering::default(),
        // pchans: HashMap::default(),
      }
      .is_simulated()?
      .identification()?
      .devchan_ana()?
      .devchan_dig()?
      .devchan_ctrio()?,
    )
  }

  fn devchan_dig(mut self) -> Result<Self> {
    let bsize: (i32, i32, i32, i32) = (
      unsafe {
        DAQmxGetDevDOPorts(self.devname.as_ptr(), std::ptr::null_mut(), 0_u32)
      },
      unsafe {
        DAQmxGetDevDOLines(self.devname.as_ptr(), std::ptr::null_mut(), 0_u32)
      },
      unsafe {
        DAQmxGetDevDIPorts(self.devname.as_ptr(), std::ptr::null_mut(), 0_u32)
      },
      unsafe {
        DAQmxGetDevDILines(self.devname.as_ptr(), std::ptr::null_mut(), 0_u32)
      },
    );
    let mut buffer = (
      vec![0_u8; bsize.0 as usize], // DOPorts
      vec![0_u8; bsize.1 as usize], // DOLines
      vec![0_u8; bsize.2 as usize], // DIPorts
      vec![0_u8; bsize.3 as usize], // DILines
    );
    unsafe {
      daqmx_check(DAQmxGetDevDOPorts(
        self.devname.as_ptr(),
        buffer.0.as_mut_ptr() as *mut i8,
        bsize.0 as u32,
      ))?;
      daqmx_check(DAQmxGetDevDOLines(
        self.devname.as_ptr(),
        buffer.1.as_mut_ptr() as *mut i8,
        bsize.1 as u32,
      ))?;
      daqmx_check(DAQmxGetDevDIPorts(
        self.devname.as_ptr(),
        buffer.2.as_mut_ptr() as *mut i8,
        bsize.2 as u32,
      ))?;
      daqmx_check(DAQmxGetDevDILines(
        self.devname.as_ptr(),
        buffer.3.as_mut_ptr() as *mut i8,
        bsize.3 as u32,
      ))?;
    }
    let (dop, dol, dip, dil) = (
      CStr::from_bytes_with_nul(buffer.0.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
      CStr::from_bytes_with_nul(buffer.1.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
      CStr::from_bytes_with_nul(buffer.2.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
      CStr::from_bytes_with_nul(buffer.3.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
    );

    for el in dop.split_whitespace() {
      // self.pchans.insert(el.to_string(), PhysicalChannel::new(el));
    }
    // for el in dol.split_whitespace() {
    //     self.pchans.insert(
    //         el.to_string(),
    //         PhysicalChannel::new(el),
    //     );
    // }
    for el in dip.split_whitespace() {
      // self.pchans.insert(el.to_string(), PhysicalChannel::new(el));
    }
    // for el in dil.split_whitespace() {
    //     self.pchans.insert(
    //         el.to_string(),
    //         PhysicalChannel::new(el),
    //     );
    // }

    Ok(self)
  }

  fn devchan_ana(mut self) -> Result<Self> {
    let bsize: (i32, i32) = (
      unsafe {
        DAQmxGetDevAOPhysicalChans(
          self.devname.as_ptr(),
          std::ptr::null_mut(),
          0_u32,
        )
      },
      unsafe {
        DAQmxGetDevAIPhysicalChans(
          self.devname.as_ptr(),
          std::ptr::null_mut(),
          0_u32,
        )
      },
    );
    let mut buffer =
      (vec![0_u8; bsize.0 as usize], vec![0_u8; bsize.1 as usize]);
    daqmx_check(unsafe {
      DAQmxGetDevAOPhysicalChans(
        self.devname.as_ptr(),
        buffer.0.as_mut_ptr() as *mut i8,
        bsize.0 as u32,
      )
    })?;
    daqmx_check(unsafe {
      DAQmxGetDevAIPhysicalChans(
        self.devname.as_ptr(),
        buffer.1.as_mut_ptr() as *mut i8,
        bsize.1 as u32,
      )
    })?;

    let (first, second) = (
      CStr::from_bytes_with_nul(buffer.0.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
      CStr::from_bytes_with_nul(buffer.1.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
    );

    for ao in first.split_whitespace() {
      // self.pchans.insert(
      //     ao.to_string(),
      //     PhysicalChannel::new_id(ao).unwrap().ao().into(),
      // );
    }
    for ai in second.split_whitespace() {
      // self.pchans
      //     .insert(ai.to_string(), PhysicalChannel::new_id(ai).unwrap().ai());
    }
    Ok(self)
  }
  fn devchan_ctrio(mut self) -> Result<Self> {
    let bsize: (i32, i32) = (
      unsafe {
        DAQmxGetDevCIPhysicalChans(
          self.devname.as_ptr(),
          std::ptr::null_mut(),
          0_u32,
        )
      },
      unsafe {
        DAQmxGetDevCOPhysicalChans(
          self.devname.as_ptr(),
          std::ptr::null_mut(),
          0_u32,
        )
      },
    );
    let mut buffer: (Vec<u8>, Vec<u8>) =
      (vec![0_u8; bsize.0 as usize], vec![0_u8; bsize.1 as usize]);
    daqmx_check(unsafe {
      DAQmxGetDevCIPhysicalChans(
        self.devname.as_ptr(),
        buffer.0.as_mut_ptr() as *mut i8,
        bsize.0 as u32,
      )
    })?;
    daqmx_check(unsafe {
      DAQmxGetDevCOPhysicalChans(
        self.devname.as_ptr(),
        buffer.1.as_mut_ptr() as *mut i8,
        bsize.1 as u32,
      )
    })?;
    let mut cihash: HashSet<String> = HashSet::new();
    let mut cohash: HashSet<String> = HashSet::new();
    let (first, second) = (
      CStr::from_bytes_with_nul(buffer.0.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
      CStr::from_bytes_with_nul(buffer.1.as_ref())?
        .to_str()?
        .to_string()
        .replace(",", ""),
    );

    for ci in first.split_whitespace() {
      cihash.insert(ci.to_string());
    }
    for co in second.split_whitespace() {
      cohash.insert(co.to_string());
    }

    // Counter input channels can be counter output channels simultaneously

    // // Only counter input channels
    // cihash.difference(&cohash).for_each(|el| {
    //     self.pchans.insert(el.to_string(), PhysicalChannel::new(el));
    // });
    // // Only counter output channels
    // cohash.difference(&cihash).for_each(|el| {
    //     self.pchans.insert(el.to_string(), PhysicalChannel::new(el));
    // });
    // // Both counter input and output
    // cihash.intersection(&cohash).for_each(|el| {
    //     self.pchans.insert(el.to_string(), PhysicalChannel::new(el));
    // });
    Ok(self)
  }

  fn is_simulated(mut self) -> Result<Self> {
    daqmx_check(unsafe {
      DAQmxGetDevIsSimulated(
        self.devname.as_ptr(),
        addr_of_mut!(self.is_simulated.0),
      )
    })?;
    Ok(self)
  }

  fn identification(mut self) -> Result<Self> {
    // Product identification
    let mut data: i32 = 0;
    daqmx_check(unsafe {
      DAQmxGetDevProductCategory(self.devname.as_ptr(), addr_of_mut!(data))
    })?;
    self.identification.0.category = ProductCategory::from_primitive(data);
    let buffersize: i32 = unsafe {
      DAQmxGetDevProductType(self.devname.as_ptr(), std::ptr::null_mut(), 0_u32)
    };
    let mut buffer = vec![0_u8; buffersize as usize];
    daqmx_check(unsafe {
      DAQmxGetDevProductType(
        self.devname.as_ptr(),
        buffer.as_mut_ptr() as *mut i8,
        buffersize as u32,
      )
    })?;
    self.identification.0.ty = CStr::from_bytes_with_nul(buffer.as_ref())?
      .to_str()?
      .to_string();
    daqmx_check(unsafe {
      DAQmxGetDevProductNum(
        self.devname.as_ptr(),
        addr_of_mut!(self.identification.0.number),
      )
    })?;
    daqmx_check(unsafe {
      DAQmxGetDevSerialNum(
        self.devname.as_ptr(),
        addr_of_mut!(self.identification.0.sn),
      )
    })?;
    Ok(self)

    // Triggering
  }
}

#[test]
fn test_device_pxi1slot5() {
  let pxi1slot5 = Device::new("PXI1Slot5").unwrap();
  println!("Device: {:#?}", pxi1slot5);
}

#[test]
fn test_device_pxi1slot6() {
  let pxi1slot6 = Device::new("PXI1Slot6").unwrap();
  println!("Device: {:#?}", pxi1slot6);
}

#[test]
fn test_device_pxi1slot4() {
  let pxi1slot4 = Device::new("PXI1Slot4").unwrap();
  println!("Device: {:#?}", pxi1slot4);
}

// //********** Device **********
// //*** Set/Get functions for DAQmx_Dev_IsSimulated ***
// int32 __CFUNC DAQmxGetDevIsSimulated(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_ProductCategory ***
// // Uses value set ProductCategory
// int32 __CFUNC DAQmxGetDevProductCategory(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_ProductType ***
// int32 __CFUNC DAQmxGetDevProductType(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_ProductNum ***
// int32 __CFUNC DAQmxGetDevProductNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_SerialNum ***
// int32 __CFUNC DAQmxGetDevSerialNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_Accessory_ProductTypes ***
// int32 __CFUNC DAQmxGetDevAccessoryProductTypes(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_Accessory_ProductNums ***
// int32 __CFUNC DAQmxGetDevAccessoryProductNums(const char device[], uInt32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_Accessory_SerialNums ***
// int32 __CFUNC DAQmxGetDevAccessorySerialNums(const char device[], uInt32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Carrier_SerialNum ***
// int32 __CFUNC DAQmxGetCarrierSerialNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_FieldDAQ_DevName ***
// int32 __CFUNC DAQmxGetFieldDAQDevName(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_FieldDAQ_BankDevNames ***
// int32 __CFUNC DAQmxGetFieldDAQBankDevNames(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_Chassis_ModuleDevNames ***
// int32 __CFUNC DAQmxGetDevChassisModuleDevNames(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_AnlgTrigSupported ***
// int32 __CFUNC DAQmxGetDevAnlgTrigSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_DigTrigSupported ***
// int32 __CFUNC DAQmxGetDevDigTrigSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_TimeTrigSupported ***
// int32 __CFUNC DAQmxGetDevTimeTrigSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_PhysicalChans ***
// int32 __CFUNC DAQmxGetDevAIPhysicalChans(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_AI_SupportedMeasTypes ***
// // Uses value set AIMeasurementType
// int32 __CFUNC DAQmxGetDevAISupportedMeasTypes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_MaxSingleChanRate ***
// int32 __CFUNC DAQmxGetDevAIMaxSingleChanRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_MaxMultiChanRate ***
// int32 __CFUNC DAQmxGetDevAIMaxMultiChanRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_MinRate ***
// int32 __CFUNC DAQmxGetDevAIMinRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_SimultaneousSamplingSupported ***
// int32 __CFUNC DAQmxGetDevAISimultaneousSamplingSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_NumSampTimingEngines ***
// int32 __CFUNC DAQmxGetDevAINumSampTimingEngines(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_SampModes ***
// // Uses value set AcquisitionType
// int32 __CFUNC DAQmxGetDevAISampModes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_NumSyncPulseSrcs ***
// int32 __CFUNC DAQmxGetDevAINumSyncPulseSrcs(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_TrigUsage ***
// // Uses bits from enum TriggerUsageTypeBits
// int32 __CFUNC DAQmxGetDevAITrigUsage(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_VoltageRngs ***
// int32 __CFUNC DAQmxGetDevAIVoltageRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_VoltageIntExcitDiscreteVals ***
// int32 __CFUNC DAQmxGetDevAIVoltageIntExcitDiscreteVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_VoltageIntExcitRangeVals ***
// int32 __CFUNC DAQmxGetDevAIVoltageIntExcitRangeVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_ChargeRngs ***
// int32 __CFUNC DAQmxGetDevAIChargeRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_CurrentRngs ***
// int32 __CFUNC DAQmxGetDevAICurrentRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_CurrentIntExcitDiscreteVals ***
// int32 __CFUNC DAQmxGetDevAICurrentIntExcitDiscreteVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_BridgeRngs ***
// int32 __CFUNC DAQmxGetDevAIBridgeRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_ResistanceRngs ***
// int32 __CFUNC DAQmxGetDevAIResistanceRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_FreqRngs ***
// int32 __CFUNC DAQmxGetDevAIFreqRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_Gains ***
// int32 __CFUNC DAQmxGetDevAIGains(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_Couplings ***
// // Uses bits from enum CouplingTypeBits
// int32 __CFUNC DAQmxGetDevAICouplings(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_AI_LowpassCutoffFreqDiscreteVals ***
// int32 __CFUNC DAQmxGetDevAILowpassCutoffFreqDiscreteVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_LowpassCutoffFreqRangeVals ***
// int32 __CFUNC DAQmxGetDevAILowpassCutoffFreqRangeVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_AI_DigFltr_Types ***
// // Uses value set FilterType2
// int32 __CFUNC DAQmxGetAIDigFltrTypes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqDiscreteVals ***
// int32 __CFUNC DAQmxGetDevAIDigFltrLowpassCutoffFreqDiscreteVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqRangeVals ***
// int32 __CFUNC DAQmxGetDevAIDigFltrLowpassCutoffFreqRangeVals(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AO_PhysicalChans ***
// int32 __CFUNC DAQmxGetDevAOPhysicalChans(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_AO_SupportedOutputTypes ***
// // Uses value set AOOutputChannelType
// int32 __CFUNC DAQmxGetDevAOSupportedOutputTypes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AO_MaxRate ***
// int32 __CFUNC DAQmxGetDevAOMaxRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_AO_MinRate ***
// int32 __CFUNC DAQmxGetDevAOMinRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_AO_SampClkSupported ***
// int32 __CFUNC DAQmxGetDevAOSampClkSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_AO_NumSampTimingEngines ***
// int32 __CFUNC DAQmxGetDevAONumSampTimingEngines(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_AO_SampModes ***
// // Uses value set AcquisitionType
// int32 __CFUNC DAQmxGetDevAOSampModes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AO_NumSyncPulseSrcs ***
// int32 __CFUNC DAQmxGetDevAONumSyncPulseSrcs(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_AO_TrigUsage ***
// // Uses bits from enum TriggerUsageTypeBits
// int32 __CFUNC DAQmxGetDevAOTrigUsage(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_AO_VoltageRngs ***
// int32 __CFUNC DAQmxGetDevAOVoltageRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AO_CurrentRngs ***
// int32 BiDAQmxGetDevAOCurrentRngs(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_AO_Gains ***
// int32 __CFUNC DAQmxGetDevAOGains(const char device[], float64 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_DI_Lines ***
// int32 __CFUNC DAQmxGetDevDILines(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_DI_Ports ***
// int32 __CFUNC DAQmxGetDevDIPorts(const char device[], char *Bi, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_DI_MaxRate ***
// int32 __CFUNC DAQmxGetDevDIMaxRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_DI_NumSampTimingEngines ***
// int32 __CFUNC DAQmxGetDevDINumSampTimingEngines(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_DI_TrigUsage ***
// // Uses bits from enum TriggerUsageTypeBits
// int32 __CFUNC DAQmxGetDevDITrigUsage(const char device[], int32 Bidata);
// //*** Set/Get functions for DAQmx_Dev_DO_Lines ***
// int32 __CFUNC DAQmxGetDevDOLines(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_DO_Ports ***
// int32 __CFUNC DAQmxGetDevDOPorts(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_DO_MaxRate ***
// int32 __CFUNC DAQmxGetDevDOMaxRate(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_DO_NumSampTimingEngines ***
// int32 __CFUNC DAQmxGetDevDONumSampTimingEngines(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_DO_TrigUsage ***
// // Uses bits from enum TriggerUsageTypeBits
// int32 __CFUNC DAQmxGetDevDOTrigUsage(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_CI_PhysicalChans ***
// int32 __CFUNC DAQmxGetDevCIPhysicalChans(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_CI_SupportedMeasTypes ***
// // Uses value set CIMeasurementType
// int32 __CFUNC DAQmxGetDevCISupportedMeasTypes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_CI_TrigUsage ***
// // Uses bits from enum TriggerUsageTypeBits
// int32 __CFUNC DAQmxGetDevCITrigUsage(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_CI_SampClkSupported ***
// int32 __CFUNC DAQmxGetDevCISampClkSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_CI_SampModes ***
// // Uses value set AcquisitionType
// int32 __CFUNC DAQmxGetDevCISampModes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_CI_MaxSize ***
// int32 __CFUNC DAQmxGetDevCIMaxSize(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_CI_MaxTimebase ***
// int32 __CFUNC DAQmxGetDevCIMaxTimebase(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_CO_PhysicalChans ***
// int32 __CFUNC DAQmxGetDevCOPhysicalChans(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_CO_SupportedOutputTypes ***
// // Uses value set COOutputType
// int32 __CFUNC DAQmxGetDevCOSupportedOutputTypes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_CO_SampClkSupported ***
// int32 __CFUNC DAQmxGetDevCOSampClkSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_CO_SampModes ***
// // Uses value set AcquisitionType
// int32 __CFUNC DAQmxGetDevCOSampModes(const char device[], int32 *data, uInt32 arraySizeInElements);
// //*** Set/Get functions for DAQmx_Dev_CO_TrigUsage ***
// // Uses bits from enum TriggerUsageTypeBits
// int32 __CFUNC DAQmxGetDevCOTrigUsage(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_CO_MaxSize ***
// int32 __CFUNC DAQmxGetDevCOMaxSize(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_CO_MaxTimebase ***
// int32 __CFUNC DAQmxGetDevCOMaxTimebase(const char device[], float64 *data);
// //*** Set/Get functions for DAQmx_Dev_TEDS_HWTEDSSupported ***
// int32 __CFUNC DAQmxGetDevTEDSHWTEDSSupported(const char device[], bool32 *data);
// //*** Set/Get functions for DAQmx_Dev_NumDMAChans ***
// int32 __CFUNC DAQmxGetDevNumDMAChans(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_BusType ***
// // Uses value set BusType
// int32 __CFUNC DAQmxGetDevBusType(const char device[], int32 *data);
// //*** Set/Get functions for DAQmx_Dev_PCI_BusNum ***
// int32 __CFUNC DAQmxGetDevPCIBusNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_PCI_DevNum ***
// int32 __CFUNC DAQmxGetDevPCIDevNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_PXI_ChassisNum ***
// int32 __CFUNC DAQmxGetDevPXIChassisNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_PXI_SlotNum ***
// int32 __CFUNC DAQmxGetDevPXISlotNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_CompactDAQ_ChassisDevName ***
// int32 __CFUNC DAQmxGetDevCompactDAQChassisDevName(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_CompactDAQ_SlotNum ***
// int32 __CFUNC DAQmxGetDevCompactDAQSlotNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_CompactRIO_ChassisDevName ***
// int32 __CFUNC DAQmxGetDevCompactRIOChassisDevName(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_CompactRIO_SlotNum ***
// int32 __CFUNC DAQmxGetDevCompactRIOSlotNum(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_TCPIP_Hostname ***
// int32 __CFUNC DAQmxGetDevTCPIPHostname(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_TCPIP_EthernetIP ***
// int32 __CFUNC DAQmxGetDevTCPIPEthernetIP(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_TCPIP_WirelessIP ***
// int32 __CFUNC DAQmxGetDevTCPIPWirelessIP(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_Terminals ***
// int32 __CFUNC DAQmxGetDevTerminals(const char device[], char *data, uInt32 bufferSize);
// //*** Set/Get functions for DAQmx_Dev_NumTimeTrigs ***
// int32 __CFUNC DAQmxGetDevNumTimeTrigs(const char device[], uInt32 *data);
// //*** Set/Get functions for DAQmx_Dev_NumTimestampEngines ***
// int32 __CFUNC DAQmxGetDevNumTimestampEngines(const char device[], uInt32 *data);
