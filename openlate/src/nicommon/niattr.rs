#![allow(non_camel_case_types)]
use nidaqmx_sys;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ACExcitWireMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "4-Wire"]
  FourWire = nidaqmx_sys::DAQmx_Val_4Wire,
  #[doc = "5-Wire"]
  FiveWire = nidaqmx_sys::DAQmx_Val_5Wire,
  #[doc = "6-Wire"]
  SixWire = nidaqmx_sys::DAQmx_Val_6Wire,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ADCTimingMode {
  #[default]
  #[doc = "Automatic"]
  Automatic = nidaqmx_sys::DAQmx_Val_Automatic,
  #[doc = "High Resolution"]
  HighResolution = nidaqmx_sys::DAQmx_Val_HighResolution,
  #[doc = "High Speed"]
  HighSpeed = nidaqmx_sys::DAQmx_Val_HighSpeed,
  #[doc = "Best 50 Hz Rejection"]
  Best50HzRejection = nidaqmx_sys::DAQmx_Val_Best50HzRejection,
  #[doc = "Best 60 Hz Rejection"]
  Best60HzRejection = nidaqmx_sys::DAQmx_Val_Best60HzRejection,
  #[doc = "Custom"]
  Custom = nidaqmx_sys::DAQmx_Val_Custom,
}

#[derive(Default, Debug, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AIMeasurementType {
  #[default]
  #[doc = "Voltage"]
  Voltage = nidaqmx_sys::DAQmx_Val_Voltage,
  #[doc = "Voltage RMS"]
  VoltageRMS = nidaqmx_sys::DAQmx_Val_VoltageRMS,
  #[doc = "Current"]
  Current = nidaqmx_sys::DAQmx_Val_Current,
  #[doc = "Current RMS"]
  CurrentRMS = nidaqmx_sys::DAQmx_Val_CurrentRMS,
  #[doc = "More - Voltage - Custom with Excitation"]
  Voltage_CustomWithExcitation =
    nidaqmx_sys::DAQmx_Val_Voltage_CustomWithExcitation,
  #[doc = "More - Bridge (V/V)"]
  Bridge = nidaqmx_sys::DAQmx_Val_Bridge,
  #[doc = "Frequency"]
  Freq_Voltage = nidaqmx_sys::DAQmx_Val_Freq_Voltage,
  #[doc = "Resistance"]
  Resistance = nidaqmx_sys::DAQmx_Val_Resistance,
  #[doc = "Temperature - Thermocouple"]
  Temp_TC = nidaqmx_sys::DAQmx_Val_Temp_TC,
  #[doc = "Temperature - Thermistor"]
  Temp_Thrmstr = nidaqmx_sys::DAQmx_Val_Temp_Thrmstr,
  #[doc = "Temperature - RTD"]
  Temp_RTD = nidaqmx_sys::DAQmx_Val_Temp_RTD,
  #[doc = "Temperature - Built-in Sensor"]
  Temp_BuiltInSensor = nidaqmx_sys::DAQmx_Val_Temp_BuiltInSensor,
  #[doc = "Strain Gage"]
  Strain_Gage = nidaqmx_sys::DAQmx_Val_Strain_Gage,
  #[doc = "Rosette Strain Gage"]
  Rosette_Strain_Gage = nidaqmx_sys::DAQmx_Val_Rosette_Strain_Gage,
  #[doc = "Position - LVDT"]
  Position_LVDT = nidaqmx_sys::DAQmx_Val_Position_LVDT,
  #[doc = "Position - RVDT"]
  Position_RVDT = nidaqmx_sys::DAQmx_Val_Position_RVDT,
  #[doc = "Position - Eddy Current Pro ximity Probe"]
  Position_EddyCurrentProximityProbe =
    nidaqmx_sys::DAQmx_Val_Position_EddyCurrentProximityProbe,
  #[doc = "Accelerometer"]
  Accelerometer = nidaqmx_sys::DAQmx_Val_Accelerometer,
  #[doc = "Acceleration - Charge"]
  Acceleration_Charge = nidaqmx_sys::DAQmx_Val_Acceleration_Charge,
  #[doc = "Acceleration - 4 Wire DC Voltage"]
  Acceleration_4WireDCVoltage =
    nidaqmx_sys::DAQmx_Val_Acceleration_4WireDCVoltage,
  #[doc = "Velocity - IEPE Sensor"]
  Velocity_IEPESensor = nidaqmx_sys::DAQmx_Val_Velocity_IEPESensor,
  #[doc = "Force - Bridge"]
  Force_Bridge = nidaqmx_sys::DAQmx_Val_Force_Bridge,
  #[doc = "Force - IEPE Sensor"]
  Force_IEPESensor = nidaqmx_sys::DAQmx_Val_Force_IEPESensor,
  #[doc = "Pressure - Bridge"]
  Pressure_Bridge = nidaqmx_sys::DAQmx_Val_Pressure_Bridge,
  #[doc = "Sound Pressure - Microphone"]
  SoundPressure_Microphone = nidaqmx_sys::DAQmx_Val_SoundPressure_Microphone,
  #[doc = "Torque - Bridge"]
  Torque_Bridge = nidaqmx_sys::DAQmx_Val_Torque_Bridge,
  #[doc = "TEDS Sensor"]
  TEDS_Sensor = nidaqmx_sys::DAQmx_Val_TEDS_Sensor,
  #[doc = "Charge"]
  Charge = nidaqmx_sys::DAQmx_Val_Charge,
  #[doc = "Power"]
  Power = nidaqmx_sys::DAQmx_Val_Power,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AOIdleOutputBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Zero Volts"]
  ZeroVolts = nidaqmx_sys::DAQmx_Val_ZeroVolts,
  #[doc = "High-Impedance"]
  HighImpedance = nidaqmx_sys::DAQmx_Val_HighImpedance,
  #[doc = "Maintain Existing Value"]
  MaintainExistingValue = nidaqmx_sys::DAQmx_Val_MaintainExistingValue,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AOOutputChannelType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Voltage"]
  Voltage = nidaqmx_sys::DAQmx_Val_Voltage,
  #[doc = "Current"]
  Current = nidaqmx_sys::DAQmx_Val_Current,
  #[doc = "Function Generation"]
  FuncGen = nidaqmx_sys::DAQmx_Val_FuncGen,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AccelChargeSensitivityUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "PicoCoulombs per g"]
  PicoCoulombsPerG = nidaqmx_sys::DAQmx_Val_PicoCoulombsPerG,
  #[doc = "PicoCoulombs per m/s^2"]
  PicoCoulombsPerMetersPerSecondSquared =
    nidaqmx_sys::DAQmx_Val_PicoCoulombsPerMetersPerSecondSquared,
  #[doc = "PicoCoulombs per in/s^2"]
  PicoCoulombsPerInchesPerSecondSquared =
    nidaqmx_sys::DAQmx_Val_PicoCoulombsPerInchesPerSecondSquared,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AccelSensitivityUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "mVolts/g"]
  mVoltsPerG = nidaqmx_sys::DAQmx_Val_mVoltsPerG,
  #[doc = "Volts/g"]
  VoltsPerG = nidaqmx_sys::DAQmx_Val_VoltsPerG,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AccelUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "g"]
  AccelUnit_g = nidaqmx_sys::DAQmx_Val_AccelUnit_g,
  #[doc = "m/s^2"]
  MetersPerSecondSquared = nidaqmx_sys::DAQmx_Val_MetersPerSecondSquared,
  #[doc = "in/s^2"]
  InchesPerSecondSquared = nidaqmx_sys::DAQmx_Val_InchesPerSecondSquared,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AcquisitionType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Finite Samples"]
  FiniteSamps = nidaqmx_sys::DAQmx_Val_FiniteSamps,
  #[doc = "Continuous Samples"]
  ContSamps = nidaqmx_sys::DAQmx_Val_ContSamps,
  #[doc = "Hardware Timed Single Point"]
  HWTimedSinglePoint = nidaqmx_sys::DAQmx_Val_HWTimedSinglePoint,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ActiveLevel {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Above Level"]
  AboveLvl = nidaqmx_sys::DAQmx_Val_AboveLvl,
  #[doc = "Below Level"]
  BelowLvl = nidaqmx_sys::DAQmx_Val_BelowLvl,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AngleUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Degrees"]
  Degrees = nidaqmx_sys::DAQmx_Val_Degrees,
  #[doc = "Radians"]
  Radians = nidaqmx_sys::DAQmx_Val_Radians,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AngleUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Degrees"]
  Degrees = nidaqmx_sys::DAQmx_Val_Degrees,
  #[doc = "Radians"]
  Radians = nidaqmx_sys::DAQmx_Val_Radians,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AngularVelocityUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "RPM"]
  RPM = nidaqmx_sys::DAQmx_Val_RPM,
  #[doc = "Radians/s"]
  RadiansPerSecond = nidaqmx_sys::DAQmx_Val_RadiansPerSecond,
  #[doc = "Degrees/s"]
  DegreesPerSecond = nidaqmx_sys::DAQmx_Val_DegreesPerSecond,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AutoZerotype1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
  #[doc = "Once"]
  Once = nidaqmx_sys::DAQmx_Val_Once,
  #[doc = "Every Sample"]
  EverySample = nidaqmx_sys::DAQmx_Val_EverySample,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BreakMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "No Action"]
  NoAction = nidaqmx_sys::DAQmx_Val_NoAction,
  #[doc = "Break Before Make"]
  BreakBeforeMake = nidaqmx_sys::DAQmx_Val_BreakBeforeMake,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BridgeConfiguration1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Full Bridge"]
  FullBridge = nidaqmx_sys::DAQmx_Val_FullBridge,
  #[doc = "Half Bridge"]
  HalfBridge = nidaqmx_sys::DAQmx_Val_HalfBridge,
  #[doc = "Quarter Bridge"]
  QuarterBridge = nidaqmx_sys::DAQmx_Val_QuarterBridge,
  #[doc = "No Bridge"]
  NoBridge = nidaqmx_sys::DAQmx_Val_NoBridge,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BridgeElectricalUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Volts/Volt"]
  VoltsPerVolt = nidaqmx_sys::DAQmx_Val_VoltsPerVolt,
  #[doc = "mVolts/Volt"]
  mVoltsPerVolt = nidaqmx_sys::DAQmx_Val_mVoltsPerVolt,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BridgePhysicalUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Newtons"]
  Newtons = nidaqmx_sys::DAQmx_Val_Newtons,
  #[doc = "Pounds"]
  Pounds = nidaqmx_sys::DAQmx_Val_Pounds,
  #[doc = "kgf"]
  KilogramForce = nidaqmx_sys::DAQmx_Val_KilogramForce,
  #[doc = "Pascals"]
  Pascals = nidaqmx_sys::DAQmx_Val_Pascals,
  #[doc = "psi"]
  PoundsPerSquareInch = nidaqmx_sys::DAQmx_Val_PoundsPerSquareInch,
  #[doc = "bar"]
  Bar = nidaqmx_sys::DAQmx_Val_Bar,
  #[doc = "Nm"]
  NewtonMeters = nidaqmx_sys::DAQmx_Val_NewtonMeters,
  #[doc = "oz-in"]
  InchOunces = nidaqmx_sys::DAQmx_Val_InchOunces,
  #[doc = "lb-in"]
  InchPounds = nidaqmx_sys::DAQmx_Val_InchPounds,
  #[doc = "lb-ft"]
  FootPounds = nidaqmx_sys::DAQmx_Val_FootPounds,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BridgeUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Volts/Volt"]
  VoltsPerVolt = nidaqmx_sys::DAQmx_Val_VoltsPerVolt,
  #[doc = "mVolts/Volt"]
  mVoltsPerVolt = nidaqmx_sys::DAQmx_Val_mVoltsPerVolt,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
  #[doc = "From TEDS"]
  FromTEDS = nidaqmx_sys::DAQmx_Val_FromTEDS,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BusType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "PCI"]
  PCI = nidaqmx_sys::DAQmx_Val_PCI,
  #[doc = "PCIe"]
  PCIe = nidaqmx_sys::DAQmx_Val_PCIe,
  #[doc = "PXI"]
  PXI = nidaqmx_sys::DAQmx_Val_PXI,
  #[doc = "PXIe"]
  PXIe = nidaqmx_sys::DAQmx_Val_PXIe,
  #[doc = "SCXI"]
  SCXI = nidaqmx_sys::DAQmx_Val_SCXI,
  #[doc = "SCC"]
  SCC = nidaqmx_sys::DAQmx_Val_SCC,
  #[doc = "PCCard"]
  PCCard = nidaqmx_sys::DAQmx_Val_PCCard,
  #[doc = "USB"]
  USB = nidaqmx_sys::DAQmx_Val_USB,
  #[doc = "CompactDAQ"]
  CompactDAQ = nidaqmx_sys::DAQmx_Val_CompactDAQ,
  #[doc = "CompactRIO"]
  CompactRIO = nidaqmx_sys::DAQmx_Val_CompactRIO,
  #[doc = "TCP/IP"]
  TCPIP = nidaqmx_sys::DAQmx_Val_TCPIP,
  #[doc = "Unknown"]
  Unknown = nidaqmx_sys::DAQmx_Val_Unknown,
  #[doc = "SwitchBlock"]
  SwitchBlock = nidaqmx_sys::DAQmx_Val_SwitchBlock,
}
#[derive(Debug, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CIMeasurementType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Count Edges"]
  CountEdges = nidaqmx_sys::DAQmx_Val_CountEdges,
  #[doc = "Frequency"]
  Freq = nidaqmx_sys::DAQmx_Val_Freq,
  #[doc = "Period"]
  Period = nidaqmx_sys::DAQmx_Val_Period,
  #[doc = "Pulse Width"]
  PulseWidth = nidaqmx_sys::DAQmx_Val_PulseWidth,
  #[doc = "Semi Period"]
  SemiPeriod = nidaqmx_sys::DAQmx_Val_SemiPeriod,
  #[doc = "Pulse Frequency"]
  PulseFrequency = nidaqmx_sys::DAQmx_Val_PulseFrequency,
  #[doc = "Pulse Time"]
  PulseTime = nidaqmx_sys::DAQmx_Val_PulseTime,
  #[doc = "Pulse Ticks"]
  PulseTicks = nidaqmx_sys::DAQmx_Val_PulseTicks,
  #[doc = "Duty Cycle"]
  DutyCycle = nidaqmx_sys::DAQmx_Val_DutyCycle,
  #[doc = "Position - Angular Encoder"]
  Position_AngEncoder = nidaqmx_sys::DAQmx_Val_Position_AngEncoder,
  #[doc = "Position - Linear Encoder"]
  Position_LinEncoder = nidaqmx_sys::DAQmx_Val_Position_LinEncoder,
  #[doc = "Velocity - Angular Encoder"]
  Velocity_AngEncoder = nidaqmx_sys::DAQmx_Val_Velocity_AngEncoder,
  #[doc = "Velocity - Linear Encoder"]
  Velocity_LinEncoder = nidaqmx_sys::DAQmx_Val_Velocity_LinEncoder,
  #[doc = "Two Edge Separation"]
  TwoEdgeSep = nidaqmx_sys::DAQmx_Val_TwoEdgeSep,
  #[doc = "GPS Timestamp"]
  GPS_Timestamp = nidaqmx_sys::DAQmx_Val_GPS_Timestamp,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CJCSource1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Built-In"]
  BuiltIn = nidaqmx_sys::DAQmx_Val_BuiltIn,
  #[doc = "Constant Value"]
  ConstVal = nidaqmx_sys::DAQmx_Val_ConstVal,
  #[doc = "Channel"]
  Chan = nidaqmx_sys::DAQmx_Val_Chan,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum COOutputType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pulse - Time"]
  Pulse_Time = nidaqmx_sys::DAQmx_Val_Pulse_Time,
  #[doc = "Pulse - Frequency"]
  Pulse_Freq = nidaqmx_sys::DAQmx_Val_Pulse_Freq,
  #[doc = "Pulse - Ticks"]
  Pulse_Ticks = nidaqmx_sys::DAQmx_Val_Pulse_Ticks,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ChannelType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Analog Input"]
  AI = nidaqmx_sys::DAQmx_Val_AI,
  #[doc = "Analog Output"]
  AO = nidaqmx_sys::DAQmx_Val_AO,
  #[doc = "Digital Input"]
  DI = nidaqmx_sys::DAQmx_Val_DI,
  #[doc = "Digital Output"]
  DO = nidaqmx_sys::DAQmx_Val_DO,
  #[doc = "Counter Input"]
  CI = nidaqmx_sys::DAQmx_Val_CI,
  #[doc = "Counter Output"]
  CO = nidaqmx_sys::DAQmx_Val_CO,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ConstrainedGenMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Unconstrained"]
  Unconstrained = nidaqmx_sys::DAQmx_Val_Unconstrained,
  #[doc = "Fixed High Frequency"]
  FixedHighFreq = nidaqmx_sys::DAQmx_Val_FixedHighFreq,
  #[doc = "Fixed Low Frequency"]
  FixedLowFreq = nidaqmx_sys::DAQmx_Val_FixedLowFreq,
  #[doc = "Fixed 50 Percent Duty Cycle"]
  Fixed50PercentDutyCycle = nidaqmx_sys::DAQmx_Val_Fixed50PercentDutyCycle,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CountDirection1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Count Up"]
  CountUp = nidaqmx_sys::DAQmx_Val_CountUp,
  #[doc = "Count  down"]
  CountDown = nidaqmx_sys::DAQmx_Val_CountDown,
  #[doc = "Externally Controlled"]
  ExtControlled = nidaqmx_sys::DAQmx_Val_ExtControlled,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CounterFrequencyMethod {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Low Frequency with 1 Counter"]
  LowFreq1Ctr = nidaqmx_sys::DAQmx_Val_LowFreq1Ctr,
  #[doc = "High F requency with 2 Counters"]
  HighFreq2Ctr = nidaqmx_sys::DAQmx_Val_HighFreq2Ctr,
  #[doc = "Large Range with 2 Counters"]
  LargeRng2Ctr = nidaqmx_sys::DAQmx_Val_LargeRng2Ctr,
  #[doc = "Dynamic Averaging"]
  DynAvg = nidaqmx_sys::DAQmx_Val_DynAvg,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Coupling1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "AC"]
  AC = nidaqmx_sys::DAQmx_Val_AC,
  #[doc = "DC"]
  DC = nidaqmx_sys::DAQmx_Val_DC,
  #[doc = "GND"]
  GND = nidaqmx_sys::DAQmx_Val_GND,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Coupling2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "AC"]
  AC = nidaqmx_sys::DAQmx_Val_AC,
  #[doc = "DC"]
  DC = nidaqmx_sys::DAQmx_Val_DC,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CurrentShuntResistorLocation1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Internal"]
  Internal = nidaqmx_sys::DAQmx_Val_Internal,
  #[doc = "External"]
  External = nidaqmx_sys::DAQmx_Val_External,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BridgeShuntCalSource {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Built-In"]
  BuiltIn = nidaqmx_sys::DAQmx_Val_BuiltIn,
  #[doc = "User Provided"]
  UserProvided = nidaqmx_sys::DAQmx_Val_UserProvided,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ChargeUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Coulombs"]
  Coulombs = nidaqmx_sys::DAQmx_Val_Coulombs,
  #[doc = "PicoCoulombs"]
  PicoCoulombs = nidaqmx_sys::DAQmx_Val_PicoCoulombs,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CurrentUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Amps"]
  Amps = nidaqmx_sys::DAQmx_Val_Amps,
  #[doc = "From Custom Scale"]
  fromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,

  #[doc = "From TEDS"]
  FromTEDS = nidaqmx_sys::DAQmx_Val_FromTEDS,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CurrentUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Amps"]
  Amps = nidaqmx_sys::DAQmx_Val_Amps,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DataJustification1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Right-Justified"]
  RightJustified = nidaqmx_sys::DAQmx_Val_RightJustified,
  #[doc = "Left-Justified"]
  LeftJustified = nidaqmx_sys::DAQmx_Val_LeftJustified,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DataTransferMechanism {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "DMA"]
  DMA = nidaqmx_sys::DAQmx_Val_DMA,
  #[doc = "Interrupts"]
  Interrupts = nidaqmx_sys::DAQmx_Val_Interrupts,
  #[doc = "Programmed I/O"]
  ProgrammedIO = nidaqmx_sys::DAQmx_Val_ProgrammedIO,
  #[doc = "USB Bulk"]
  USBbulk = nidaqmx_sys::DAQmx_Val_USBbulk,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DeassertCondition {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Onboard Memory More than Half Full"]
  OnbrdMemMoreThanHalfFull = nidaqmx_sys::DAQmx_Val_OnbrdMemMoreThanHalfFull,
  #[doc = "Onboard Memory Full"]
  OnbrdMemFull = nidaqmx_sys::DAQmx_Val_OnbrdMemFull,
  #[doc = "Onboard Memory Custom Threshold"]
  OnbrdMemCustomThreshold = nidaqmx_sys::DAQmx_Val_OnbrdMemCustomThreshold,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalDriveType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Active Drive"]
  ActiveDrive = nidaqmx_sys::DAQmx_Val_ActiveDrive,
  #[doc = "Open Collector"]
  OpenCollector = nidaqmx_sys::DAQmx_Val_OpenCollector,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalLineState {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "High"]
  High = nidaqmx_sys::DAQmx_Val_High,
  #[doc = "Low"]
  Low = nidaqmx_sys::DAQmx_Val_Low,
  #[doc = "Tristate"]
  Tristate = nidaqmx_sys::DAQmx_Val_Tristate,
  #[doc = "No Change"]
  NoChange = nidaqmx_sys::DAQmx_Val_NoChange,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalPatternCondition1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pattern Matches"]
  PatternMatches = nidaqmx_sys::DAQmx_Val_PatternMatches,
  #[doc = "Pattern Does Not Match"]
  PatternDoesNotMatch = nidaqmx_sys::DAQmx_Val_PatternDoesNotMatch,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalWidthUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Sample Clock Periods"]
  SampClkPeriods = nidaqmx_sys::DAQmx_Val_SampClkPeriods,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalWidthUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalWidthUnits3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DigitalWidthUnits4 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
  #[doc = "Sample Clock Periods"]
  SampleClkPeriods = nidaqmx_sys::DAQmx_Val_SampleClkPeriods,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum EddyCurrentProxProbeSensitivityUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "mVolts/mil"]
  mVoltsPerMil = nidaqmx_sys::DAQmx_Val_mVoltsPerMil,
  #[doc = "Volts/mil"]
  VoltsPerMil = nidaqmx_sys::DAQmx_Val_VoltsPerMil,
  #[doc = "mVolts/mMeter"]
  mVoltsPerMillimeter = nidaqmx_sys::DAQmx_Val_mVoltsPerMillimeter,
  #[doc = "Volts/mMeter"]
  VoltsPerMillimeter = nidaqmx_sys::DAQmx_Val_VoltsPerMillimeter,
  #[doc = "mVolts/micron"]
  mVoltsPerMicron = nidaqmx_sys::DAQmx_Val_mVoltsPerMicron,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Edge1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Rising"]
  Rising = nidaqmx_sys::DAQmx_Val_Rising,
  #[doc = "Falling"]
  Falling = nidaqmx_sys::DAQmx_Val_Falling,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum EncoderType2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "X1"]
  X1 = nidaqmx_sys::DAQmx_Val_X1,
  #[doc = "X2"]
  X2 = nidaqmx_sys::DAQmx_Val_X2,
  #[doc = "X4"]
  X4 = nidaqmx_sys::DAQmx_Val_X4,
  #[doc = "Two Pulse Counting"]
  TwoPulseCounting = nidaqmx_sys::DAQmx_Val_TwoPulseCounting,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum EncoderZindexPhase1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "A High B High"]
  AHighBHigh = nidaqmx_sys::DAQmx_Val_AHighBHigh,
  #[doc = "A High B Low"]
  AHighBLow = nidaqmx_sys::DAQmx_Val_AHighBLow,
  #[doc = "A Low B High"]
  ALowBHigh = nidaqmx_sys::DAQmx_Val_ALowBHigh,
  #[doc = "A Low B Low"]
  ALowBLow = nidaqmx_sys::DAQmx_Val_ALowBLow,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ExcitationDCorAC {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "DC"]
  DC = nidaqmx_sys::DAQmx_Val_DC,
  #[doc = "AC"]
  AC = nidaqmx_sys::DAQmx_Val_AC,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Excitationsource {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Internal"]
  Internal = nidaqmx_sys::DAQmx_Val_Internal,
  #[doc = "External"]
  External = nidaqmx_sys::DAQmx_Val_External,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ExcitationVoltageOrCurrent {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Voltage"]
  Voltage = nidaqmx_sys::DAQmx_Val_Voltage,
  #[doc = "Current"]
  Current = nidaqmx_sys::DAQmx_Val_Current,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ExportActions2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pulse"]
  Pulse = nidaqmx_sys::DAQmx_Val_Pulse,
  #[doc = "Toggle"]
  Toggle = nidaqmx_sys::DAQmx_Val_Toggle,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ExportActions3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pulse"]
  Pulse = nidaqmx_sys::DAQmx_Val_Pulse,
  #[doc = "Level"]
  Lvl = nidaqmx_sys::DAQmx_Val_Lvl,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ExportActions5 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Interlocked"]
  Interlocked = nidaqmx_sys::DAQmx_Val_Interlocked,
  #[doc = "Pulse"]
  Pulse = nidaqmx_sys::DAQmx_Val_Pulse,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FilterType2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Lowpass"]
  Lowpass = nidaqmx_sys::DAQmx_Val_Lowpass,
  #[doc = "Highpass"]
  Highpass = nidaqmx_sys::DAQmx_Val_Highpass,
  #[doc = "Bandpass"]
  Bandpass = nidaqmx_sys::DAQmx_Val_Bandpass,
  #[doc = "Notch"]
  Notch = nidaqmx_sys::DAQmx_Val_Notch,
  #[doc = "Custom"]
  Custom = nidaqmx_sys::DAQmx_Val_Custom,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FilterResponse {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Constant Group Delay"]
  ConstantGroupDelay = nidaqmx_sys::DAQmx_Val_ConstantGroupDelay,
  #[doc = "Butterworth"]
  Butterworth = nidaqmx_sys::DAQmx_Val_Butterworth,
  #[doc = "Elliptical"]
  Elliptical = nidaqmx_sys::DAQmx_Val_Elliptical,
  #[doc = "Hardware Defined"]
  HardwareDefined = nidaqmx_sys::DAQmx_Val_HardwareDefined,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FilterResponse1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Comb"]
  Comb = nidaqmx_sys::DAQmx_Val_Comb,
  #[doc = "Bessel"]
  Bessel = nidaqmx_sys::DAQmx_Val_Bessel,
  #[doc = "Brickwall"]
  Brickwall = nidaqmx_sys::DAQmx_Val_Brickwall,
  #[doc = "Butterworth"]
  Butterworth = nidaqmx_sys::DAQmx_Val_Butterworth,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ForceIEPESensorSensitivityUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "mVolts/N"]
  mVoltsPerNewton = nidaqmx_sys::DAQmx_Val_mVoltsPerNewton,
  #[doc = "mVolts /lb"]
  mVoltsPerPound = nidaqmx_sys::DAQmx_Val_mVoltsPerPound,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ForceUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Newtons"]
  Newtons = nidaqmx_sys::DAQmx_Val_Newtons,
  #[doc = "Pounds"]
  Pounds = nidaqmx_sys::DAQmx_Val_Pounds,
  #[doc = "kgf"]
  KilogramForce = nidaqmx_sys::DAQmx_Val_KilogramForce,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FrequencyUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Hz"]
  Hz = nidaqmx_sys::DAQmx_Val_Hz,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FrequencyUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Hz"]
  Hz = nidaqmx_sys::DAQmx_Val_Hz,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FrequencyUnits3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Hz"]
  Hz = nidaqmx_sys::DAQmx_Val_Hz,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FuncGenType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Sine"]
  Sine = nidaqmx_sys::DAQmx_Val_Sine,
  #[doc = "Triangle"]
  Triangle = nidaqmx_sys::DAQmx_Val_Triangle,
  #[doc = "Square"]
  Square = nidaqmx_sys::DAQmx_Val_Square,
  #[doc = "Sawtooth"]
  Sawtooth = nidaqmx_sys::DAQmx_Val_Sawtooth,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GpsSignalType1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "IRIG-B"]
  IRIGB = nidaqmx_sys::DAQmx_Val_IRIGB,
  #[doc = "PPS"]
  PPS = nidaqmx_sys::DAQmx_Val_PPS,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum HandshakeStartCondition {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Immediate"]
  Immediate = nidaqmx_sys::DAQmx_Val_Immediate,
  #[doc = "Wait For Handshake Trigger Assert"]
  WaitForHandshakeTriggerAssert =
    nidaqmx_sys::DAQmx_Val_WaitForHandshakeTriggerAssert,
  #[doc = "Wait For Handshake Trigger Deassert"]
  WaitForHandshakeTriggerDeassert =
    nidaqmx_sys::DAQmx_Val_WaitForHandshakeTriggerDeassert,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum InputDataTransferCondition {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Onboard Memory More than Half Full"]
  OnBrdMemMoreThanHalfFull = nidaqmx_sys::DAQmx_Val_OnBrdMemMoreThanHalfFull,
  #[doc = "Onboard Memory Not Empty"]
  OnBrdMemNotEmpty = nidaqmx_sys::DAQmx_Val_OnBrdMemNotEmpty,
  #[doc = "Onboard Memory Custom Threshold"]
  OnbrdMemCustomThreshold = nidaqmx_sys::DAQmx_Val_OnbrdMemCustomThreshold,
  #[doc = "When Acquisition Complete"]
  WhenAcqComplete = nidaqmx_sys::DAQmx_Val_WhenAcqComplete,
}
#[derive(Debug, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum InputTermCfg {
  #[default]
  #[doc = "RSE"]
  RSE = nidaqmx_sys::DAQmx_Val_RSE,
  #[doc = "NRSE"]
  NRSE = nidaqmx_sys::DAQmx_Val_NRSE,
  #[doc = "Differential"]
  Diff = nidaqmx_sys::DAQmx_Val_Diff,
  #[doc = "Pseudodifferential"]
  PseudoDiff = nidaqmx_sys::DAQmx_Val_PseudoDiff,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum InputTermCfg2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Differential"]
  Diff = nidaqmx_sys::DAQmx_Val_Diff,
  #[doc = "RSE"]
  RSE = nidaqmx_sys::DAQmx_Val_RSE,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LVDTSensitivityUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "mVolts/Volt/mMeter"]
  mVoltsPerVoltPerMillimeter =
    nidaqmx_sys::DAQmx_Val_mVoltsPerVoltPerMillimeter,
  #[doc = "mVolts/Volt/0.001 Inch"]
  mVoltsPerVoltPerMilliInch = nidaqmx_sys::DAQmx_Val_mVoltsPerVoltPerMilliInch,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LengthUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Meters"]
  Meters = nidaqmx_sys::DAQmx_Val_Meters,
  #[doc = "Inches"]
  Inches = nidaqmx_sys::DAQmx_Val_Inches,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LengthUnits3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Meters"]
  Meters = nidaqmx_sys::DAQmx_Val_Meters,
  #[doc = "Inches"]
  Inches = nidaqmx_sys::DAQmx_Val_Inches,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Level1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "High"]
  High = nidaqmx_sys::DAQmx_Val_High,
  #[doc = "Low"]
  Low = nidaqmx_sys::DAQmx_Val_Low,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LoggingMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Off"]
  Off = nidaqmx_sys::DAQmx_Val_Off,
  #[doc = "Log"]
  Log = nidaqmx_sys::DAQmx_Val_Log,
  #[doc = "Log and Read"]
  LogAndRead = nidaqmx_sys::DAQmx_Val_LogAndRead,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LoggingOperation {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Open"]
  Open = nidaqmx_sys::DAQmx_Val_Open,
  #[doc = "Open or Create"]
  OpenOrCreate = nidaqmx_sys::DAQmx_Val_OpenOrCreate,
  #[doc = "Create or Replace"]
  CreateOrReplace = nidaqmx_sys::DAQmx_Val_CreateOrReplace,
  #[doc = "Create"]
  Create = nidaqmx_sys::DAQmx_Val_Create,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LogicFamily {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "2.5 V"]
  TwoPoint5V = nidaqmx_sys::DAQmx_Val_2point5V,
  #[doc = "3.3 V"]
  ThreePoint3V = nidaqmx_sys::DAQmx_Val_3point3V,
  #[doc = "5.0 V"]
  FiveV = nidaqmx_sys::DAQmx_Val_5V,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum MIOAIConvertTbSrc {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Same as Sample Timebase"]
  SameAsSampTimebase = nidaqmx_sys::DAQmx_Val_SameAsSampTimebase,
  #[doc = "Same as Master Timebase"]
  SameAsMasterTimebase = nidaqmx_sys::DAQmx_Val_SameAsMasterTimebase,
  #[doc = "100 MHz Timebase"]
  Timebase100MHz = nidaqmx_sys::DAQmx_Val_100MHzTimebase,
  #[doc = "80 MHz Timebase"]
  Timebase80MHz = nidaqmx_sys::DAQmx_Val_80MHzTimebase,
  #[doc = "20 MHz Timebase"]
  Timebase20MHz = nidaqmx_sys::DAQmx_Val_20MHzTimebase,
  #[doc = "8 MHz Timebase"]
  Timebase8Mhz = nidaqmx_sys::DAQmx_Val_8MHzTimebase,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ModulationType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "AM"]
  AM = nidaqmx_sys::DAQmx_Val_AM,
  #[doc = "FM"]
  Fm = nidaqmx_sys::DAQmx_Val_FM,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum OutputDataTransferCondition {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Onboard Memory Empty"]
  OnBrdMemEmpty = nidaqmx_sys::DAQmx_Val_OnBrdMemEmpty,
  #[doc = "Onboard Memory Half Full or Less"]
  OnBrdMemHalfFullOrLess = nidaqmx_sys::DAQmx_Val_OnBrdMemHalfFullOrLess,
  #[doc = "Onboard Memory Less than Full"]
  OnBrdMemNotFull = nidaqmx_sys::DAQmx_Val_OnBrdMemNotFull,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum OutputTermCfg {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "RSE"]
  RSE = nidaqmx_sys::DAQmx_Val_RSE,
  #[doc = "Differential"]
  Diff = nidaqmx_sys::DAQmx_Val_Diff,
  #[doc = "Pseudodifferential"]
  PseudoDiff = nidaqmx_sys::DAQmx_Val_PseudoDiff,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum OverflowBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Stop Task And Error"]
  StopTaskAndError = nidaqmx_sys::DAQmx_Val_StopTaskAndError,
  #[doc = "Ignore Overruns"]
  IgnoreOverruns = nidaqmx_sys::DAQmx_Val_IgnoreOverruns,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum OverwriteMode1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Overwrite Unread Samples"]
  OverwriteUnreadSamps = nidaqmx_sys::DAQmx_Val_OverwriteUnreadSamps,
  #[doc = "Do Not Overwrite Unread Samples"]
  DoNotOverwriteUnreadSamps = nidaqmx_sys::DAQmx_Val_DoNotOverwriteUnreadSamps,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Polarity2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Active High"]
  ActiveHigh = nidaqmx_sys::DAQmx_Val_ActiveHigh,
  #[doc = "Active Low"]
  ActiveLow = nidaqmx_sys::DAQmx_Val_ActiveLow,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PowerIdleOutputBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Output Disabled"]
  OutputDisabled = nidaqmx_sys::DAQmx_Val_OutputDisabled,
  #[doc = "Maintain Existing Value"]
  MaintainExistingValue = nidaqmx_sys::DAQmx_Val_MaintainExistingValue,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PowerOutputState {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Constant Voltage"]
  ConstantVoltage = nidaqmx_sys::DAQmx_Val_ConstantVoltage,
  #[doc = "Constant Current"]
  ConstantCurrent = nidaqmx_sys::DAQmx_Val_ConstantCurrent,
  #[doc = "Overvoltage"]
  Overvoltage = nidaqmx_sys::DAQmx_Val_Overvoltage,
  #[doc = "Output Disabled"]
  OutputDisabled = nidaqmx_sys::DAQmx_Val_OutputDisabled,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PressureUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pascals"]
  Pascals = nidaqmx_sys::DAQmx_Val_Pascals,
  #[doc = "psi"]
  PoundsPerSquareInch = nidaqmx_sys::DAQmx_Val_PoundsPerSquareInch,
  #[doc = "bar"]
  Bar = nidaqmx_sys::DAQmx_Val_Bar,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(Default, Clone, Debug, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ProductCategory {
  #[doc = "M Series DAQ"]
  MSeriesDAQ = nidaqmx_sys::DAQmx_Val_MSeriesDAQ,
  #[doc = "X Series DAQ"]
  XSeriesDAQ = nidaqmx_sys::DAQmx_Val_XSeriesDAQ,
  #[doc = "E Series DAQ"]
  ESeriesDAQ = nidaqmx_sys::DAQmx_Val_ESeriesDAQ,
  #[doc = "S Series DAQ"]
  SSeriesDAQ = nidaqmx_sys::DAQmx_Val_SSeriesDAQ,
  #[doc = "B Series DAQ"]
  BSeriesDAQ = nidaqmx_sys::DAQmx_Val_BSeriesDAQ,
  #[doc = "SC Series DAQ"]
  SCSeriesDAQ = nidaqmx_sys::DAQmx_Val_SCSeriesDAQ,
  #[doc = "USB DAQ"]
  USBDAQ = nidaqmx_sys::DAQmx_Val_USBDAQ,
  #[doc = "AO Series"]
  AOSeries = nidaqmx_sys::DAQmx_Val_AOSeries,
  #[doc = "Digital I/O"]
  DigitalIO = nidaqmx_sys::DAQmx_Val_DigitalIO,
  #[doc = "TIO Series"]
  TIOSeries = nidaqmx_sys::DAQmx_Val_TIOSeries,
  #[doc = "Dynamic Signal Acquisition"]
  DynamicSignalAcquisition = nidaqmx_sys::DAQmx_Val_DynamicSignalAcquisition,
  #[doc = "Switches"]
  Switches = nidaqmx_sys::DAQmx_Val_Switches,
  #[doc = "CompactDAQ Chassis"]
  CompactDAQChassis = nidaqmx_sys::DAQmx_Val_CompactDAQChassis,
  #[doc = "CompactRIO Chassis"]
  CompactRIOChassis = nidaqmx_sys::DAQmx_Val_CompactRIOChassis,
  #[doc = "C Series Module"]
  CSeriesModule = nidaqmx_sys::DAQmx_Val_CSeriesModule,
  #[doc = "SCXI Module"]
  SCXIModule = nidaqmx_sys::DAQmx_Val_SCXIModule,
  #[doc = "SCC Connector Block"]
  SCCConnectorBlock = nidaqmx_sys::DAQmx_Val_SCCConnectorBlock,
  #[doc = "SCC Module"]
  SCCModule = nidaqmx_sys::DAQmx_Val_SCCModule,
  #[doc = "niELVIS"]
  NIELVIS = nidaqmx_sys::DAQmx_Val_NIELVIS,
  #[doc = "Network DAQ"]
  NetworkDAQ = nidaqmx_sys::DAQmx_Val_NetworkDAQ,
  #[doc = "SC Express"]
  SCExpress = nidaqmx_sys::DAQmx_Val_SCExpress,
  #[doc = "FieldDAQ"]
  FieldDAQ = nidaqmx_sys::DAQmx_Val_FieldDAQ,
  #[doc = "TestScale Chassis"]
  TestScaleChassis = nidaqmx_sys::DAQmx_Val_TestScaleChassis,
  #[doc = "TestScale Module"]
  TestScaleModule = nidaqmx_sys::DAQmx_Val_TestScaleModule,
  #[default]
  #[doc = "Unknown"]
  Unknown = nidaqmx_sys::DAQmx_Val_Unknown,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RTDType1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pt3750"]
  Pt3750 = nidaqmx_sys::DAQmx_Val_Pt3750,
  #[doc = "Pt3851"]
  Pt3851 = nidaqmx_sys::DAQmx_Val_Pt3851,
  #[doc = "Pt3911"]
  Pt3911 = nidaqmx_sys::DAQmx_Val_Pt3911,
  #[doc = "Pt3916"]
  Pt3916 = nidaqmx_sys::DAQmx_Val_Pt3916,
  #[doc = "Pt3920"]
  Pt3920 = nidaqmx_sys::DAQmx_Val_Pt3920,
  #[doc = "Pt3928"]
  Pt3928 = nidaqmx_sys::DAQmx_Val_Pt3928,
  #[doc = "Custom"]
  Custom = nidaqmx_sys::DAQmx_Val_Custom,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RVDTSensitivityUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "mVolts/Volt/Degree"]
  mVoltsPerVoltPerDegree = nidaqmx_sys::DAQmx_Val_mVoltsPerVoltPerDegree,
  #[doc = "mVolts/Volt/Radian"]
  mVoltsPerVoltPerRadian = nidaqmx_sys::DAQmx_Val_mVoltsPerVoltPerRadian,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RawDataCompressionType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
  #[doc = "Lossless Packing"]
  LosslessPacking = nidaqmx_sys::DAQmx_Val_LosslessPacking,
  #[doc = "Lossy LSB Removal"]
  LossyLSBRemoval = nidaqmx_sys::DAQmx_Val_LossyLSBRemoval,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ReadRelativeTo {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "First Sample"]
  FirstSample = nidaqmx_sys::DAQmx_Val_FirstSample,
  #[doc = "Current Read Position"]
  CurrReadPos = nidaqmx_sys::DAQmx_Val_CurrReadPos,
  #[doc = "Reference Trigger"]
  RefTrig = nidaqmx_sys::DAQmx_Val_RefTrig,
  #[doc = "First Pretrigger Sample"]
  FirstPretrigSamp = nidaqmx_sys::DAQmx_Val_FirstPretrigSamp,
  #[doc = "Most Recent Sample"]
  MostRecentSamp = nidaqmx_sys::DAQmx_Val_MostRecentSamp,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RegenerationMode1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Allow Regeneration"]
  AllowRegen = nidaqmx_sys::DAQmx_Val_AllowRegen,
  #[doc = "Do Not Allow Regeneration"]
  DoNotAllowRegen = nidaqmx_sys::DAQmx_Val_DoNotAllowRegen,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Resistanceconfiguration {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "2-Wire"]
  TwoWire = nidaqmx_sys::DAQmx_Val_2Wire,
  #[doc = "3-Wire"]
  ThreeWire = nidaqmx_sys::DAQmx_Val_3Wire,
  #[doc = "4-Wire"]
  FourWire = nidaqmx_sys::DAQmx_Val_4Wire,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ResistanceUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Ohms"]
  Ohms = nidaqmx_sys::DAQmx_Val_Ohms,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
  #[doc = "From TEDS"]
  FromTEDS = nidaqmx_sys::DAQmx_Val_FromTEDS,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ResistanceUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Ohms"]
  Ohms = nidaqmx_sys::DAQmx_Val_Ohms,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ResolutionType1 {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "Bits"]
  Bits = nidaqmx_sys::DAQmx_Val_Bits,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SCXI1124Range {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "0V t o 1V"]
  SCXI1124Range0to1V = nidaqmx_sys::DAQmx_Val_SCXI1124Range0to1V,
  #[doc = "0V to 5V"]
  SCXI1124Range0to5V = nidaqmx_sys::DAQmx_Val_SCXI1124Range0to5V,
  #[doc = "0V to 10V"]
  SCXI1124Range0to10V = nidaqmx_sys::DAQmx_Val_SCXI1124Range0to10V,
  #[doc = "-1V to 1V"]
  SCXI1124RangeNeg1to1V = nidaqmx_sys::DAQmx_Val_SCXI1124RangeNeg1to1V,
  #[doc = "-5V to 5V"]
  SCXI1124RangeNeg5to5V = nidaqmx_sys::DAQmx_Val_SCXI1124RangeNeg5to5V,
  #[doc = "-10V to 10V"]
  SCXI1124RangeNeg10to10V = nidaqmx_sys::DAQmx_Val_SCXI1124RangeNeg10to10V,
  #[doc = "0mA to 20mA"]
  SCXI1124Range0to20mA = nidaqmx_sys::DAQmx_Val_SCXI1124Range0to20mA,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SampleClockActiveOrInactiveEdgeSelection {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Sample Clock Active Edge"]
  SampClkActiveEdge = nidaqmx_sys::DAQmx_Val_SampClkActiveEdge,
  #[doc = "Sample Clock Inactive Edge"]
  SampClkInactiveEdge = nidaqmx_sys::DAQmx_Val_SampClkInactiveEdge,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SampleInputDataWhen {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Handshake Trigger Asserts"]
  HandshakeTriggerAsserts = nidaqmx_sys::DAQmx_Val_HandshakeTriggerAsserts,
  #[doc = "Handshake Trigger Deasserts"]
  HandshakeTriggerDeasserts = nidaqmx_sys::DAQmx_Val_HandshakeTriggerDeasserts,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SampleTimingType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Sample Clock"]
  SampClk = nidaqmx_sys::DAQmx_Val_SampClk,
  #[doc = "Burst Handshake"]
  BurstHandshake = nidaqmx_sys::DAQmx_Val_BurstHandshake,
  #[doc = "Handshake"]
  Handshake = nidaqmx_sys::DAQmx_Val_Handshake,
  #[doc = "Implicit"]
  Implicit = nidaqmx_sys::DAQmx_Val_Implicit,
  #[doc = "On Demand"]
  OnDemand = nidaqmx_sys::DAQmx_Val_OnDemand,
  #[doc = "Change Detection"]
  ChangeDetection = nidaqmx_sys::DAQmx_Val_ChangeDetection,
  #[doc = "Pipelined Sample Clock"]
  PipelinedSampClk = nidaqmx_sys::DAQmx_Val_PipelinedSampClk,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ScaleType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Linear"]
  Linear = nidaqmx_sys::DAQmx_Val_Linear,
  #[doc = "Map Ranges"]
  MapRanges = nidaqmx_sys::DAQmx_Val_MapRanges,
  #[doc = "Polynomial"]
  Polynomial = nidaqmx_sys::DAQmx_Val_Polynomial,
  #[doc = "Table"]
  Table = nidaqmx_sys::DAQmx_Val_Table,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ScaleType2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Polynomial"]
  Polynomial = nidaqmx_sys::DAQmx_Val_Polynomial,
  #[doc = "Table"]
  Table = nidaqmx_sys::DAQmx_Val_Table,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ScaleType3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Polynomial"]
  Polynomial = nidaqmx_sys::DAQmx_Val_Polynomial,
  #[doc = "Table"]
  Table = nidaqmx_sys::DAQmx_Val_Table,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ScaleType4 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
  #[doc = "Two-Point Linear"]
  TwoPointLinear = nidaqmx_sys::DAQmx_Val_TwoPointLinear,
  #[doc = "Table"]
  Table = nidaqmx_sys::DAQmx_Val_Table,
  #[doc = "Polynomial"]
  Polynomial = nidaqmx_sys::DAQmx_Val_Polynomial,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SensorPowerCfg {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "No Change"]
  NoChange = nidaqmx_sys::DAQmx_Val_NoChange,
  #[doc = "Enabled"]
  Enabled = nidaqmx_sys::DAQmx_Val_Enabled,
  #[doc = "Disabled"]
  Disabled = nidaqmx_sys::DAQmx_Val_Disabled,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SensorPowerType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "DC"]
  DC = nidaqmx_sys::DAQmx_Val_DC,
  #[doc = "AC"]
  AC = nidaqmx_sys::DAQmx_Val_AC,
  #[doc = "BipolarDC"]
  BipolarDC = nidaqmx_sys::DAQmx_Val_BipolarDC,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ShuntCalselect {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "A"]
  A = nidaqmx_sys::DAQmx_Val_A,
  #[doc = "B"]
  B = nidaqmx_sys::DAQmx_Val_B,
  #[doc = "A and B"]
  AandB = nidaqmx_sys::DAQmx_Val_AandB,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ShuntElementLocation {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "R1"]
  R1 = nidaqmx_sys::DAQmx_Val_R1,
  #[doc = "R2"]
  R2 = nidaqmx_sys::DAQmx_Val_R2,
  #[doc = "R3"]
  R3 = nidaqmx_sys::DAQmx_Val_R3,
  #[doc = "R4"]
  R4 = nidaqmx_sys::DAQmx_Val_R4,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Signal {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "AI Convert Clock"]
  AIConvertClock = nidaqmx_sys::DAQmx_Val_AIConvertClock,
  #[doc = "10MHz Reference Clock"]
  RefClock10MHz = nidaqmx_sys::DAQmx_Val_10MHzRefClock,
  #[doc = "20MHz Timebase Clock"]
  TimebaseClock20MHz = nidaqmx_sys::DAQmx_Val_20MHzTimebaseClock,
  #[doc = "Sample Clock"]
  SampleClock = nidaqmx_sys::DAQmx_Val_SampleClock,
  #[doc = "Advance Trigger"]
  AdvanceTrigger = nidaqmx_sys::DAQmx_Val_AdvanceTrigger,
  #[doc = "Reference Trigger"]
  ReferenceTrigger = nidaqmx_sys::DAQmx_Val_ReferenceTrigger,
  #[doc = "Start Trigger"]
  StartTrigger = nidaqmx_sys::DAQmx_Val_StartTrigger,
  #[doc = "Advance Complete Event"]
  AdvCmpltEvent = nidaqmx_sys::DAQmx_Val_AdvCmpltEvent,
  #[doc = "AI Hold Complete Event"]
  AIHoldCmpltEvent = nidaqmx_sys::DAQmx_Val_AIHoldCmpltEvent,
  #[doc = "Counter Output Event"]
  counterOutputEvent = nidaqmx_sys::DAQmx_Val_CounterOutputEvent,
  #[doc = "Change Detection Event"]
  ChangeDetectionEvent = nidaqmx_sys::DAQmx_Val_ChangeDetectionEvent,
  #[doc = "Watchdog Timer Expired Event"]
  WDTExpiredEvent = nidaqmx_sys::DAQmx_Val_WDTExpiredEvent,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Signal2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Sample Complete Event"]
  SampleCompleteEvent = nidaqmx_sys::DAQmx_Val_SampleCompleteEvent,
  #[doc = "Counter Output Event"]
  CounterOutputEvent = nidaqmx_sys::DAQmx_Val_CounterOutputEvent,
  #[doc = "Change Detection Event"]
  ChangeDetectionEvent = nidaqmx_sys::DAQmx_Val_ChangeDetectionEvent,
  #[doc = "Sample Clock"]
  SampleClock = nidaqmx_sys::DAQmx_Val_SampleClock,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Slope1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Rising"]
  RisingSlope = nidaqmx_sys::DAQmx_Val_RisingSlope,
  #[doc = "Falling"]
  FallingSlope = nidaqmx_sys::DAQmx_Val_FallingSlope,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SoundPressureUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Pascals"]
  Pascals = nidaqmx_sys::DAQmx_Val_Pascals,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SourceSelection {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Internal"]
  Internal = nidaqmx_sys::DAQmx_Val_Internal,
  #[doc = "External"]
  External = nidaqmx_sys::DAQmx_Val_External,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StrainGageBridgeType1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Full Bridge I"]
  FullBridgeI = nidaqmx_sys::DAQmx_Val_FullBridgeI,
  #[doc = "Full Bridge II"]
  FullBridgeII = nidaqmx_sys::DAQmx_Val_FullBridgeII,
  #[doc = "Full Bridge III"]
  FullBridgeIII = nidaqmx_sys::DAQmx_Val_FullBridgeIII,
  #[doc = "Half Bridge I"]
  HalfBridgeI = nidaqmx_sys::DAQmx_Val_HalfBridgeI,
  #[doc = "Half Bridge II"]
  HalfBridgeII = nidaqmx_sys::DAQmx_Val_HalfBridgeII,
  #[doc = "Quarter Bridge I"]
  QuarterBridgeI = nidaqmx_sys::DAQmx_Val_QuarterBridgeI,
  #[doc = "Quarter Bridge II"]
  QuarterBridgeII = nidaqmx_sys::DAQmx_Val_QuarterBridgeII,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StrainGageRosetteType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Rectangular Rosette"]
  RectangularRosette = nidaqmx_sys::DAQmx_Val_RectangularRosette,
  #[doc = "Delta Rosette"]
  DeltaRosette = nidaqmx_sys::DAQmx_Val_DeltaRosette,
  #[doc = "Tee Rosette"]
  TeeRosette = nidaqmx_sys::DAQmx_Val_TeeRosette,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StrainGageRosetteMeasurementType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Principal Strain 1"]
  PrincipalStrain1 = nidaqmx_sys::DAQmx_Val_PrincipalStrain1,
  #[doc = "Principal Strain 2"]
  PrincipalStrain2 = nidaqmx_sys::DAQmx_Val_PrincipalStrain2,
  #[doc = "Principal Strain Angle"]
  PrincipalStrainAngle = nidaqmx_sys::DAQmx_Val_PrincipalStrainAngle,
  #[doc = "Cartesian Strain X"]
  CartesianStrainX = nidaqmx_sys::DAQmx_Val_CartesianStrainX,
  #[doc = "Cartesian Strain Y"]
  CartesianStrainY = nidaqmx_sys::DAQmx_Val_CartesianStrainY,
  #[doc = "Cartesian Shear Strain XY"]
  CartesianShearStrainXY = nidaqmx_sys::DAQmx_Val_CartesianShearStrainXY,
  #[doc = "Maximum Shear Strain"]
  MaxShearStrain = nidaqmx_sys::DAQmx_Val_MaxShearStrain,
  #[doc = "Maximum Shear Strain Angle"]
  MaxShearStrainAngle = nidaqmx_sys::DAQmx_Val_MaxShearStrainAngle,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StrainUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Strain"]
  Strain = nidaqmx_sys::DAQmx_Val_Strain,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SwitchScanRepeatMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Finite"]
  Finite = nidaqmx_sys::DAQmx_Val_Finite,
  #[doc = "Contin uous"]
  Cont = nidaqmx_sys::DAQmx_Val_Cont,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SwitchUsageTypes {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Source"]
  Source = nidaqmx_sys::DAQmx_Val_Source,
  #[doc = "Load"]
  Load = nidaqmx_sys::DAQmx_Val_Load,
  #[doc = "Reserved for Routing"]
  ReservedForRouting = nidaqmx_sys::DAQmx_Val_ReservedForRouting,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SyncPulseType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Onboard"]
  Onboard = nidaqmx_sys::DAQmx_Val_Onboard,
  #[doc = "Digital Edge"]
  DigEdge = nidaqmx_sys::DAQmx_Val_DigEdge,
  #[doc = "Time"]
  Time = nidaqmx_sys::DAQmx_Val_Time,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SyncType {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
  #[doc = "Master"]
  Master = nidaqmx_sys::DAQmx_Val_Master,
  #[doc = "Slave"]
  Slave = nidaqmx_sys::DAQmx_Val_Slave,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SyncUnlockBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Stop Task and Error"]
  StopTaskAndError = nidaqmx_sys::DAQmx_Val_StopTaskAndError,
  #[doc = "Ignore Lost Sync Lock"]
  IgnoreLostSyncLock = nidaqmx_sys::DAQmx_Val_IgnoreLostSyncLock,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TEDSUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
  #[doc = "From TEDS"]
  FromTEDS = nidaqmx_sys::DAQmx_Val_FromTEDS,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TemperatureUnits1 {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "Deg C"]
  DegC = nidaqmx_sys::DAQmx_Val_DegC,
  #[doc = "Deg F"]
  DegF = nidaqmx_sys::DAQmx_Val_DegF,
  #[doc = "Kelvins"]
  Kelvins = nidaqmx_sys::DAQmx_Val_Kelvins,
  #[doc = "Deg R"]
  DegR = nidaqmx_sys::DAQmx_Val_DegR,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ThermocoupleType1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "J"]
  J_Type_TC = nidaqmx_sys::DAQmx_Val_J_Type_TC,
  #[doc = "K"]
  K_Type_TC = nidaqmx_sys::DAQmx_Val_K_Type_TC,
  #[doc = "N"]
  N_Type_TC = nidaqmx_sys::DAQmx_Val_N_Type_TC,
  #[doc = "R"]
  R_Type_TC = nidaqmx_sys::DAQmx_Val_R_Type_TC,
  #[doc = "S"]
  S_Type_TC = nidaqmx_sys::DAQmx_Val_S_Type_TC,
  #[doc = "T"]
  T_Type_TC = nidaqmx_sys::DAQmx_Val_T_Type_TC,
  #[doc = "B"]
  B_Type_TC = nidaqmx_sys::DAQmx_Val_B_Type_TC,
  #[doc = "E"]
  E_Type_TC = nidaqmx_sys::DAQmx_Val_E_Type_TC,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Timescale2 {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "Host Time"]
  HostTime = nidaqmx_sys::DAQmx_Val_HostTime,
  #[doc = "I/O Device Time"]
  IODeviceTime = nidaqmx_sys::DAQmx_Val_IODeviceTime,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TimeUnits {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TimeUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TimeUnits3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TimingResponseMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Single-cycle"]
  SingleCycle = nidaqmx_sys::DAQmx_Val_SingleCycle,
  #[doc = "Multic ycle"]
  Multicycle = nidaqmx_sys::DAQmx_Val_Multicycle,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TorqueUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Nm"]
  NewtonMeters = nidaqmx_sys::DAQmx_Val_NewtonMeters,
  #[doc = "oz-in"]
  inchOunces = nidaqmx_sys::DAQmx_Val_InchOunces,
  #[doc = "lb-in"]
  InchPounds = nidaqmx_sys::DAQmx_Val_InchPounds,
  #[doc = "lb-ft"]
  FootPounds = nidaqmx_sys::DAQmx_Val_FootPounds,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerType4 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Digital Edge"]
  DigEdge = nidaqmx_sys::DAQmx_Val_DigEdge,
  #[doc = "Time"]
  Time = nidaqmx_sys::DAQmx_Val_Time,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerType5 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Digital Edge"]
  DigEdge = nidaqmx_sys::DAQmx_Val_DigEdge,
  #[doc = "Software"]
  Software = nidaqmx_sys::DAQmx_Val_Software,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerType6 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Analog Level"]
  AnlgLvl = nidaqmx_sys::DAQmx_Val_AnlgLvl,
  #[doc = "Analog Window"]
  AnlgWin = nidaqmx_sys::DAQmx_Val_AnlgWin,
  #[doc = "Digital Level"]
  DigLvl = nidaqmx_sys::DAQmx_Val_DigLvl,
  #[doc = "Digital Pattern"]
  DigPattern = nidaqmx_sys::DAQmx_Val_DigPattern,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerType8 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Analog Edge"]
  AnlgEdge = nidaqmx_sys::DAQmx_Val_AnlgEdge,
  #[doc = "Analog Multi Edge"]
  AnlgMultiEdge = nidaqmx_sys::DAQmx_Val_AnlgMultiEdge,
  #[doc = "Digital Edge"]
  DigEdge = nidaqmx_sys::DAQmx_Val_DigEdge,
  #[doc = "Digital Pattern"]
  DigPattern = nidaqmx_sys::DAQmx_Val_DigPattern,
  #[doc = "Analog Window"]
  AnlgWin = nidaqmx_sys::DAQmx_Val_AnlgWin,
  #[doc = "Time"]
  Time = nidaqmx_sys::DAQmx_Val_Time,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerType9 {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "Interlocked"]
  Interlocked = nidaqmx_sys::DAQmx_Val_Interlocked,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerType10 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Anal og Edge"]
  AnlgEdge = nidaqmx_sys::DAQmx_Val_AnlgEdge,
  #[doc = "Analog Multi Edge"]
  AnlgMultiEdge = nidaqmx_sys::DAQmx_Val_AnlgMultiEdge,
  #[doc = "Digital Edge"]
  DigEdge = nidaqmx_sys::DAQmx_Val_DigEdge,
  #[doc = "Digital Pattern"]
  DigPattern = nidaqmx_sys::DAQmx_Val_DigPattern,
  #[doc = "Analog Window"]
  AnlgWin = nidaqmx_sys::DAQmx_Val_AnlgWin,
  #[doc = "Time"]
  Time = nidaqmx_sys::DAQmx_Val_Time,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum UnderflowBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Halt Output and Error"]
  HaltOutputAndError = nidaqmx_sys::DAQmx_Val_HaltOutputAndError,
  #[doc = "Pause until Data Available"]
  PauseUntilDataAvailable = nidaqmx_sys::DAQmx_Val_PauseUntilDataAvailable,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum UnitsPreScaled {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Volts"]
  Volts = nidaqmx_sys::DAQmx_Val_Volts,
  #[doc = "Amps"]
  Amps = nidaqmx_sys::DAQmx_Val_Amps,
  #[doc = "Deg F"]
  DegF = nidaqmx_sys::DAQmx_Val_DegF,
  #[doc = "Deg C"]
  DegC = nidaqmx_sys::DAQmx_Val_DegC,
  #[doc = "Deg R"]
  DegR = nidaqmx_sys::DAQmx_Val_DegR,
  #[doc = "Kelvins"]
  Kelvins = nidaqmx_sys::DAQmx_Val_Kelvins,
  #[doc = "Strain"]
  Strain = nidaqmx_sys::DAQmx_Val_Strain,
  #[doc = "Ohms"]
  Ohms = nidaqmx_sys::DAQmx_Val_Ohms,
  #[doc = "Hz"]
  Hz = nidaqmx_sys::DAQmx_Val_Hz,
  #[doc = "Seconds"]
  Seconds = nidaqmx_sys::DAQmx_Val_Seconds,
  #[doc = "Meters"]
  Meters = nidaqmx_sys::DAQmx_Val_Meters,
  #[doc = "Inches"]
  Inches = nidaqmx_sys::DAQmx_Val_Inches,
  #[doc = "Degrees"]
  Degrees = nidaqmx_sys::DAQmx_Val_Degrees,
  #[doc = "Radians"]
  Radians = nidaqmx_sys::DAQmx_Val_Radians,
  #[doc = "Ticks"]
  Ticks = nidaqmx_sys::DAQmx_Val_Ticks,
  #[doc = "RPM"]
  RPM = nidaqmx_sys::DAQmx_Val_RPM,
  #[doc = "Radians/s"]
  RadiansPerSecond = nidaqmx_sys::DAQmx_Val_RadiansPerSecond,
  #[doc = "Degrees/s"]
  DegreesPerSecond = nidaqmx_sys::DAQmx_Val_DegreesPerSecond,
  #[doc = "g"]
  g = nidaqmx_sys::DAQmx_Val_g,
  #[doc = "m/s^2"]
  MetersPerSecondSquared = nidaqmx_sys::DAQmx_Val_MetersPerSecondSquared,
  #[doc = "in/s^2"]
  InchesPerSecondSquared = nidaqmx_sys::DAQmx_Val_InchesPerSecondSquared,
  #[doc = "m/s"]
  MetersPerSecond = nidaqmx_sys::DAQmx_Val_MetersPerSecond,
  #[doc = "in/s"]
  InchesPerSecond = nidaqmx_sys::DAQmx_Val_InchesPerSecond,
  #[doc = "Pascals"]
  Pascals = nidaqmx_sys::DAQmx_Val_Pascals,
  #[doc = "Newtons"]
  Newtons = nidaqmx_sys::DAQmx_Val_Newtons,
  #[doc = "Pounds"]
  Pounds = nidaqmx_sys::DAQmx_Val_Pounds,
  #[doc = "kgf"]
  KilogramForce = nidaqmx_sys::DAQmx_Val_KilogramForce,
  #[doc = "psi"]
  PoundsPerSquareInch = nidaqmx_sys::DAQmx_Val_PoundsPerSquareInch,
  #[doc = "bar"]
  Bar = nidaqmx_sys::DAQmx_Val_Bar,
  #[doc = "Nm"]
  NewtonMeters = nidaqmx_sys::DAQmx_Val_NewtonMeters,
  #[doc = "oz-in"]
  InchOunces = nidaqmx_sys::DAQmx_Val_InchOunces,
  #[doc = "lb-in"]
  InchPounds = nidaqmx_sys::DAQmx_Val_InchPounds,
  #[doc = "lb-ft"]
  FootPounds = nidaqmx_sys::DAQmx_Val_FootPounds,
  #[doc = "Volts/Volt"]
  VoltsPerVolt = nidaqmx_sys::DAQmx_Val_VoltsPerVolt,
  #[doc = "mVolts/Volt"]
  mVoltsPerVolt = nidaqmx_sys::DAQmx_Val_mVoltsPerVolt,
  #[doc = "Coulombs"]
  Coulombs = nidaqmx_sys::DAQmx_Val_Coulombs,
  #[doc = "PicoCoulombs"]
  PicoCoulombs = nidaqmx_sys::DAQmx_Val_PicoCoulombs,
  #[doc = "From TEDS"]
  FromTEDS = nidaqmx_sys::DAQmx_Val_FromTEDS,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VelocityIEPESensorSensitivityUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "mVolts/mm/s"]
  MillivoltsPerMillimeterPerSecond =
    nidaqmx_sys::DAQmx_Val_MillivoltsPerMillimeterPerSecond,
  #[doc = "mVolts/in/s"]
  MilliVoltsPerInchPerSecond =
    nidaqmx_sys::DAQmx_Val_MilliVoltsPerInchPerSecond,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VelocityUnits {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "m/s"]
  MetersPerSecond = nidaqmx_sys::DAQmx_Val_MetersPerSecond,
  #[doc = "in/s"]
  InchesPerSecond = nidaqmx_sys::DAQmx_Val_InchesPerSecond,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VoltageUnits1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Volts"]
  Volts = nidaqmx_sys::DAQmx_Val_Volts,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
  #[doc = "From TEDS"]
  FromTEDS = nidaqmx_sys::DAQmx_Val_FromTEDS,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VoltageUnits2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Volts"]
  Volts = nidaqmx_sys::DAQmx_Val_Volts,
  #[doc = "From Custom Scale"]
  FromCustomScale = nidaqmx_sys::DAQmx_Val_FromCustomScale,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WaitMode {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Wait For Interrupt"]
  WaitForInterrupt = nidaqmx_sys::DAQmx_Val_WaitForInterrupt,
  #[doc = "Poll"]
  Poll = nidaqmx_sys::DAQmx_Val_Poll,
  #[doc = "Yield"]
  Yield = nidaqmx_sys::DAQmx_Val_Yield,
  #[doc = "Sleep"]
  Sleep = nidaqmx_sys::DAQmx_Val_Sleep,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WaitMode2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Poll"]
  Poll = nidaqmx_sys::DAQmx_Val_Poll,
  #[doc = "Yield"]
  Yield = nidaqmx_sys::DAQmx_Val_Yield,
  #[doc = "Sleep"]
  Sleep = nidaqmx_sys::DAQmx_Val_Sleep,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WaitMode3 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Wait For Interrupt"]
  WaitForInterrupt = nidaqmx_sys::DAQmx_Val_WaitForInterrupt,
  #[doc = "Poll"]
  Poll = nidaqmx_sys::DAQmx_Val_Poll,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WaitMode4 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Wait For Interrupt"]
  WaitForInterrupt = nidaqmx_sys::DAQmx_Val_WaitForInterrupt,
  #[doc = "Poll"]
  Poll = nidaqmx_sys::DAQmx_Val_Poll,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WatchdogAOExpirState {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Voltage"]
  Voltage = nidaqmx_sys::DAQmx_Val_Voltage,
  #[doc = "Current"]
  Current = nidaqmx_sys::DAQmx_Val_Current,
  #[doc = "No Change"]
  NoChange = nidaqmx_sys::DAQmx_Val_NoChange,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WatchdogCOExpirState {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Low"]
  Low = nidaqmx_sys::DAQmx_Val_Low,
  #[doc = "High"]
  High = nidaqmx_sys::DAQmx_Val_High,
  #[doc = "No Change"]
  NoChange = nidaqmx_sys::DAQmx_Val_NoChange,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WindowTriggerCondition1 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Entering Window"]
  EnteringWin = nidaqmx_sys::DAQmx_Val_EnteringWin,
  #[doc = "Leaving Window"]
  LeavingWin = nidaqmx_sys::DAQmx_Val_LeavingWin,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WindowTriggerCondition2 {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Inside Window"]
  InsideWin = nidaqmx_sys::DAQmx_Val_InsideWin,
  #[doc = "Outside Window"]
  OutsideWin = nidaqmx_sys::DAQmx_Val_OutsideWin,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WriteBasicTEDSOptions {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "Write To EEPROM"]
  WriteToEEPROM = nidaqmx_sys::DAQmx_Val_WriteToEEPROM,
  #[doc = "Write To PROM Once"]
  WriteToPROM = nidaqmx_sys::DAQmx_Val_WriteToPROM,
  #[doc = "Do Not Write"]
  DoNotWrite = nidaqmx_sys::DAQmx_Val_DoNotWrite,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WriteRelativeTo {
  #[doc = "Invalid"]
  #[default]
  invalid = 22222,
  #[doc = "First Sample"]
  FirstSample = nidaqmx_sys::DAQmx_Val_FirstSample,
  #[doc = "Current Write Position"]
  CurrWritePos = nidaqmx_sys::DAQmx_Val_CurrWritePos,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ExcitationIdleOutputBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Zero  Volts or Amps"]
  ZeroVoltsOrAmps = nidaqmx_sys::DAQmx_Val_ZeroVoltsOrAmps,
  #[doc = "Maintain Existing Value"]
  MaintainExistingValue = nidaqmx_sys::DAQmx_Val_MaintainExistingValue,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SampClkOverrunBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Repe ated Data"]
  RepeatedData = nidaqmx_sys::DAQmx_Val_RepeatedData,
  #[doc = "Sentinel Value"]
  SentinelValue = nidaqmx_sys::DAQmx_Val_SentinelValue,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LogicLvlBehavior {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Logic Level Pull-up"]
  LogicLevelPullUp = nidaqmx_sys::DAQmx_Val_LogicLevelPullUp,
  #[doc = "None"]
  None = nidaqmx_sys::DAQmx_Val_None,
}
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Sense {
  #[doc = "Invalid"]
  #[default]
  Invalid = 22222,
  #[doc = "Local"]
  Local = nidaqmx_sys::DAQmx_Val_Local,
  #[doc = "Remote"]
  Remote = nidaqmx_sys::DAQmx_Val_Remote,
}

// DAQmx_AI_ACExcit_WireMode
//*** Value set ACExcitWireMode ***

// DAQmx_AI_ADCTimingMode
//*** Value set ADCTimingMode ***

// DAQmx_AI_MeasType
// DAQmx_Dev_AI_SupportedMeasTypes
// DAQmx_PhysicalChan_AI_SupportedMeasTypes
//*** Value set AIMeasurementType ***

// DAQmx_AO_IdleOutputBehavior
//*** Value set AOIdleOutputBehavior ***

// DAQmx_AO_OutputType
// DAQmx_Dev_AO_SupportedOutputTypes
// DAQmx_PhysicalChan_AO_SupportedOutputTypes
//*** Value set AOOutputChannelType ***

// DAQmx_AI_Accel_Charge_SensitivityUnits
//*** Value set AccelChargeSensitivityUnits ***

// DAQmx_AI_Accel_4WireDCVoltage_SensitivityUnits
// DAQmx_AI_Accel_SensitivityUnits
//*** Value set AccelSensitivityUnits1 ***

// DAQmx_AI_Accel_Units
//*** Value set AccelUnits2 ***

// DAQmx_Dev_AI_SampModes
// DAQmx_Dev_AO_SampModes
// DAQmx_Dev_CI_SampModes
// DAQmx_Dev_CO_SampModes
// DAQmx_PhysicalChan_DI_SampModes
// DAQmx_PhysicalChan_DO_SampModes
// DAQmx_SampQuant_SampMode
//*** Value set AcquisitionType ***

// DAQmx_AnlgLvl_PauseTrig_When
//*** Value set ActiveLevel ***

// DAQmx_AI_RVDT_Units
//*** Value set AngleUnits1 ***

// DAQmx_CI_AngEncoder_Units
//*** Value set AngleUnits2 ***

// DAQmx_CI_Velocity_AngEncoder_Units
//*** Value set AngularVelocityUnits ***

// DAQmx_AI_AutoZeroMode
//*** Value set AutoZeroType1 ***

// DAQmx_SwitchScan_BreakMode
//*** Value set BreakMode ***

// DAQmx_AI_Bridge_Cfg
//*** Value set BridgeConfiguration1 ***

// DAQmx_AI_Bridge_ElectricalUnits
//*** Value set BridgeElectricalUnits ***

// DAQmx_AI_Bridge_PhysicalUnits
//*** Value set BridgePhysicalUnits ***

// DAQmx_AI_Bridge_Units
//*** Value set BridgeUnits ***

// DAQmx_Dev_BusType
//*** Value set BusType ***

// DAQmx_CI_MeasType
// DAQmx_Dev_CI_SupportedMeasTypes
// DAQmx_PhysicalChan_CI_SupportedMeasTypes
//*** Value set CIMeasurementType ***

// DAQmx_AI_Thrmcpl_CJCSrc
//*** Value set CJCSource1 ***

// DAQmx_CO_OutputType
// DAQmx_Dev_CO_SupportedOutputTypes
// DAQmx_PhysicalChan_CO_SupportedOutputTypes
//*** Value set COOutputType ***

// DAQmx_ChanType
//*** Value set ChannelType ***

// DAQmx_CO_ConstrainedGenMode
//*** Value set ConstrainedGenMode ***

// DAQmx_CI_CountEdges_Dir
//*** Value set CountDirection1 ***

// DAQmx_CI_Freq_MeasMeth
// DAQmx_CI_Period_MeasMeth
//*** Value set CounterFrequencyMethod ***

// DAQmx_AI_Coupling
//*** Value set Coupling1 ***

// DAQmx_AnlgEdge_StartTrig_Coupling
// DAQmx_AnlgMultiEdge_StartTrig_Couplings
// DAQmx_AnlgWin_StartTrig_Coupling
// DAQmx_AnlgEdge_RefTrig_Coupling
// DAQmx_AnlgMultiEdge_RefTrig_Couplings
// DAQmx_AnlgWin_RefTrig_Coupling
// DAQmx_AnlgLvl_PauseTrig_Coupling
// DAQmx_AnlgWin_PauseTrig_Coupling
//*** Value set Coupling2 ***

// DAQmx_AI_CurrentShunt_Loc
//*** Value set CurrentShuntResistorLocation1 ***

// DAQmx_AI_Bridge_ShuntCal_ShuntCalASrc
//*** Value set BridgeShuntCalSource ***

// DAQmx_AI_Charge_Units
//*** Value set ChargeUnits ***

// DAQmx_AI_Current_Units
// DAQmx_AI_Current_ACRMS_Units
// DAQmx_AO_Current_Units
//*** Value set CurrentUnits1 ***

//*** Value set CurrentUnits2 ***

// DAQmx_AI_RawSampJustification
//*** Value set DataJustification1 ***

// DAQmx_AI_DataXferMech
// DAQmx_AO_DataXferMech
// DAQmx_DI_DataXferMech
// DAQmx_DO_DataXferMech
// DAQmx_CI_DataXferMech
// DAQmx_CO_DataXferMech
//*** Value set DataTransferMechanism ***

// DAQmx_Exported_RdyForXferEvent_DeassertCond
//*** Value set DeassertCondition ***

// DAQmx_DO_OutputDriveType
//*** Value set DigitalDriveType ***

// DAQmx_DO_LineStates_StartState
// DAQmx_DO_LineStates_PausedState
// DAQmx_DO_LineStates_DoneState
// DAQmx_Watchdog_DO_ExpirState
//*** Value set DigitalLineState ***

// DAQmx_DigPattern_StartTrig_When
// DAQmx_DigPattern_RefTrig_When
// DAQmx_DigPattern_PauseTrig_When
//*** Value set DigitalPatternCondition1 ***

// DAQmx_StartTrig_DelayUnits
//*** Value set DigitalWidthUnits1 ***

// DAQmx_DelayFromSampClk_DelayUnits
//*** Value set DigitalWidthUnits2 ***

// DAQmx_Exported_AdvTrig_Pulse_WidthUnits
//*** Value set DigitalWidthUnits3 ***

// DAQmx_AI_FilterDelayUnits
// DAQmx_AO_FilterDelayUnits
// DAQmx_CI_FilterDelayUnits
//*** Value set DigitalWidthUnits4 ***

// DAQmx_AI_EddyCurrentProxProbe_SensitivityUnits
//*** Value set EddyCurrentProxProbeSensitivityUnits ***

// DAQmx_CI_Freq_StartingEdge
// DAQmx_CI_Period_StartingEdge
// DAQmx_CI_CountEdges_ActiveEdge
// DAQmx_CI_CountEdges_CountReset_ActiveEdge
// DAQmx_CI_DutyCycle_StartingEdge
// DAQmx_CI_PulseWidth_StartingEdge
// DAQmx_CI_TwoEdgeSep_FirstEdge
// DAQmx_CI_TwoEdgeSep_SecondEdge
// DAQmx_CI_SemiPeriod_StartingEdge
// DAQmx_CI_Pulse_Freq_Start_Edge
// DAQmx_CI_Pulse_Time_StartEdge
// DAQmx_CI_Pulse_Ticks_StartEdge
// DAQmx_CI_CtrTimebaseActiveEdge
// DAQmx_CO_CtrTimebaseActiveEdge
// DAQmx_SampClk_ActiveEdge
// DAQmx_SampClk_Timebase_ActiveEdge
// DAQmx_AIConv_ActiveEdge
// DAQmx_DigEdge_StartTrig_Edge
// DAQmx_DigEdge_RefTrig_Edge
// DAQmx_DigEdge_AdvTrig_Edge
// DAQmx_DigEdge_ArmStartTrig_Edge
// DAQmx_DigEdge_WatchdogExpirTrig_Edge
//*** Value set Edge1 ***

// DAQmx_CI_Encoder_DecodingType
// DAQmx_CI_Velocity_Encoder_DecodingType
//*** Value set EncoderType2 ***

// DAQmx_CI_Encoder_ZIndexPhase
//*** Value set EncoderZIndexPhase1 ***

// DAQmx_AI_Excit_DCorAC
//*** Value set ExcitationDCorAC ***

// DAQmx_AI_Excit_Src
//*** Value set ExcitationSource ***

// DAQmx_AI_Excit_VoltageOrCurrent
//*** Value set ExcitationVoltageOrCurrent ***

// DAQmx_Exported_CtrOutEvent_OutputBehavior
//*** Value set ExportActions2 ***

// DAQmx_Exported_SampClk_OutputBehavior
//*** Value set ExportActions3 ***

// DAQmx_Exported_HshkEvent_OutputBehavior
//*** Value set ExportActions5 ***

// DAQmx_AI_DigFltr_Type
// DAQmx_AI_DigFltr_Types
//*** Value set FilterType2 ***

// DAQmx_AI_DigFltr_Response
//*** Value set FilterResponse ***

// DAQmx_AI_Filter_Response
// DAQmx_CI_Filter_Response
//*** Value set FilterResponse1 ***

// DAQmx_AI_Force_IEPESensor_SensitivityUnits
//*** Value set ForceIEPESensorSensitivityUnits ***

// DAQmx_AI_Force_Units
//*** Value set ForceUnits ***

// DAQmx_AI_Freq_Units
//*** Value set FrequencyUnits ***

// DAQmx_CI_Pulse_Freq_Units
// DAQmx_CO_Pulse_Freq_Units
//*** Value set FrequencyUnits2 ***

// DAQmx_CI_Freq_Units
//*** Value set FrequencyUnits3 ***

// DAQmx_AO_FuncGen_Type
//*** Value set FuncGenType ***

// DAQmx_CI_GPS_SyncMethod
//*** Value set GpsSignalType1 ***

// DAQmx_Hshk_StartCond
//*** Value set HandshakeStartCondition ***

// DAQmx_AI_DataXferReqCond
// DAQmx_DI_DataXferReqCond
// DAQmx_CI_DataXferReqCond
//*** Value set InputDataTransferCondition ***

// DAQmx_AI_TermCfg
//*** Value set InputTermCfg ***

// DAQmx_CI_Freq_TermCfg
// DAQmx_CI_Period_TermCfg
// DAQmx_CI_CountEdges_TermCfg
// DAQmx_CI_CountEdges_CountDir_TermCfg
// DAQmx_CI_CountEdges_CountReset_TermCfg
// DAQmx_CI_CountEdges_Gate_TermCfg
// DAQmx_CI_DutyCycle_TermCfg
// DAQmx_CI_Encoder_AInputTermCfg
// DAQmx_CI_Encoder_BInputTermCfg
// DAQmx_CI_Encoder_ZInputTermCfg
// DAQmx_CI_PulseWidth_TermCfg
// DAQmx_CI_Velocity_Encoder_AInputTermCfg
// DAQmx_CI_Velocity_Encoder_BInputTermCfg
// DAQmx_CI_TwoEdgeSep_FirstTermCfg
// DAQmx_CI_TwoEdgeSep_SecondTermCfg
// DAQmx_CI_SemiPeriod_TermCfg
// DAQmx_CI_Pulse_Freq_TermCfg
// DAQmx_CI_Pulse_Time_TermCfg
// DAQmx_CI_Pulse_Ticks_TermCfg
//*** Value set InputTermCfg2 ***

// DAQmx_AI_LVDT_SensitivityUnits
//*** Value set LVDTSensitivityUnits1 ***

// DAQmx_AI_LVDT_Units
// DAQmx_AI_EddyCurrentProxProbe_Units
//*** Value set LengthUnits2 ***

// DAQmx_CI_LinEncoder_Units
//*** Value set LengthUnits3 ***

// DAQmx_CI_CountEdges_GateWhen
// DAQmx_CI_OutputState
// DAQmx_CO_Pulse_IdleState
// DAQmx_CO_OutputState
// DAQmx_Exported_CtrOutEvent_Toggle_IdleState
// DAQmx_Exported_HshkEvent_Interlocked_AssertedLvl
// DAQmx_Interlocked_HshkTrig_AssertedLvl
// DAQmx_DigLvl_PauseTrig_When
//*** Value set Level1 ***

// DAQmx_Logging_Mode
//*** Value set LoggingMode ***

// DAQmx_Logging_TDMS_Operation
//*** Value set LoggingOperation ***

// DAQmx_DI_LogicFamily
// DAQmx_DO_LogicFamily
//*** Value set LogicFamily ***

// DAQmx_AIConv_Timebase_Src
//*** Value set MIOAIConvertTbSrc ***

// DAQmx_AO_FuncGen_ModulationType
//*** Value set ModulationType ***

// DAQmx_AO_DataXferReqCond
// DAQmx_DO_DataXferReqCond
// DAQmx_CO_DataXferReqCond
//*** Value set OutputDataTransferCondition ***

// DAQmx_AO_TermCfg
//*** Value set OutputTermCfg ***

// DAQmx_SampClk_OverrunBehavior
//*** Value set OverflowBehavior ***

// DAQmx_Read_OverWrite
//*** Value set OverwriteMode1 ***

// DAQmx_Exported_AIConvClk_Pulse_Polarity
// DAQmx_Exported_SampClk_Pulse_Polarity
// DAQmx_Exported_AdvTrig_Pulse_Polarity
// DAQmx_Exported_PauseTrig_Lvl_ActiveLvl
// DAQmx_Exported_RefTrig_Pulse_Polarity
// DAQmx_Exported_StartTrig_Pulse_Polarity
// DAQmx_Exported_AdvCmpltEvent_Pulse_Polarity
// DAQmx_Exported_AIHoldCmpltEvent_PulsePolarity
// DAQmx_Exported_ChangeDetectEvent_Pulse_Polarity
// DAQmx_Exported_CtrOutEvent_Pulse_Polarity
// DAQmx_Exported_HshkEvent_Pulse_Polarity
// DAQmx_Exported_RdyForXferEvent_Lvl_ActiveLvl
// DAQmx_Exported_DataActiveEvent_Lvl_ActiveLvl
// DAQmx_Exported_RdyForStartEvent_Lvl_ActiveLvl
//*** Value set Polarity2 ***

// DAQmx_Pwr_IdleOutputBehavior
//*** Value set PowerIdleOutputBehavior ***

// DAQmx_Pwr_OutputState
//*** Value set PowerOutputState ***

// DAQmx_AI_Pressure_Units
//*** Value set PressureUnits ***

// DAQmx_Dev_ProductCategory
//*** Value set ProductCategory ***

// DAQmx_AI_RTD_Type
//*** Value set RTDType1 ***

// DAQmx_AI_RVDT_SensitivityUnits
//*** Value set RVDTSensitivityUnits1 ***

// DAQmx_AI_RawDataCompressionType
//*** Value set RawDataCompressionType ***

// DAQmx_Read_RelativeTo
//*** Value set ReadRelativeTo ***

// DAQmx_Write_RegenMode
//*** Value set RegenerationMode1 ***

// DAQmx_AI_ResistanceCfg
//*** Value set ResistanceConfiguration ***

// DAQmx_AI_Resistance_Units
//*** Value set ResistanceUnits1 ***

//*** Value set ResistanceUnits2 ***

// DAQmx_AI_ResolutionUnits
// DAQmx_AO_ResolutionUnits
//*** Value set ResolutionType1 ***

//*** Value set SCXI1124Range ***

// DAQmx_DI_AcquireOn
// DAQmx_DO_GenerateOn
//*** Value set SampleClockActiveOrInactiveEdgeSelection ***

// DAQmx_Hshk_SampleInputDataWhen
//*** Value set SampleInputDataWhen ***

// DAQmx_SampTimingType
//*** Value set SampleTimingType ***

// DAQmx_Scale_Type
//*** Value set ScaleType ***

// DAQmx_AI_Thrmcpl_ScaleType
//*** Value set ScaleType2 ***

// DAQmx_AI_ChanCal_ScaleType
//*** Value set ScaleType3 ***

// DAQmx_AI_Bridge_ScaleType
//*** Value set ScaleType4 ***

// DAQmx_AI_SensorPower_Cfg
//*** Value set SensorPowerCfg ***

// DAQmx_AI_SensorPower_Type
// DAQmx_PhysicalChan_AI_SensorPower_Types
// DAQmx_PhysicalChan_AI_PowerControl_Type
//*** Value set SensorPowerType ***

// DAQmx_AI_Bridge_ShuntCal_Select
//*** Value set ShuntCalSelect ***

//*** Value set ShuntElementLocation ***

//*** Value set Signal ***

//*** Value set Signal2 ***

// DAQmx_AnlgEdge_StartTrig_Slope
// DAQmx_AnlgMultiEdge_StartTrig_Slopes
// DAQmx_AnlgEdge_RefTrig_Slope
// DAQmx_AnlgMultiEdge_RefTrig_Slopes
//*** Value set Slope1 ***

// DAQmx_AI_SoundPressure_Units
//*** Value set SoundPressureUnits1 ***

// DAQmx_AI_Lowpass_SwitchCap_ClkSrc
// DAQmx_AO_DAC_Ref_Src
// DAQmx_AO_DAC_Offset_Src
//*** Value set SourceSelection ***

// DAQmx_AI_StrainGage_Cfg
//*** Value set StrainGageBridgeType1 ***

// DAQmx_AI_RosetteStrainGage_RosetteType
//*** Value set StrainGageRosetteType ***

// DAQmx_AI_RosetteStrainGage_RosetteMeasType
//*** Value set StrainGageRosetteMeasurementType ***

// DAQmx_AI_Strain_Units
//*** Value set StrainUnits1 ***

// DAQmx_SwitchScan_RepeatMode
//*** Value set SwitchScanRepeatMode ***

// DAQmx_SwitchChan_Usage
//*** Value set SwitchUsageTypes ***

// DAQmx_SyncPulse_Type
//*** Value set SyncPulseType ***

// DAQmx_Trigger_SyncType
//*** Value set SyncType ***

// DAQmx_Chan_SyncUnlockBehavior
//*** Value set SyncUnlockBehavior ***

//*** Value set TEDSUnits ***

// DAQmx_AI_Temp_Units
//*** Value set TemperatureUnits1 ***

// DAQmx_AI_Thrmcpl_Type
//*** Value set ThermocoupleType1 ***

// DAQmx_SyncPulse_Time_Timescale
// DAQmx_FirstSampTimestamp_Timescale
// DAQmx_FirstSampClk_Timescale
// DAQmx_StartTrig_Timescale
// DAQmx_StartTrig_TimestampTimescale
// DAQmx_RefTrig_TimestampTimescale
// DAQmx_ArmStartTrig_Timescale
// DAQmx_ArmStartTrig_TimestampTimescale
//*** Value set Timescale2 ***

// DAQmx_CI_Timestamp_Units
//*** Value set TimeUnits ***

// DAQmx_CI_Pulse_Time_Units
// DAQmx_CO_Pulse_Time_Units
//*** Value set TimeUnits2 ***

// DAQmx_CI_Period_Units
// DAQmx_CI_PulseWidth_Units
// DAQmx_CI_TwoEdgeSep_Units
// DAQmx_CI_SemiPeriod_Units
//*** Value set TimeUnits3 ***

//*** Value set TimingResponseMode ***

// DAQmx_AI_Torque_Units
//*** Value set TorqueUnits ***

// DAQmx_ArmStartTrig_Type
// DAQmx_WatchdogExpirTrig_Type
//*** Value set TriggerType4 ***

// DAQmx_AdvTrig_Type
//*** Value set TriggerType5 ***

// DAQmx_PauseTrig_Type
//*** Value set TriggerType6 ***

// DAQmx_RefTrig_Type
//*** Value set TriggerType8 ***

// DAQmx_HshkTrig_Type
//*** Value set TriggerType9 ***

// DAQmx_StartTrig_Type
//*** Value set TriggerType10 ***

// DAQmx_SampClk_UnderflowBehavior
// DAQmx_Implicit_UnderflowBehavior
//*** Value set UnderflowBehavior ***

// DAQmx_Scale_PreScaledUnits
//*** Value set UnitsPreScaled ***

// DAQmx_AI_Velocity_IEPESensor_SensitivityUnits
//*** Value set VelocityIEPESensorSensitivityUnits ***

// DAQmx_AI_Velocity_Units
// DAQmx_CI_Velocity_LinEncoder_Units
//*** Value set VelocityUnits ***

// DAQmx_AI_Voltage_Units
// DAQmx_AI_Voltage_ACRMS_Units
//*** Value set VoltageUnits1 ***

// DAQmx_AO_Voltage_Units
//*** Value set VoltageUnits2 ***

// DAQmx_Read_WaitMode
//*** Value set WaitMode ***

// DAQmx_Write_WaitMode
//*** Value set WaitMode2 ***

// DAQmx_RealTime_WaitForNextSampClkWaitMode
//*** Value set WaitMode3 ***

// DAQmx_RealTime_WriteRecoveryMode
//*** Value set WaitMode4 ***

// DAQmx_Watchdog_AO_OutputType
//*** Value set WatchdogAOExpirState ***

// DAQmx_Watchdog_CO_ExpirState
//*** Value set WatchdogCOExpirState ***

// DAQmx_AnlgWin_StartTrig_When
// DAQmx_AnlgWin_RefTrig_When
//*** Value set WindowTriggerCondition1 ***

// DAQmx_AnlgWin_PauseTrig_When
//*** Value set WindowTriggerCondition2 ***

//*** Value set WriteBasicTEDSOptions ***

// DAQmx_Write_RelativeTo
//*** Value set WriteRelativeTo ***

// DAQmx_AI_Excit_IdleOutputBehavior
//*** Value set ExcitationIdleOutputBehavior ***

// DAQmx_CI_SampClkOverrunBehavior
//*** Value set SampClkOverrunBehavior ***

// DAQmx_CI_Freq_LogicLvlBehavior
// DAQmx_CI_Period_LogicLvlBehavior
// DAQmx_CI_CountEdges_LogicLvlBehavior
// DAQmx_CI_CountEdges_CountDir_LogicLvlBehavior
// DAQmx_CI_CountEdges_CountReset_LogicLvlBehavior
// DAQmx_CI_CountEdges_Gate_LogicLvlBehavior
// DAQmx_CI_DutyCycle_LogicLvlBehavior
// DAQmx_CI_Encoder_AInputLogicLvlBehavior
// DAQmx_CI_Encoder_BInputLogicLvlBehavior
// DAQmx_CI_Encoder_ZInputLogicLvlBehavior
// DAQmx_CI_PulseWidth_LogicLvlBehavior
// DAQmx_CI_Velocity_Encoder_AInputLogicLvlBehavior
// DAQmx_CI_Velocity_Encoder_BInputLogicLvlBehavior
// DAQmx_CI_TwoEdgeSep_FirstLogicLvlBehavior
// DAQmx_CI_TwoEdgeSep_SecondLogicLvlBehavior
// DAQmx_CI_SemiPeriod_LogicLvlBehavior
// DAQmx_CI_Pulse_Freq_LogicLvlBehavior
// DAQmx_CI_Pulse_Time_LogicLvlBehavior
// DAQmx_CI_Pulse_Ticks_LogicLvlBehavior
//*** Value set LogicLvlBehavior ***

// DAQmx_AI_Excit_Sense
// DAQmx_Pwr_RemoteSense
//*** Value set Sense ***
