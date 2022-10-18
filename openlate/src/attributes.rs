#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use nidaqmx_sys;
use strum::{self, EnumString, EnumVariantNames, FromRepr};

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum BufferStrum {
    #[doc = "Specifies the number of samples the input buffer can hold for each channel in the task. Zero indicates to allocate no buffer. Use a buffer size of 0 to perform a hardware-timed operation without using a buffer. Setting this property overrides the automatic input buffer allocation that NI-performs."]
    InputBufSize = nidaqmx_sys::DAQmx_Buf_Input_BufSize,
    #[doc = "Indicates in samples per channel the size of the onboard input buffer of the device."]
    InputOnbrdBufSize = nidaqmx_sys::DAQmx_Buf_Input_OnbrdBufSize,
    #[doc = "Specifies the number of samples the input buffer can hold for each channel in the task. Zero indicates to allocate no buffer. Use a buffer size of 0 to perform a hardware-timed operation without using a buffer. Setting this property overrides the automatic input buffer allocation that NI-performs."]
    OutputBufSize = nidaqmx_sys::DAQmx_Buf_Output_BufSize,
    #[doc = "Indicates in samples per channel the size of the onboard input buffer of the device."]
    OutputOnbrdBufSize = nidaqmx_sys::DAQmx_Buf_Output_OnbrdBufSize,
}

pub enum BufferAttr {
    #[doc = "Specifies the number of samples the input buffer can hold for each channel in the task. Zero indicates to allocate no buffer. Use a buffer size of 0 to perform a hardware-timed operation without using a buffer. Setting this property overrides the automatic input buffer allocation that NI-performs."]
    InputBufSize,
    #[doc = "Indicates in samples per channel the size of the onboard input buffer of the device."]
    InputOnbrdBufSize,
    #[doc = "Specifies the number of samples the input buffer can hold for each channel in the task. Zero indicates to allocate no buffer. Use a buffer size of 0 to perform a hardware-timed operation without using a buffer. Setting this property overrides the automatic input buffer allocation that NI-performs."]
    OutputBufSize,
    #[doc = "Indicates in samples per channel the size of the onboard input buffer of the device."]
    OutputOnbrdBufSize,
}

impl From<BufferAttr> for i32 {
    fn from(attr: BufferAttr) -> Self {
        match attr {
            BufferAttr::InputBufSize => nidaqmx_sys::DAQmx_Buf_Input_BufSize,
            BufferAttr::InputOnbrdBufSize => nidaqmx_sys::DAQmx_Buf_Input_OnbrdBufSize,
            BufferAttr::OutputBufSize => nidaqmx_sys::DAQmx_Buf_Output_BufSize,
            BufferAttr::OutputOnbrdBufSize => nidaqmx_sys::DAQmx_Buf_Output_OnbrdBufSize,
        }
    }
}

impl From<i32> for BufferAttr {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_Buf_Input_BufSize => BufferAttr::InputBufSize,
            nidaqmx_sys::DAQmx_Buf_Input_OnbrdBufSize => BufferAttr::InputOnbrdBufSize,
            nidaqmx_sys::DAQmx_Buf_Output_BufSize => BufferAttr::OutputBufSize,
            nidaqmx_sys::DAQmx_Buf_Output_OnbrdBufSize => BufferAttr::OutputOnbrdBufSize,
            _ => panic!("Unknown BufferAttr!"),
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum CalibrationInfo {
    #[doc = "Indicates whether the device supports self-calibration."]
    SelfCal_Supported,
    #[doc = "Indicates in degrees Celsius the temperature of the device at the time of the last self-calibration. Compare this temperature to the current onboard temperature to determine if you should perform another calibration."]
    SelfCal_LastTemp,
    #[doc = "Indicates in months the National Instruments recommended interval between each external calibration of the device."]
    ExtCal_RecommendedInterval,
    #[doc = "Indicates in degrees Celsius the temperature of the device at the time of the last external calibration. Compare this temperature to the current onboard temperature to determine if you should perform another calibration."]
    ExtCal_LastTemp,
    #[doc = "Specifies a string that contains arbitrary, user-defined information. This number of characters in this string can be no more than Max Size."]
    Cal_UserDefinedInfo,
    #[doc = "Indicates the maximum length in characters of Information."]
    Cal_UserDefinedInfo_MaxSize,
    #[doc = "Indicates in degrees Celsius the current temperature of the device."]
    Cal_DevTemp,
    #[doc = "Specifies the number of times a particular connection that results in tangible wear and tear of onboard components has been made on the accessory. This connection count is useful for tracking accessory life and usage."]
    Cal_AccConnectionCount,
    #[doc = "Indicates the recommended connection count limit for an accessory. If the accessory connection count exceeds this limit, the accessory could require maintenance."]
    Cal_RecommendedAccConnectionCountLimit,
}

impl From<CalibrationInfo> for i32 {
    fn from(attr: CalibrationInfo) -> Self {
        match attr {
            CalibrationInfo::SelfCal_Supported => nidaqmx_sys::DAQmx_SelfCal_Supported,
            CalibrationInfo::SelfCal_LastTemp => nidaqmx_sys::DAQmx_SelfCal_LastTemp,
            CalibrationInfo::ExtCal_RecommendedInterval => {
                nidaqmx_sys::DAQmx_ExtCal_RecommendedInterval
            }
            CalibrationInfo::ExtCal_LastTemp => nidaqmx_sys::DAQmx_ExtCal_LastTemp,
            CalibrationInfo::Cal_UserDefinedInfo => nidaqmx_sys::DAQmx_Cal_UserDefinedInfo,
            CalibrationInfo::Cal_UserDefinedInfo_MaxSize => {
                nidaqmx_sys::DAQmx_Cal_UserDefinedInfo_MaxSize
            }
            CalibrationInfo::Cal_DevTemp => nidaqmx_sys::DAQmx_Cal_DevTemp,
            CalibrationInfo::Cal_AccConnectionCount => nidaqmx_sys::DAQmx_Cal_AccConnectionCount,
            CalibrationInfo::Cal_RecommendedAccConnectionCountLimit => {
                nidaqmx_sys::DAQmx_Cal_RecommendedAccConnectionCountLimit
            }
        }
    }
}

impl From<i32> for CalibrationInfo {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_SelfCal_Supported => CalibrationInfo::SelfCal_Supported,
            nidaqmx_sys::DAQmx_SelfCal_LastTemp => CalibrationInfo::SelfCal_LastTemp,
            nidaqmx_sys::DAQmx_ExtCal_RecommendedInterval => {
                CalibrationInfo::ExtCal_RecommendedInterval
            }
            nidaqmx_sys::DAQmx_ExtCal_LastTemp => CalibrationInfo::ExtCal_LastTemp,
            nidaqmx_sys::DAQmx_Cal_UserDefinedInfo => CalibrationInfo::Cal_UserDefinedInfo,
            nidaqmx_sys::DAQmx_Cal_UserDefinedInfo_MaxSize => {
                CalibrationInfo::Cal_UserDefinedInfo_MaxSize
            }
            nidaqmx_sys::DAQmx_Cal_DevTemp => CalibrationInfo::Cal_DevTemp,
            nidaqmx_sys::DAQmx_Cal_AccConnectionCount => CalibrationInfo::Cal_AccConnectionCount,
            nidaqmx_sys::DAQmx_Cal_RecommendedAccConnectionCountLimit => {
                CalibrationInfo::Cal_RecommendedAccConnectionCountLimit
            }
            _ => panic!("Unknown CalibrationInfo!"),
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum AnalogInput {
    #[doc = "Specifies the maximum value you expect to measure. This value is in the units you specify with a units property. When you query this property, it returns the coerced maximum value that the device can measure with the current settings."]
    Max,
    #[doc = "Specifies the minimum value you expect to measure. This value is in the units you specify with a units property.  When you query this property, it returns the coerced minimum value that the device can measure with the current settings."]
    Min,
    #[doc = "Specifies the name of a custom scale for the channel."]
    CustomScaleName,
    #[doc = "Indicates the measurement to take with the analog input channel and in some cases, such as for temperature measurements, the sensor to use."]
    MeasType,
    #[doc = "Specifies the units to use to return voltage measurements from the channel."]
    Voltage_Units,
    #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes."]
    Voltage_dBRef,
    #[doc = "Specifies the units to use to return voltage RMS measurements from the channel."]
    Voltage_ACRMS_Units,
    #[doc = "Specifies the units to use to return temperature measurements from the channel."]
    Temp_Units,
    #[doc = "Specifies the type of thermocouple connected to the channel. Thermocouple types differ in composition and measurement range."]
    Thrmcpl_Type,
    #[doc = "Specifies the method or equation form that the thermocouple scale uses."]
    Thrmcpl_ScaleType,
    #[doc = "Indicates the source of cold-junction compensation."]
    Thrmcpl_CJCSrc,
    #[doc = "Specifies the temperature of the cold junction if CJC Source is DAQmx_Val_ConstVal. Specify this value in the units of the measurement."]
    Thrmcpl_CJCVal,
    #[doc = "Indicates the channel that acquires the temperature of the cold junction if CJC Source is DAQmx_Val_Chan. If the channel is a temperature channel, NI-DAQmx acquires the temperature in the correct units. Other channel types, such as a resistance channel with a custom sensor, must use a custom scale to scale values to degrees Celsius."]
    Thrmcpl_CJCChan,
    #[doc = "Specifies the type of RTD connected to the channel."]
    RTD_Type,
    #[doc = "Specifies in ohms the sensor resistance at 0 deg C. The Callendar-Van Dusen equation requires this value. Refer to the sensor documentation to determine this value."]
    RTD_R0,
    #[doc = "Specifies the 'A' constant of the Callendar-Van Dusen equation. NI-DAQmx requires this value when you use a custom RTD."]
    RTD_A,
    #[doc = "Specifies the 'B' constant of the Callendar-Van Dusen equation. NI-DAQmx requires this value when you use a custom RTD."]
    RTD_B,
    #[doc = "Specifies the 'C' constant of the Callendar-Van Dusen equation. NI-DAQmx requires this value when you use a custom RTD."]
    RTD_C,
    #[doc = "Specifies the 'A' constant of the Steinhart-Hart thermistor equation."]
    Thrmstr_A,
    #[doc = "Specifies the 'B' constant of the Steinhart-Hart thermistor equation."]
    Thrmstr_B,
    #[doc = "Specifies the 'C' constant of the Steinhart-Hart thermistor equation."]
    Thrmstr_C,
    #[doc = "Specifies in ohms the value of the reference resistor for the thermistor if you use voltage excitation. NI-DAQmx ignores this value for current excitation."]
    Thrmstr_R1,
    #[doc = "Specifies whether to read from the channel if it is a cold-junction compensation channel. By default, an NI-DAQmx Read function does not return data from cold-junction compensation channels.  Setting this property to TRUE forces read operations to return the cold-junction compensation channel data with the other channels in the task."]
    ForceReadFromChan,
    #[doc = "Specifies the units to use to return current measurements from the channel."]
    Current_Units,
    #[doc = "Specifies the units to use to return current RMS measurements from the channel."]
    Current_ACRMS_Units,
    #[doc = "Specifies the units to use to return strain measurements from the channel."]
    Strain_Units,
    #[doc = "Specifies whether the data is returned by an NI-DAQmx Read function when set on a raw strain channel that is part of a rosette configuration."]
    StrainGage_ForceReadFromChan,
    #[doc = "Specifies the sensitivity of the strain gage.  Gage factor relates the change in electrical resistance to the change in strain. Refer to the sensor documentation for this value."]
    StrainGage_GageFactor,
    #[doc = "Specifies the ratio of lateral strain to axial strain in the material you are measuring."]
    StrainGage_PoissonRatio,
    #[doc = "Specifies the bridge configuration of the strain gages."]
    StrainGage_Cfg,
    #[doc = "Indicates the type of rosette gage."]
    RosetteStrainGage_RosetteType,
    #[doc = "Specifies gage orientation in degrees with respect to the X axis."]
    RosetteStrainGage_Orientation,
    #[doc = "Indicates the raw strain channels that comprise the strain rosette."]
    RosetteStrainGage_StrainChans,
    #[doc = "Specifies the type of rosette measurement."]
    RosetteStrainGage_RosetteMeasType,
    #[doc = "Specifies the units to use to return resistance measurements."]
    Resistance_Units,
    #[doc = "Specifies the units to use to return frequency measurements from the channel."]
    Freq_Units,
    #[doc = "Specifies the voltage level at which to recognize waveform repetitions. You should select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
    Freq_ThreshVoltage,
    #[doc = "Specifies in volts a window below Threshold Level. The input voltage must pass below Threshold Level minus this value before NI-DAQmx recognizes a waveform repetition at Threshold Level. Hysteresis can improve the measurement accuracy when the signal contains noise or jitter."]
    Freq_Hyst,
    #[doc = "Specifies the units to use to return linear position measurements from the channel."]
    LVDT_Units,
    #[doc = "Specifies the sensitivity of the LVDT. This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
    LVDT_Sensitivity,
    #[doc = "Specifies the units of Sensitivity."]
    LVDT_SensitivityUnits,
    #[doc = "Specifies the units to use to return angular position measurements from the channel."]
    RVDT_Units,
    #[doc = "Specifies the sensitivity of the RVDT. This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
    RVDT_Sensitivity,
    #[doc = "Specifies the units of Sensitivity."]
    RVDT_SensitivityUnits,
    #[doc = "Specifies the units to use to return proximity measurements from the channel."]
    EddyCurrentProxProbe_Units,
    #[doc = "Specifies the sensitivity of the eddy current proximity probe . This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
    EddyCurrentProxProbe_Sensitivity,
    #[doc = "Specifies the units of Sensitivity."]
    EddyCurrentProxProbe_SensitivityUnits,
    #[doc = "Specifies the maximum instantaneous sound pressure level you expect to measure. This value is in decibels, referenced to 20 micropascals. NI-DAQmx uses the maximum sound pressure level to calculate values in pascals for Maximum Value and Minimum Value for the channel."]
    SoundPressure_MaxSoundPressureLvl,
    #[doc = "Specifies the units to use to return sound pressure measurements from the channel."]
    SoundPressure_Units,
    #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes. NI-DAQmx also uses the decibel reference level when converting Maximum Sound Pressure Level to a voltage level."]
    SoundPressure_dBRef,
    #[doc = "Specifies the sensitivity of the microphone. This value is in mV/Pa. Refer to the sensor documentation to determine this value."]
    Microphone_Sensitivity,
    #[doc = "Specifies the units to use to return acceleration measurements from the channel."]
    Accel_Units,
    #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes."]
    Accel_dBRef,
    #[doc = "Specifies the sensitivity of the 4 wire DC voltage acceleration sensor connected to the channel. This value is the units you specify with AI.Accel.4WireDCVoltage.SensitivityUnits. Refer to the sensor documentation to determine this value."]
    Accel_4WireDCVoltage_Sensitivity,
    #[doc = "Specifies the units of AI.Accel.4WireDCVoltage.Sensitivity."]
    Accel_4WireDCVoltage_SensitivityUnits,
    #[doc = "Specifies the sensitivity of the accelerometer. This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
    Accel_Sensitivity,
    #[doc = "Specifies the units of Sensitivity."]
    Accel_SensitivityUnits,
    #[doc = "Specifies the sensitivity of the charge acceleration sensor connected to the channel. This value is the units you specify with AI.Accel.Charge.SensitivityUnits. Refer to the sensor documentation to determine this value."]
    Accel_Charge_Sensitivity,
    #[doc = "Specifies the units of AI.Accel.Charge.Sensitivity."]
    Accel_Charge_SensitivityUnits,
    #[doc = "Specifies in which unit to return velocity measurements from the channel."]
    Velocity_Units,
    #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes."]
    Velocity_IEPESensor_dBRef,
    #[doc = "Specifies the sensitivity of the IEPE velocity sensor connected to the channel. Specify this value in the unit indicated by Sensitivity Units."]
    Velocity_IEPESensor_Sensitivity,
    #[doc = "Specifies the units for Sensitivity."]
    Velocity_IEPESensor_SensitivityUnits,
    #[doc = "Specifies in which unit to return force or load measurements from the channel."]
    Force_Units,
    #[doc = "Specifies the sensitivity of the IEPE force sensor connected to the channel. Specify this value in the unit indicated by Sensitivity Units."]
    Force_IEPESensor_Sensitivity,
    #[doc = "Specifies the units for Sensitivity."]
    Force_IEPESensor_SensitivityUnits,
    #[doc = "Specifies  in which unit to return pressure measurements from the channel."]
    Pressure_Units,
    #[doc = "Specifies in which unit to return torque measurements from the channel."]
    Torque_Units,
    #[doc = "Specifies in which unit to return voltage ratios from the channel."]
    Bridge_Units,
    #[doc = "Specifies from which electrical unit to scale data. Select  the same unit that the sensor data sheet or calibration certificate uses for electrical values."]
    Bridge_ElectricalUnits,
    #[doc = "Specifies to which physical unit to scale electrical data. Select the same unit that the sensor data sheet or calibration certificate uses for physical values."]
    Bridge_PhysicalUnits,
    #[doc = "Specifies the scaling type to use when scaling electrical values from the sensor to physical units."]
    Bridge_ScaleType,
    #[doc = "Specifies the first electrical value, corresponding to Physical Value. Specify this value in the unit indicated by Electrical Units."]
    Bridge_TwoPointLin_First_ElectricalVal,
    #[doc = "Specifies the first physical value, corresponding to Electrical Value. Specify this value in the unit indicated by Physical Units."]
    Bridge_TwoPointLin_First_PhysicalVal,
    #[doc = "Specifies the second electrical value, corresponding to Physical Value. Specify this value in the unit indicated by Electrical Units."]
    Bridge_TwoPointLin_Second_ElectricalVal,
    #[doc = "Specifies the second physical value, corresponding to Electrical Value. Specify this value in the unit indicated by Physical Units."]
    Bridge_TwoPointLin_Second_PhysicalVal,
    #[doc = "Specifies the array of electrical values that map to the values in Physical Values. Specify this value in the unit indicated by Electrical Units."]
    Bridge_Table_ElectricalVals,
    #[doc = "Specifies the array of physical values that map to the values in Electrical Values. Specify this value in the unit indicated by Physical Units."]
    Bridge_Table_PhysicalVals,
    #[doc = "Specifies an array of coefficients for the polynomial that converts electrical values to physical values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9x^3."]
    Bridge_Poly_ForwardCoeff,
    #[doc = "Specifies an array of coefficients for the polynomial that converts physical values to electrical values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9x^3."]
    Bridge_Poly_ReverseCoeff,
    #[doc = "Specifies the units to use to return charge measurements from the channel."]
    Charge_Units,
    #[doc = "Indicates if the virtual channel was initialized using a TEDS bitstream from the corresponding physical channel."]
    Is_TEDS,
    #[doc = "Indicates the units defined by TEDS information associated with the channel."]
    TEDS_Units,
    #[doc = "Specifies the coupling for the channel."]
    Coupling,
    #[doc = "Specifies the input impedance of the channel."]
    Impedance,
    #[doc = "Specifies the terminal configuration for the channel."]
    TermCfg,
    #[doc = "Specifies the source of the channel. You can use the signal from the I/O connector or one of several calibration signals. Certain devices have a single calibration signal bus. For these devices, you must specify the same calibration signal for all channels you connect to a calibration signal."]
    InputSrc,
    #[doc = "Specifies the resistance configuration for the channel. NI-DAQmx uses this value for any resistance-based measurements, including temperature measurement using a thermistor or RTD."]
    ResistanceCfg,
    #[doc = "Specifies in ohms the resistance of the wires that lead to the sensor."]
    LeadWireResistance,
    #[doc = "Specifies the type of Wheatstone bridge connected to the channel."]
    Bridge_Cfg,
    #[doc = "Specifies in ohms the resistance of the bridge while not under load."]
    Bridge_NomResistance,
    #[doc = "Specifies in volts the output voltage of the bridge while not under load. NI-DAQmx subtracts this value from any measurements before applying scaling equations.  If you set Initial Bridge Ratio, NI-DAQmx coerces this property to Initial Bridge Ratio times Actual Excitation Value. This property is set by DAQmx Perform Bridge Offset Nulling Calibration. If you set this property, NI-DAQmx coerces Initial Bridge Ratio..."]
    Bridge_InitialVoltage,
    #[doc = "Specifies in volts per volt the ratio of output voltage from the bridge to excitation voltage supplied to the bridge while not under load. NI-DAQmx subtracts this value from any measurements before applying scaling equations. If you set Initial Bridge Voltage, NI-DAQmx coerces this property  to Initial Bridge Voltage divided by Actual Excitation Value. If you set this property, NI-DAQmx coerces Initial Bridge Volt..."]
    Bridge_InitialRatio,
    #[doc = "Specifies whether to enable a shunt calibration switch. Use Shunt Cal Select to select the switch(es) to enable."]
    Bridge_ShuntCal_Enable,
    #[doc = "Specifies which shunt calibration switch(es) to enable.  Use Shunt Cal Enable to enable the switch(es) you specify with this property."]
    Bridge_ShuntCal_Select,
    #[doc = "Specifies whether to use internal or external shunt when Shunt Cal A is selected."]
    Bridge_ShuntCal_ShuntCalASrc,
    #[doc = "Specifies the result of a shunt calibration. This property is set by DAQmx Perform Shunt Calibration. NI-DAQmx multiplies data read from the channel by the value of this property. This value should be close to 1.0."]
    Bridge_ShuntCal_GainAdjust,
    #[doc = "Specifies in ohms the desired value of the internal shunt calibration A resistor."]
    Bridge_ShuntCal_ShuntCalAResistance,
    #[doc = "Specifies in ohms the actual value of the internal shunt calibration A resistor."]
    Bridge_ShuntCal_ShuntCalAActualResistance,
    #[doc = "Specifies in ohms the desired value of the internal shunt calibration B resistor."]
    Bridge_ShuntCal_ShuntCalBResistance,
    #[doc = "Specifies in ohms the actual value of the internal shunt calibration B resistor."]
    Bridge_ShuntCal_ShuntCalBActualResistance,
    #[doc = "Specifies by how much to compensate for offset in the signal. This value can be between 0 and 127."]
    Bridge_Balance_CoarsePot,
    #[doc = "Specifies by how much to compensate for offset in the signal. This value can be between 0 and 4095."]
    Bridge_Balance_FinePot,
    #[doc = "Specifies the shunt resistor location for current measurements."]
    CurrentShunt_Loc,
    #[doc = "Specifies in ohms the external shunt resistance for current measurements."]
    CurrentShunt_Resistance,
    #[doc = "Specifies whether to use local or remote sense to sense excitation."]
    Excit_Sense,
    #[doc = "Specifies the source of excitation."]
    Excit_Src,
    #[doc = "Specifies the amount of excitation that the sensor requires. If Voltage or Current is  DAQmx_Val_Voltage, this value is in volts. If Voltage or Current is  DAQmx_Val_Current, this value is in amperes."]
    Excit_Val,
    #[doc = "Specifies if NI-DAQmx divides the measurement by the excitation. You should typically set this property to TRUE for ratiometric transducers. If you set this property to TRUE, set Maximum Value and Minimum Value to reflect the scaling."]
    Excit_UseForScaling,
    #[doc = "Specifies if the SCXI-1122 multiplexes the excitation to the upper half of the channels as it advances through the scan list."]
    Excit_UseMultiplexed,
    #[doc = "Specifies the actual amount of excitation supplied by an internal excitation source.  If you read an internal excitation source more precisely with an external device, set this property to the value you read.  NI-DAQmx ignores this value for external excitation. When performing shunt calibration, some devices set this property automatically."]
    Excit_ActualVal,
    #[doc = "Specifies if the excitation supply is DC or AC."]
    Excit_DCorAC,
    #[doc = "Specifies if the channel uses current or voltage excitation."]
    Excit_VoltageOrCurrent,
    #[doc = "Specifies whether this channel will disable excitation after the task is uncommitted. Setting this to Zero Volts or Amps disables excitation after task uncommit. Setting this attribute to Maintain Existing Value leaves the excitation on after task uncommit."]
    Excit_IdleOutputBehavior,
    #[doc = "Specifies the AC excitation frequency in Hertz."]
    ACExcit_Freq,
    #[doc = "Specifies whether to synchronize the AC excitation source of the channel to that of another channel. Synchronize the excitation sources of multiple channels to use multichannel sensors. Set this property to FALSE for the master channel and to TRUE for the slave channels."]
    ACExcit_SyncEnable,
    #[doc = "Specifies the number of leads on the LVDT or RVDT. Some sensors require you to tie leads together to create a four- or five- wire sensor. Refer to the sensor documentation for more information."]
    ACExcit_WireMode,
    #[doc = "Specifies the voltage level for the sensor's power supply."]
    SensorPower_Voltage,
    #[doc = "Specifies whether to turn on the sensor's power supply or to leave the configuration unchanged."]
    SensorPower_Cfg,
    #[doc = "Specifies the type of power supplied to the sensor."]
    SensorPower_Type,
    #[doc = "Specifies whether to apply the open thermocouple detection bias voltage to the channel. Changing the value of this property on a channel may require settling time before the data returned is valid. To compensate for this settling time, discard unsettled data or add a delay between committing and starting the task. Refer to your device specifications for the required settling time. When open thermocouple detection ..."]
    OpenThrmcplDetectEnable,
    #[doc = "Specifies the lead offset nulling voltage to subtract from measurements on a device. This property is ignored if open thermocouple detection is disabled."]
    Thrmcpl_LeadOffsetVoltage,
    #[doc = "Specifies the amount of attenuation to use."]
    Atten,
    #[doc = "Specifies the amount of attenuation provided by the probe connected to the channel. Specify this attenuation as a ratio."]
    ProbeAtten,
    #[doc = "Specifies whether to enable the lowpass filter of the channel."]
    Lowpass_Enable,
    #[doc = "Specifies the frequency in Hertz that corresponds to the -3dB cutoff of the filter."]
    Lowpass_CutoffFreq,
    #[doc = "Specifies the source of the filter clock. If you need a higher resolution for the filter, you can supply an external clock to increase the resolution. Refer to the SCXI-1141/1142/1143 User Manual for more information."]
    Lowpass_SwitchCap_ClkSrc,
    #[doc = "Specifies the frequency of the external clock when you set Clock Source to DAQmx_Val_External.  NI-DAQmx uses this frequency to set the pre- and post- filters on the SCXI-1141, SCXI-1142, and SCXI-1143. On those devices, NI-DAQmx determines the filter cutoff by using the equation f/(100*n), where f is the external frequency, and n is the external clock divisor. Refer to the SCXI-1141/1142/1143 User Manual for more..."]
    Lowpass_SwitchCap_ExtClkFreq,
    #[doc = "Specifies the divisor for the external clock when you set Clock Source to DAQmx_Val_External. On the SCXI-1141, SCXI-1142, and SCXI-1143, NI-DAQmx determines the filter cutoff by using the equation f/(100*n), where f is the external frequency, and n is the external clock divisor. Refer to the SCXI-1141/1142/1143 User Manual for more information."]
    Lowpass_SwitchCap_ExtClkDiv,
    #[doc = "Specifies the divisor for the output clock.  NI-DAQmx uses the cutoff frequency to determine the output clock frequency. Refer to the SCXI-1141/1142/1143 User Manual for more information."]
    Lowpass_SwitchCap_OutClkDiv,
    #[doc = "Specifies whether the digital filter is enabled or disabled."]
    DigFltr_Enable,
    #[doc = "Specifies the digital filter type."]
    DigFltr_Type,
    #[doc = "Specifies the digital filter response."]
    DigFltr_Response,
    #[doc = "Specifies the order of the digital filter."]
    DigFltr_Order,
    #[doc = "Specifies the lowpass cutoff frequency of the digital filter."]
    DigFltr_Lowpass_CutoffFreq,
    #[doc = "Specifies the highpass cutoff frequency of the digital filter."]
    DigFltr_Highpass_CutoffFreq,
    #[doc = "Specifies the center frequency of the passband for the digital filter."]
    DigFltr_Bandpass_CenterFreq,
    #[doc = "Specifies the width of the passband centered around the center frequency for the digital filter."]
    DigFltr_Bandpass_Width,
    #[doc = "Specifies the center frequency of the stopband for the digital filter."]
    DigFltr_Notch_CenterFreq,
    #[doc = "Specifies the width of the stopband centered around the center frequency for the digital filter."]
    DigFltr_Notch_Width,
    #[doc = "Specifies the digital filter coefficients."]
    DigFltr_Coeff,
    #[doc = "Specifies the corresponding filter enable/disable state."]
    Filter_Enable,
    #[doc = "Specifies the corresponding filter frequency (cutoff or center) of the filter response."]
    Filter_Freq,
    #[doc = "Specifies the corresponding filter response and defines the shape of the filter response."]
    Filter_Response,
    #[doc = "Specifies the corresponding filter order and defines the slope of the filter response."]
    Filter_Order,
    #[doc = "Indicates the amount of time between when the ADC samples data and when the sample is read by the host device. This value is in the units you specify with Filter Delay Units. You can adjust this amount of time using Filter Delay Adjustment."]
    FilterDelay,
    #[doc = "Specifies the units of Filter Delay and Filter Delay Adjustment."]
    FilterDelayUnits,
    #[doc = "Specifies if filter delay removal is enabled on the device."]
    RemoveFilterDelay,
    #[doc = "Specifies the amount of filter delay that gets removed if Remove Filter Delay is enabled. This delay adjustment is in addition to the value indicated by Filter Delay. This delay adjustment is in the units you specify with Filter Delay Units."]
    FilterDelayAdjustment,
    #[doc = "Specifies the number of samples to average while acquiring data. Increasing the number of samples to average reduces noise in your measurement."]
    AveragingWinSize,
    #[doc = "Indicates the units of Resolution Value."]
    ResolutionUnits,
    #[doc = "Indicates the resolution of the analog-to-digital converter of the channel. This value is in the units you specify with Resolution Units."]
    Resolution,
    #[doc = "Indicates in bits the size of a raw sample from the device."]
    RawSampSize,
    #[doc = "Indicates the justification of a raw sample from the device."]
    RawSampJustification,
    #[doc = "Specifies the ADC timing mode, controlling the tradeoff between speed and effective resolution. Some ADC timing modes provide increased powerline noise rejection. On devices that have an AI Convert clock, this setting affects both the maximum and default values for Rate. You must use the same ADC timing mode for all channels on a device, but you can use different ADC timing modes for different devices in the same ..."]
    ADCTimingMode,
    #[doc = "Specifies the timing mode of the ADC when Timing Mode is DAQmx_Val_Custom."]
    ADCCustomTimingMode,
    #[doc = "Specifies whether to enable dithering.  Dithering adds Gaussian noise to the input signal. You can use dithering to achieve higher resolution measurements by over sampling the input signal and averaging the results."]
    Dither_Enable,
    #[doc = "Indicates if the channel has calibration information."]
    ChanCal_HasValidCalInfo,
    #[doc = "Specifies whether to enable the channel calibration associated with the channel."]
    ChanCal_EnableCal,
    #[doc = "Specifies whether to apply the channel calibration to the channel after the expiration date has passed."]
    ChanCal_ApplyCalIfExp,
    #[doc = "Specifies the method or equation form that the calibration scale uses."]
    ChanCal_ScaleType,
    #[doc = "Specifies the reference values collected when calibrating the channel."]
    ChanCal_Table_PreScaledVals,
    #[doc = "Specifies the acquired values collected when calibrating the channel."]
    ChanCal_Table_ScaledVals,
    #[doc = "Specifies the forward polynomial values used for calibrating the channel."]
    ChanCal_Poly_ForwardCoeff,
    #[doc = "Specifies the reverse polynomial values used for calibrating the channel."]
    ChanCal_Poly_ReverseCoeff,
    #[doc = "Specifies the name of the operator who performed the channel calibration."]
    ChanCal_OperatorName,
    #[doc = "Specifies the description entered for the calibration of the channel."]
    ChanCal_Desc,
    #[doc = "Specifies the reference values collected when verifying the calibration. NI-DAQmx stores these values as a record of calibration accuracy and does not use them in the scaling process."]
    ChanCal_Verif_RefVals,
    #[doc = "Specifies the acquired values collected when verifying the calibration. NI-DAQmx stores these values as a record of calibration accuracy and does not use them in the scaling process."]
    ChanCal_Verif_AcqVals,
    #[doc = "Specifies the upper limit of the input range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
    Rng_High,
    #[doc = "Specifies the lower limit of the input range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
    Rng_Low,
    #[doc = "Specifies the DC value to add to the input range of the device. Use High and Low to specify the input range. This offset is in the native units of the device ."]
    DCOffset,
    #[doc = "Specifies a gain factor to apply to the channel."]
    Gain,
    #[doc = "Specifies whether to enable the sample and hold circuitry of the device. When you disable sample and hold circuitry, a small voltage offset might be introduced into the signal.  You can eliminate this offset by using Auto Zero Mode to perform an auto zero on the channel."]
    SampAndHold_Enable,
    #[doc = "Specifies how often to measure ground. NI-DAQmx subtracts the measured ground voltage from every sample."]
    AutoZeroMode,
    #[doc = "Specifies whether the device will chop its inputs. Chopping removes offset voltages and other low frequency errors."]
    ChopEnable,
    #[doc = "Specifies the rate in B/s to transfer data from the device. If this value is not set, then the device will transfer data at a rate based on the bus detected. Modify this value to affect performance under different combinations of operating system, configuration, and device."]
    DataXferMaxRate,
    #[doc = "Specifies the data transfer mode for the device."]
    DataXferMech,
    #[doc = "Specifies under what condition to transfer data from the onboard memory of the device to the buffer."]
    DataXferReqCond,
    #[doc = "Specifies the number of samples that must be in the FIFO to transfer data from the device if Data Transfer Request Condition is DAQmx_Val_OnbrdMemCustomThreshold."]
    DataXferCustomThreshold,
    #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqSize,
    #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqCount,
    #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
    MemMapEnable,
    #[doc = "Specifies the type of compression to apply to raw samples returned from the device."]
    RawDataCompressionType,
    #[doc = "Specifies the number of bits to return in a raw sample when Raw Data Compression Type is set to DAQmx_Val_LossyLSBRemoval."]
    LossyLSBRemoval_CompressedSampSize,
    #[doc = "Indicates the coefficients of a polynomial equation that NI-DAQmx uses to scale values from the native format of the device to volts. Each element of the array corresponds to a term of the equation. For example, if index two of the array is 4, the third term of the equation is 4x^2. Scaling coefficients do not account for any custom scales or sensors contained by the channel."]
    DevScalingCoeff,
    #[doc = "Specifies whether to enable enhanced alias rejection. Leave this property set to the default value for most applications."]
    EnhancedAliasRejectionEnable,
    #[doc = "Specifies whether to enable open channel detection."]
    OpenChanDetectEnable,
    #[doc = "Specifies the level of the upper limit for input limits detection. An input sample outside the upper and lower bounds causes a fault. Note: Fault detection applies to both positive and negative inputs. For instance, if you specify a lower limit of 2 mA and an upper limit of 12 mA, NI-DAQmx detects a fault at 15 mA and -15 mA, but not at -6 mA because it is in the range of -12 mA to -2 mA."]
    InputLimitsFaultDetect_UpperLimit,
    #[doc = "Specifies the level of the lower limit for input limits detection. An input sample outside the upper and lower bounds causes a fault. Note: Fault detection applies to both positive and negative inputs. For instance, if you specify a lower limit of 2 mA and an upper limit of 12 mA, NI-DAQmx detects a fault at 15 mA and -15 mA, but not at -6 mA because it is in the range of -12 mA to -2 mA."]
    InputLimitsFaultDetect_LowerLimit,
    #[doc = "Specifies whether to enable input limits fault detection."]
    InputLimitsFaultDetectEnable,
    #[doc = "Specifies whether to enable power supply fault detection."]
    PowerSupplyFaultDetectEnable,
    #[doc = "Specifies whether to enable overcurrent detection."]
    OvercurrentDetectEnable,
}

impl From<AnalogInput> for i32 {
    fn from(attr: AnalogInput) -> Self {
        match attr {
            nidaqmx_sys::AI_Max => AnalogInput::Max,
            nidaqmx_sys::AI_Min => AnalogInput::Min,
            nidaqmx_sys::AI_CustomScaleName => AnalogInput::CustomScaleName,
            nidaqmx_sys::AI_MeasType => AnalogInput::MeasType,
            nidaqmx_sys::AI_Voltage_Units => AnalogInput::Voltage_Units,
            nidaqmx_sys::AI_Voltage_dBRef => AnalogInput::Voltage_dBRef,
            nidaqmx_sys::AI_Voltage_ACRMS_Units => AnalogInput::Voltage_ACRMS_Units,
            nidaqmx_sys::AI_Temp_Units => AnalogInput::Temp_Units,
            nidaqmx_sys::AI_Thrmcpl_Type => AnalogInput::Thrmcpl_Type,
            nidaqmx_sys::AI_Thrmcpl_ScaleType => AnalogInput::Thrmcpl_ScaleType,
            nidaqmx_sys::AI_Thrmcpl_CJCSrc => AnalogInput::Thrmcpl_CJCSrc,
            nidaqmx_sys::AI_Thrmcpl_CJCVal => AnalogInput::Thrmcpl_CJCVal,
            nidaqmx_sys::AI_Thrmcpl_CJCChan => AnalogInput::Thrmcpl_CJCChan,
            nidaqmx_sys::AI_RTD_Type => AnalogInput::RTD_Type,
            nidaqmx_sys::AI_RTD_R0 => AnalogInput::RTD_R0,
            nidaqmx_sys::AI_RTD_A => AnalogInput::RTD_A,
            nidaqmx_sys::AI_RTD_B => AnalogInput::RTD_B,
            nidaqmx_sys::AI_RTD_C => AnalogInput::RTD_C,
            nidaqmx_sys::AI_Thrmstr_A => AnalogInput::Thrmstr_A,
            nidaqmx_sys::AI_Thrmstr_B => AnalogInput::Thrmstr_B,
            nidaqmx_sys::AI_Thrmstr_C => AnalogInput::Thrmstr_C,
            nidaqmx_sys::AI_Thrmstr_R1 => AnalogInput::Thrmstr_R1,
            nidaqmx_sys::AI_ForceReadFromChan => AnalogInput::ForceReadFromChan,
            nidaqmx_sys::AI_Current_Units => AnalogInput::Current_Units,
            nidaqmx_sys::AI_Current_ACRMS_Units => AnalogInput::Current_ACRMS_Units,
            nidaqmx_sys::AI_Strain_Units => AnalogInput::Strain_Units,
            nidaqmx_sys::AI_StrainGage_ForceReadFromChan => {
                AnalogInput::StrainGage_ForceReadFromChan
            }
            nidaqmx_sys::AI_StrainGage_GageFactor => AnalogInput::StrainGage_GageFactor,
            nidaqmx_sys::AI_StrainGage_PoissonRatio => AnalogInput::StrainGage_PoissonRatio,
            nidaqmx_sys::AI_StrainGage_Cfg => AnalogInput::StrainGage_Cfg,
            nidaqmx_sys::AI_RosetteStrainGage_RosetteType => {
                AnalogInput::RosetteStrainGage_RosetteType
            }
            nidaqmx_sys::AI_RosetteStrainGage_Orientation => {
                AnalogInput::RosetteStrainGage_Orientation
            }
            nidaqmx_sys::AI_RosetteStrainGage_StrainChans => {
                AnalogInput::RosetteStrainGage_StrainChans
            }
            nidaqmx_sys::AI_RosetteStrainGage_RosetteMeasType => {
                AnalogInput::RosetteStrainGage_RosetteMeasType
            }
            nidaqmx_sys::AI_Resistance_Units => AnalogInput::Resistance_Units,
            nidaqmx_sys::AI_Freq_Units => AnalogInput::Freq_Units,
            nidaqmx_sys::AI_Freq_ThreshVoltage => AnalogInput::Freq_ThreshVoltage,
            nidaqmx_sys::AI_Freq_Hyst => AnalogInput::Freq_Hyst,
            nidaqmx_sys::AI_LVDT_Units => AnalogInput::LVDT_Units,
            nidaqmx_sys::AI_LVDT_Sensitivity => AnalogInput::LVDT_Sensitivity,
            nidaqmx_sys::AI_LVDT_SensitivityUnits => AnalogInput::LVDT_SensitivityUnits,
            nidaqmx_sys::AI_RVDT_Units => AnalogInput::RVDT_Units,
            nidaqmx_sys::AI_RVDT_Sensitivity => AnalogInput::RVDT_Sensitivity,
            nidaqmx_sys::AI_RVDT_SensitivityUnits => AnalogInput::RVDT_SensitivityUnits,
            nidaqmx_sys::AI_EddyCurrentProxProbe_Units => AnalogInput::EddyCurrentProxProbe_Units,
            nidaqmx_sys::AI_EddyCurrentProxProbe_Sensitivity => {
                AnalogInput::EddyCurrentProxProbe_Sensitivity
            }
            nidaqmx_sys::AI_EddyCurrentProxProbe_SensitivityUnits => {
                AnalogInput::EddyCurrentProxProbe_SensitivityUnits
            }
            nidaqmx_sys::AI_SoundPressure_MaxSoundPressureLvl => {
                AnalogInput::SoundPressure_MaxSoundPressureLvl
            }
            nidaqmx_sys::AI_SoundPressure_Units => AnalogInput::SoundPressure_Units,
            nidaqmx_sys::AI_SoundPressure_dBRef => AnalogInput::SoundPressure_dBRef,
            nidaqmx_sys::AI_Microphone_Sensitivity => AnalogInput::Microphone_Sensitivity,
            nidaqmx_sys::AI_Accel_Units => AnalogInput::Accel_Units,
            nidaqmx_sys::AI_Accel_dBRef => AnalogInput::Accel_dBRef,
            nidaqmx_sys::AI_Accel_4WireDCVoltage_Sensitivity => {
                AnalogInput::Accel_4WireDCVoltage_Sensitivity
            }
            nidaqmx_sys::AI_Accel_4WireDCVoltage_SensitivityUnits => {
                AnalogInput::Accel_4WireDCVoltage_SensitivityUnits
            }
            nidaqmx_sys::AI_Accel_Sensitivity => AnalogInput::Accel_Sensitivity,
            nidaqmx_sys::AI_Accel_SensitivityUnits => AnalogInput::Accel_SensitivityUnits,
            nidaqmx_sys::AI_Accel_Charge_Sensitivity => AnalogInput::Accel_Charge_Sensitivity,
            nidaqmx_sys::AI_Accel_Charge_SensitivityUnits => {
                AnalogInput::Accel_Charge_SensitivityUnits
            }
            nidaqmx_sys::AI_Velocity_Units => AnalogInput::Velocity_Units,
            nidaqmx_sys::AI_Velocity_IEPESensor_dBRef => AnalogInput::Velocity_IEPESensor_dBRef,
            nidaqmx_sys::AI_Velocity_IEPESensor_Sensitivity => {
                AnalogInput::Velocity_IEPESensor_Sensitivity
            }
            nidaqmx_sys::AI_Velocity_IEPESensor_SensitivityUnits => {
                AnalogInput::Velocity_IEPESensor_SensitivityUnits
            }
            nidaqmx_sys::AI_Force_Units => AnalogInput::Force_Units,
            nidaqmx_sys::AI_Force_IEPESensor_Sensitivity => {
                AnalogInput::Force_IEPESensor_Sensitivity
            }
            nidaqmx_sys::AI_Force_IEPESensor_SensitivityUnits => {
                AnalogInput::Force_IEPESensor_SensitivityUnits
            }
            nidaqmx_sys::AI_Pressure_Units => AnalogInput::Pressure_Units,
            nidaqmx_sys::AI_Torque_Units => AnalogInput::Torque_Units,
            nidaqmx_sys::AI_Bridge_Units => AnalogInput::Bridge_Units,
            nidaqmx_sys::AI_Bridge_ElectricalUnits => AnalogInput::Bridge_ElectricalUnits,
            nidaqmx_sys::AI_Bridge_PhysicalUnits => AnalogInput::Bridge_PhysicalUnits,
            nidaqmx_sys::AI_Bridge_ScaleType => AnalogInput::Bridge_ScaleType,
            nidaqmx_sys::AI_Bridge_TwoPointLin_First_ElectricalVal => {
                AnalogInput::Bridge_TwoPointLin_First_ElectricalVal
            }
            nidaqmx_sys::AI_Bridge_TwoPointLin_First_PhysicalVal => {
                AnalogInput::Bridge_TwoPointLin_First_PhysicalVal
            }
            nidaqmx_sys::AI_Bridge_TwoPointLin_Second_ElectricalVal => {
                AnalogInput::Bridge_TwoPointLin_Second_ElectricalVal
            }
            nidaqmx_sys::AI_Bridge_TwoPointLin_Second_PhysicalVal => {
                AnalogInput::Bridge_TwoPointLin_Second_PhysicalVal
            }
            nidaqmx_sys::AI_Bridge_Table_ElectricalVals => AnalogInput::Bridge_Table_ElectricalVals,
            nidaqmx_sys::AI_Bridge_Table_PhysicalVals => AnalogInput::Bridge_Table_PhysicalVals,
            nidaqmx_sys::AI_Bridge_Poly_ForwardCoeff => AnalogInput::Bridge_Poly_ForwardCoeff,
            nidaqmx_sys::AI_Bridge_Poly_ReverseCoeff => AnalogInput::Bridge_Poly_ReverseCoeff,
            nidaqmx_sys::AI_Charge_Units => AnalogInput::Charge_Units,
            nidaqmx_sys::AI_Is_TEDS => AnalogInput::Is_TEDS,
            nidaqmx_sys::AI_TEDS_Units => AnalogInput::TEDS_Units,
            nidaqmx_sys::AI_Coupling => AnalogInput::Coupling,
            nidaqmx_sys::AI_Impedance => AnalogInput::Impedance,
            nidaqmx_sys::AI_TermCfg => AnalogInput::TermCfg,
            nidaqmx_sys::AI_InputSrc => AnalogInput::InputSrc,
            nidaqmx_sys::AI_ResistanceCfg => AnalogInput::ResistanceCfg,
            nidaqmx_sys::AI_LeadWireResistance => AnalogInput::LeadWireResistance,
            nidaqmx_sys::AI_Bridge_Cfg => AnalogInput::Bridge_Cfg,
            nidaqmx_sys::AI_Bridge_NomResistance => AnalogInput::Bridge_NomResistance,
            nidaqmx_sys::AI_Bridge_InitialVoltage => AnalogInput::Bridge_InitialVoltage,
            nidaqmx_sys::AI_Bridge_InitialRatio => AnalogInput::Bridge_InitialRatio,
            nidaqmx_sys::AI_Bridge_ShuntCal_Enable => AnalogInput::Bridge_ShuntCal_Enable,
            nidaqmx_sys::AI_Bridge_ShuntCal_Select => AnalogInput::Bridge_ShuntCal_Select,
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalASrc => {
                AnalogInput::Bridge_ShuntCal_ShuntCalASrc
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_GainAdjust => AnalogInput::Bridge_ShuntCal_GainAdjust,
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalAResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalAResistance
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalAActualResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalAActualResistance
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalBResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalBResistance
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalBActualResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalBActualResistance
            }
            nidaqmx_sys::AI_Bridge_Balance_CoarsePot => AnalogInput::Bridge_Balance_CoarsePot,
            nidaqmx_sys::AI_Bridge_Balance_FinePot => AnalogInput::Bridge_Balance_FinePot,
            nidaqmx_sys::AI_CurrentShunt_Loc => AnalogInput::CurrentShunt_Loc,
            nidaqmx_sys::AI_CurrentShunt_Resistance => AnalogInput::CurrentShunt_Resistance,
            nidaqmx_sys::AI_Excit_Sense => AnalogInput::Excit_Sense,
            nidaqmx_sys::AI_Excit_Src => AnalogInput::Excit_Src,
            nidaqmx_sys::AI_Excit_Val => AnalogInput::Excit_Val,
            nidaqmx_sys::AI_Excit_UseForScaling => AnalogInput::Excit_UseForScaling,
            nidaqmx_sys::AI_Excit_UseMultiplexed => AnalogInput::Excit_UseMultiplexed,
            nidaqmx_sys::AI_Excit_ActualVal => AnalogInput::Excit_ActualVal,
            nidaqmx_sys::AI_Excit_DCorAC => AnalogInput::Excit_DCorAC,
            nidaqmx_sys::AI_Excit_VoltageOrCurrent => AnalogInput::Excit_VoltageOrCurrent,
            nidaqmx_sys::AI_Excit_IdleOutputBehavior => AnalogInput::Excit_IdleOutputBehavior,
            nidaqmx_sys::AI_ACExcit_Freq => AnalogInput::ACExcit_Freq,
            nidaqmx_sys::AI_ACExcit_SyncEnable => AnalogInput::ACExcit_SyncEnable,
            nidaqmx_sys::AI_ACExcit_WireMode => AnalogInput::ACExcit_WireMode,
            nidaqmx_sys::AI_SensorPower_Voltage => AnalogInput::SensorPower_Voltage,
            nidaqmx_sys::AI_SensorPower_Cfg => AnalogInput::SensorPower_Cfg,
            nidaqmx_sys::AI_SensorPower_Type => AnalogInput::SensorPower_Type,
            nidaqmx_sys::AI_OpenThrmcplDetectEnable => AnalogInput::OpenThrmcplDetectEnable,
            nidaqmx_sys::AI_Thrmcpl_LeadOffsetVoltage => AnalogInput::Thrmcpl_LeadOffsetVoltage,
            nidaqmx_sys::AI_Atten => AnalogInput::Atten,
            nidaqmx_sys::AI_ProbeAtten => AnalogInput::ProbeAtten,
            nidaqmx_sys::AI_Lowpass_Enable => AnalogInput::Lowpass_Enable,
            nidaqmx_sys::AI_Lowpass_CutoffFreq => AnalogInput::Lowpass_CutoffFreq,
            nidaqmx_sys::AI_Lowpass_SwitchCap_ClkSrc => AnalogInput::Lowpass_SwitchCap_ClkSrc,
            nidaqmx_sys::AI_Lowpass_SwitchCap_ExtClkFreq => {
                AnalogInput::Lowpass_SwitchCap_ExtClkFreq
            }
            nidaqmx_sys::AI_Lowpass_SwitchCap_ExtClkDiv => AnalogInput::Lowpass_SwitchCap_ExtClkDiv,
            nidaqmx_sys::AI_Lowpass_SwitchCap_OutClkDiv => AnalogInput::Lowpass_SwitchCap_OutClkDiv,
            nidaqmx_sys::AI_DigFltr_Enable => AnalogInput::DigFltr_Enable,
            nidaqmx_sys::AI_DigFltr_Type => AnalogInput::DigFltr_Type,
            nidaqmx_sys::AI_DigFltr_Response => AnalogInput::DigFltr_Response,
            nidaqmx_sys::AI_DigFltr_Order => AnalogInput::DigFltr_Order,
            nidaqmx_sys::AI_DigFltr_Lowpass_CutoffFreq => AnalogInput::DigFltr_Lowpass_CutoffFreq,
            nidaqmx_sys::AI_DigFltr_Highpass_CutoffFreq => AnalogInput::DigFltr_Highpass_CutoffFreq,
            nidaqmx_sys::AI_DigFltr_Bandpass_CenterFreq => AnalogInput::DigFltr_Bandpass_CenterFreq,
            nidaqmx_sys::AI_DigFltr_Bandpass_Width => AnalogInput::DigFltr_Bandpass_Width,
            nidaqmx_sys::AI_DigFltr_Notch_CenterFreq => AnalogInput::DigFltr_Notch_CenterFreq,
            nidaqmx_sys::AI_DigFltr_Notch_Width => AnalogInput::DigFltr_Notch_Width,
            nidaqmx_sys::AI_DigFltr_Coeff => AnalogInput::DigFltr_Coeff,
            nidaqmx_sys::AI_Filter_Enable => AnalogInput::Filter_Enable,
            nidaqmx_sys::AI_Filter_Freq => AnalogInput::Filter_Freq,
            nidaqmx_sys::AI_Filter_Response => AnalogInput::Filter_Response,
            nidaqmx_sys::AI_Filter_Order => AnalogInput::Filter_Order,
            nidaqmx_sys::AI_FilterDelay => AnalogInput::FilterDelay,
            nidaqmx_sys::AI_FilterDelayUnits => AnalogInput::FilterDelayUnits,
            nidaqmx_sys::AI_RemoveFilterDelay => AnalogInput::RemoveFilterDelay,
            nidaqmx_sys::AI_FilterDelayAdjustment => AnalogInput::FilterDelayAdjustment,
            nidaqmx_sys::AI_AveragingWinSize => AnalogInput::AveragingWinSize,
            nidaqmx_sys::AI_ResolutionUnits => AnalogInput::ResolutionUnits,
            nidaqmx_sys::AI_Resolution => AnalogInput::Resolution,
            nidaqmx_sys::AI_RawSampSize => AnalogInput::RawSampSize,
            nidaqmx_sys::AI_RawSampJustification => AnalogInput::RawSampJustification,
            nidaqmx_sys::AI_ADCTimingMode => AnalogInput::ADCTimingMode,
            nidaqmx_sys::AI_ADCCustomTimingMode => AnalogInput::ADCCustomTimingMode,
            nidaqmx_sys::AI_Dither_Enable => AnalogInput::Dither_Enable,
            nidaqmx_sys::AI_ChanCal_HasValidCalInfo => AnalogInput::ChanCal_HasValidCalInfo,
            nidaqmx_sys::AI_ChanCal_EnableCal => AnalogInput::ChanCal_EnableCal,
            nidaqmx_sys::AI_ChanCal_ApplyCalIfExp => AnalogInput::ChanCal_ApplyCalIfExp,
            nidaqmx_sys::AI_ChanCal_ScaleType => AnalogInput::ChanCal_ScaleType,
            nidaqmx_sys::AI_ChanCal_Table_PreScaledVals => AnalogInput::ChanCal_Table_PreScaledVals,
            nidaqmx_sys::AI_ChanCal_Table_ScaledVals => AnalogInput::ChanCal_Table_ScaledVals,
            nidaqmx_sys::AI_ChanCal_Poly_ForwardCoeff => AnalogInput::ChanCal_Poly_ForwardCoeff,
            nidaqmx_sys::AI_ChanCal_Poly_ReverseCoeff => AnalogInput::ChanCal_Poly_ReverseCoeff,
            nidaqmx_sys::AI_ChanCal_OperatorName => AnalogInput::ChanCal_OperatorName,
            nidaqmx_sys::AI_ChanCal_Desc => AnalogInput::ChanCal_Desc,
            nidaqmx_sys::AI_ChanCal_Verif_RefVals => AnalogInput::ChanCal_Verif_RefVals,
            nidaqmx_sys::AI_ChanCal_Verif_AcqVals => AnalogInput::ChanCal_Verif_AcqVals,
            nidaqmx_sys::AI_Rng_High => AnalogInput::Rng_High,
            nidaqmx_sys::AI_Rng_Low => AnalogInput::Rng_Low,
            nidaqmx_sys::AI_DCOffset => AnalogInput::DCOffset,
            nidaqmx_sys::AI_Gain => AnalogInput::Gain,
            nidaqmx_sys::AI_SampAndHold_Enable => AnalogInput::SampAndHold_Enable,
            nidaqmx_sys::AI_AutoZeroMode => AnalogInput::AutoZeroMode,
            nidaqmx_sys::AI_ChopEnable => AnalogInput::ChopEnable,
            nidaqmx_sys::AI_DataXferMaxRate => AnalogInput::DataXferMaxRate,
            nidaqmx_sys::AI_DataXferMech => AnalogInput::DataXferMech,
            nidaqmx_sys::AI_DataXferReqCond => AnalogInput::DataXferReqCond,
            nidaqmx_sys::AI_DataXferCustomThreshold => AnalogInput::DataXferCustomThreshold,
            nidaqmx_sys::AI_UsbXferReqSize => AnalogInput::UsbXferReqSize,
            nidaqmx_sys::AI_UsbXferReqCount => AnalogInput::UsbXferReqCount,
            nidaqmx_sys::AI_MemMapEnable => AnalogInput::MemMapEnable,
            nidaqmx_sys::AI_RawDataCompressionType => AnalogInput::RawDataCompressionType,
            nidaqmx_sys::AI_LossyLSBRemoval_CompressedSampSize => {
                AnalogInput::LossyLSBRemoval_CompressedSampSize
            }
            nidaqmx_sys::AI_DevScalingCoeff => AnalogInput::DevScalingCoeff,
            nidaqmx_sys::AI_EnhancedAliasRejectionEnable => {
                AnalogInput::EnhancedAliasRejectionEnable
            }
            nidaqmx_sys::AI_OpenChanDetectEnable => AnalogInput::OpenChanDetectEnable,
            nidaqmx_sys::AI_InputLimitsFaultDetect_UpperLimit => {
                AnalogInput::InputLimitsFaultDetect_UpperLimit
            }
            nidaqmx_sys::AI_InputLimitsFaultDetect_LowerLimit => {
                AnalogInput::InputLimitsFaultDetect_LowerLimit
            }
            nidaqmx_sys::AI_InputLimitsFaultDetectEnable => {
                AnalogInput::InputLimitsFaultDetectEnable
            }
            nidaqmx_sys::AI_PowerSupplyFaultDetectEnable => {
                AnalogInput::PowerSupplyFaultDetectEnable
            }
            nidaqmx_sys::AI_OvercurrentDetectEnable => AnalogInput::OvercurrentDetectEnable,
        }
    }
}

impl From<i32> for AnalogInput {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::AI_Max => AnalogInput::Max,
            nidaqmx_sys::AI_Min => AnalogInput::Min,
            nidaqmx_sys::AI_CustomScaleName => AnalogInput::CustomScaleName,
            nidaqmx_sys::AI_MeasType => AnalogInput::MeasType,
            nidaqmx_sys::AI_Voltage_Units => AnalogInput::Voltage_Units,
            nidaqmx_sys::AI_Voltage_dBRef => AnalogInput::Voltage_dBRef,
            nidaqmx_sys::AI_Voltage_ACRMS_Units => AnalogInput::Voltage_ACRMS_Units,
            nidaqmx_sys::AI_Temp_Units => AnalogInput::Temp_Units,
            nidaqmx_sys::AI_Thrmcpl_Type => AnalogInput::Thrmcpl_Type,
            nidaqmx_sys::AI_Thrmcpl_ScaleType => AnalogInput::Thrmcpl_ScaleType,
            nidaqmx_sys::AI_Thrmcpl_CJCSrc => AnalogInput::Thrmcpl_CJCSrc,
            nidaqmx_sys::AI_Thrmcpl_CJCVal => AnalogInput::Thrmcpl_CJCVal,
            nidaqmx_sys::AI_Thrmcpl_CJCChan => AnalogInput::Thrmcpl_CJCChan,
            nidaqmx_sys::AI_RTD_Type => AnalogInput::RTD_Type,
            nidaqmx_sys::AI_RTD_R0 => AnalogInput::RTD_R0,
            nidaqmx_sys::AI_RTD_A => AnalogInput::RTD_A,
            nidaqmx_sys::AI_RTD_B => AnalogInput::RTD_B,
            nidaqmx_sys::AI_RTD_C => AnalogInput::RTD_C,
            nidaqmx_sys::AI_Thrmstr_A => AnalogInput::Thrmstr_A,
            nidaqmx_sys::AI_Thrmstr_B => AnalogInput::Thrmstr_B,
            nidaqmx_sys::AI_Thrmstr_C => AnalogInput::Thrmstr_C,
            nidaqmx_sys::AI_Thrmstr_R1 => AnalogInput::Thrmstr_R1,
            nidaqmx_sys::AI_ForceReadFromChan => AnalogInput::ForceReadFromChan,
            nidaqmx_sys::AI_Current_Units => AnalogInput::Current_Units,
            nidaqmx_sys::AI_Current_ACRMS_Units => AnalogInput::Current_ACRMS_Units,
            nidaqmx_sys::AI_Strain_Units => AnalogInput::Strain_Units,
            nidaqmx_sys::AI_StrainGage_ForceReadFromChan => {
                AnalogInput::StrainGage_ForceReadFromChan
            }
            nidaqmx_sys::AI_StrainGage_GageFactor => AnalogInput::StrainGage_GageFactor,
            nidaqmx_sys::AI_StrainGage_PoissonRatio => AnalogInput::StrainGage_PoissonRatio,
            nidaqmx_sys::AI_StrainGage_Cfg => AnalogInput::StrainGage_Cfg,
            nidaqmx_sys::AI_RosetteStrainGage_RosetteType => {
                AnalogInput::RosetteStrainGage_RosetteType
            }
            nidaqmx_sys::AI_RosetteStrainGage_Orientation => {
                AnalogInput::RosetteStrainGage_Orientation
            }
            nidaqmx_sys::AI_RosetteStrainGage_StrainChans => {
                AnalogInput::RosetteStrainGage_StrainChans
            }
            nidaqmx_sys::AI_RosetteStrainGage_RosetteMeasType => {
                AnalogInput::RosetteStrainGage_RosetteMeasType
            }
            nidaqmx_sys::AI_Resistance_Units => AnalogInput::Resistance_Units,
            nidaqmx_sys::AI_Freq_Units => AnalogInput::Freq_Units,
            nidaqmx_sys::AI_Freq_ThreshVoltage => AnalogInput::Freq_ThreshVoltage,
            nidaqmx_sys::AI_Freq_Hyst => AnalogInput::Freq_Hyst,
            nidaqmx_sys::AI_LVDT_Units => AnalogInput::LVDT_Units,
            nidaqmx_sys::AI_LVDT_Sensitivity => AnalogInput::LVDT_Sensitivity,
            nidaqmx_sys::AI_LVDT_SensitivityUnits => AnalogInput::LVDT_SensitivityUnits,
            nidaqmx_sys::AI_RVDT_Units => AnalogInput::RVDT_Units,
            nidaqmx_sys::AI_RVDT_Sensitivity => AnalogInput::RVDT_Sensitivity,
            nidaqmx_sys::AI_RVDT_SensitivityUnits => AnalogInput::RVDT_SensitivityUnits,
            nidaqmx_sys::AI_EddyCurrentProxProbe_Units => AnalogInput::EddyCurrentProxProbe_Units,
            nidaqmx_sys::AI_EddyCurrentProxProbe_Sensitivity => {
                AnalogInput::EddyCurrentProxProbe_Sensitivity
            }
            nidaqmx_sys::AI_EddyCurrentProxProbe_SensitivityUnits => {
                AnalogInput::EddyCurrentProxProbe_SensitivityUnits
            }
            nidaqmx_sys::AI_SoundPressure_MaxSoundPressureLvl => {
                AnalogInput::SoundPressure_MaxSoundPressureLvl
            }
            nidaqmx_sys::AI_SoundPressure_Units => AnalogInput::SoundPressure_Units,
            nidaqmx_sys::AI_SoundPressure_dBRef => AnalogInput::SoundPressure_dBRef,
            nidaqmx_sys::AI_Microphone_Sensitivity => AnalogInput::Microphone_Sensitivity,
            nidaqmx_sys::AI_Accel_Units => AnalogInput::Accel_Units,
            nidaqmx_sys::AI_Accel_dBRef => AnalogInput::Accel_dBRef,
            nidaqmx_sys::AI_Accel_4WireDCVoltage_Sensitivity => {
                AnalogInput::Accel_4WireDCVoltage_Sensitivity
            }
            nidaqmx_sys::AI_Accel_4WireDCVoltage_SensitivityUnits => {
                AnalogInput::Accel_4WireDCVoltage_SensitivityUnits
            }
            nidaqmx_sys::AI_Accel_Sensitivity => AnalogInput::Accel_Sensitivity,
            nidaqmx_sys::AI_Accel_SensitivityUnits => AnalogInput::Accel_SensitivityUnits,
            nidaqmx_sys::AI_Accel_Charge_Sensitivity => AnalogInput::Accel_Charge_Sensitivity,
            nidaqmx_sys::AI_Accel_Charge_SensitivityUnits => {
                AnalogInput::Accel_Charge_SensitivityUnits
            }
            nidaqmx_sys::AI_Velocity_Units => AnalogInput::Velocity_Units,
            nidaqmx_sys::AI_Velocity_IEPESensor_dBRef => AnalogInput::Velocity_IEPESensor_dBRef,
            nidaqmx_sys::AI_Velocity_IEPESensor_Sensitivity => {
                AnalogInput::Velocity_IEPESensor_Sensitivity
            }
            nidaqmx_sys::AI_Velocity_IEPESensor_SensitivityUnits => {
                AnalogInput::Velocity_IEPESensor_SensitivityUnits
            }
            nidaqmx_sys::AI_Force_Units => AnalogInput::Force_Units,
            nidaqmx_sys::AI_Force_IEPESensor_Sensitivity => {
                AnalogInput::Force_IEPESensor_Sensitivity
            }
            nidaqmx_sys::AI_Force_IEPESensor_SensitivityUnits => {
                AnalogInput::Force_IEPESensor_SensitivityUnits
            }
            nidaqmx_sys::AI_Pressure_Units => AnalogInput::Pressure_Units,
            nidaqmx_sys::AI_Torque_Units => AnalogInput::Torque_Units,
            nidaqmx_sys::AI_Bridge_Units => AnalogInput::Bridge_Units,
            nidaqmx_sys::AI_Bridge_ElectricalUnits => AnalogInput::Bridge_ElectricalUnits,
            nidaqmx_sys::AI_Bridge_PhysicalUnits => AnalogInput::Bridge_PhysicalUnits,
            nidaqmx_sys::AI_Bridge_ScaleType => AnalogInput::Bridge_ScaleType,
            nidaqmx_sys::AI_Bridge_TwoPointLin_First_ElectricalVal => {
                AnalogInput::Bridge_TwoPointLin_First_ElectricalVal
            }
            nidaqmx_sys::AI_Bridge_TwoPointLin_First_PhysicalVal => {
                AnalogInput::Bridge_TwoPointLin_First_PhysicalVal
            }
            nidaqmx_sys::AI_Bridge_TwoPointLin_Second_ElectricalVal => {
                AnalogInput::Bridge_TwoPointLin_Second_ElectricalVal
            }
            nidaqmx_sys::AI_Bridge_TwoPointLin_Second_PhysicalVal => {
                AnalogInput::Bridge_TwoPointLin_Second_PhysicalVal
            }
            nidaqmx_sys::AI_Bridge_Table_ElectricalVals => AnalogInput::Bridge_Table_ElectricalVals,
            nidaqmx_sys::AI_Bridge_Table_PhysicalVals => AnalogInput::Bridge_Table_PhysicalVals,
            nidaqmx_sys::AI_Bridge_Poly_ForwardCoeff => AnalogInput::Bridge_Poly_ForwardCoeff,
            nidaqmx_sys::AI_Bridge_Poly_ReverseCoeff => AnalogInput::Bridge_Poly_ReverseCoeff,
            nidaqmx_sys::AI_Charge_Units => AnalogInput::Charge_Units,
            nidaqmx_sys::AI_Is_TEDS => AnalogInput::Is_TEDS,
            nidaqmx_sys::AI_TEDS_Units => AnalogInput::TEDS_Units,
            nidaqmx_sys::AI_Coupling => AnalogInput::Coupling,
            nidaqmx_sys::AI_Impedance => AnalogInput::Impedance,
            nidaqmx_sys::AI_TermCfg => AnalogInput::TermCfg,
            nidaqmx_sys::AI_InputSrc => AnalogInput::InputSrc,
            nidaqmx_sys::AI_ResistanceCfg => AnalogInput::ResistanceCfg,
            nidaqmx_sys::AI_LeadWireResistance => AnalogInput::LeadWireResistance,
            nidaqmx_sys::AI_Bridge_Cfg => AnalogInput::Bridge_Cfg,
            nidaqmx_sys::AI_Bridge_NomResistance => AnalogInput::Bridge_NomResistance,
            nidaqmx_sys::AI_Bridge_InitialVoltage => AnalogInput::Bridge_InitialVoltage,
            nidaqmx_sys::AI_Bridge_InitialRatio => AnalogInput::Bridge_InitialRatio,
            nidaqmx_sys::AI_Bridge_ShuntCal_Enable => AnalogInput::Bridge_ShuntCal_Enable,
            nidaqmx_sys::AI_Bridge_ShuntCal_Select => AnalogInput::Bridge_ShuntCal_Select,
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalASrc => {
                AnalogInput::Bridge_ShuntCal_ShuntCalASrc
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_GainAdjust => AnalogInput::Bridge_ShuntCal_GainAdjust,
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalAResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalAResistance
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalAActualResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalAActualResistance
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalBResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalBResistance
            }
            nidaqmx_sys::AI_Bridge_ShuntCal_ShuntCalBActualResistance => {
                AnalogInput::Bridge_ShuntCal_ShuntCalBActualResistance
            }
            nidaqmx_sys::AI_Bridge_Balance_CoarsePot => AnalogInput::Bridge_Balance_CoarsePot,
            nidaqmx_sys::AI_Bridge_Balance_FinePot => AnalogInput::Bridge_Balance_FinePot,
            nidaqmx_sys::AI_CurrentShunt_Loc => AnalogInput::CurrentShunt_Loc,
            nidaqmx_sys::AI_CurrentShunt_Resistance => AnalogInput::CurrentShunt_Resistance,
            nidaqmx_sys::AI_Excit_Sense => AnalogInput::Excit_Sense,
            nidaqmx_sys::AI_Excit_Src => AnalogInput::Excit_Src,
            nidaqmx_sys::AI_Excit_Val => AnalogInput::Excit_Val,
            nidaqmx_sys::AI_Excit_UseForScaling => AnalogInput::Excit_UseForScaling,
            nidaqmx_sys::AI_Excit_UseMultiplexed => AnalogInput::Excit_UseMultiplexed,
            nidaqmx_sys::AI_Excit_ActualVal => AnalogInput::Excit_ActualVal,
            nidaqmx_sys::AI_Excit_DCorAC => AnalogInput::Excit_DCorAC,
            nidaqmx_sys::AI_Excit_VoltageOrCurrent => AnalogInput::Excit_VoltageOrCurrent,
            nidaqmx_sys::AI_Excit_IdleOutputBehavior => AnalogInput::Excit_IdleOutputBehavior,
            nidaqmx_sys::AI_ACExcit_Freq => AnalogInput::ACExcit_Freq,
            nidaqmx_sys::AI_ACExcit_SyncEnable => AnalogInput::ACExcit_SyncEnable,
            nidaqmx_sys::AI_ACExcit_WireMode => AnalogInput::ACExcit_WireMode,
            nidaqmx_sys::AI_SensorPower_Voltage => AnalogInput::SensorPower_Voltage,
            nidaqmx_sys::AI_SensorPower_Cfg => AnalogInput::SensorPower_Cfg,
            nidaqmx_sys::AI_SensorPower_Type => AnalogInput::SensorPower_Type,
            nidaqmx_sys::AI_OpenThrmcplDetectEnable => AnalogInput::OpenThrmcplDetectEnable,
            nidaqmx_sys::AI_Thrmcpl_LeadOffsetVoltage => AnalogInput::Thrmcpl_LeadOffsetVoltage,
            nidaqmx_sys::AI_Atten => AnalogInput::Atten,
            nidaqmx_sys::AI_ProbeAtten => AnalogInput::ProbeAtten,
            nidaqmx_sys::AI_Lowpass_Enable => AnalogInput::Lowpass_Enable,
            nidaqmx_sys::AI_Lowpass_CutoffFreq => AnalogInput::Lowpass_CutoffFreq,
            nidaqmx_sys::AI_Lowpass_SwitchCap_ClkSrc => AnalogInput::Lowpass_SwitchCap_ClkSrc,
            nidaqmx_sys::AI_Lowpass_SwitchCap_ExtClkFreq => {
                AnalogInput::Lowpass_SwitchCap_ExtClkFreq
            }
            nidaqmx_sys::AI_Lowpass_SwitchCap_ExtClkDiv => AnalogInput::Lowpass_SwitchCap_ExtClkDiv,
            nidaqmx_sys::AI_Lowpass_SwitchCap_OutClkDiv => AnalogInput::Lowpass_SwitchCap_OutClkDiv,
            nidaqmx_sys::AI_DigFltr_Enable => AnalogInput::DigFltr_Enable,
            nidaqmx_sys::AI_DigFltr_Type => AnalogInput::DigFltr_Type,
            nidaqmx_sys::AI_DigFltr_Response => AnalogInput::DigFltr_Response,
            nidaqmx_sys::AI_DigFltr_Order => AnalogInput::DigFltr_Order,
            nidaqmx_sys::AI_DigFltr_Lowpass_CutoffFreq => AnalogInput::DigFltr_Lowpass_CutoffFreq,
            nidaqmx_sys::AI_DigFltr_Highpass_CutoffFreq => AnalogInput::DigFltr_Highpass_CutoffFreq,
            nidaqmx_sys::AI_DigFltr_Bandpass_CenterFreq => AnalogInput::DigFltr_Bandpass_CenterFreq,
            nidaqmx_sys::AI_DigFltr_Bandpass_Width => AnalogInput::DigFltr_Bandpass_Width,
            nidaqmx_sys::AI_DigFltr_Notch_CenterFreq => AnalogInput::DigFltr_Notch_CenterFreq,
            nidaqmx_sys::AI_DigFltr_Notch_Width => AnalogInput::DigFltr_Notch_Width,
            nidaqmx_sys::AI_DigFltr_Coeff => AnalogInput::DigFltr_Coeff,
            nidaqmx_sys::AI_Filter_Enable => AnalogInput::Filter_Enable,
            nidaqmx_sys::AI_Filter_Freq => AnalogInput::Filter_Freq,
            nidaqmx_sys::AI_Filter_Response => AnalogInput::Filter_Response,
            nidaqmx_sys::AI_Filter_Order => AnalogInput::Filter_Order,
            nidaqmx_sys::AI_FilterDelay => AnalogInput::FilterDelay,
            nidaqmx_sys::AI_FilterDelayUnits => AnalogInput::FilterDelayUnits,
            nidaqmx_sys::AI_RemoveFilterDelay => AnalogInput::RemoveFilterDelay,
            nidaqmx_sys::AI_FilterDelayAdjustment => AnalogInput::FilterDelayAdjustment,
            nidaqmx_sys::AI_AveragingWinSize => AnalogInput::AveragingWinSize,
            nidaqmx_sys::AI_ResolutionUnits => AnalogInput::ResolutionUnits,
            nidaqmx_sys::AI_Resolution => AnalogInput::Resolution,
            nidaqmx_sys::AI_RawSampSize => AnalogInput::RawSampSize,
            nidaqmx_sys::AI_RawSampJustification => AnalogInput::RawSampJustification,
            nidaqmx_sys::AI_ADCTimingMode => AnalogInput::ADCTimingMode,
            nidaqmx_sys::AI_ADCCustomTimingMode => AnalogInput::ADCCustomTimingMode,
            nidaqmx_sys::AI_Dither_Enable => AnalogInput::Dither_Enable,
            nidaqmx_sys::AI_ChanCal_HasValidCalInfo => AnalogInput::ChanCal_HasValidCalInfo,
            nidaqmx_sys::AI_ChanCal_EnableCal => AnalogInput::ChanCal_EnableCal,
            nidaqmx_sys::AI_ChanCal_ApplyCalIfExp => AnalogInput::ChanCal_ApplyCalIfExp,
            nidaqmx_sys::AI_ChanCal_ScaleType => AnalogInput::ChanCal_ScaleType,
            nidaqmx_sys::AI_ChanCal_Table_PreScaledVals => AnalogInput::ChanCal_Table_PreScaledVals,
            nidaqmx_sys::AI_ChanCal_Table_ScaledVals => AnalogInput::ChanCal_Table_ScaledVals,
            nidaqmx_sys::AI_ChanCal_Poly_ForwardCoeff => AnalogInput::ChanCal_Poly_ForwardCoeff,
            nidaqmx_sys::AI_ChanCal_Poly_ReverseCoeff => AnalogInput::ChanCal_Poly_ReverseCoeff,
            nidaqmx_sys::AI_ChanCal_OperatorName => AnalogInput::ChanCal_OperatorName,
            nidaqmx_sys::AI_ChanCal_Desc => AnalogInput::ChanCal_Desc,
            nidaqmx_sys::AI_ChanCal_Verif_RefVals => AnalogInput::ChanCal_Verif_RefVals,
            nidaqmx_sys::AI_ChanCal_Verif_AcqVals => AnalogInput::ChanCal_Verif_AcqVals,
            nidaqmx_sys::AI_Rng_High => AnalogInput::Rng_High,
            nidaqmx_sys::AI_Rng_Low => AnalogInput::Rng_Low,
            nidaqmx_sys::AI_DCOffset => AnalogInput::DCOffset,
            nidaqmx_sys::AI_Gain => AnalogInput::Gain,
            nidaqmx_sys::AI_SampAndHold_Enable => AnalogInput::SampAndHold_Enable,
            nidaqmx_sys::AI_AutoZeroMode => AnalogInput::AutoZeroMode,
            nidaqmx_sys::AI_ChopEnable => AnalogInput::ChopEnable,
            nidaqmx_sys::AI_DataXferMaxRate => AnalogInput::DataXferMaxRate,
            nidaqmx_sys::AI_DataXferMech => AnalogInput::DataXferMech,
            nidaqmx_sys::AI_DataXferReqCond => AnalogInput::DataXferReqCond,
            nidaqmx_sys::AI_DataXferCustomThreshold => AnalogInput::DataXferCustomThreshold,
            nidaqmx_sys::AI_UsbXferReqSize => AnalogInput::UsbXferReqSize,
            nidaqmx_sys::AI_UsbXferReqCount => AnalogInput::UsbXferReqCount,
            nidaqmx_sys::AI_MemMapEnable => AnalogInput::MemMapEnable,
            nidaqmx_sys::AI_RawDataCompressionType => AnalogInput::RawDataCompressionType,
            nidaqmx_sys::AI_LossyLSBRemoval_CompressedSampSize => {
                AnalogInput::LossyLSBRemoval_CompressedSampSize
            }
            nidaqmx_sys::AI_DevScalingCoeff => AnalogInput::DevScalingCoeff,
            nidaqmx_sys::AI_EnhancedAliasRejectionEnable => {
                AnalogInput::EnhancedAliasRejectionEnable
            }
            nidaqmx_sys::AI_OpenChanDetectEnable => AnalogInput::OpenChanDetectEnable,
            nidaqmx_sys::AI_InputLimitsFaultDetect_UpperLimit => {
                AnalogInput::InputLimitsFaultDetect_UpperLimit
            }
            nidaqmx_sys::AI_InputLimitsFaultDetect_LowerLimit => {
                AnalogInput::InputLimitsFaultDetect_LowerLimit
            }
            nidaqmx_sys::AI_InputLimitsFaultDetectEnable => {
                AnalogInput::InputLimitsFaultDetectEnable
            }
            nidaqmx_sys::AI_PowerSupplyFaultDetectEnable => {
                AnalogInput::PowerSupplyFaultDetectEnable
            }
            nidaqmx_sys::AI_OvercurrentDetectEnable => AnalogInput::OvercurrentDetectEnable,
            _ => panic!("Unknown AnalogInputAttr!"),
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum AnalogOutput {
    #[doc = "Specifies the maximum value you expect to generate. The value is in the units you specify with a units property. If you try to write a value larger than the maximum value, NI-DAQmx generates an error. NI-DAQmx might coerce this value to a smaller value if other task settings restrict the device from generating the desired maximum."]
    Max,
    #[doc = "Specifies the minimum value you expect to generate. The value is in the units you specify with a units property. If you try to write a value smaller than the minimum value, NI-DAQmx generates an error. NI-DAQmx might coerce this value to a larger value if other task settings restrict the device from generating the desired minimum."]
    Min,
    #[doc = "Specifies the name of a custom scale for the channel."]
    CustomScaleName,
    #[doc = "Indicates whether the channel generates voltage,  current, or a waveform."]
    OutputType,
    #[doc = "Specifies in what units to generate voltage on the channel. Write data to the channel in the units you select."]
    Voltage_Units,
    #[doc = "Specifies the current limit, in amperes, for the voltage channel."]
    Voltage_CurrentLimit,
    #[doc = "Specifies in what units to generate current on the channel. Write data to the channel in the units you select."]
    Current_Units,
    #[doc = "Specifies the kind of the waveform to generate."]
    FuncGen_Type,
    #[doc = "Specifies the frequency of the waveform to generate in hertz."]
    FuncGen_Freq,
    #[doc = "Specifies the zero-to-peak amplitude of the waveform to generate in volts. Zero and negative values are valid."]
    FuncGen_Amplitude,
    #[doc = "Specifies the voltage offset of the waveform to generate."]
    FuncGen_Offset,
    #[doc = "Specifies the starting phase in degrees of the waveform to generate."]
    FuncGen_StartPhase,
    #[doc = "Specifies the square wave duty cycle of the waveform to generate."]
    FuncGen_Square_DutyCycle,
    #[doc = "Specifies if the device generates a modulated version of the waveform using the original waveform as a carrier and input from an external terminal as the signal."]
    FuncGen_ModulationType,
    #[doc = "Specifies the FM deviation in hertz per volt when Type is DAQmx_Val_FM."]
    FuncGen_FMDeviation,
    #[doc = "Specifies in ohms the impedance of the analog output stage of the device."]
    OutputImpedance,
    #[doc = "Specifies in ohms the load impedance connected to the analog output channel."]
    LoadImpedance,
    #[doc = "Specifies the state of the channel when no generation is in progress."]
    IdleOutputBehavior,
    #[doc = "Specifies the terminal configuration of the channel."]
    TermCfg,
    #[doc = "Specifies the common-mode offset of the AO channel. Use the property only when Terminal Configuration is set to Differential."]
    Common_Mode_Offset,
    #[doc = "Specifies the units of Resolution Value."]
    ResolutionUnits,
    #[doc = "Indicates the resolution of the digital-to-analog converter of the channel. This value is in the units you specify with Resolution Units."]
    Resolution,
    #[doc = "Specifies the upper limit of the output range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
    DAC_Rng_High,
    #[doc = "Specifies the lower limit of the output range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
    DAC_Rng_Low,
    #[doc = "Specifies whether to ground the internal DAC reference. Grounding the internal DAC reference has the effect of grounding all analog output channels and stopping waveform generation across all analog output channels regardless of whether the channels belong to the current task. You can ground the internal DAC reference only when Source is DAQmx_Val_Internal and Allow Connecting DAC Reference to Ground at Runtime is..."]
    DAC_Ref_ConnToGnd,
    #[doc = "Specifies whether to allow grounding the internal DAC reference at run time. You must set this property to TRUE and set Source to DAQmx_Val_Internal before you can set Connect DAC Reference to Ground to TRUE."]
    DAC_Ref_AllowConnToGnd,
    #[doc = "Specifies the source of the DAC reference voltage. The value of this voltage source determines the full-scale value of the DAC."]
    DAC_Ref_Src,
    #[doc = "Specifies the source of the DAC reference voltage if Source is DAQmx_Val_External. The valid sources for this signal vary by device."]
    DAC_Ref_ExtSrc,
    #[doc = "Specifies in volts the value of the DAC reference voltage. This voltage determines the full-scale range of the DAC. Smaller reference voltages result in smaller ranges, but increased resolution."]
    DAC_Ref_Val,
    #[doc = "Specifies the source of the DAC offset voltage. The value of this voltage source determines the full-scale value of the DAC."]
    DAC_Offset_Src,
    #[doc = "Specifies the source of the DAC offset voltage if Source is DAQmx_Val_External. The valid sources for this signal vary by device."]
    DAC_Offset_ExtSrc,
    #[doc = "Specifies in volts the value of the DAC offset voltage. To achieve best accuracy, the DAC offset value should be hand calibrated."]
    DAC_Offset_Val,
    #[doc = "Specifies whether to enable reglitching.  The output of a DAC normally glitches whenever the DAC is updated with a new value. The amount of glitching differs from code to code and is generally largest at major code transitions.  Reglitching generates uniform glitch energy at each code transition and provides for more uniform glitches.  Uniform glitch energy makes it easier to filter out the noise introduced from g..."]
    ReglitchEnable,
    #[doc = "Specifies the amount of time between when the sample is written by the host device and when the sample is output by the DAC. This value is in the units you specify with Filter Delay Units."]
    FilterDelay,
    #[doc = "Specifies the units of Filter Delay and Filter Delay Adjustment."]
    FilterDelayUnits,
    #[doc = "Specifies an additional amount of time to wait between when the sample is written by the host device and when the sample is output by the DAC. This delay adjustment is in addition to the value indicated by Filter Delay. This delay adjustment is in the units you specify with Filter Delay Units."]
    FilterDelayAdjustment,
    #[doc = "Specifies in decibels the gain factor to apply to the channel."]
    Gain,
    #[doc = "Specifies whether to write samples directly to the onboard memory of the device, bypassing the memory buffer. Generally, you cannot update onboard memory directly after you start the task. Onboard memory includes data FIFOs."]
    UseOnlyOnBrdMem,
    #[doc = "Specifies the data transfer mode for the device."]
    DataXferMech,
    #[doc = "Specifies under what condition to transfer data from the buffer to the onboard memory of the device."]
    DataXferReqCond,
    #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqSize,
    #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqCount,
    #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
    MemMapEnable,
    #[doc = "Indicates the coefficients of a linear equation that NI-DAQmx uses to scale values from a voltage to the native format of the device. Each element of the array corresponds to a term of the equation. The first element of the array corresponds to the y-intercept, and the second element corresponds to the slope. Scaling coefficients do not account for any custom scales that may be applied to the channel."]
    DevScalingCoeff,
    #[doc = "Specifies whether to enable the DAC interpolation filter. Disable the interpolation filter to improve DAC signal-to-noise ratio at the expense of degraded image rejection."]
    EnhancedImageRejectionEnable,
}

impl From<AnalogOutput> for i32 {
    fn from(attr: AnalogOutput) -> Self {
        match attr {
            AnalogOutput::Max => nidaqmx_sys::DAQmx_AO_Max,
            AnalogOutput::Min => nidaqmx_sys::DAQmx_AO_Min,
            AnalogOutput::CustomScaleName => nidaqmx_sys::DAQmx_AO_CustomScaleName,
            AnalogOutput::OutputType => nidaqmx_sys::DAQmx_AO_OutputType,
            AnalogOutput::Voltage_Units => nidaqmx_sys::DAQmx_AO_Voltage_Units,
            AnalogOutput::Voltage_CurrentLimit => nidaqmx_sys::DAQmx_AO_Voltage_CurrentLimit,
            AnalogOutput::Current_Units => nidaqmx_sys::DAQmx_AO_Current_Units,
            AnalogOutput::FuncGen_Type => nidaqmx_sys::DAQmx_AO_FuncGen_Type,
            AnalogOutput::FuncGen_Freq => nidaqmx_sys::DAQmx_AO_FuncGen_Freq,
            AnalogOutput::FuncGen_Amplitude => nidaqmx_sys::DAQmx_AO_FuncGen_Amplitude,
            AnalogOutput::FuncGen_Offset => nidaqmx_sys::DAQmx_AO_FuncGen_Offset,
            AnalogOutput::FuncGen_StartPhase => nidaqmx_sys::DAQmx_AO_FuncGen_StartPhase,
            AnalogOutput::FuncGen_Square_DutyCycle => {
                nidaqmx_sys::DAQmx_AO_FuncGen_Square_DutyCycle
            }
            AnalogOutput::FuncGen_ModulationType => nidaqmx_sys::DAQmx_AO_FuncGen_ModulationType,
            AnalogOutput::FuncGen_FMDeviation => nidaqmx_sys::DAQmx_AO_FuncGen_FMDeviation,
            AnalogOutput::OutputImpedance => nidaqmx_sys::DAQmx_AO_OutputImpedance,
            AnalogOutput::LoadImpedance => nidaqmx_sys::DAQmx_AO_LoadImpedance,
            AnalogOutput::IdleOutputBehavior => nidaqmx_sys::DAQmx_AO_IdleOutputBehavior,
            AnalogOutput::TermCfg => nidaqmx_sys::DAQmx_AO_TermCfg,
            AnalogOutput::Common_Mode_Offset => nidaqmx_sys::DAQmx_AO_Common_Mode_Offset,
            AnalogOutput::ResolutionUnits => nidaqmx_sys::DAQmx_AO_ResolutionUnits,
            AnalogOutput::Resolution => nidaqmx_sys::DAQmx_AO_Resolution,
            AnalogOutput::DAC_Rng_High => nidaqmx_sys::DAQmx_AO_DAC_Rng_High,
            AnalogOutput::DAC_Rng_Low => nidaqmx_sys::DAQmx_AO_DAC_Rng_Low,
            AnalogOutput::DAC_Ref_ConnToGnd => nidaqmx_sys::DAQmx_AO_DAC_Ref_ConnToGnd,
            AnalogOutput::DAC_Ref_AllowConnToGnd => nidaqmx_sys::DAQmx_AO_DAC_Ref_AllowConnToGnd,
            AnalogOutput::DAC_Ref_Src => nidaqmx_sys::DAQmx_AO_DAC_Ref_Src,
            AnalogOutput::DAC_Ref_ExtSrc => nidaqmx_sys::DAQmx_AO_DAC_Ref_ExtSrc,
            AnalogOutput::DAC_Ref_Val => nidaqmx_sys::DAQmx_AO_DAC_Ref_Val,
            AnalogOutput::DAC_Offset_Src => nidaqmx_sys::DAQmx_AO_DAC_Offset_Src,
            AnalogOutput::DAC_Offset_ExtSrc => nidaqmx_sys::DAQmx_AO_DAC_Offset_ExtSrc,
            AnalogOutput::DAC_Offset_Val => nidaqmx_sys::DAQmx_AO_DAC_Offset_Val,
            AnalogOutput::ReglitchEnable => nidaqmx_sys::DAQmx_AO_ReglitchEnable,
            AnalogOutput::FilterDelay => nidaqmx_sys::DAQmx_AO_FilterDelay,
            AnalogOutput::FilterDelayUnits => nidaqmx_sys::DAQmx_AO_FilterDelayUnits,
            AnalogOutput::FilterDelayAdjustment => nidaqmx_sys::DAQmx_AO_FilterDelayAdjustment,
            AnalogOutput::Gain => nidaqmx_sys::DAQmx_AO_Gain,
            AnalogOutput::UseOnlyOnBrdMem => nidaqmx_sys::DAQmx_AO_UseOnlyOnBrdMem,
            AnalogOutput::DataXferMech => nidaqmx_sys::DAQmx_AO_DataXferMech,
            AnalogOutput::DataXferReqCond => nidaqmx_sys::DAQmx_AO_DataXferReqCond,
            AnalogOutput::UsbXferReqSize => nidaqmx_sys::DAQmx_AO_UsbXferReqSize,
            AnalogOutput::UsbXferReqCount => nidaqmx_sys::DAQmx_AO_UsbXferReqCount,
            AnalogOutput::MemMapEnable => nidaqmx_sys::DAQmx_AO_MemMapEnable,
            AnalogOutput::DevScalingCoeff => nidaqmx_sys::DAQmx_AO_DevScalingCoeff,
            AnalogOutput::EnhancedImageRejectionEnable => {
                nidaqmx_sys::DAQmx_AO_EnhancedImageRejectionEnable
            }
        }
    }
}

impl From<i32> for AnalogOutput {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_AO_Max => AnalogOutput::Max,
            nidaqmx_sys::DAQmx_AO_Min => AnalogOutput::Min,
            nidaqmx_sys::DAQmx_AO_CustomScaleName => AnalogOutput::CustomScaleName,
            nidaqmx_sys::DAQmx_AO_OutputType => AnalogOutput::OutputType,
            nidaqmx_sys::DAQmx_AO_Voltage_Units => AnalogOutput::Voltage_Units,
            nidaqmx_sys::DAQmx_AO_Voltage_CurrentLimit => AnalogOutput::Voltage_CurrentLimit,
            nidaqmx_sys::DAQmx_AO_Current_Units => AnalogOutput::Current_Units,
            nidaqmx_sys::DAQmx_AO_FuncGen_Type => AnalogOutput::FuncGen_Type,
            nidaqmx_sys::DAQmx_AO_FuncGen_Freq => AnalogOutput::FuncGen_Freq,
            nidaqmx_sys::DAQmx_AO_FuncGen_Amplitude => AnalogOutput::FuncGen_Amplitude,
            nidaqmx_sys::DAQmx_AO_FuncGen_Offset => AnalogOutput::FuncGen_Offset,
            nidaqmx_sys::DAQmx_AO_FuncGen_StartPhase => AnalogOutput::FuncGen_StartPhase,
            nidaqmx_sys::DAQmx_AO_FuncGen_Square_DutyCycle => {
                AnalogOutput::FuncGen_Square_DutyCycle
            }
            nidaqmx_sys::DAQmx_AO_FuncGen_ModulationType => AnalogOutput::FuncGen_ModulationType,
            nidaqmx_sys::DAQmx_AO_FuncGen_FMDeviation => AnalogOutput::FuncGen_FMDeviation,
            nidaqmx_sys::DAQmx_AO_OutputImpedance => AnalogOutput::OutputImpedance,
            nidaqmx_sys::DAQmx_AO_LoadImpedance => AnalogOutput::LoadImpedance,
            nidaqmx_sys::DAQmx_AO_IdleOutputBehavior => AnalogOutput::IdleOutputBehavior,
            nidaqmx_sys::DAQmx_AO_TermCfg => AnalogOutput::TermCfg,
            nidaqmx_sys::DAQmx_AO_Common_Mode_Offset => AnalogOutput::Common_Mode_Offset,
            nidaqmx_sys::DAQmx_AO_ResolutionUnits => AnalogOutput::ResolutionUnits,
            nidaqmx_sys::DAQmx_AO_Resolution => AnalogOutput::Resolution,
            nidaqmx_sys::DAQmx_AO_DAC_Rng_High => AnalogOutput::DAC_Rng_High,
            nidaqmx_sys::DAQmx_AO_DAC_Rng_Low => AnalogOutput::DAC_Rng_Low,
            nidaqmx_sys::DAQmx_AO_DAC_Ref_ConnToGnd => AnalogOutput::DAC_Ref_ConnToGnd,
            nidaqmx_sys::DAQmx_AO_DAC_Ref_AllowConnToGnd => AnalogOutput::DAC_Ref_AllowConnToGnd,
            nidaqmx_sys::DAQmx_AO_DAC_Ref_Src => AnalogOutput::DAC_Ref_Src,
            nidaqmx_sys::DAQmx_AO_DAC_Ref_ExtSrc => AnalogOutput::DAC_Ref_ExtSrc,
            nidaqmx_sys::DAQmx_AO_DAC_Ref_Val => AnalogOutput::DAC_Ref_Val,
            nidaqmx_sys::DAQmx_AO_DAC_Offset_Src => AnalogOutput::DAC_Offset_Src,
            nidaqmx_sys::DAQmx_AO_DAC_Offset_ExtSrc => AnalogOutput::DAC_Offset_ExtSrc,
            nidaqmx_sys::DAQmx_AO_DAC_Offset_Val => AnalogOutput::DAC_Offset_Val,
            nidaqmx_sys::DAQmx_AO_ReglitchEnable => AnalogOutput::ReglitchEnable,
            nidaqmx_sys::DAQmx_AO_FilterDelay => AnalogOutput::FilterDelay,
            nidaqmx_sys::DAQmx_AO_FilterDelayUnits => AnalogOutput::FilterDelayUnits,
            nidaqmx_sys::DAQmx_AO_FilterDelayAdjustment => AnalogOutput::FilterDelayAdjustment,
            nidaqmx_sys::DAQmx_AO_Gain => AnalogOutput::Gain,
            nidaqmx_sys::DAQmx_AO_UseOnlyOnBrdMem => AnalogOutput::UseOnlyOnBrdMem,
            nidaqmx_sys::DAQmx_AO_DataXferMech => AnalogOutput::DataXferMech,
            nidaqmx_sys::DAQmx_AO_DataXferReqCond => AnalogOutput::DataXferReqCond,
            nidaqmx_sys::DAQmx_AO_UsbXferReqSize => AnalogOutput::UsbXferReqSize,
            nidaqmx_sys::DAQmx_AO_UsbXferReqCount => AnalogOutput::UsbXferReqCount,
            nidaqmx_sys::DAQmx_AO_MemMapEnable => AnalogOutput::MemMapEnable,
            nidaqmx_sys::DAQmx_AO_DevScalingCoeff => AnalogOutput::DevScalingCoeff,
            nidaqmx_sys::DAQmx_AO_EnhancedImageRejectionEnable => {
                AnalogOutput::EnhancedImageRejectionEnable
            }
            _ => panic!("Unknown AnalogOutputAttr!"),
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum DigitalInput {
    #[doc = "Specifies whether to invert the lines in the channel. If you set this property to TRUE, the lines are at high logic when off and at low logic when on."]
    InvertLines,
    #[doc = "Indicates the number of digital lines in the channel."]
    NumLines,
    #[doc = "Specifies whether to enable the digital filter for the line(s) or port(s). You can enable the filter on a line-by-line basis. You do not have to enable the filter for all lines in a channel."]
    DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes as a valid high or low state transition."]
    DigFltr_MinPulseWidth,
    #[doc = "Specifies whether to enable bus mode for digital filtering. If you set this property to TRUE, NI-DAQmx treats all lines that use common filtering settings as a bus. If any line in the bus has jitter, all lines in the bus hold state until the entire bus stabilizes, or until 2 times the minimum pulse width elapses. If you set this property to FALSE, NI-DAQmx filters all lines individually. Jitter in one line does no..."]
    DigFltr_EnableBusMode,
    #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
    DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    DigSync_Enable,
    #[doc = "Specifies whether to tristate the lines in the channel. If you set this property to TRUE, NI-DAQmx tristates the lines in the channel. If you set this property to FALSE, NI-DAQmx does not modify the configuration of the lines even if the lines were previously tristated. Set this property to FALSE to read lines in other tasks or to read output-only lines."]
    Tristate,
    #[doc = "Specifies the logic family to use for acquisition. A logic family corresponds to voltage thresholds that are compatible with a group of voltage standards. Refer to the device documentation for information on the logic high and logic low voltages for these logic families."]
    LogicFamily,
    #[doc = "Specifies the data transfer mode for the device."]
    DataXferMech,
    #[doc = "Specifies under what condition to transfer data from the onboard memory of the device to the buffer."]
    DataXferReqCond,
    #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqSize,
    #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqCount,
    #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
    MemMapEnable,
    #[doc = "Specifies on which edge of the sample clock to acquire samples."]
    AcquireOn,
}

impl From<DigitalInput> for i32 {
    fn from(attr: DigitalInput) -> Self {
        match attr {
            DigitalInput::InvertLines => nidaqmx_sys::DAQmx_DI_InvertLines,
            DigitalInput::NumLines => nidaqmx_sys::DAQmx_DI_NumLines,
            DigitalInput::DigFltr_Enable => nidaqmx_sys::DAQmx_DI_DigFltr_Enable,
            DigitalInput::DigFltr_MinPulseWidth => nidaqmx_sys::DAQmx_DI_DigFltr_MinPulseWidth,
            DigitalInput::DigFltr_EnableBusMode => nidaqmx_sys::DAQmx_DI_DigFltr_EnableBusMode,
            DigitalInput::DigFltr_TimebaseSrc => nidaqmx_sys::DAQmx_DI_DigFltr_TimebaseSrc,
            DigitalInput::DigFltr_TimebaseRate => nidaqmx_sys::DAQmx_DI_DigFltr_TimebaseRate,
            DigitalInput::DigSync_Enable => nidaqmx_sys::DAQmx_DI_DigSync_Enable,
            DigitalInput::Tristate => nidaqmx_sys::DAQmx_DI_Tristate,
            DigitalInput::LogicFamily => nidaqmx_sys::DAQmx_DI_LogicFamily,
            DigitalInput::DataXferMech => nidaqmx_sys::DAQmx_DI_DataXferMech,
            DigitalInput::DataXferReqCond => nidaqmx_sys::DAQmx_DI_DataXferReqCond,
            DigitalInput::UsbXferReqSize => nidaqmx_sys::DAQmx_DI_UsbXferReqSize,
            DigitalInput::UsbXferReqCount => nidaqmx_sys::DAQmx_DI_UsbXferReqCount,
            DigitalInput::MemMapEnable => nidaqmx_sys::DAQmx_DI_MemMapEnable,
            DigitalInput::AcquireOn => nidaqmx_sys::DAQmx_DI_AcquireOn,
        }
    }
}

impl From<i32> for DigitalInput {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_DI_InvertLines => DigitalInput::InvertLines,
            nidaqmx_sys::DAQmx_DI_NumLines => DigitalInput::NumLines,
            nidaqmx_sys::DAQmx_DI_DigFltr_Enable => DigitalInput::DigFltr_Enable,
            nidaqmx_sys::DAQmx_DI_DigFltr_MinPulseWidth => DigitalInput::DigFltr_MinPulseWidth,
            nidaqmx_sys::DAQmx_DI_DigFltr_EnableBusMode => DigitalInput::DigFltr_EnableBusMode,
            nidaqmx_sys::DAQmx_DI_DigFltr_TimebaseSrc => DigitalInput::DigFltr_TimebaseSrc,
            nidaqmx_sys::DAQmx_DI_DigFltr_TimebaseRate => DigitalInput::DigFltr_TimebaseRate,
            nidaqmx_sys::DAQmx_DI_DigSync_Enable => DigitalInput::DigSync_Enable,
            nidaqmx_sys::DAQmx_DI_Tristate => DigitalInput::Tristate,
            nidaqmx_sys::DAQmx_DI_LogicFamily => DigitalInput::LogicFamily,
            nidaqmx_sys::DAQmx_DI_DataXferMech => DigitalInput::DataXferMech,
            nidaqmx_sys::DAQmx_DI_DataXferReqCond => DigitalInput::DataXferReqCond,
            nidaqmx_sys::DAQmx_DI_UsbXferReqSize => DigitalInput::UsbXferReqSize,
            nidaqmx_sys::DAQmx_DI_UsbXferReqCount => DigitalInput::UsbXferReqCount,
            nidaqmx_sys::DAQmx_DI_MemMapEnable => DigitalInput::MemMapEnable,
            nidaqmx_sys::DAQmx_DI_AcquireOn => DigitalInput::AcquireOn,
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum DigitalOutput {
    #[doc = "Specifies the drive type for digital output channels."]
    OutputDriveType,
    #[doc = "Specifies whether to invert the lines in the channel. If you set this property to TRUE, the lines are at high logic when off and at low logic when on."]
    InvertLines,
    #[doc = "Indicates the number of digital lines in the channel."]
    NumLines,
    #[doc = "Specifies whether to stop driving the channel and set it to a high-impedance state. You must commit the task for this setting to take effect."]
    Tristate,
    #[doc = "Specifies the state of the lines in a digital output task when the task starts."]
    LineStates_StartState,
    #[doc = "Specifies the state of the lines in a digital output task when the task pauses."]
    LineStates_PausedState,
    #[doc = "Specifies the state of the lines in a digital output task when the task completes execution."]
    LineStates_DoneState,
    #[doc = "Specifies the logic family to use for generation. A logic family corresponds to voltage thresholds that are compatible with a group of voltage standards. Refer to the device documentation for information on the logic high and logic low voltages for these logic families."]
    LogicFamily,
    #[doc = "Specifies the current threshold in Amperes for the channel. A value of 0 means the channel observes no limit. Devices can monitor only a finite number of current thresholds simultaneously. If you attempt to monitor additional thresholds, NI-DAQmx returns an error."]
    Overcurrent_Limit,
    #[doc = "Specifies whether to automatically reenable channels after they no longer exceed the current limit specified by Current Limit."]
    Overcurrent_AutoReenable,
    #[doc = "Specifies the delay in seconds between the time a channel no longer exceeds the current limit and the reactivation of that channel, if Automatic Re-enable is TRUE."]
    Overcurrent_ReenablePeriod,
    #[doc = "Specifies whether to write samples directly to the onboard memory of the device, bypassing the memory buffer. Generally, you cannot update onboard memory after you start the task. Onboard memory includes data FIFOs."]
    UseOnlyOnBrdMem,
    #[doc = "Specifies the data transfer mode for the device."]
    DataXferMech,
    #[doc = "Specifies under what condition to transfer data from the buffer to the onboard memory of the device."]
    DataXferReqCond,
    #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqSize,
    #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqCount,
    #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
    MemMapEnable,
    #[doc = "Specifies on which edge of the sample clock to generate samples."]
    GenerateOn,
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum CounterInput {
    #[doc = "Specifies the maximum value you expect to measure. This value is in the units you specify with a units property. When you query this property, it returns the coerced maximum value that the hardware can measure with the current settings."]
    Max,
    #[doc = "Specifies the minimum value you expect to measure. This value is in the units you specify with a units property. When you query this property, it returns the coerced minimum value that the hardware can measure with the current settings."]
    Min,
    #[doc = "Specifies the name of a custom scale for the channel."]
    CustomScaleName,
    #[doc = "Indicates the measurement to take with the channel."]
    MeasType,
    #[doc = "Specifies the units to use to return frequency measurements."]
    Freq_Units,
    #[doc = "Specifies the input terminal of the signal to measure."]
    Freq_Term,
    #[doc = "Specifies the input terminal configuration."]
    Freq_TermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    Freq_LogicLvlBehavior,
    #[doc = "Specifies the voltage level at which to recognize waveform repetitions. Select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
    Freq_ThreshVoltage,
    #[doc = "Specifies a hysteresis level to apply to Threshold Level. When Starting Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Starting Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
    Freq_Hyst,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Freq_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Freq_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Freq_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Freq_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Freq_DigSync_Enable,
    #[doc = "Specifies between which edges to measure the frequency of the signal."]
    Freq_StartingEdge,
    #[doc = "Specifies the method to use to measure the frequency of the signal."]
    Freq_MeasMeth,
    #[doc = "Specifies whether to enable averaging mode for Sample Clock-timed frequency measurements."]
    Freq_EnableAveraging,
    #[doc = "Specifies in seconds the length of time to measure the frequency of the signal if Method is DAQmx_Val_HighFreq2Ctr. Measurement accuracy increases with increased measurement time and with increased signal frequency. If you measure a high-frequency signal for too long, however, the count register could roll over, which results in an incorrect measurement."]
    Freq_MeasTime,
    #[doc = "Specifies the value by which to divide the input signal if  Method is DAQmx_Val_LargeRng2Ctr. The larger the divisor, the more accurate the measurement. However, too large a value could cause the count register to roll over, which results in an incorrect measurement."]
    Freq_Div,
    #[doc = "Specifies the unit to use to return period measurements."]
    Period_Units,
    #[doc = "Specifies the input terminal of the signal to measure."]
    Period_Term,
    #[doc = "Specifies the input terminal configuration."]
    Period_TermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    Period_LogicLvlBehavior,
    #[doc = "Specifies the voltage level at which to recognize waveform repetitions. Select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
    Period_ThreshVoltage,
    #[doc = "Specifies a hysteresis level to apply to Threshold Level. When Starting Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Starting Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
    Period_Hyst,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Period_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Period_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Period_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Period_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Period_DigSync_Enable,
    #[doc = "Specifies between which edges to measure the period of the signal."]
    Period_StartingEdge,
    #[doc = "Specifies the method to use to measure the period of the signal."]
    Period_MeasMeth,
    #[doc = "Specifies whether to enable averaging mode for Sample Clock-timed period measurements."]
    Period_EnableAveraging,
    #[doc = "Specifies in seconds the length of time to measure the period of the signal if Method is DAQmx_Val_HighFreq2Ctr. Measurement accuracy increases with increased measurement time and with increased signal frequency. If you measure a high-frequency signal for too long, however, the count register could roll over, which results in an incorrect measurement."]
    Period_MeasTime,
    #[doc = "Specifies the value by which to divide the input signal if Method is DAQmx_Val_LargeRng2Ctr. The larger the divisor, the more accurate the measurement. However, too large a value could cause the count register to roll over, which results in an incorrect measurement."]
    Period_Div,
    #[doc = "Specifies the input terminal of the signal to measure."]
    CountEdges_Term,
    #[doc = "Specifies the input terminal configuration."]
    CountEdges_TermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    CountEdges_LogicLvlBehavior,
    #[doc = "Specifies the voltage level at which to recognize waveform repetitions. Select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
    CountEdges_ThreshVoltage,
    #[doc = "Specifies a hysteresis level to apply to Threshold Level. When Active Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Active Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
    CountEdges_Hyst,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    CountEdges_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    CountEdges_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    CountEdges_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    CountEdges_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    CountEdges_DigSync_Enable,
    #[doc = "Specifies whether to increment or decrement the counter on each edge."]
    CountEdges_Dir,
    #[doc = "Specifies the source terminal of the digital signal that controls the count direction if Direction is DAQmx_Val_ExtControlled."]
    CountEdges_DirTerm,
    #[doc = "Specifies the input terminal configuration."]
    CountEdges_CountDir_TermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    CountEdges_CountDir_LogicLvlBehavior,
    #[doc = "Specifies the voltage level applied to the Count Direction terminal. When the signal is above this threshold, the counter counts up. When the signal is below this threshold, the counter counts down."]
    CountEdges_CountDir_ThreshVoltage,
    #[doc = "Specifies a hysteresis level applied to the Threshold Level. The source signal must fall below Threshold Level minus the hysteresis before a change in count direction occurs."]
    CountEdges_CountDir_Hyst,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    CountEdges_CountDir_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    CountEdges_CountDir_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    CountEdges_CountDir_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    CountEdges_CountDir_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    CountEdges_CountDir_DigSync_Enable,
    #[doc = "Specifies the starting value from which to count."]
    CountEdges_InitialCnt,
    #[doc = "Specifies on which edges to increment or decrement the counter."]
    CountEdges_ActiveEdge,
    #[doc = "Specifies whether to reset the count on the active edge specified with Terminal."]
    CountEdges_CountReset_Enable,
    #[doc = "Specifies the value to reset the count to."]
    CountEdges_CountReset_ResetCount,
    #[doc = "Specifies the input terminal of the signal to reset the count."]
    CountEdges_CountReset_Term,
    #[doc = "Specifies the input terminal configuration."]
    CountEdges_CountReset_TermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    CountEdges_CountReset_LogicLvlBehavior,
    #[doc = "Specifies the voltage level at which to recognize the counter reset event."]
    CountEdges_CountReset_ThreshVoltage,
    #[doc = "Specifies a hysteresis level applied to Threshold Level. When Active Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Active Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
    CountEdges_CountReset_Hyst,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    CountEdges_CountReset_DigFltr_Enable,
    #[doc = "Specifies the minimum pulse width the filter recognizes."]
    CountEdges_CountReset_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input of the signal to use as the timebase of the pulse width filter."]
    CountEdges_CountReset_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    CountEdges_CountReset_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    CountEdges_CountReset_DigSync_Enable,
    #[doc = "Specifies on which edge of the signal to reset the count."]
    CountEdges_CountReset_ActiveEdge,
    #[doc = "Specifies whether to enable the functionality to gate the counter input signal for a count edges measurement."]
    CountEdges_Gate_Enable,
    #[doc = "Specifies the gate terminal."]
    CountEdges_Gate_Term,
    #[doc = "Specifies the gate terminal configuration."]
    CountEdges_Gate_TermCfg,
    #[doc = "Specifies the logic level behavior on the gate input line."]
    CountEdges_Gate_LogicLvlBehavior,
    #[doc = "Specifies the voltage level at which to recognize the counter gate signal."]
    CountEdges_Gate_ThreshVoltage,
    #[doc = "Specifies a hysteresis level applied to the Threshold Level. When Pause When is High, the source signal must fall below Threshold Level minus the hysteresis before the counter resumes counting. When Pause When is Low, the source signal must rise above Threshold Level plus the hysteresis before the counter resumes counting."]
    CountEdges_Gate_Hyst,
    #[doc = "Specifies whether to apply the pulse width filter to the gate input signal."]
    CountEdges_Gate_DigFltrEnable,
    #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
    CountEdges_Gate_DigFltrMinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    CountEdges_Gate_DigFltrTimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    CountEdges_Gate_DigFltrTimebaseRate,
    #[doc = "Specifies whether the counter gates input pulses while the signal is high or low."]
    CountEdges_GateWhen,
    #[doc = "Specifies the input terminal of the signal to measure."]
    DutyCycle_Term,
    #[doc = "Specifies the input terminal configuration."]
    DutyCycle_TermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    DutyCycle_LogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    DutyCycle_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
    DutyCycle_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    DutyCycle_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    DutyCycle_DigFltr_TimebaseRate,
    #[doc = "Specifies which edge of the input signal to begin the duty cycle measurement."]
    DutyCycle_StartingEdge,
    #[doc = "Specifies the units to use to return angular position measurements from the channel."]
    AngEncoder_Units,
    #[doc = "Specifies the number of pulses the encoder generates per revolution. This value is the number of pulses on either signal A or signal B, not the total number of pulses on both signal A and signal B."]
    AngEncoder_PulsesPerRev,
    #[doc = "Specifies the starting angle of the encoder. This value is in the units you specify with Units."]
    AngEncoder_InitialAngle,
    #[doc = "Specifies the units to use to return linear encoder measurements from the channel."]
    LinEncoder_Units,
    #[doc = "Specifies the distance to measure for each pulse the encoder generates on signal A or signal B. This value is in the units you specify with Units."]
    LinEncoder_DistPerPulse,
    #[doc = "Specifies the position of the encoder when the measurement begins. This value is in the units you specify with Units."]
    LinEncoder_InitialPos,
    #[doc = "Specifies how to count and interpret the pulses the encoder generates on signal A and signal B. DAQmx_Val_X1, DAQmx_Val_X2, and DAQmx_Val_X4 are valid for quadrature encoders only. DAQmx_Val_TwoPulseCounting is valid for two-pulse encoders only."]
    Encoder_DecodingType,
    #[doc = "Specifies the terminal to which signal A is connected."]
    Encoder_AInputTerm,
    #[doc = "Specifies the input terminal configuration."]
    Encoder_AInputTermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    Encoder_AInputLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Encoder_AInput_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Encoder_AInput_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Encoder_AInput_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Encoder_AInput_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Encoder_AInput_DigSync_Enable,
    #[doc = "Specifies the terminal to which signal B is connected."]
    Encoder_BInputTerm,
    #[doc = "Specifies the input terminal configuration."]
    Encoder_BInputTermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    Encoder_BInputLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Encoder_BInput_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Encoder_BInput_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Encoder_BInput_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Encoder_BInput_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Encoder_BInput_DigSync_Enable,
    #[doc = "Specifies the terminal to which signal Z is connected."]
    Encoder_ZInputTerm,
    #[doc = "Specifies the input terminal configuration."]
    Encoder_ZInputTermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    Encoder_ZInputLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Encoder_ZInput_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Encoder_ZInput_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Encoder_ZInput_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Encoder_ZInput_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Encoder_ZInput_DigSync_Enable,
    #[doc = "Specifies whether to use Z indexing for the channel."]
    Encoder_ZIndexEnable,
    #[doc = "Specifies the value to which to reset the measurement when signal Z is high and signal A and signal B are at the states you specify with Z Index Phase. Specify this value in the units of the measurement."]
    Encoder_ZIndexVal,
    #[doc = "Specifies the states at which signal A and signal B must be while signal Z is high for NI-DAQmx to reset the measurement. If signal Z is never high while signal A and signal B are high, for example, you must choose a phase other than DAQmx_Val_AHighBHigh."]
    Encoder_ZIndexPhase,
    #[doc = "Specifies the units to use to return pulse width measurements."]
    PulseWidth_Units,
    #[doc = "Specifies the input terminal of the signal to measure."]
    PulseWidth_Term,
    #[doc = "Specifies the input terminal configuration."]
    PulseWidth_TermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    PulseWidth_LogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    PulseWidth_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    PulseWidth_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    PulseWidth_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    PulseWidth_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    PulseWidth_DigSync_Enable,
    #[doc = "Specifies on which edge of the input signal to begin each pulse width measurement."]
    PulseWidth_StartingEdge,
    #[doc = "Specifies the units to use to return timestamp measurements."]
    Timestamp_Units,
    #[doc = "Specifies the number of seconds that elapsed since the beginning of the current year. This value is ignored if  Synchronization Method is DAQmx_Val_IRIGB."]
    Timestamp_InitialSeconds,
    #[doc = "Specifies the method to use to synchronize the counter to a GPS receiver."]
    GPS_SyncMethod,
    #[doc = "Specifies the terminal to which the GPS synchronization signal is connected."]
    GPS_SyncSrc,
    #[doc = "Specifies the units to use to return angular velocity counter measurements."]
    Velocity_AngEncoder_Units,
    #[doc = "Specifies the number of pulses the encoder generates per revolution. This value is the number of pulses on either signal A or signal B, not the total number of pulses on both signal A and signal B."]
    Velocity_AngEncoder_PulsesPerRev,
    #[doc = "Specifies the units to use to return linear encoder velocity measurements from the channel."]
    Velocity_LinEncoder_Units,
    #[doc = "Specifies the distance to measure for each pulse the encoder generates on signal A or signal B. This value is in the units you specify in CI.Velocity.LinEncoder.DistUnits."]
    Velocity_LinEncoder_DistPerPulse,
    #[doc = "Specifies how to count and interpret the pulses the encoder generates on signal A and signal B. X1, X2, and X4 are valid for quadrature encoders only. Two Pulse Counting is valid for two-pulse encoders only."]
    Velocity_Encoder_DecodingType,
    #[doc = "Specifies the terminal to which signal A is connected."]
    Velocity_Encoder_AInputTerm,
    #[doc = "Specifies the input terminal configuration."]
    Velocity_Encoder_AInputTermCfg,
    #[doc = "Specifies the logic level behavior of the input terminal."]
    Velocity_Encoder_AInputLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Velocity_Encoder_AInputDigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
    Velocity_Encoder_AInputDigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Velocity_Encoder_AInputDigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Velocity_Encoder_AInputDigFltr_TimebaseRate,
    #[doc = "Specifies the terminal to which signal B is connected."]
    Velocity_Encoder_BInputTerm,
    #[doc = "Specifies the input terminal configuration."]
    Velocity_Encoder_BInputTermCfg,
    #[doc = "Specifies the logic level behavior of the input terminal."]
    Velocity_Encoder_BInputLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    Velocity_Encoder_BInputDigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
    Velocity_Encoder_BInputDigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    Velocity_Encoder_BInputDigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Velocity_Encoder_BInputDigFltr_TimebaseRate,
    #[doc = "Specifies in seconds the length of time to measure the velocity of the signal."]
    Velocity_MeasTime,
    #[doc = "Specifies the value by which to divide the input signal."]
    Velocity_Div,
    #[doc = "Specifies the units to use to return two-edge separation measurements from the channel."]
    TwoEdgeSep_Units,
    #[doc = "Specifies the source terminal of the digital signal that starts each measurement."]
    TwoEdgeSep_FirstTerm,
    #[doc = "Specifies the input terminal configuration."]
    TwoEdgeSep_FirstTermCfg,
    #[doc = "Specifies the logic level behavior on the input line."]
    TwoEdgeSep_FirstLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    TwoEdgeSep_First_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    TwoEdgeSep_First_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    TwoEdgeSep_First_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    TwoEdgeSep_First_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    TwoEdgeSep_First_DigSync_Enable,
    #[doc = "Specifies on which edge of the first signal to start each measurement."]
    TwoEdgeSep_FirstEdge,
    #[doc = "Specifies the source terminal of the digital signal that stops each measurement."]
    TwoEdgeSep_SecondTerm,
    #[doc = "Specifies the input terminal configuration."]
    TwoEdgeSep_SecondTermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    TwoEdgeSep_SecondLogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    TwoEdgeSep_Second_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    TwoEdgeSep_Second_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    TwoEdgeSep_Second_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    TwoEdgeSep_Second_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    TwoEdgeSep_Second_DigSync_Enable,
    #[doc = "Specifies on which edge of the second signal to stop each measurement."]
    TwoEdgeSep_SecondEdge,
    #[doc = "Specifies the units to use to return semi-period measurements."]
    SemiPeriod_Units,
    #[doc = "Specifies the input terminal of the signal to measure."]
    SemiPeriod_Term,
    #[doc = "Specifies the input terminal configuration."]
    SemiPeriod_TermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    SemiPeriod_LogicLvlBehavior,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    SemiPeriod_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    SemiPeriod_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    SemiPeriod_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    SemiPeriod_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    SemiPeriod_DigSync_Enable,
    #[doc = "Specifies on which edge of the input signal to begin semi-period measurement. Semi-period measurements alternate between high time and low time, starting on this edge."]
    SemiPeriod_StartingEdge,
    #[doc = "Specifies the units to use to return pulse specifications in terms of frequency."]
    Pulse_Freq_Units,
    #[doc = "Specifies the input terminal of the signal to measure."]
    Pulse_Freq_Term,
    #[doc = "Specifies the input terminal configuration."]
    Pulse_Freq_TermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    Pulse_Freq_LogicLvlBehavior,
    #[doc = "Specifies whether to apply a digital filter to the signal to measure."]
    Pulse_Freq_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Pulse_Freq_DigFltr_MinPulseWidth,
    #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
    Pulse_Freq_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Pulse_Freq_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Pulse_Freq_DigSync_Enable,
    #[doc = "Specifies on which edge of the input signal to begin pulse measurement."]
    Pulse_Freq_Start_Edge,
    #[doc = "Specifies the units to use to return pulse specifications in terms of high time and low time."]
    Pulse_Time_Units,
    #[doc = "Specifies the input terminal of the signal to measure."]
    Pulse_Time_Term,
    #[doc = "Specifies the input terminal configuration."]
    Pulse_Time_TermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    Pulse_Time_LogicLvlBehavior,
    #[doc = "Specifies whether to apply a digital filter to the signal to measure."]
    Pulse_Time_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Pulse_Time_DigFltr_MinPulseWidth,
    #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
    Pulse_Time_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Pulse_Time_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Pulse_Time_DigSync_Enable,
    #[doc = "Specifies on which edge of the input signal to begin pulse measurement."]
    Pulse_Time_StartEdge,
    #[doc = "Specifies the input terminal of the signal to measure."]
    Pulse_Ticks_Term,
    #[doc = "Specifies the input terminal configuration."]
    Pulse_Ticks_TermCfg,
    #[doc = "Specifies the logic level behavior on the count reset line."]
    Pulse_Ticks_LogicLvlBehavior,
    #[doc = "Specifies whether to apply a digital filter to the signal to measure."]
    Pulse_Ticks_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    Pulse_Ticks_DigFltr_MinPulseWidth,
    #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
    Pulse_Ticks_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    Pulse_Ticks_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    Pulse_Ticks_DigSync_Enable,
    #[doc = "Specifies on which edge of the input signal to begin pulse measurement."]
    Pulse_Ticks_StartEdge,
    #[doc = "Specifies the terminal of the timebase to use for the counter."]
    CtrTimebaseSrc,
    #[doc = "Specifies in Hertz the frequency of the counter timebase. Specifying the rate of a counter timebase allows you to take measurements in terms of time or frequency rather than in ticks of the timebase. If you use an external timebase and do not specify the rate, you can take measurements only in terms of ticks of the timebase."]
    CtrTimebaseRate,
    #[doc = "Specifies whether a timebase cycle is from rising edge to rising edge or from falling edge to falling edge."]
    CtrTimebaseActiveEdge,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    CtrTimebase_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    CtrTimebase_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    CtrTimebase_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    CtrTimebase_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    CtrTimebase_DigSync_Enable,
    #[doc = "Specifies the digital threshold value in Volts for high and low input transitions. Some devices do not support this for differential channels."]
    ThreshVoltage,
    #[doc = "Specifies the corresponding filter enable/disable state."]
    Filter_Enable,
    #[doc = "Specifies the corresponding filter frequency (cutoff or center) of the filter response."]
    Filter_Freq,
    #[doc = "Specifies the corresponding filter response and defines the shape of the filter response."]
    Filter_Response,
    #[doc = "Specifies the corresponding filter order and defines the slope of the filter response."]
    Filter_Order,
    #[doc = "Indicates the amount of time between when the input signal transitions and when the filtered sample is read by the host device. This value is in the units specified with Filter Delay Units."]
    FilterDelay,
    #[doc = "Specifies the units of Filter Delay."]
    FilterDelayUnits,
    #[doc = "Indicates the current value of the count register."]
    Count,
    #[doc = "Indicates the current state of the out terminal of the counter."]
    OutputState,
    #[doc = "Indicates whether the counter rolled over. When you query this property, NI-DAQmx resets it to FALSE."]
    TCReached,
    #[doc = "Specifies the divisor for an external counter timebase. You can divide the counter timebase in order to measure slower signals without causing the count register to roll over."]
    CtrTimebaseMasterTimebaseDiv,
    #[doc = "Specifies the counter behavior when data is read but a new value was not detected during a sample clock."]
    SampClkOverrunBehavior,
    #[doc = "Specifies the sentinel value returned when the No New Sample Behavior is set to Sentinel Value."]
    SampClkOverrunSentinelVal,
    #[doc = "Specifies the data transfer mode for the channel."]
    DataXferMech,
    #[doc = "Specifies under what condition to transfer data from the onboard memory of the device to the buffer."]
    DataXferReqCond,
    #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqSize,
    #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqCount,
    #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
    MemMapEnable,
    #[doc = "Indicates the number of samples that the device might have overwritten before it could transfer them to the buffer."]
    NumPossiblyInvalidSamps,
    #[doc = "Specifies whether to enable duplicate count prevention for the channel. Duplicate count prevention is enabled by default. Setting  Prescaler disables duplicate count prevention unless you explicitly enable it."]
    DupCountPrevent,
    #[doc = "Specifies the divisor to apply to the signal you connect to the counter source terminal. Scaled data that you read takes this setting into account. You should use a prescaler only when you connect an external signal to the counter source terminal and when that signal has a higher frequency than the fastest onboard timebase. Setting this value disables duplicate count prevention unless you explicitly set Duplicate ..."]
    Prescaler,
    #[doc = "Specifies the maximum period (in seconds) in which the device will recognize signals. For frequency measurements, a signal with a higher period than the one set in this property will return 0 Hz. For duty cycle, the device will return 0 or 1 depending on the state of the line during the max defined period of time. Period measurements will return NaN. Pulse width measurement will return zero."]
    MaxMeasPeriod,
}

impl From<CounterInput> for i32 {
    fn from(attr: CounterInput) -> Self {
        match attr {
            CounterInput::Max => nidaqmx_sys::DAQmx_CI_Max,
            CounterInput::Min => nidaqmx_sys::DAQmx_CI_Min,
            CounterInput::CustomScaleName => nidaqmx_sys::DAQmx_CI_CustomScaleName,
            CounterInput::MeasType => nidaqmx_sys::DAQmx_CI_MeasType,
            CounterInput::Freq_Units => nidaqmx_sys::DAQmx_CI_Freq_Units,
            CounterInput::Freq_Term => nidaqmx_sys::DAQmx_CI_Freq_Term,
            CounterInput::Freq_TermCfg => nidaqmx_sys::DAQmx_CI_Freq_TermCfg,
            CounterInput::Freq_LogicLvlBehavior => nidaqmx_sys::DAQmx_CI_Freq_LogicLvlBehavior,
            CounterInput::Freq_ThreshVoltage => nidaqmx_sys::DAQmx_CI_Freq_ThreshVoltage,
            CounterInput::Freq_Hyst => nidaqmx_sys::DAQmx_CI_Freq_Hyst,
            CounterInput::Freq_DigFltr_Enable => nidaqmx_sys::DAQmx_CI_Freq_DigFltr_Enable,
            CounterInput::Freq_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Freq_DigFltr_MinPulseWidth
            }
            CounterInput::Freq_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Freq_DigFltr_TimebaseSrc
            }
            CounterInput::Freq_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Freq_DigFltr_TimebaseRate
            }
            CounterInput::Freq_DigSync_Enable => nidaqmx_sys::DAQmx_CI_Freq_DigSync_Enable,
            CounterInput::Freq_StartingEdge => nidaqmx_sys::DAQmx_CI_Freq_StartingEdge,
            CounterInput::Freq_MeasMeth => nidaqmx_sys::DAQmx_CI_Freq_MeasMeth,
            CounterInput::Freq_EnableAveraging => nidaqmx_sys::DAQmx_CI_Freq_EnableAveraging,
            CounterInput::Freq_MeasTime => nidaqmx_sys::DAQmx_CI_Freq_MeasTime,
            CounterInput::Freq_Div => nidaqmx_sys::DAQmx_CI_Freq_Div,
            CounterInput::Period_Units => nidaqmx_sys::DAQmx_CI_Period_Units,
            CounterInput::Period_Term => nidaqmx_sys::DAQmx_CI_Period_Term,
            CounterInput::Period_TermCfg => nidaqmx_sys::DAQmx_CI_Period_TermCfg,
            CounterInput::Period_LogicLvlBehavior => nidaqmx_sys::DAQmx_CI_Period_LogicLvlBehavior,
            CounterInput::Period_ThreshVoltage => nidaqmx_sys::DAQmx_CI_Period_ThreshVoltage,
            CounterInput::Period_Hyst => nidaqmx_sys::DAQmx_CI_Period_Hyst,
            CounterInput::Period_DigFltr_Enable => nidaqmx_sys::DAQmx_CI_Period_DigFltr_Enable,
            CounterInput::Period_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Period_DigFltr_MinPulseWidth
            }
            CounterInput::Period_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Period_DigFltr_TimebaseSrc
            }
            CounterInput::Period_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Period_DigFltr_TimebaseRate
            }
            CounterInput::Period_DigSync_Enable => nidaqmx_sys::DAQmx_CI_Period_DigSync_Enable,
            CounterInput::Period_StartingEdge => nidaqmx_sys::DAQmx_CI_Period_StartingEdge,
            CounterInput::Period_MeasMeth => nidaqmx_sys::DAQmx_CI_Period_MeasMeth,
            CounterInput::Period_EnableAveraging => nidaqmx_sys::DAQmx_CI_Period_EnableAveraging,
            CounterInput::Period_MeasTime => nidaqmx_sys::DAQmx_CI_Period_MeasTime,
            CounterInput::Period_Div => nidaqmx_sys::DAQmx_CI_Period_Div,
            CounterInput::CountEdges_Term => nidaqmx_sys::DAQmx_CI_CountEdges_Term,
            CounterInput::CountEdges_TermCfg => nidaqmx_sys::DAQmx_CI_CountEdges_TermCfg,
            CounterInput::CountEdges_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_CountEdges_LogicLvlBehavior
            }
            CounterInput::CountEdges_ThreshVoltage => {
                nidaqmx_sys::DAQmx_CI_CountEdges_ThreshVoltage
            }
            CounterInput::CountEdges_Hyst => nidaqmx_sys::DAQmx_CI_CountEdges_Hyst,
            CounterInput::CountEdges_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_Enable
            }
            CounterInput::CountEdges_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_MinPulseWidth
            }
            CounterInput::CountEdges_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_TimebaseSrc
            }
            CounterInput::CountEdges_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_TimebaseRate
            }
            CounterInput::CountEdges_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_DigSync_Enable
            }
            CounterInput::CountEdges_Dir => nidaqmx_sys::DAQmx_CI_CountEdges_Dir,
            CounterInput::CountEdges_DirTerm => nidaqmx_sys::DAQmx_CI_CountEdges_DirTerm,
            CounterInput::CountEdges_CountDir_TermCfg => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_TermCfg
            }
            CounterInput::CountEdges_CountDir_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_LogicLvlBehavior
            }
            CounterInput::CountEdges_CountDir_ThreshVoltage => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_ThreshVoltage
            }
            CounterInput::CountEdges_CountDir_Hyst => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_Hyst
            }
            CounterInput::CountEdges_CountDir_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_Enable
            }
            CounterInput::CountEdges_CountDir_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_MinPulseWidth
            }
            CounterInput::CountEdges_CountDir_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_TimebaseSrc
            }
            CounterInput::CountEdges_CountDir_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_TimebaseRate
            }
            CounterInput::CountEdges_CountDir_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigSync_Enable
            }
            CounterInput::CountEdges_InitialCnt => nidaqmx_sys::DAQmx_CI_CountEdges_InitialCnt,
            CounterInput::CountEdges_ActiveEdge => nidaqmx_sys::DAQmx_CI_CountEdges_ActiveEdge,
            CounterInput::CountEdges_CountReset_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_Enable
            }
            CounterInput::CountEdges_CountReset_ResetCount => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_ResetCount
            }
            CounterInput::CountEdges_CountReset_Term => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_Term
            }
            CounterInput::CountEdges_CountReset_TermCfg => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_TermCfg
            }
            CounterInput::CountEdges_CountReset_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_LogicLvlBehavior
            }
            CounterInput::CountEdges_CountReset_ThreshVoltage => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_ThreshVoltage
            }
            CounterInput::CountEdges_CountReset_Hyst => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_Hyst
            }
            CounterInput::CountEdges_CountReset_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_Enable
            }
            CounterInput::CountEdges_CountReset_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_MinPulseWidth
            }
            CounterInput::CountEdges_CountReset_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_TimebaseSrc
            }
            CounterInput::CountEdges_CountReset_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_TimebaseRate
            }
            CounterInput::CountEdges_CountReset_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigSync_Enable
            }
            CounterInput::CountEdges_CountReset_ActiveEdge => {
                nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_ActiveEdge
            }
            CounterInput::CountEdges_Gate_Enable => nidaqmx_sys::DAQmx_CI_CountEdges_Gate_Enable,
            CounterInput::CountEdges_Gate_Term => nidaqmx_sys::DAQmx_CI_CountEdges_Gate_Term,
            CounterInput::CountEdges_Gate_TermCfg => nidaqmx_sys::DAQmx_CI_CountEdges_Gate_TermCfg,
            CounterInput::CountEdges_Gate_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_CountEdges_Gate_LogicLvlBehavior
            }
            CounterInput::CountEdges_Gate_ThreshVoltage => {
                nidaqmx_sys::DAQmx_CI_CountEdges_Gate_ThreshVoltage
            }
            CounterInput::CountEdges_Gate_Hyst => nidaqmx_sys::DAQmx_CI_CountEdges_Gate_Hyst,
            CounterInput::CountEdges_Gate_DigFltrEnable => {
                nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrEnable
            }
            CounterInput::CountEdges_Gate_DigFltrMinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrMinPulseWidth
            }
            CounterInput::CountEdges_Gate_DigFltrTimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrTimebaseSrc
            }
            CounterInput::CountEdges_Gate_DigFltrTimebaseRate => {
                nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrTimebaseRate
            }
            CounterInput::CountEdges_GateWhen => nidaqmx_sys::DAQmx_CI_CountEdges_GateWhen,
            CounterInput::DutyCycle_Term => nidaqmx_sys::DAQmx_CI_DutyCycle_Term,
            CounterInput::DutyCycle_TermCfg => nidaqmx_sys::DAQmx_CI_DutyCycle_TermCfg,
            CounterInput::DutyCycle_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_DutyCycle_LogicLvlBehavior
            }
            CounterInput::DutyCycle_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_Enable
            }
            CounterInput::DutyCycle_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_MinPulseWidth
            }
            CounterInput::DutyCycle_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_TimebaseSrc
            }
            CounterInput::DutyCycle_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_TimebaseRate
            }
            CounterInput::DutyCycle_StartingEdge => nidaqmx_sys::DAQmx_CI_DutyCycle_StartingEdge,
            CounterInput::AngEncoder_Units => nidaqmx_sys::DAQmx_CI_AngEncoder_Units,
            CounterInput::AngEncoder_PulsesPerRev => nidaqmx_sys::DAQmx_CI_AngEncoder_PulsesPerRev,
            CounterInput::AngEncoder_InitialAngle => nidaqmx_sys::DAQmx_CI_AngEncoder_InitialAngle,
            CounterInput::LinEncoder_Units => nidaqmx_sys::DAQmx_CI_LinEncoder_Units,
            CounterInput::LinEncoder_DistPerPulse => nidaqmx_sys::DAQmx_CI_LinEncoder_DistPerPulse,
            CounterInput::LinEncoder_InitialPos => nidaqmx_sys::DAQmx_CI_LinEncoder_InitialPos,
            CounterInput::Encoder_DecodingType => nidaqmx_sys::DAQmx_CI_Encoder_DecodingType,
            CounterInput::Encoder_AInputTerm => nidaqmx_sys::DAQmx_CI_Encoder_AInputTerm,
            CounterInput::Encoder_AInputTermCfg => nidaqmx_sys::DAQmx_CI_Encoder_AInputTermCfg,
            CounterInput::Encoder_AInputLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Encoder_AInputLogicLvlBehavior
            }
            CounterInput::Encoder_AInput_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_Enable
            }
            CounterInput::Encoder_AInput_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_MinPulseWidth
            }
            CounterInput::Encoder_AInput_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_TimebaseSrc
            }
            CounterInput::Encoder_AInput_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_TimebaseRate
            }
            CounterInput::Encoder_AInput_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigSync_Enable
            }
            CounterInput::Encoder_BInputTerm => nidaqmx_sys::DAQmx_CI_Encoder_BInputTerm,
            CounterInput::Encoder_BInputTermCfg => nidaqmx_sys::DAQmx_CI_Encoder_BInputTermCfg,
            CounterInput::Encoder_BInputLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Encoder_BInputLogicLvlBehavior
            }
            CounterInput::Encoder_BInput_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_Enable
            }
            CounterInput::Encoder_BInput_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_MinPulseWidth
            }
            CounterInput::Encoder_BInput_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_TimebaseSrc
            }
            CounterInput::Encoder_BInput_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_TimebaseRate
            }
            CounterInput::Encoder_BInput_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigSync_Enable
            }
            CounterInput::Encoder_ZInputTerm => nidaqmx_sys::DAQmx_CI_Encoder_ZInputTerm,
            CounterInput::Encoder_ZInputTermCfg => nidaqmx_sys::DAQmx_CI_Encoder_ZInputTermCfg,
            CounterInput::Encoder_ZInputLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Encoder_ZInputLogicLvlBehavior
            }
            CounterInput::Encoder_ZInput_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_Enable
            }
            CounterInput::Encoder_ZInput_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_MinPulseWidth
            }
            CounterInput::Encoder_ZInput_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_TimebaseSrc
            }
            CounterInput::Encoder_ZInput_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_TimebaseRate
            }
            CounterInput::Encoder_ZInput_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigSync_Enable
            }
            CounterInput::Encoder_ZIndexEnable => nidaqmx_sys::DAQmx_CI_Encoder_ZIndexEnable,
            CounterInput::Encoder_ZIndexVal => nidaqmx_sys::DAQmx_CI_Encoder_ZIndexVal,
            CounterInput::Encoder_ZIndexPhase => nidaqmx_sys::DAQmx_CI_Encoder_ZIndexPhase,
            CounterInput::PulseWidth_Units => nidaqmx_sys::DAQmx_CI_PulseWidth_Units,
            CounterInput::PulseWidth_Term => nidaqmx_sys::DAQmx_CI_PulseWidth_Term,
            CounterInput::PulseWidth_TermCfg => nidaqmx_sys::DAQmx_CI_PulseWidth_TermCfg,
            CounterInput::PulseWidth_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_PulseWidth_LogicLvlBehavior
            }
            CounterInput::PulseWidth_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_Enable
            }
            CounterInput::PulseWidth_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_MinPulseWidth
            }
            CounterInput::PulseWidth_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_TimebaseSrc
            }
            CounterInput::PulseWidth_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_TimebaseRate
            }
            CounterInput::PulseWidth_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_PulseWidth_DigSync_Enable
            }
            CounterInput::PulseWidth_StartingEdge => nidaqmx_sys::DAQmx_CI_PulseWidth_StartingEdge,
            CounterInput::Timestamp_Units => nidaqmx_sys::DAQmx_CI_Timestamp_Units,
            CounterInput::Timestamp_InitialSeconds => {
                nidaqmx_sys::DAQmx_CI_Timestamp_InitialSeconds
            }
            CounterInput::GPS_SyncMethod => nidaqmx_sys::DAQmx_CI_GPS_SyncMethod,
            CounterInput::GPS_SyncSrc => nidaqmx_sys::DAQmx_CI_GPS_SyncSrc,
            CounterInput::Velocity_AngEncoder_Units => {
                nidaqmx_sys::DAQmx_CI_Velocity_AngEncoder_Units
            }
            CounterInput::Velocity_AngEncoder_PulsesPerRev => {
                nidaqmx_sys::DAQmx_CI_Velocity_AngEncoder_PulsesPerRev
            }
            CounterInput::Velocity_LinEncoder_Units => {
                nidaqmx_sys::DAQmx_CI_Velocity_LinEncoder_Units
            }
            CounterInput::Velocity_LinEncoder_DistPerPulse => {
                nidaqmx_sys::DAQmx_CI_Velocity_LinEncoder_DistPerPulse
            }
            CounterInput::Velocity_Encoder_DecodingType => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_DecodingType
            }
            CounterInput::Velocity_Encoder_AInputTerm => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputTerm
            }
            CounterInput::Velocity_Encoder_AInputTermCfg => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputTermCfg
            }
            CounterInput::Velocity_Encoder_AInputLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputLogicLvlBehavior
            }
            CounterInput::Velocity_Encoder_AInputDigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_Enable
            }
            CounterInput::Velocity_Encoder_AInputDigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_MinPulseWidth
            }
            CounterInput::Velocity_Encoder_AInputDigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_TimebaseSrc
            }
            CounterInput::Velocity_Encoder_AInputDigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_TimebaseRate
            }
            CounterInput::Velocity_Encoder_BInputTerm => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputTerm
            }
            CounterInput::Velocity_Encoder_BInputTermCfg => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputTermCfg
            }
            CounterInput::Velocity_Encoder_BInputLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputLogicLvlBehavior
            }
            CounterInput::Velocity_Encoder_BInputDigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_Enable
            }
            CounterInput::Velocity_Encoder_BInputDigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_MinPulseWidth
            }
            CounterInput::Velocity_Encoder_BInputDigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_TimebaseSrc
            }
            CounterInput::Velocity_Encoder_BInputDigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_TimebaseRate
            }
            CounterInput::Velocity_MeasTime => nidaqmx_sys::DAQmx_CI_Velocity_MeasTime,
            CounterInput::Velocity_Div => nidaqmx_sys::DAQmx_CI_Velocity_Div,
            CounterInput::TwoEdgeSep_Units => nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Units,
            CounterInput::TwoEdgeSep_FirstTerm => nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstTerm,
            CounterInput::TwoEdgeSep_FirstTermCfg => nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstTermCfg,
            CounterInput::TwoEdgeSep_FirstLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstLogicLvlBehavior
            }
            CounterInput::TwoEdgeSep_First_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_Enable
            }
            CounterInput::TwoEdgeSep_First_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_MinPulseWidth
            }
            CounterInput::TwoEdgeSep_First_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_TimebaseSrc
            }
            CounterInput::TwoEdgeSep_First_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_TimebaseRate
            }
            CounterInput::TwoEdgeSep_First_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigSync_Enable
            }
            CounterInput::TwoEdgeSep_FirstEdge => nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstEdge,
            CounterInput::TwoEdgeSep_SecondTerm => nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondTerm,
            CounterInput::TwoEdgeSep_SecondTermCfg => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondTermCfg
            }
            CounterInput::TwoEdgeSep_SecondLogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondLogicLvlBehavior
            }
            CounterInput::TwoEdgeSep_Second_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_Enable
            }
            CounterInput::TwoEdgeSep_Second_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_MinPulseWidth
            }
            CounterInput::TwoEdgeSep_Second_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_TimebaseSrc
            }
            CounterInput::TwoEdgeSep_Second_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_TimebaseRate
            }
            CounterInput::TwoEdgeSep_Second_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigSync_Enable
            }
            CounterInput::TwoEdgeSep_SecondEdge => nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondEdge,
            CounterInput::SemiPeriod_Units => nidaqmx_sys::DAQmx_CI_SemiPeriod_Units,
            CounterInput::SemiPeriod_Term => nidaqmx_sys::DAQmx_CI_SemiPeriod_Term,
            CounterInput::SemiPeriod_TermCfg => nidaqmx_sys::DAQmx_CI_SemiPeriod_TermCfg,
            CounterInput::SemiPeriod_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_SemiPeriod_LogicLvlBehavior
            }
            CounterInput::SemiPeriod_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_Enable
            }
            CounterInput::SemiPeriod_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_MinPulseWidth
            }
            CounterInput::SemiPeriod_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_TimebaseSrc
            }
            CounterInput::SemiPeriod_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_TimebaseRate
            }
            CounterInput::SemiPeriod_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_SemiPeriod_DigSync_Enable
            }
            CounterInput::SemiPeriod_StartingEdge => nidaqmx_sys::DAQmx_CI_SemiPeriod_StartingEdge,
            CounterInput::Pulse_Freq_Units => nidaqmx_sys::DAQmx_CI_Pulse_Freq_Units,
            CounterInput::Pulse_Freq_Term => nidaqmx_sys::DAQmx_CI_Pulse_Freq_Term,
            CounterInput::Pulse_Freq_TermCfg => nidaqmx_sys::DAQmx_CI_Pulse_Freq_TermCfg,
            CounterInput::Pulse_Freq_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Pulse_Freq_LogicLvlBehavior
            }
            CounterInput::Pulse_Freq_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_Enable
            }
            CounterInput::Pulse_Freq_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_MinPulseWidth
            }
            CounterInput::Pulse_Freq_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_TimebaseSrc
            }
            CounterInput::Pulse_Freq_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_TimebaseRate
            }
            CounterInput::Pulse_Freq_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigSync_Enable
            }
            CounterInput::Pulse_Freq_Start_Edge => nidaqmx_sys::DAQmx_CI_Pulse_Freq_Start_Edge,
            CounterInput::Pulse_Time_Units => nidaqmx_sys::DAQmx_CI_Pulse_Time_Units,
            CounterInput::Pulse_Time_Term => nidaqmx_sys::DAQmx_CI_Pulse_Time_Term,
            CounterInput::Pulse_Time_TermCfg => nidaqmx_sys::DAQmx_CI_Pulse_Time_TermCfg,
            CounterInput::Pulse_Time_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Pulse_Time_LogicLvlBehavior
            }
            CounterInput::Pulse_Time_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_Enable
            }
            CounterInput::Pulse_Time_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_MinPulseWidth
            }
            CounterInput::Pulse_Time_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_TimebaseSrc
            }
            CounterInput::Pulse_Time_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_TimebaseRate
            }
            CounterInput::Pulse_Time_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_Pulse_Time_DigSync_Enable
            }
            CounterInput::Pulse_Time_StartEdge => nidaqmx_sys::DAQmx_CI_Pulse_Time_StartEdge,
            CounterInput::Pulse_Ticks_Term => nidaqmx_sys::DAQmx_CI_Pulse_Ticks_Term,
            CounterInput::Pulse_Ticks_TermCfg => nidaqmx_sys::DAQmx_CI_Pulse_Ticks_TermCfg,
            CounterInput::Pulse_Ticks_LogicLvlBehavior => {
                nidaqmx_sys::DAQmx_CI_Pulse_Ticks_LogicLvlBehavior
            }
            CounterInput::Pulse_Ticks_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_Enable
            }
            CounterInput::Pulse_Ticks_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_MinPulseWidth
            }
            CounterInput::Pulse_Ticks_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_TimebaseSrc
            }
            CounterInput::Pulse_Ticks_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_TimebaseRate
            }
            CounterInput::Pulse_Ticks_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigSync_Enable
            }
            CounterInput::Pulse_Ticks_StartEdge => nidaqmx_sys::DAQmx_CI_Pulse_Ticks_StartEdge,
            CounterInput::CtrTimebaseSrc => nidaqmx_sys::DAQmx_CI_CtrTimebaseSrc,
            CounterInput::CtrTimebaseRate => nidaqmx_sys::DAQmx_CI_CtrTimebaseRate,
            CounterInput::CtrTimebaseActiveEdge => nidaqmx_sys::DAQmx_CI_CtrTimebaseActiveEdge,
            CounterInput::CtrTimebase_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_Enable
            }
            CounterInput::CtrTimebase_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_MinPulseWidth
            }
            CounterInput::CtrTimebase_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_TimebaseSrc
            }
            CounterInput::CtrTimebase_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_TimebaseRate
            }
            CounterInput::CtrTimebase_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CI_CtrTimebase_DigSync_Enable
            }
            CounterInput::ThreshVoltage => nidaqmx_sys::DAQmx_CI_ThreshVoltage,
            CounterInput::Filter_Enable => nidaqmx_sys::DAQmx_CI_Filter_Enable,
            CounterInput::Filter_Freq => nidaqmx_sys::DAQmx_CI_Filter_Freq,
            CounterInput::Filter_Response => nidaqmx_sys::DAQmx_CI_Filter_Response,
            CounterInput::Filter_Order => nidaqmx_sys::DAQmx_CI_Filter_Order,
            CounterInput::FilterDelay => nidaqmx_sys::DAQmx_CI_FilterDelay,
            CounterInput::FilterDelayUnits => nidaqmx_sys::DAQmx_CI_FilterDelayUnits,
            CounterInput::Count => nidaqmx_sys::DAQmx_CI_Count,
            CounterInput::OutputState => nidaqmx_sys::DAQmx_CI_OutputState,
            CounterInput::TCReached => nidaqmx_sys::DAQmx_CI_TCReached,
            CounterInput::CtrTimebaseMasterTimebaseDiv => {
                nidaqmx_sys::DAQmx_CI_CtrTimebaseMasterTimebaseDiv
            }
            CounterInput::SampClkOverrunBehavior => nidaqmx_sys::DAQmx_CI_SampClkOverrunBehavior,
            CounterInput::SampClkOverrunSentinelVal => {
                nidaqmx_sys::DAQmx_CI_SampClkOverrunSentinelVal
            }
            CounterInput::DataXferMech => nidaqmx_sys::DAQmx_CI_DataXferMech,
            CounterInput::DataXferReqCond => nidaqmx_sys::DAQmx_CI_DataXferReqCond,
            CounterInput::UsbXferReqSize => nidaqmx_sys::DAQmx_CI_UsbXferReqSize,
            CounterInput::UsbXferReqCount => nidaqmx_sys::DAQmx_CI_UsbXferReqCount,
            CounterInput::MemMapEnable => nidaqmx_sys::DAQmx_CI_MemMapEnable,
            CounterInput::NumPossiblyInvalidSamps => nidaqmx_sys::DAQmx_CI_NumPossiblyInvalidSamps,
            CounterInput::DupCountPrevent => nidaqmx_sys::DAQmx_CI_DupCountPrevent,
            CounterInput::Prescaler => nidaqmx_sys::DAQmx_CI_Prescaler,
            CounterInput::MaxMeasPeriod => nidaqmx_sys::DAQmx_CI_MaxMeasPeriod,
        }
    }
}

impl From<i32> for CounterInput {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_CI_Max => CounterInput::Max,
            nidaqmx_sys::DAQmx_CI_Min => CounterInput::Min,
            nidaqmx_sys::DAQmx_CI_CustomScaleName => CounterInput::CustomScaleName,
            nidaqmx_sys::DAQmx_CI_MeasType => CounterInput::MeasType,
            nidaqmx_sys::DAQmx_CI_Freq_Units => CounterInput::Freq_Units,
            nidaqmx_sys::DAQmx_CI_Freq_Term => CounterInput::Freq_Term,
            nidaqmx_sys::DAQmx_CI_Freq_TermCfg => CounterInput::Freq_TermCfg,
            nidaqmx_sys::DAQmx_CI_Freq_LogicLvlBehavior => CounterInput::Freq_LogicLvlBehavior,
            nidaqmx_sys::DAQmx_CI_Freq_ThreshVoltage => CounterInput::Freq_ThreshVoltage,
            nidaqmx_sys::DAQmx_CI_Freq_Hyst => CounterInput::Freq_Hyst,
            nidaqmx_sys::DAQmx_CI_Freq_DigFltr_Enable => CounterInput::Freq_DigFltr_Enable,
            nidaqmx_sys::DAQmx_CI_Freq_DigFltr_MinPulseWidth => {
                CounterInput::Freq_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Freq_DigFltr_TimebaseSrc => {
                CounterInput::Freq_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Freq_DigFltr_TimebaseRate => {
                CounterInput::Freq_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Freq_DigSync_Enable => CounterInput::Freq_DigSync_Enable,
            nidaqmx_sys::DAQmx_CI_Freq_StartingEdge => CounterInput::Freq_StartingEdge,
            nidaqmx_sys::DAQmx_CI_Freq_MeasMeth => CounterInput::Freq_MeasMeth,
            nidaqmx_sys::DAQmx_CI_Freq_EnableAveraging => CounterInput::Freq_EnableAveraging,
            nidaqmx_sys::DAQmx_CI_Freq_MeasTime => CounterInput::Freq_MeasTime,
            nidaqmx_sys::DAQmx_CI_Freq_Div => CounterInput::Freq_Div,
            nidaqmx_sys::DAQmx_CI_Period_Units => CounterInput::Period_Units,
            nidaqmx_sys::DAQmx_CI_Period_Term => CounterInput::Period_Term,
            nidaqmx_sys::DAQmx_CI_Period_TermCfg => CounterInput::Period_TermCfg,
            nidaqmx_sys::DAQmx_CI_Period_LogicLvlBehavior => CounterInput::Period_LogicLvlBehavior,
            nidaqmx_sys::DAQmx_CI_Period_ThreshVoltage => CounterInput::Period_ThreshVoltage,
            nidaqmx_sys::DAQmx_CI_Period_Hyst => CounterInput::Period_Hyst,
            nidaqmx_sys::DAQmx_CI_Period_DigFltr_Enable => CounterInput::Period_DigFltr_Enable,
            nidaqmx_sys::DAQmx_CI_Period_DigFltr_MinPulseWidth => {
                CounterInput::Period_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Period_DigFltr_TimebaseSrc => {
                CounterInput::Period_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Period_DigFltr_TimebaseRate => {
                CounterInput::Period_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Period_DigSync_Enable => CounterInput::Period_DigSync_Enable,
            nidaqmx_sys::DAQmx_CI_Period_StartingEdge => CounterInput::Period_StartingEdge,
            nidaqmx_sys::DAQmx_CI_Period_MeasMeth => CounterInput::Period_MeasMeth,
            nidaqmx_sys::DAQmx_CI_Period_EnableAveraging => CounterInput::Period_EnableAveraging,
            nidaqmx_sys::DAQmx_CI_Period_MeasTime => CounterInput::Period_MeasTime,
            nidaqmx_sys::DAQmx_CI_Period_Div => CounterInput::Period_Div,
            nidaqmx_sys::DAQmx_CI_CountEdges_Term => CounterInput::CountEdges_Term,
            nidaqmx_sys::DAQmx_CI_CountEdges_TermCfg => CounterInput::CountEdges_TermCfg,
            nidaqmx_sys::DAQmx_CI_CountEdges_LogicLvlBehavior => {
                CounterInput::CountEdges_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_ThreshVoltage => {
                CounterInput::CountEdges_ThreshVoltage
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Hyst => CounterInput::CountEdges_Hyst,
            nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_Enable => {
                CounterInput::CountEdges_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_MinPulseWidth => {
                CounterInput::CountEdges_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_TimebaseSrc => {
                CounterInput::CountEdges_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_DigFltr_TimebaseRate => {
                CounterInput::CountEdges_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_DigSync_Enable => {
                CounterInput::CountEdges_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Dir => CounterInput::CountEdges_Dir,
            nidaqmx_sys::DAQmx_CI_CountEdges_DirTerm => CounterInput::CountEdges_DirTerm,
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_TermCfg => {
                CounterInput::CountEdges_CountDir_TermCfg
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_LogicLvlBehavior => {
                CounterInput::CountEdges_CountDir_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_ThreshVoltage => {
                CounterInput::CountEdges_CountDir_ThreshVoltage
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_Hyst => {
                CounterInput::CountEdges_CountDir_Hyst
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_Enable => {
                CounterInput::CountEdges_CountDir_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_MinPulseWidth => {
                CounterInput::CountEdges_CountDir_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_TimebaseSrc => {
                CounterInput::CountEdges_CountDir_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigFltr_TimebaseRate => {
                CounterInput::CountEdges_CountDir_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountDir_DigSync_Enable => {
                CounterInput::CountEdges_CountDir_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_InitialCnt => CounterInput::CountEdges_InitialCnt,
            nidaqmx_sys::DAQmx_CI_CountEdges_ActiveEdge => CounterInput::CountEdges_ActiveEdge,
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_Enable => {
                CounterInput::CountEdges_CountReset_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_ResetCount => {
                CounterInput::CountEdges_CountReset_ResetCount
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_Term => {
                CounterInput::CountEdges_CountReset_Term
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_TermCfg => {
                CounterInput::CountEdges_CountReset_TermCfg
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_LogicLvlBehavior => {
                CounterInput::CountEdges_CountReset_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_ThreshVoltage => {
                CounterInput::CountEdges_CountReset_ThreshVoltage
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_Hyst => {
                CounterInput::CountEdges_CountReset_Hyst
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_Enable => {
                CounterInput::CountEdges_CountReset_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_MinPulseWidth => {
                CounterInput::CountEdges_CountReset_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_TimebaseSrc => {
                CounterInput::CountEdges_CountReset_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigFltr_TimebaseRate => {
                CounterInput::CountEdges_CountReset_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_DigSync_Enable => {
                CounterInput::CountEdges_CountReset_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_CountReset_ActiveEdge => {
                CounterInput::CountEdges_CountReset_ActiveEdge
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_Enable => CounterInput::CountEdges_Gate_Enable,
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_Term => CounterInput::CountEdges_Gate_Term,
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_TermCfg => CounterInput::CountEdges_Gate_TermCfg,
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_LogicLvlBehavior => {
                CounterInput::CountEdges_Gate_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_ThreshVoltage => {
                CounterInput::CountEdges_Gate_ThreshVoltage
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_Hyst => CounterInput::CountEdges_Gate_Hyst,
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrEnable => {
                CounterInput::CountEdges_Gate_DigFltrEnable
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrMinPulseWidth => {
                CounterInput::CountEdges_Gate_DigFltrMinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrTimebaseSrc => {
                CounterInput::CountEdges_Gate_DigFltrTimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_Gate_DigFltrTimebaseRate => {
                CounterInput::CountEdges_Gate_DigFltrTimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_CountEdges_GateWhen => CounterInput::CountEdges_GateWhen,
            nidaqmx_sys::DAQmx_CI_DutyCycle_Term => CounterInput::DutyCycle_Term,
            nidaqmx_sys::DAQmx_CI_DutyCycle_TermCfg => CounterInput::DutyCycle_TermCfg,
            nidaqmx_sys::DAQmx_CI_DutyCycle_LogicLvlBehavior => {
                CounterInput::DutyCycle_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_Enable => {
                CounterInput::DutyCycle_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_MinPulseWidth => {
                CounterInput::DutyCycle_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_TimebaseSrc => {
                CounterInput::DutyCycle_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_DutyCycle_DigFltr_TimebaseRate => {
                CounterInput::DutyCycle_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_DutyCycle_StartingEdge => CounterInput::DutyCycle_StartingEdge,
            nidaqmx_sys::DAQmx_CI_AngEncoder_Units => CounterInput::AngEncoder_Units,
            nidaqmx_sys::DAQmx_CI_AngEncoder_PulsesPerRev => CounterInput::AngEncoder_PulsesPerRev,
            nidaqmx_sys::DAQmx_CI_AngEncoder_InitialAngle => CounterInput::AngEncoder_InitialAngle,
            nidaqmx_sys::DAQmx_CI_LinEncoder_Units => CounterInput::LinEncoder_Units,
            nidaqmx_sys::DAQmx_CI_LinEncoder_DistPerPulse => CounterInput::LinEncoder_DistPerPulse,
            nidaqmx_sys::DAQmx_CI_LinEncoder_InitialPos => CounterInput::LinEncoder_InitialPos,
            nidaqmx_sys::DAQmx_CI_Encoder_DecodingType => CounterInput::Encoder_DecodingType,
            nidaqmx_sys::DAQmx_CI_Encoder_AInputTerm => CounterInput::Encoder_AInputTerm,
            nidaqmx_sys::DAQmx_CI_Encoder_AInputTermCfg => CounterInput::Encoder_AInputTermCfg,
            nidaqmx_sys::DAQmx_CI_Encoder_AInputLogicLvlBehavior => {
                CounterInput::Encoder_AInputLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_Enable => {
                CounterInput::Encoder_AInput_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_MinPulseWidth => {
                CounterInput::Encoder_AInput_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_TimebaseSrc => {
                CounterInput::Encoder_AInput_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigFltr_TimebaseRate => {
                CounterInput::Encoder_AInput_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Encoder_AInput_DigSync_Enable => {
                CounterInput::Encoder_AInput_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_Encoder_BInputTerm => CounterInput::Encoder_BInputTerm,
            nidaqmx_sys::DAQmx_CI_Encoder_BInputTermCfg => CounterInput::Encoder_BInputTermCfg,
            nidaqmx_sys::DAQmx_CI_Encoder_BInputLogicLvlBehavior => {
                CounterInput::Encoder_BInputLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_Enable => {
                CounterInput::Encoder_BInput_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_MinPulseWidth => {
                CounterInput::Encoder_BInput_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_TimebaseSrc => {
                CounterInput::Encoder_BInput_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigFltr_TimebaseRate => {
                CounterInput::Encoder_BInput_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Encoder_BInput_DigSync_Enable => {
                CounterInput::Encoder_BInput_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZInputTerm => CounterInput::Encoder_ZInputTerm,
            nidaqmx_sys::DAQmx_CI_Encoder_ZInputTermCfg => CounterInput::Encoder_ZInputTermCfg,
            nidaqmx_sys::DAQmx_CI_Encoder_ZInputLogicLvlBehavior => {
                CounterInput::Encoder_ZInputLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_Enable => {
                CounterInput::Encoder_ZInput_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_MinPulseWidth => {
                CounterInput::Encoder_ZInput_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_TimebaseSrc => {
                CounterInput::Encoder_ZInput_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigFltr_TimebaseRate => {
                CounterInput::Encoder_ZInput_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZInput_DigSync_Enable => {
                CounterInput::Encoder_ZInput_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_Encoder_ZIndexEnable => CounterInput::Encoder_ZIndexEnable,
            nidaqmx_sys::DAQmx_CI_Encoder_ZIndexVal => CounterInput::Encoder_ZIndexVal,
            nidaqmx_sys::DAQmx_CI_Encoder_ZIndexPhase => CounterInput::Encoder_ZIndexPhase,
            nidaqmx_sys::DAQmx_CI_PulseWidth_Units => CounterInput::PulseWidth_Units,
            nidaqmx_sys::DAQmx_CI_PulseWidth_Term => CounterInput::PulseWidth_Term,
            nidaqmx_sys::DAQmx_CI_PulseWidth_TermCfg => CounterInput::PulseWidth_TermCfg,
            nidaqmx_sys::DAQmx_CI_PulseWidth_LogicLvlBehavior => {
                CounterInput::PulseWidth_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_Enable => {
                CounterInput::PulseWidth_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_MinPulseWidth => {
                CounterInput::PulseWidth_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_TimebaseSrc => {
                CounterInput::PulseWidth_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_PulseWidth_DigFltr_TimebaseRate => {
                CounterInput::PulseWidth_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_PulseWidth_DigSync_Enable => {
                CounterInput::PulseWidth_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_PulseWidth_StartingEdge => CounterInput::PulseWidth_StartingEdge,
            nidaqmx_sys::DAQmx_CI_Timestamp_Units => CounterInput::Timestamp_Units,
            nidaqmx_sys::DAQmx_CI_Timestamp_InitialSeconds => {
                CounterInput::Timestamp_InitialSeconds
            }
            nidaqmx_sys::DAQmx_CI_GPS_SyncMethod => CounterInput::GPS_SyncMethod,
            nidaqmx_sys::DAQmx_CI_GPS_SyncSrc => CounterInput::GPS_SyncSrc,
            nidaqmx_sys::DAQmx_CI_Velocity_AngEncoder_Units => {
                CounterInput::Velocity_AngEncoder_Units
            }
            nidaqmx_sys::DAQmx_CI_Velocity_AngEncoder_PulsesPerRev => {
                CounterInput::Velocity_AngEncoder_PulsesPerRev
            }
            nidaqmx_sys::DAQmx_CI_Velocity_LinEncoder_Units => {
                CounterInput::Velocity_LinEncoder_Units
            }
            nidaqmx_sys::DAQmx_CI_Velocity_LinEncoder_DistPerPulse => {
                CounterInput::Velocity_LinEncoder_DistPerPulse
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_DecodingType => {
                CounterInput::Velocity_Encoder_DecodingType
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputTerm => {
                CounterInput::Velocity_Encoder_AInputTerm
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputTermCfg => {
                CounterInput::Velocity_Encoder_AInputTermCfg
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputLogicLvlBehavior => {
                CounterInput::Velocity_Encoder_AInputLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_Enable => {
                CounterInput::Velocity_Encoder_AInputDigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_MinPulseWidth => {
                CounterInput::Velocity_Encoder_AInputDigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_TimebaseSrc => {
                CounterInput::Velocity_Encoder_AInputDigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_AInputDigFltr_TimebaseRate => {
                CounterInput::Velocity_Encoder_AInputDigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputTerm => {
                CounterInput::Velocity_Encoder_BInputTerm
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputTermCfg => {
                CounterInput::Velocity_Encoder_BInputTermCfg
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputLogicLvlBehavior => {
                CounterInput::Velocity_Encoder_BInputLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_Enable => {
                CounterInput::Velocity_Encoder_BInputDigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_MinPulseWidth => {
                CounterInput::Velocity_Encoder_BInputDigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_TimebaseSrc => {
                CounterInput::Velocity_Encoder_BInputDigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Velocity_Encoder_BInputDigFltr_TimebaseRate => {
                CounterInput::Velocity_Encoder_BInputDigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Velocity_MeasTime => CounterInput::Velocity_MeasTime,
            nidaqmx_sys::DAQmx_CI_Velocity_Div => CounterInput::Velocity_Div,
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Units => CounterInput::TwoEdgeSep_Units,
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstTerm => CounterInput::TwoEdgeSep_FirstTerm,
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstTermCfg => CounterInput::TwoEdgeSep_FirstTermCfg,
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstLogicLvlBehavior => {
                CounterInput::TwoEdgeSep_FirstLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_Enable => {
                CounterInput::TwoEdgeSep_First_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_MinPulseWidth => {
                CounterInput::TwoEdgeSep_First_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_TimebaseSrc => {
                CounterInput::TwoEdgeSep_First_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigFltr_TimebaseRate => {
                CounterInput::TwoEdgeSep_First_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_First_DigSync_Enable => {
                CounterInput::TwoEdgeSep_First_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_FirstEdge => CounterInput::TwoEdgeSep_FirstEdge,
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondTerm => CounterInput::TwoEdgeSep_SecondTerm,
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondTermCfg => {
                CounterInput::TwoEdgeSep_SecondTermCfg
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondLogicLvlBehavior => {
                CounterInput::TwoEdgeSep_SecondLogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_Enable => {
                CounterInput::TwoEdgeSep_Second_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_MinPulseWidth => {
                CounterInput::TwoEdgeSep_Second_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_TimebaseSrc => {
                CounterInput::TwoEdgeSep_Second_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigFltr_TimebaseRate => {
                CounterInput::TwoEdgeSep_Second_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_Second_DigSync_Enable => {
                CounterInput::TwoEdgeSep_Second_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_TwoEdgeSep_SecondEdge => CounterInput::TwoEdgeSep_SecondEdge,
            nidaqmx_sys::DAQmx_CI_SemiPeriod_Units => CounterInput::SemiPeriod_Units,
            nidaqmx_sys::DAQmx_CI_SemiPeriod_Term => CounterInput::SemiPeriod_Term,
            nidaqmx_sys::DAQmx_CI_SemiPeriod_TermCfg => CounterInput::SemiPeriod_TermCfg,
            nidaqmx_sys::DAQmx_CI_SemiPeriod_LogicLvlBehavior => {
                CounterInput::SemiPeriod_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_Enable => {
                CounterInput::SemiPeriod_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_MinPulseWidth => {
                CounterInput::SemiPeriod_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_TimebaseSrc => {
                CounterInput::SemiPeriod_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_SemiPeriod_DigFltr_TimebaseRate => {
                CounterInput::SemiPeriod_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_SemiPeriod_DigSync_Enable => {
                CounterInput::SemiPeriod_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_SemiPeriod_StartingEdge => CounterInput::SemiPeriod_StartingEdge,
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_Units => CounterInput::Pulse_Freq_Units,
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_Term => CounterInput::Pulse_Freq_Term,
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_TermCfg => CounterInput::Pulse_Freq_TermCfg,
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_LogicLvlBehavior => {
                CounterInput::Pulse_Freq_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_Enable => {
                CounterInput::Pulse_Freq_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_MinPulseWidth => {
                CounterInput::Pulse_Freq_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_TimebaseSrc => {
                CounterInput::Pulse_Freq_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigFltr_TimebaseRate => {
                CounterInput::Pulse_Freq_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_DigSync_Enable => {
                CounterInput::Pulse_Freq_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Freq_Start_Edge => CounterInput::Pulse_Freq_Start_Edge,
            nidaqmx_sys::DAQmx_CI_Pulse_Time_Units => CounterInput::Pulse_Time_Units,
            nidaqmx_sys::DAQmx_CI_Pulse_Time_Term => CounterInput::Pulse_Time_Term,
            nidaqmx_sys::DAQmx_CI_Pulse_Time_TermCfg => CounterInput::Pulse_Time_TermCfg,
            nidaqmx_sys::DAQmx_CI_Pulse_Time_LogicLvlBehavior => {
                CounterInput::Pulse_Time_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_Enable => {
                CounterInput::Pulse_Time_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_MinPulseWidth => {
                CounterInput::Pulse_Time_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_TimebaseSrc => {
                CounterInput::Pulse_Time_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Time_DigFltr_TimebaseRate => {
                CounterInput::Pulse_Time_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Time_DigSync_Enable => {
                CounterInput::Pulse_Time_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Time_StartEdge => CounterInput::Pulse_Time_StartEdge,
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_Term => CounterInput::Pulse_Ticks_Term,
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_TermCfg => CounterInput::Pulse_Ticks_TermCfg,
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_LogicLvlBehavior => {
                CounterInput::Pulse_Ticks_LogicLvlBehavior
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_Enable => {
                CounterInput::Pulse_Ticks_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_MinPulseWidth => {
                CounterInput::Pulse_Ticks_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_TimebaseSrc => {
                CounterInput::Pulse_Ticks_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigFltr_TimebaseRate => {
                CounterInput::Pulse_Ticks_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_DigSync_Enable => {
                CounterInput::Pulse_Ticks_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_Pulse_Ticks_StartEdge => CounterInput::Pulse_Ticks_StartEdge,
            nidaqmx_sys::DAQmx_CI_CtrTimebaseSrc => CounterInput::CtrTimebaseSrc,
            nidaqmx_sys::DAQmx_CI_CtrTimebaseRate => CounterInput::CtrTimebaseRate,
            nidaqmx_sys::DAQmx_CI_CtrTimebaseActiveEdge => CounterInput::CtrTimebaseActiveEdge,
            nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_Enable => {
                CounterInput::CtrTimebase_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_MinPulseWidth => {
                CounterInput::CtrTimebase_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_TimebaseSrc => {
                CounterInput::CtrTimebase_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CI_CtrTimebase_DigFltr_TimebaseRate => {
                CounterInput::CtrTimebase_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CI_CtrTimebase_DigSync_Enable => {
                CounterInput::CtrTimebase_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CI_ThreshVoltage => CounterInput::ThreshVoltage,
            nidaqmx_sys::DAQmx_CI_Filter_Enable => CounterInput::Filter_Enable,
            nidaqmx_sys::DAQmx_CI_Filter_Freq => CounterInput::Filter_Freq,
            nidaqmx_sys::DAQmx_CI_Filter_Response => CounterInput::Filter_Response,
            nidaqmx_sys::DAQmx_CI_Filter_Order => CounterInput::Filter_Order,
            nidaqmx_sys::DAQmx_CI_FilterDelay => CounterInput::FilterDelay,
            nidaqmx_sys::DAQmx_CI_FilterDelayUnits => CounterInput::FilterDelayUnits,
            nidaqmx_sys::DAQmx_CI_Count => CounterInput::Count,
            nidaqmx_sys::DAQmx_CI_OutputState => CounterInput::OutputState,
            nidaqmx_sys::DAQmx_CI_TCReached => CounterInput::TCReached,
            nidaqmx_sys::DAQmx_CI_CtrTimebaseMasterTimebaseDiv => {
                CounterInput::CtrTimebaseMasterTimebaseDiv
            }
            nidaqmx_sys::DAQmx_CI_SampClkOverrunBehavior => CounterInput::SampClkOverrunBehavior,
            nidaqmx_sys::DAQmx_CI_SampClkOverrunSentinelVal => {
                CounterInput::SampClkOverrunSentinelVal
            }
            nidaqmx_sys::DAQmx_CI_DataXferMech => CounterInput::DataXferMech,
            nidaqmx_sys::DAQmx_CI_DataXferReqCond => CounterInput::DataXferReqCond,
            nidaqmx_sys::DAQmx_CI_UsbXferReqSize => CounterInput::UsbXferReqSize,
            nidaqmx_sys::DAQmx_CI_UsbXferReqCount => CounterInput::UsbXferReqCount,
            nidaqmx_sys::DAQmx_CI_MemMapEnable => CounterInput::MemMapEnable,
            nidaqmx_sys::DAQmx_CI_NumPossiblyInvalidSamps => CounterInput::NumPossiblyInvalidSamps,
            nidaqmx_sys::DAQmx_CI_DupCountPrevent => CounterInput::DupCountPrevent,
            nidaqmx_sys::DAQmx_CI_Prescaler => CounterInput::Prescaler,
            nidaqmx_sys::DAQmx_CI_MaxMeasPeriod => CounterInput::MaxMeasPeriod,
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum CounterOutput {
    #[doc = "Indicates how to define pulses generated on the channel."]
    OutputType,
    #[doc = "Specifies the resting state of the output terminal."]
    Pulse_IdleState,
    #[doc = "Specifies on which terminal to generate pulses."]
    Pulse_Term,
    #[doc = "Specifies the units in which to define high and low pulse time."]
    Pulse_Time_Units,
    #[doc = "Specifies the amount of time that the pulse is at a high voltage. This value is in the units you specify with Units or when you create the channel."]
    Pulse_HighTime,
    #[doc = "Specifies the amount of time that the pulse is at a low voltage. This value is in the units you specify with Units or when you create the channel."]
    Pulse_LowTime,
    #[doc = "Specifies in seconds the amount of time to wait before generating the first pulse."]
    Pulse_Time_InitialDelay,
    #[doc = "Specifies the duty cycle of the pulses. The duty cycle of a signal is the width of the pulse divided by period. NI-DAQmx uses this ratio and the pulse frequency to determine the width of the pulses and the delay between pulses."]
    Pulse_DutyCyc,
    #[doc = "Specifies the units in which to define pulse frequency."]
    Pulse_Freq_Units,
    #[doc = "Specifies the frequency of the pulses to generate. This value is in the units you specify with Units or when you create the channel."]
    Pulse_Freq,
    #[doc = "Specifies in seconds the amount of time to wait before generating the first pulse."]
    Pulse_Freq_InitialDelay,
    #[doc = "Specifies the number of ticks the pulse is high."]
    Pulse_HighTicks,
    #[doc = "Specifies the number of ticks the pulse is low."]
    Pulse_LowTicks,
    #[doc = "Specifies the number of ticks to wait before generating the first pulse."]
    Pulse_Ticks_InitialDelay,
    #[doc = "Specifies the terminal of the timebase to use for the counter. Typically, NI-DAQmx uses one of the internal counter timebases when generating pulses. Use this property to specify an external timebase and produce custom pulse widths that are not possible using the internal timebases."]
    CtrTimebaseSrc,
    #[doc = "Specifies in Hertz the frequency of the counter timebase. Specifying the rate of a counter timebase allows you to define output pulses in seconds rather than in ticks of the timebase. If you use an external timebase and do not specify the rate, you can define output pulses only in ticks of the timebase."]
    CtrTimebaseRate,
    #[doc = "Specifies whether a timebase cycle is from rising edge to rising edge or from falling edge to falling edge."]
    CtrTimebaseActiveEdge,
    #[doc = "Specifies whether to apply the pulse width filter to the signal."]
    CtrTimebase_DigFltr_Enable,
    #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
    CtrTimebase_DigFltr_MinPulseWidth,
    #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
    CtrTimebase_DigFltr_TimebaseSrc,
    #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
    CtrTimebase_DigFltr_TimebaseRate,
    #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
    CtrTimebase_DigSync_Enable,
    #[doc = "Indicates the current value of the count register."]
    Count,
    #[doc = "Indicates the current state of the output terminal of the counter."]
    OutputState,
    #[doc = "Specifies a number of timebase ticks by which to increase the time spent in the idle state for each successive pulse."]
    AutoIncrCnt,
    #[doc = "Specifies the divisor for an external counter timebase. You can divide the counter timebase in order to generate slower signals without causing the count register to roll over."]
    CtrTimebaseMasterTimebaseDiv,
    #[doc = "Indicates if the task completed pulse generation. Use this value for retriggerable pulse generation when you need to determine if the device generated the current pulse. For retriggerable tasks, when you query this property, NI-DAQmx resets it to FALSE."]
    PulseDone,
    #[doc = "Specifies whether to apply the initial delay to retriggered pulse trains."]
    EnableInitialDelayOnRetrigger,
    #[doc = "Specifies constraints to apply when the counter generates pulses. Constraining the counter reduces the device resources required for counter operation. Constraining the counter can also allow additional analog or counter tasks on the device to run concurrently. For continuous counter tasks, NI-DAQmx consumes no device resources when the counter is constrained. For finite counter tasks, resource use increases with ..."]
    ConstrainedGenMode,
    #[doc = "Specifies whether to write samples directly to the onboard memory of the device, bypassing the memory buffer. Generally, you cannot update onboard memory directly after you start the task. Onboard memory includes data FIFOs."]
    UseOnlyOnBrdMem,
    #[doc = "Specifies the data transfer mode for the device. For buffered operations, use DMA or USB Bulk. For non-buffered operations, use Polled."]
    DataXferMech,
    #[doc = "Specifies under what condition to transfer data from the buffer to the onboard memory of the device."]
    DataXferReqCond,
    #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqSize,
    #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
    UsbXferReqCount,
    #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
    MemMapEnable,
    #[doc = "Specifies the divisor to apply to the signal you connect to the counter source terminal. Pulse generations defined by frequency or time take this setting into account, but pulse generations defined by ticks do not. You should use a prescaler only when you connect an external signal to the counter source terminal and when that signal has a higher frequency than the fastest onboard timebase."]
    Prescaler,
    #[doc = "Indicates whether the counter is ready for new continuous pulse train values."]
    RdyForNewVal,
}

impl From<CounterOutput> for i32 {
    fn from(attr: CounterOutput) -> Self {
        match attr {
            CounterOutput::OutputType => nidaqmx_sys::DAQmx_CO_OutputType,
            CounterOutput::Pulse_IdleState => nidaqmx_sys::DAQmx_CO_Pulse_IdleState,
            CounterOutput::Pulse_Term => nidaqmx_sys::DAQmx_CO_Pulse_Term,
            CounterOutput::Pulse_Time_Units => nidaqmx_sys::DAQmx_CO_Pulse_Time_Units,
            CounterOutput::Pulse_HighTime => nidaqmx_sys::DAQmx_CO_Pulse_HighTime,
            CounterOutput::Pulse_LowTime => nidaqmx_sys::DAQmx_CO_Pulse_LowTime,
            CounterOutput::Pulse_Time_InitialDelay => nidaqmx_sys::DAQmx_CO_Pulse_Time_InitialDelay,
            CounterOutput::Pulse_DutyCyc => nidaqmx_sys::DAQmx_CO_Pulse_DutyCyc,
            CounterOutput::Pulse_Freq_Units => nidaqmx_sys::DAQmx_CO_Pulse_Freq_Units,
            CounterOutput::Pulse_Freq => nidaqmx_sys::DAQmx_CO_Pulse_Freq,
            CounterOutput::Pulse_Freq_InitialDelay => nidaqmx_sys::DAQmx_CO_Pulse_Freq_InitialDelay,
            CounterOutput::Pulse_HighTicks => nidaqmx_sys::DAQmx_CO_Pulse_HighTicks,
            CounterOutput::Pulse_LowTicks => nidaqmx_sys::DAQmx_CO_Pulse_LowTicks,
            CounterOutput::Pulse_Ticks_InitialDelay => {
                nidaqmx_sys::DAQmx_CO_Pulse_Ticks_InitialDelay
            }
            CounterOutput::CtrTimebaseSrc => nidaqmx_sys::DAQmx_CO_CtrTimebaseSrc,
            CounterOutput::CtrTimebaseRate => nidaqmx_sys::DAQmx_CO_CtrTimebaseRate,
            CounterOutput::CtrTimebaseActiveEdge => nidaqmx_sys::DAQmx_CO_CtrTimebaseActiveEdge,
            CounterOutput::CtrTimebase_DigFltr_Enable => {
                nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_Enable
            }
            CounterOutput::CtrTimebase_DigFltr_MinPulseWidth => {
                nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_MinPulseWidth
            }
            CounterOutput::CtrTimebase_DigFltr_TimebaseSrc => {
                nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_TimebaseSrc
            }
            CounterOutput::CtrTimebase_DigFltr_TimebaseRate => {
                nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_TimebaseRate
            }
            CounterOutput::CtrTimebase_DigSync_Enable => {
                nidaqmx_sys::DAQmx_CO_CtrTimebase_DigSync_Enable
            }
            CounterOutput::Count => nidaqmx_sys::DAQmx_CO_Count,
            CounterOutput::OutputState => nidaqmx_sys::DAQmx_CO_OutputState,
            CounterOutput::AutoIncrCnt => nidaqmx_sys::DAQmx_CO_AutoIncrCnt,
            CounterOutput::CtrTimebaseMasterTimebaseDiv => {
                nidaqmx_sys::DAQmx_CO_CtrTimebaseMasterTimebaseDiv
            }
            CounterOutput::PulseDone => nidaqmx_sys::DAQmx_CO_PulseDone,
            CounterOutput::EnableInitialDelayOnRetrigger => {
                nidaqmx_sys::DAQmx_CO_EnableInitialDelayOnRetrigger
            }
            CounterOutput::ConstrainedGenMode => nidaqmx_sys::DAQmx_CO_ConstrainedGenMode,
            CounterOutput::UseOnlyOnBrdMem => nidaqmx_sys::DAQmx_CO_UseOnlyOnBrdMem,
            CounterOutput::DataXferMech => nidaqmx_sys::DAQmx_CO_DataXferMech,
            CounterOutput::DataXferReqCond => nidaqmx_sys::DAQmx_CO_DataXferReqCond,
            CounterOutput::UsbXferReqSize => nidaqmx_sys::DAQmx_CO_UsbXferReqSize,
            CounterOutput::UsbXferReqCount => nidaqmx_sys::DAQmx_CO_UsbXferReqCount,
            CounterOutput::MemMapEnable => nidaqmx_sys::DAQmx_CO_MemMapEnable,
            CounterOutput::Prescaler => nidaqmx_sys::DAQmx_CO_Prescaler,
            CounterOutput::RdyForNewVal => nidaqmx_sys::DAQmx_CO_RdyForNewVal,
        }
    }
}

impl From<i32> for CounterOutput {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_CO_OutputType => CounterOutput::OutputType,
            nidaqmx_sys::DAQmx_CO_Pulse_IdleState => CounterOutput::Pulse_IdleState,
            nidaqmx_sys::DAQmx_CO_Pulse_Term => CounterOutput::Pulse_Term,
            nidaqmx_sys::DAQmx_CO_Pulse_Time_Units => CounterOutput::Pulse_Time_Units,
            nidaqmx_sys::DAQmx_CO_Pulse_HighTime => CounterOutput::Pulse_HighTime,
            nidaqmx_sys::DAQmx_CO_Pulse_LowTime => CounterOutput::Pulse_LowTime,
            nidaqmx_sys::DAQmx_CO_Pulse_Time_InitialDelay => CounterOutput::Pulse_Time_InitialDelay,
            nidaqmx_sys::DAQmx_CO_Pulse_DutyCyc => CounterOutput::Pulse_DutyCyc,
            nidaqmx_sys::DAQmx_CO_Pulse_Freq_Units => CounterOutput::Pulse_Freq_Units,
            nidaqmx_sys::DAQmx_CO_Pulse_Freq => CounterOutput::Pulse_Freq,
            nidaqmx_sys::DAQmx_CO_Pulse_Freq_InitialDelay => CounterOutput::Pulse_Freq_InitialDelay,
            nidaqmx_sys::DAQmx_CO_Pulse_HighTicks => CounterOutput::Pulse_HighTicks,
            nidaqmx_sys::DAQmx_CO_Pulse_LowTicks => CounterOutput::Pulse_LowTicks,
            nidaqmx_sys::DAQmx_CO_Pulse_Ticks_InitialDelay => {
                CounterOutput::Pulse_Ticks_InitialDelay
            }
            nidaqmx_sys::DAQmx_CO_CtrTimebaseSrc => CounterOutput::CtrTimebaseSrc,
            nidaqmx_sys::DAQmx_CO_CtrTimebaseRate => CounterOutput::CtrTimebaseRate,
            nidaqmx_sys::DAQmx_CO_CtrTimebaseActiveEdge => CounterOutput::CtrTimebaseActiveEdge,
            nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_Enable => {
                CounterOutput::CtrTimebase_DigFltr_Enable
            }
            nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_MinPulseWidth => {
                CounterOutput::CtrTimebase_DigFltr_MinPulseWidth
            }
            nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_TimebaseSrc => {
                CounterOutput::CtrTimebase_DigFltr_TimebaseSrc
            }
            nidaqmx_sys::DAQmx_CO_CtrTimebase_DigFltr_TimebaseRate => {
                CounterOutput::CtrTimebase_DigFltr_TimebaseRate
            }
            nidaqmx_sys::DAQmx_CO_CtrTimebase_DigSync_Enable => {
                CounterOutput::CtrTimebase_DigSync_Enable
            }
            nidaqmx_sys::DAQmx_CO_Count => CounterOutput::Count,
            nidaqmx_sys::DAQmx_CO_OutputState => CounterOutput::OutputState,
            nidaqmx_sys::DAQmx_CO_AutoIncrCnt => CounterOutput::AutoIncrCnt,
            nidaqmx_sys::DAQmx_CO_CtrTimebaseMasterTimebaseDiv => {
                CounterOutput::CtrTimebaseMasterTimebaseDiv
            }
            nidaqmx_sys::DAQmx_CO_PulseDone => CounterOutput::PulseDone,
            nidaqmx_sys::DAQmx_CO_EnableInitialDelayOnRetrigger => {
                CounterOutput::EnableInitialDelayOnRetrigger
            }
            nidaqmx_sys::DAQmx_CO_ConstrainedGenMode => CounterOutput::ConstrainedGenMode,
            nidaqmx_sys::DAQmx_CO_UseOnlyOnBrdMem => CounterOutput::UseOnlyOnBrdMem,
            nidaqmx_sys::DAQmx_CO_DataXferMech => CounterOutput::DataXferMech,
            nidaqmx_sys::DAQmx_CO_DataXferReqCond => CounterOutput::DataXferReqCond,
            nidaqmx_sys::DAQmx_CO_UsbXferReqSize => CounterOutput::UsbXferReqSize,
            nidaqmx_sys::DAQmx_CO_UsbXferReqCount => CounterOutput::UsbXferReqCount,
            nidaqmx_sys::DAQmx_CO_MemMapEnable => CounterOutput::MemMapEnable,
            nidaqmx_sys::DAQmx_CO_Prescaler => CounterOutput::Prescaler,
            nidaqmx_sys::DAQmx_CO_RdyForNewVal => CounterOutput::RdyForNewVal,
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum Channel {
    #[doc = "Specifies the constant output voltage, in volts. Can be set while a task is running. Can be read at any time during a task."]
    Pwr_Voltage_Setpoint,
    #[doc = "Indicates the coefficients of the polynomial equation that NI-DAQmx uses to scale values from the native format of the device to volts. Can be read at any time during a task."]
    Pwr_Voltage_DevScalingCoeff,
    #[doc = "Specifies the output current, in amperes. If the load draws current greater than the specified value, the device will operate in Constant Current mode."]
    Pwr_Current_Setpoint,
    #[doc = "Indicates the coefficients of the polynomial equation that NI-DAQmx uses to scale values from the native format of the device to amperes. Can be read at any time during a task."]
    Pwr_Current_DevScalingCoeff,
    #[doc = "Specifies whether to enable or disable power module output. Can be set while a task is running. Can be read at any time during a task. When a task is running, the output is enabled immediately. Otherwise, the output is not enabled until the task enters the Committed state."]
    Pwr_OutputEnable,
    #[doc = "Indicates power channel operating state. Can be read at any time during a task."]
    Pwr_OutputState,
    #[doc = "Specifies whether to disable the output or maintain the existing value after the task is uncommitted."]
    Pwr_IdleOutputBehavior,
    #[doc = "Specifies whether to use local or remote sense to sense the output voltage. DAQmx Read (Power) will return remote or local voltage based on the Remote Sense attribute value. Reading this property will return the user-defined value."]
    Pwr_RemoteSense,
    #[doc = "Indicates the type of the virtual channel."]
    ChanType,
    #[doc = "Specifies the name of the physical channel upon which this virtual channel is based."]
    PhysicalChanName,
    #[doc = "Specifies a user-defined description for the channel."]
    ChanDescr,
    #[doc = "Indicates whether the channel is a global channel."]
    ChanIsGlobal,
    #[doc = "Specifies the action to take if the target loses its synchronization to the grand master."]
    Chan_SyncUnlockBehavior,
}

impl From<Channel> for i32 {
    fn from(attr: Channel) -> Self {
        match attr {
            Channel::Pwr_Voltage_Setpoint => nidaqmx_sys::DAQmx_Pwr_Voltage_Setpoint,
            Channel::Pwr_Voltage_DevScalingCoeff => nidaqmx_sys::DAQmx_Pwr_Voltage_DevScalingCoeff,
            Channel::Pwr_Current_Setpoint => nidaqmx_sys::DAQmx_Pwr_Current_Setpoint,
            Channel::Pwr_Current_DevScalingCoeff => nidaqmx_sys::DAQmx_Pwr_Current_DevScalingCoeff,
            Channel::Pwr_OutputEnable => nidaqmx_sys::DAQmx_Pwr_OutputEnable,
            Channel::Pwr_OutputState => nidaqmx_sys::DAQmx_Pwr_OutputState,
            Channel::Pwr_IdleOutputBehavior => nidaqmx_sys::DAQmx_Pwr_IdleOutputBehavior,
            Channel::Pwr_RemoteSense => nidaqmx_sys::DAQmx_Pwr_RemoteSense,
            Channel::ChanType => nidaqmx_sys::DAQmx_ChanType,
            Channel::PhysicalChanName => nidaqmx_sys::DAQmx_PhysicalChanName,
            Channel::ChanDescr => nidaqmx_sys::DAQmx_ChanDescr,
            Channel::ChanIsGlobal => nidaqmx_sys::DAQmx_ChanIsGlobal,
            Channel::Chan_SyncUnlockBehavior => nidaqmx_sys::DAQmx_Chan_SyncUnlockBehavior,
        }
    }
}

impl From<i32> for Channel {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_Pwr_Voltage_Setpoint => Channel::Pwr_Voltage_Setpoint,
            nidaqmx_sys::DAQmx_Pwr_Voltage_DevScalingCoeff => Channel::Pwr_Voltage_DevScalingCoeff,
            nidaqmx_sys::DAQmx_Pwr_Current_Setpoint => Channel::Pwr_Current_Setpoint,
            nidaqmx_sys::DAQmx_Pwr_Current_DevScalingCoeff => Channel::Pwr_Current_DevScalingCoeff,
            nidaqmx_sys::DAQmx_Pwr_OutputEnable => Channel::Pwr_OutputEnable,
            nidaqmx_sys::DAQmx_Pwr_OutputState => Channel::Pwr_OutputState,
            nidaqmx_sys::DAQmx_Pwr_IdleOutputBehavior => Channel::Pwr_IdleOutputBehavior,
            nidaqmx_sys::DAQmx_Pwr_RemoteSense => Channel::Pwr_RemoteSense,
            nidaqmx_sys::DAQmx_ChanType => Channel::ChanType,
            nidaqmx_sys::DAQmx_PhysicalChanName => Channel::PhysicalChanName,
            nidaqmx_sys::DAQmx_ChanDescr => Channel::ChanDescr,
            nidaqmx_sys::DAQmx_ChanIsGlobal => Channel::ChanIsGlobal,
            nidaqmx_sys::DAQmx_Chan_SyncUnlockBehavior => Channel::Chan_SyncUnlockBehavior,
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, FromRepr)]
#[strum(serialize_all = "kebab_case")]
#[repr(i32)]
pub enum DeviceAttr {
    #[doc = "Indicates if the device is a simulated device."]
    IsSimulated,
    #[doc = "Indicates the product category of the device. This category corresponds to the category displayed in MAX when creating NI-DAQmx simulated devices."]
    ProductCategory,
    #[doc = "Indicates the product name of the device."]
    ProductType,
    #[doc = "Indicates the unique hardware identification number for the device."]
    ProductNum,
    #[doc = "Indicates the serial number of the device. This value is zero if the device does not have a serial number."]
    SerialNum,
    #[doc = "Indicates the model names of accessories connected to the device. Each array element corresponds to a connector. For example, index 0 corresponds to connector 0. The array contains an empty string for each connector with no accessory connected."]
    Accessory_ProductTypes,
    #[doc = "Indicates the unique hardware identification number for accessories connected to the device. Each array element corresponds to a connector. For example, index 0 corresponds to connector 0. The array contains 0 for each connector with no accessory connected."]
    Accessory_ProductNums,
    #[doc = "Indicates the serial number for accessories connected to the device. Each array element corresponds to a connector. For example, index 0 corresponds to connector 0. The array contains 0 for each connector with no accessory connected."]
    Accessory_SerialNums,
    #[doc = "Indicates the serial number of the device carrier. This value is zero if the carrier does not have a serial number."]
    Carrier_SerialNum,
    #[doc = "Indicates the parent device which this bank is located in."]
    FieldDAQ_DevName,
    #[doc = "Indicates an array containing the names of the banks in the FieldDAQ."]
    FieldDAQ_BankDevNames,
    #[doc = "Indicates an array containing the names of the modules in the chassis."]
    Chassis_ModuleDevNames,
    #[doc = "Indicates if the device supports analog triggering."]
    AnlgTrigSupported,
    #[doc = "Indicates if the device supports digital triggering."]
    DigTrigSupported,
    #[doc = "Indicates whether the device supports time triggering."]
    TimeTrigSupported,
    #[doc = "Indicates an array containing the names of the analog input physical channels available on the device."]
    AI_PhysicalChans,
    #[doc = "Indicates the measurement types supported by the physical channels of the device. Refer to Measurement Types for information on specific channels."]
    AI_SupportedMeasTypes,
    #[doc = "Indicates the maximum rate for an analog input task if the task contains only a single channel from this device."]
    AI_MaxSingleChanRate,
    #[doc = "Indicates the maximum sampling rate for an analog input task from this device. To find the maximum rate for the task, take the minimum of Maximum Single Channel Rate or the indicated sampling rate of this device divided by the number of channels to acquire data from (including cold-junction compensation and autozero channels)."]
    AI_MaxMultiChanRate,
    #[doc = "Indicates the minimum rate for an analog input task on this device. NI-DAQmx returns a warning or error if you attempt to sample at a slower rate."]
    AI_MinRate,
    #[doc = "Indicates if the device supports simultaneous sampling."]
    AI_SimultaneousSamplingSupported,
    #[doc = "Indicates the number of Analog Input sample timing engines supported by the device."]
    AI_NumSampTimingEngines,
    #[doc = "Indicates sample modes supported by devices that support sample clocked analog input."]
    AI_SampModes,
    #[doc = "Indicates the number of Analog Input synchronization pulse sources supported by the device."]
    AI_NumSyncPulseSrcs,
    #[doc = "Indicates the triggers supported by this device for an analog input task."]
    AI_TrigUsage,
    #[doc = "Indicates pairs of input voltage ranges supported by this device. Each pair consists of the low value, followed by the high value."]
    AI_VoltageRngs,
    #[doc = "Indicates the set of discrete internal voltage excitation values supported by this device. If the device supports ranges of internal excitation values, use Range Values to determine supported excitation values."]
    AI_VoltageIntExcitDiscreteVals,
    #[doc = "Indicates pairs of internal voltage excitation ranges supported by this device. Each pair consists of the low value, followed by the high value. If the device supports a set of discrete internal excitation values, use Discrete Values to determine the supported excitation values."]
    AI_VoltageIntExcitRangeVals,
    #[doc = "Indicates in coulombs pairs of input charge ranges for the device. Each pair consists of the low value followed by the high value."]
    AI_ChargeRngs,
    #[doc = "Indicates the pairs of current input ranges supported by this device. Each pair consists of the low value, followed by the high value."]
    AI_CurrentRngs,
    #[doc = "Indicates the set of discrete internal current excitation values supported by this device."]
    AI_CurrentIntExcitDiscreteVals,
    #[doc = "Indicates pairs of input voltage ratio ranges, in volts per volt, supported by devices that acquire using ratiometric measurements. Each pair consists of the low value followed by the high value."]
    AI_BridgeRngs,
    #[doc = "Indicates pairs of input resistance ranges, in ohms, supported by devices that have the necessary signal conditioning to measure resistances. Each pair consists of the low value followed by the high value."]
    AI_ResistanceRngs,
    #[doc = "Indicates the pairs of frequency input ranges supported by this device. Each pair consists of the low value, followed by the high value."]
    AI_FreqRngs,
    #[doc = "Indicates the input gain settings supported by this device."]
    AI_Gains,
    #[doc = "Indicates the coupling types supported by this device."]
    AI_Couplings,
    #[doc = "Indicates the set of discrete lowpass cutoff frequencies supported by this device. If the device supports ranges of lowpass cutoff frequencies, use Range Values to determine supported frequencies."]
    AI_LowpassCutoffFreqDiscreteVals,
    #[doc = "Indicates pairs of lowpass cutoff frequency ranges supported by this device. Each pair consists of the low value, followed by the high value. If the device supports a set of discrete lowpass cutoff frequencies, use Discrete Values to determine the supported  frequencies."]
    AI_LowpassCutoffFreqRangeVals,
    #[doc = "Indicates the AI digital filter types supported by the device."]
    AI_DigFltr_Types,
    #[doc = "Indicates the set of discrete lowpass cutoff frequencies supported by this device. If the device supports ranges of lowpass cutoff frequencies, use AI.DigFltr.Lowpass.CutoffFreq.RangeVals to determine supported frequencies."]
    AI_DigFltr_LowpassCutoffFreqDiscreteVals,
    #[doc = "Indicates pairs of lowpass cutoff frequency ranges supported by this device. Each pair consists of the low value, followed by the high value. If the device supports a set of discrete lowpass cutoff frequencies, use AI.DigFltr.Lowpass.CutoffFreq.DiscreteVals to determine the supported frequencies."]
    AI_DigFltr_LowpassCutoffFreqRangeVals,
    #[doc = "Indicates an array containing the names of the analog output physical channels available on the device."]
    AO_PhysicalChans,
    #[doc = "Indicates the generation types supported by the physical channels of the device. Refer to Output Types for information on specific channels."]
    AO_SupportedOutputTypes,
    #[doc = "Indicates the maximum analog output rate of the device."]
    AO_MaxRate,
    #[doc = "Indicates the minimum analog output rate of the device."]
    AO_MinRate,
    #[doc = "Indicates if the device supports the sample clock timing  type for analog output tasks."]
    AO_SampClkSupported,
    #[doc = "Indicates the number of Analog Output sample timing engines supported by the device."]
    AO_NumSampTimingEngines,
    #[doc = "Indicates sample modes supported by devices that support sample clocked analog output."]
    AO_SampModes,
    #[doc = "Indicates the number of Analog Output synchronization pulse sources supported by the device."]
    AO_NumSyncPulseSrcs,
    #[doc = "Indicates the triggers supported by this device for analog output tasks."]
    AO_TrigUsage,
    #[doc = "Indicates pairs of output voltage ranges supported by this device. Each pair consists of the low value, followed by the high value."]
    AO_VoltageRngs,
    #[doc = "Indicates pairs of output current ranges supported by this device. Each pair consists of the low value, followed by the high value."]
    AO_CurrentRngs,
    #[doc = "Indicates the output gain settings supported by this device."]
    AO_Gains,
    #[doc = "Indicates an array containing the names of the digital input lines available on the device."]
    DI_Lines,
    #[doc = "Indicates an array containing the names of the digital input ports available on the device."]
    DI_Ports,
    #[doc = "Indicates the maximum digital input rate of the device."]
    DI_MaxRate,
    #[doc = "Indicates the number of Digital Input sample timing engines supported by the device."]
    DI_NumSampTimingEngines,
    #[doc = "Indicates the triggers supported by this device for digital input tasks."]
    DI_TrigUsage,
    #[doc = "Indicates an array containing the names of the digital output lines available on the device."]
    DO_Lines,
    #[doc = "Indicates an array containing the names of the digital output ports available on the device."]
    DO_Ports,
    #[doc = "Indicates the maximum digital output rate of the device."]
    DO_MaxRate,
    #[doc = "Indicates the number of Digital Output synchronization pulse sources supported by the device."]
    DO_NumSampTimingEngines,
    #[doc = "Indicates the triggers supported by this device for digital output tasks."]
    DO_TrigUsage,
    #[doc = "Indicates an array containing the names of the counter input physical channels available on the device."]
    CI_PhysicalChans,
    #[doc = "Indicates the measurement types supported by the physical channels of the device. Refer to Measurement Types for information on specific channels."]
    CI_SupportedMeasTypes,
    #[doc = "Indicates the triggers supported by this device for counter input tasks."]
    CI_TrigUsage,
    #[doc = "Indicates if the device supports the sample clock timing type for counter input tasks."]
    CI_SampClkSupported,
    #[doc = "Indicates sample modes supported by devices that support sample clocked counter input."]
    CI_SampModes,
    #[doc = "Indicates in bits the size of the counters on the device."]
    CI_MaxSize,
    #[doc = "Indicates in hertz the maximum counter timebase frequency."]
    CI_MaxTimebase,
    #[doc = "Indicates an array containing the names of the counter output physical channels available on the device."]
    CO_PhysicalChans,
    #[doc = "Indicates the generation types supported by the physical channels of the device. Refer to Output Types for information on specific channels."]
    CO_SupportedOutputTypes,
    #[doc = "Indicates if the device supports Sample Clock timing for counter output tasks."]
    CO_SampClkSupported,
    #[doc = "Indicates sample modes supported by devices that support sample clocked counter output."]
    CO_SampModes,
    #[doc = "Indicates the triggers supported by this device for counter output tasks."]
    CO_TrigUsage,
    #[doc = "Indicates in bits the size of the counters on the device."]
    CO_MaxSize,
    #[doc = "Indicates in hertz the maximum counter timebase frequency."]
    CO_MaxTimebase,
    #[doc = "Indicates whether the device supports hardware TEDS."]
    TEDS_HWTEDSSupported,
    #[doc = "Indicates the number of DMA channels on the device."]
    NumDMAChans,
    #[doc = "Indicates the bus type of the device."]
    BusType,
    #[doc = "Indicates the PCI bus number of the device."]
    PCI_BusNum,
    #[doc = "Indicates the PCI slot number of the device."]
    PCI_DevNum,
    #[doc = "Indicates the PXI chassis number of the device, as identified in MAX."]
    PXI_ChassisNum,
    #[doc = "Indicates the PXI slot number of the device."]
    PXI_SlotNum,
    #[doc = "Indicates the name of the CompactDAQ chassis that contains this module."]
    CompactDAQ_ChassisDevName,
    #[doc = "Indicates the slot number in which this module is located in the CompactDAQ chassis."]
    CompactDAQ_SlotNum,
    #[doc = "Indicates the name of the CompactRIO chassis that contains this module."]
    CompactRIO_ChassisDevName,
    #[doc = "Indicates the slot number of the CompactRIO chassis where this module is located."]
    CompactRIO_SlotNum,
    #[doc = "Indicates the IPv4 hostname of the device."]
    TCPIP_Hostname,
    #[doc = "Indicates the IPv4 address of the Ethernet interface in dotted decimal format. This property returns 0.0.0.0 if the Ethernet interface cannot acquire an address."]
    TCPIP_EthernetIP,
    #[doc = "Indicates the IPv4 address of the 802.11 wireless interface in dotted decimal format. This property returns 0.0.0.0 if the wireless interface cannot acquire an address."]
    TCPIP_WirelessIP,
    #[doc = "Indicates a list of all terminals on the device."]
    Terminals,
    #[doc = "Indicates the number of time triggers available on the device."]
    NumTimeTrigs,
    #[doc = "Indicates the number of timestamp engines available on the device."]
    NumTimestampEngines,
}

impl From<DeviceAttr> for i32 {
    fn from(attr: DeviceAttr) -> Self {
        match attr {
            DeviceAttr::IsSimulated => nidaqmx_sys::DAQmx_Dev_IsSimulated,
            DeviceAttr::ProductCategory => nidaqmx_sys::DAQmx_Dev_ProductCategory,
            DeviceAttr::ProductType => nidaqmx_sys::DAQmx_Dev_ProductType,
            DeviceAttr::ProductNum => nidaqmx_sys::DAQmx_Dev_ProductNum,
            DeviceAttr::SerialNum => nidaqmx_sys::DAQmx_Dev_SerialNum,
            DeviceAttr::Accessory_ProductTypes => nidaqmx_sys::DAQmx_Dev_Accessory_ProductTypes,
            DeviceAttr::Accessory_ProductNums => nidaqmx_sys::DAQmx_Dev_Accessory_ProductNums,
            DeviceAttr::Accessory_SerialNums => nidaqmx_sys::DAQmx_Dev_Accessory_SerialNums,
            DeviceAttr::Carrier_SerialNum => nidaqmx_sys::DAQmx_Carrier_SerialNum,
            DeviceAttr::FieldDAQ_DevName => nidaqmx_sys::DAQmx_FieldDAQ_DevName,
            DeviceAttr::FieldDAQ_BankDevNames => nidaqmx_sys::DAQmx_FieldDAQ_BankDevNames,
            DeviceAttr::Chassis_ModuleDevNames => nidaqmx_sys::DAQmx_Dev_Chassis_ModuleDevNames,
            DeviceAttr::AnlgTrigSupported => nidaqmx_sys::DAQmx_Dev_AnlgTrigSupported,
            DeviceAttr::DigTrigSupported => nidaqmx_sys::DAQmx_Dev_DigTrigSupported,
            DeviceAttr::TimeTrigSupported => nidaqmx_sys::DAQmx_Dev_TimeTrigSupported,
            DeviceAttr::AI_PhysicalChans => nidaqmx_sys::DAQmx_Dev_AI_PhysicalChans,
            DeviceAttr::AI_SupportedMeasTypes => nidaqmx_sys::DAQmx_Dev_AI_SupportedMeasTypes,
            DeviceAttr::AI_MaxSingleChanRate => nidaqmx_sys::DAQmx_Dev_AI_MaxSingleChanRate,
            DeviceAttr::AI_MaxMultiChanRate => nidaqmx_sys::DAQmx_Dev_AI_MaxMultiChanRate,
            DeviceAttr::AI_MinRate => nidaqmx_sys::DAQmx_Dev_AI_MinRate,
            DeviceAttr::AI_SimultaneousSamplingSupported => {
                nidaqmx_sys::DAQmx_Dev_AI_SimultaneousSamplingSupported
            }
            DeviceAttr::AI_NumSampTimingEngines => nidaqmx_sys::DAQmx_Dev_AI_NumSampTimingEngines,
            DeviceAttr::AI_SampModes => nidaqmx_sys::DAQmx_Dev_AI_SampModes,
            DeviceAttr::AI_NumSyncPulseSrcs => nidaqmx_sys::DAQmx_Dev_AI_NumSyncPulseSrcs,
            DeviceAttr::AI_TrigUsage => nidaqmx_sys::DAQmx_Dev_AI_TrigUsage,
            DeviceAttr::AI_VoltageRngs => nidaqmx_sys::DAQmx_Dev_AI_VoltageRngs,
            DeviceAttr::AI_VoltageIntExcitDiscreteVals => {
                nidaqmx_sys::DAQmx_Dev_AI_VoltageIntExcitDiscreteVals
            }
            DeviceAttr::AI_VoltageIntExcitRangeVals => {
                nidaqmx_sys::DAQmx_Dev_AI_VoltageIntExcitRangeVals
            }
            DeviceAttr::AI_ChargeRngs => nidaqmx_sys::DAQmx_Dev_AI_ChargeRngs,
            DeviceAttr::AI_CurrentRngs => nidaqmx_sys::DAQmx_Dev_AI_CurrentRngs,
            DeviceAttr::AI_CurrentIntExcitDiscreteVals => {
                nidaqmx_sys::DAQmx_Dev_AI_CurrentIntExcitDiscreteVals
            }
            DeviceAttr::AI_BridgeRngs => nidaqmx_sys::DAQmx_Dev_AI_BridgeRngs,
            DeviceAttr::AI_ResistanceRngs => nidaqmx_sys::DAQmx_Dev_AI_ResistanceRngs,
            DeviceAttr::AI_FreqRngs => nidaqmx_sys::DAQmx_Dev_AI_FreqRngs,
            DeviceAttr::AI_Gains => nidaqmx_sys::DAQmx_Dev_AI_Gains,
            DeviceAttr::AI_Couplings => nidaqmx_sys::DAQmx_Dev_AI_Couplings,
            DeviceAttr::AI_LowpassCutoffFreqDiscreteVals => {
                nidaqmx_sys::DAQmx_Dev_AI_LowpassCutoffFreqDiscreteVals
            }
            DeviceAttr::AI_LowpassCutoffFreqRangeVals => {
                nidaqmx_sys::DAQmx_Dev_AI_LowpassCutoffFreqRangeVals
            }
            DeviceAttr::AI_DigFltr_Types => nidaqmx_sys::DAQmx_AI_DigFltr_Types,
            DeviceAttr::AI_DigFltr_LowpassCutoffFreqDiscreteVals => {
                nidaqmx_sys::DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqDiscreteVals
            }
            DeviceAttr::AI_DigFltr_LowpassCutoffFreqRangeVals => {
                nidaqmx_sys::DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqRangeVals
            }
            DeviceAttr::AO_PhysicalChans => nidaqmx_sys::DAQmx_Dev_AO_PhysicalChans,
            DeviceAttr::AO_SupportedOutputTypes => nidaqmx_sys::DAQmx_Dev_AO_SupportedOutputTypes,
            DeviceAttr::AO_MaxRate => nidaqmx_sys::DAQmx_Dev_AO_MaxRate,
            DeviceAttr::AO_MinRate => nidaqmx_sys::DAQmx_Dev_AO_MinRate,
            DeviceAttr::AO_SampClkSupported => nidaqmx_sys::DAQmx_Dev_AO_SampClkSupported,
            DeviceAttr::AO_NumSampTimingEngines => nidaqmx_sys::DAQmx_Dev_AO_NumSampTimingEngines,
            DeviceAttr::AO_SampModes => nidaqmx_sys::DAQmx_Dev_AO_SampModes,
            DeviceAttr::AO_NumSyncPulseSrcs => nidaqmx_sys::DAQmx_Dev_AO_NumSyncPulseSrcs,
            DeviceAttr::AO_TrigUsage => nidaqmx_sys::DAQmx_Dev_AO_TrigUsage,
            DeviceAttr::AO_VoltageRngs => nidaqmx_sys::DAQmx_Dev_AO_VoltageRngs,
            DeviceAttr::AO_CurrentRngs => nidaqmx_sys::DAQmx_Dev_AO_CurrentRngs,
            DeviceAttr::AO_Gains => nidaqmx_sys::DAQmx_Dev_AO_Gains,
            DeviceAttr::DI_Lines => nidaqmx_sys::DAQmx_Dev_DI_Lines,
            DeviceAttr::DI_Ports => nidaqmx_sys::DAQmx_Dev_DI_Ports,
            DeviceAttr::DI_MaxRate => nidaqmx_sys::DAQmx_Dev_DI_MaxRate,
            DeviceAttr::DI_NumSampTimingEngines => nidaqmx_sys::DAQmx_Dev_DI_NumSampTimingEngines,
            DeviceAttr::DI_TrigUsage => nidaqmx_sys::DAQmx_Dev_DI_TrigUsage,
            DeviceAttr::DO_Lines => nidaqmx_sys::DAQmx_Dev_DO_Lines,
            DeviceAttr::DO_Ports => nidaqmx_sys::DAQmx_Dev_DO_Ports,
            DeviceAttr::DO_MaxRate => nidaqmx_sys::DAQmx_Dev_DO_MaxRate,
            DeviceAttr::DO_NumSampTimingEngines => nidaqmx_sys::DAQmx_Dev_DO_NumSampTimingEngines,
            DeviceAttr::DO_TrigUsage => nidaqmx_sys::DAQmx_Dev_DO_TrigUsage,
            DeviceAttr::CI_PhysicalChans => nidaqmx_sys::DAQmx_Dev_CI_PhysicalChans,
            DeviceAttr::CI_SupportedMeasTypes => nidaqmx_sys::DAQmx_Dev_CI_SupportedMeasTypes,
            DeviceAttr::CI_TrigUsage => nidaqmx_sys::DAQmx_Dev_CI_TrigUsage,
            DeviceAttr::CI_SampClkSupported => nidaqmx_sys::DAQmx_Dev_CI_SampClkSupported,
            DeviceAttr::CI_SampModes => nidaqmx_sys::DAQmx_Dev_CI_SampModes,
            DeviceAttr::CI_MaxSize => nidaqmx_sys::DAQmx_Dev_CI_MaxSize,
            DeviceAttr::CI_MaxTimebase => nidaqmx_sys::DAQmx_Dev_CI_MaxTimebase,
            DeviceAttr::CO_PhysicalChans => nidaqmx_sys::DAQmx_Dev_CO_PhysicalChans,
            DeviceAttr::CO_SupportedOutputTypes => nidaqmx_sys::DAQmx_Dev_CO_SupportedOutputTypes,
            DeviceAttr::CO_SampClkSupported => nidaqmx_sys::DAQmx_Dev_CO_SampClkSupported,
            DeviceAttr::CO_SampModes => nidaqmx_sys::DAQmx_Dev_CO_SampModes,
            DeviceAttr::CO_TrigUsage => nidaqmx_sys::DAQmx_Dev_CO_TrigUsage,
            DeviceAttr::CO_MaxSize => nidaqmx_sys::DAQmx_Dev_CO_MaxSize,
            DeviceAttr::CO_MaxTimebase => nidaqmx_sys::DAQmx_Dev_CO_MaxTimebase,
            DeviceAttr::TEDS_HWTEDSSupported => nidaqmx_sys::DAQmx_Dev_TEDS_HWTEDSSupported,
            DeviceAttr::NumDMAChans => nidaqmx_sys::DAQmx_Dev_NumDMAChans,
            DeviceAttr::BusType => nidaqmx_sys::DAQmx_Dev_BusType,
            DeviceAttr::PCI_BusNum => nidaqmx_sys::DAQmx_Dev_PCI_BusNum,
            DeviceAttr::PCI_DevNum => nidaqmx_sys::DAQmx_Dev_PCI_DevNum,
            DeviceAttr::PXI_ChassisNum => nidaqmx_sys::DAQmx_Dev_PXI_ChassisNum,
            DeviceAttr::PXI_SlotNum => nidaqmx_sys::DAQmx_Dev_PXI_SlotNum,
            DeviceAttr::CompactDAQ_ChassisDevName => {
                nidaqmx_sys::DAQmx_Dev_CompactDAQ_ChassisDevName
            }
            DeviceAttr::CompactDAQ_SlotNum => nidaqmx_sys::DAQmx_Dev_CompactDAQ_SlotNum,
            DeviceAttr::CompactRIO_ChassisDevName => {
                nidaqmx_sys::DAQmx_Dev_CompactRIO_ChassisDevName
            }
            DeviceAttr::CompactRIO_SlotNum => nidaqmx_sys::DAQmx_Dev_CompactRIO_SlotNum,
            DeviceAttr::TCPIP_Hostname => nidaqmx_sys::DAQmx_Dev_TCPIP_Hostname,
            DeviceAttr::TCPIP_EthernetIP => nidaqmx_sys::DAQmx_Dev_TCPIP_EthernetIP,
            DeviceAttr::TCPIP_WirelessIP => nidaqmx_sys::DAQmx_Dev_TCPIP_WirelessIP,
            DeviceAttr::Terminals => nidaqmx_sys::DAQmx_Dev_Terminals,
            DeviceAttr::NumTimeTrigs => nidaqmx_sys::DAQmx_Dev_NumTimeTrigs,
            DeviceAttr::NumTimestampEngines => nidaqmx_sys::DAQmx_Dev_NumTimestampEngines,
        }
    }
}

impl From<i32> for Device {
    fn from(raw: i32) -> Self {
        match raw {
            nidaqmx_sys::DAQmx_Dev_IsSimulated => DeviceAttr::IsSimulated,
            nidaqmx_sys::DAQmx_Dev_ProductCategory => DeviceAttr::ProductCategory,
            nidaqmx_sys::DAQmx_Dev_ProductType => DeviceAttr::ProductType,
            nidaqmx_sys::DAQmx_Dev_ProductNum => DeviceAttr::ProductNum,
            nidaqmx_sys::DAQmx_Dev_SerialNum => DeviceAttr::SerialNum,
            nidaqmx_sys::DAQmx_Dev_Accessory_ProductTypes => DeviceAttr::Accessory_ProductTypes,
            nidaqmx_sys::DAQmx_Dev_Accessory_ProductNums => DeviceAttr::Accessory_ProductNums,
            nidaqmx_sys::DAQmx_Dev_Accessory_SerialNums => DeviceAttr::Accessory_SerialNums,
            nidaqmx_sys::DAQmx_Carrier_SerialNum => DeviceAttr::Carrier_SerialNum,
            nidaqmx_sys::DAQmx_FieldDAQ_DevName => DeviceAttr::FieldDAQ_DevName,
            nidaqmx_sys::DAQmx_FieldDAQ_BankDevNames => DeviceAttr::FieldDAQ_BankDevNames,
            nidaqmx_sys::DAQmx_Dev_Chassis_ModuleDevNames => DeviceAttr::Chassis_ModuleDevNames,
            nidaqmx_sys::DAQmx_Dev_AnlgTrigSupported => DeviceAttr::AnlgTrigSupported,
            nidaqmx_sys::DAQmx_Dev_DigTrigSupported => DeviceAttr::DigTrigSupported,
            nidaqmx_sys::DAQmx_Dev_TimeTrigSupported => DeviceAttr::TimeTrigSupported,
            nidaqmx_sys::DAQmx_Dev_AI_PhysicalChans => DeviceAttr::AI_PhysicalChans,
            nidaqmx_sys::DAQmx_Dev_AI_SupportedMeasTypes => DeviceAttr::AI_SupportedMeasTypes,
            nidaqmx_sys::DAQmx_Dev_AI_MaxSingleChanRate => DeviceAttr::AI_MaxSingleChanRate,
            nidaqmx_sys::DAQmx_Dev_AI_MaxMultiChanRate => DeviceAttr::AI_MaxMultiChanRate,
            nidaqmx_sys::DAQmx_Dev_AI_MinRate => DeviceAttr::AI_MinRate,
            nidaqmx_sys::DAQmx_Dev_AI_SimultaneousSamplingSupported => {
                DeviceAttr::AI_SimultaneousSamplingSupported
            }
            nidaqmx_sys::DAQmx_Dev_AI_NumSampTimingEngines => DeviceAttr::AI_NumSampTimingEngines,
            nidaqmx_sys::DAQmx_Dev_AI_SampModes => DeviceAttr::AI_SampModes,
            nidaqmx_sys::DAQmx_Dev_AI_NumSyncPulseSrcs => DeviceAttr::AI_NumSyncPulseSrcs,
            nidaqmx_sys::DAQmx_Dev_AI_TrigUsage => DeviceAttr::AI_TrigUsage,
            nidaqmx_sys::DAQmx_Dev_AI_VoltageRngs => DeviceAttr::AI_VoltageRngs,
            nidaqmx_sys::DAQmx_Dev_AI_VoltageIntExcitDiscreteVals => {
                DeviceAttr::AI_VoltageIntExcitDiscreteVals
            }
            nidaqmx_sys::DAQmx_Dev_AI_VoltageIntExcitRangeVals => {
                DeviceAttr::AI_VoltageIntExcitRangeVals
            }
            nidaqmx_sys::DAQmx_Dev_AI_ChargeRngs => DeviceAttr::AI_ChargeRngs,
            nidaqmx_sys::DAQmx_Dev_AI_CurrentRngs => DeviceAttr::AI_CurrentRngs,
            nidaqmx_sys::DAQmx_Dev_AI_CurrentIntExcitDiscreteVals => {
                DeviceAttr::AI_CurrentIntExcitDiscreteVals
            }
            nidaqmx_sys::DAQmx_Dev_AI_BridgeRngs => DeviceAttr::AI_BridgeRngs,
            nidaqmx_sys::DAQmx_Dev_AI_ResistanceRngs => DeviceAttr::AI_ResistanceRngs,
            nidaqmx_sys::DAQmx_Dev_AI_FreqRngs => DeviceAttr::AI_FreqRngs,
            nidaqmx_sys::DAQmx_Dev_AI_Gains => DeviceAttr::AI_Gains,
            nidaqmx_sys::DAQmx_Dev_AI_Couplings => DeviceAttr::AI_Couplings,
            nidaqmx_sys::DAQmx_Dev_AI_LowpassCutoffFreqDiscreteVals => {
                DeviceAttr::AI_LowpassCutoffFreqDiscreteVals
            }
            nidaqmx_sys::DAQmx_Dev_AI_LowpassCutoffFreqRangeVals => {
                DeviceAttr::AI_LowpassCutoffFreqRangeVals
            }
            nidaqmx_sys::DAQmx_AI_DigFltr_Types => DeviceAttr::AI_DigFltr_Types,
            nidaqmx_sys::DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqDiscreteVals => {
                DeviceAttr::AI_DigFltr_LowpassCutoffFreqDiscreteVals
            }
            nidaqmx_sys::DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqRangeVals => {
                DeviceAttr::AI_DigFltr_LowpassCutoffFreqRangeVals
            }
            nidaqmx_sys::DAQmx_Dev_AO_PhysicalChans => DeviceAttr::AO_PhysicalChans,
            nidaqmx_sys::DAQmx_Dev_AO_SupportedOutputTypes => DeviceAttr::AO_SupportedOutputTypes,
            nidaqmx_sys::DAQmx_Dev_AO_MaxRate => DeviceAttr::AO_MaxRate,
            nidaqmx_sys::DAQmx_Dev_AO_MinRate => DeviceAttr::AO_MinRate,
            nidaqmx_sys::DAQmx_Dev_AO_SampClkSupported => DeviceAttr::AO_SampClkSupported,
            nidaqmx_sys::DAQmx_Dev_AO_NumSampTimingEngines => DeviceAttr::AO_NumSampTimingEngines,
            nidaqmx_sys::DAQmx_Dev_AO_SampModes => DeviceAttr::AO_SampModes,
            nidaqmx_sys::DAQmx_Dev_AO_NumSyncPulseSrcs => DeviceAttr::AO_NumSyncPulseSrcs,
            nidaqmx_sys::DAQmx_Dev_AO_TrigUsage => DeviceAttr::AO_TrigUsage,
            nidaqmx_sys::DAQmx_Dev_AO_VoltageRngs => DeviceAttr::AO_VoltageRngs,
            nidaqmx_sys::DAQmx_Dev_AO_CurrentRngs => DeviceAttr::AO_CurrentRngs,
            nidaqmx_sys::DAQmx_Dev_AO_Gains => DeviceAttr::AO_Gains,
            nidaqmx_sys::DAQmx_Dev_DI_Lines => DeviceAttr::DI_Lines,
            nidaqmx_sys::DAQmx_Dev_DI_Ports => DeviceAttr::DI_Ports,
            nidaqmx_sys::DAQmx_Dev_DI_MaxRate => DeviceAttr::DI_MaxRate,
            nidaqmx_sys::DAQmx_Dev_DI_NumSampTimingEngines => DeviceAttr::DI_NumSampTimingEngines,
            nidaqmx_sys::DAQmx_Dev_DI_TrigUsage => DeviceAttr::DI_TrigUsage,
            nidaqmx_sys::DAQmx_Dev_DO_Lines => DeviceAttr::DO_Lines,
            nidaqmx_sys::DAQmx_Dev_DO_Ports => DeviceAttr::DO_Ports,
            nidaqmx_sys::DAQmx_Dev_DO_MaxRate => DeviceAttr::DO_MaxRate,
            nidaqmx_sys::DAQmx_Dev_DO_NumSampTimingEngines => DeviceAttr::DO_NumSampTimingEngines,
            nidaqmx_sys::DAQmx_Dev_DO_TrigUsage => DeviceAttr::DO_TrigUsage,
            nidaqmx_sys::DAQmx_Dev_CI_PhysicalChans => DeviceAttr::CI_PhysicalChans,
            nidaqmx_sys::DAQmx_Dev_CI_SupportedMeasTypes => DeviceAttr::CI_SupportedMeasTypes,
            nidaqmx_sys::DAQmx_Dev_CI_TrigUsage => DeviceAttr::CI_TrigUsage,
            nidaqmx_sys::DAQmx_Dev_CI_SampClkSupported => DeviceAttr::CI_SampClkSupported,
            nidaqmx_sys::DAQmx_Dev_CI_SampModes => DeviceAttr::CI_SampModes,
            nidaqmx_sys::DAQmx_Dev_CI_MaxSize => DeviceAttr::CI_MaxSize,
            nidaqmx_sys::DAQmx_Dev_CI_MaxTimebase => DeviceAttr::CI_MaxTimebase,
            nidaqmx_sys::DAQmx_Dev_CO_PhysicalChans => DeviceAttr::CO_PhysicalChans,
            nidaqmx_sys::DAQmx_Dev_CO_SupportedOutputTypes => DeviceAttr::CO_SupportedOutputTypes,
            nidaqmx_sys::DAQmx_Dev_CO_SampClkSupported => DeviceAttr::CO_SampClkSupported,
            nidaqmx_sys::DAQmx_Dev_CO_SampModes => DeviceAttr::CO_SampModes,
            nidaqmx_sys::DAQmx_Dev_CO_TrigUsage => DeviceAttr::CO_TrigUsage,
            nidaqmx_sys::DAQmx_Dev_CO_MaxSize => DeviceAttr::CO_MaxSize,
            nidaqmx_sys::DAQmx_Dev_CO_MaxTimebase => DeviceAttr::CO_MaxTimebase,
            nidaqmx_sys::DAQmx_Dev_TEDS_HWTEDSSupported => DeviceAttr::TEDS_HWTEDSSupported,
            nidaqmx_sys::DAQmx_Dev_NumDMAChans => DeviceAttr::NumDMAChans,
            nidaqmx_sys::DAQmx_Dev_BusType => DeviceAttr::BusType,
            nidaqmx_sys::DAQmx_Dev_PCI_BusNum => DeviceAttr::PCI_BusNum,
            nidaqmx_sys::DAQmx_Dev_PCI_DevNum => DeviceAttr::PCI_DevNum,
            nidaqmx_sys::DAQmx_Dev_PXI_ChassisNum => DeviceAttr::PXI_ChassisNum,
            nidaqmx_sys::DAQmx_Dev_PXI_SlotNum => DeviceAttr::PXI_SlotNum,
            nidaqmx_sys::DAQmx_Dev_CompactDAQ_ChassisDevName => {
                DeviceAttr::CompactDAQ_ChassisDevName
            }
            nidaqmx_sys::DAQmx_Dev_CompactDAQ_SlotNum => DeviceAttr::CompactDAQ_SlotNum,
            nidaqmx_sys::DAQmx_Dev_CompactRIO_ChassisDevName => {
                DeviceAttr::CompactRIO_ChassisDevName
            }
            nidaqmx_sys::DAQmx_Dev_CompactRIO_SlotNum => DeviceAttr::CompactRIO_SlotNum,
            nidaqmx_sys::DAQmx_Dev_TCPIP_Hostname => DeviceAttr::TCPIP_Hostname,
            nidaqmx_sys::DAQmx_Dev_TCPIP_EthernetIP => DeviceAttr::TCPIP_EthernetIP,
            nidaqmx_sys::DAQmx_Dev_TCPIP_WirelessIP => DeviceAttr::TCPIP_WirelessIP,
            nidaqmx_sys::DAQmx_Dev_Terminals => DeviceAttr::Terminals,
            nidaqmx_sys::DAQmx_Dev_NumTimeTrigs => DeviceAttr::NumTimeTrigs,
            nidaqmx_sys::DAQmx_Dev_NumTimestampEngines => DeviceAttr::NumTimestampEngines,
        }
    }
}

pub enum ExportSignal {}
pub enum PersistedChannel {}
pub enum PersistedScale {}
pub enum PersistedTask {}
pub enum PhysicalChannel {}
pub enum Read {}
pub enum RealTime {}
pub enum Scale {}
pub enum SwitchChannel {}
pub enum SwitchDevice {}
pub enum SwitchScan {}
pub enum System {}
pub enum Task {}
pub enum Timing {}
pub enum Trigger {}
pub enum Watchdog {}
pub enum Write {}

// Exported_AIConvClk_OutputTerm  #[doc = "Specifies the terminal to which to route the AI Convert Clock."]
// Exported_AIConvClk_Pulse_Polarity  #[doc = "Indicates the polarity of the exported AI Convert Clock. The polarity is fixed and independent of the active edge of the source of the AI Convert Clock."]
// Exported_10MHzRefClk_OutputTerm  #[doc = "Specifies the terminal to which to route the 10MHz Clock."]
// Exported_20MHzTimebase_OutputTerm  #[doc = "Specifies the terminal to which to route the 20MHz Timebase."]
// Exported_SampClk_OutputBehavior  #[doc = "Specifies whether the exported Sample Clock issues a pulse at the beginning of a sample or changes to a high state for the duration of the sample."]
// Exported_SampClk_OutputTerm  #[doc = "Specifies the terminal to which to route the Sample Clock."]
// Exported_SampClk_DelayOffset  #[doc = "Specifies in seconds the amount of time to offset the exported Sample clock.  Refer to timing diagrams for generation applications in the device documentation for more information about this value."]
// Exported_SampClk_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Sample Clock if Output Behavior is DAQmx_Val_Pulse."]
// Exported_SampClkTimebase_OutputTerm  #[doc = "Specifies the terminal to which to route the Sample Clock Timebase."]
// Exported_DividedSampClkTimebase_OutputTerm  #[doc = "Specifies the terminal to which to route the Divided Sample Clock Timebase."]
// Exported_AdvTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Advance Trigger."]
// Exported_AdvTrig_Pulse_Polarity  #[doc = "Indicates the polarity of the exported Advance Trigger."]
// Exported_AdvTrig_Pulse_WidthUnits  #[doc = "Specifies the units of Width Value."]
// Exported_AdvTrig_Pulse_Width  #[doc = "Specifies the width of an exported Advance Trigger pulse. Specify this value in the units you specify with Width Units."]
// Exported_PauseTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Pause Trigger."]
// Exported_PauseTrig_Lvl_ActiveLvl  #[doc = "Specifies the active level of the exported Pause Trigger."]
// Exported_RefTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Reference Trigger."]
// Exported_RefTrig_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Reference Trigger."]
// Exported_StartTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Start Trigger."]
// Exported_StartTrig_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Start Trigger."]
// Exported_AdvCmpltEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Advance Complete Event."]
// Exported_AdvCmpltEvent_Delay  #[doc = "Specifies the output signal delay in periods of the sample clock."]
// Exported_AdvCmpltEvent_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Advance Complete Event."]
// Exported_AdvCmpltEvent_Pulse_Width  #[doc = "Specifies the width of the exported Advance Complete Event pulse."]
// Exported_AIHoldCmpltEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the AI Hold Complete Event."]
// Exported_AIHoldCmpltEvent_PulsePolarity  #[doc = "Specifies the polarity of an exported AI Hold Complete Event pulse."]
// Exported_ChangeDetectEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Change Detection Event."]
// Exported_ChangeDetectEvent_Pulse_Polarity  #[doc = "Specifies the polarity of an exported Change Detection Event pulse."]
// Exported_CtrOutEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Counter Output Event."]
// Exported_CtrOutEvent_OutputBehavior  #[doc = "Specifies whether the exported Counter Output Event pulses or changes from one state to the other when the counter reaches terminal count."]
// Exported_CtrOutEvent_Pulse_Polarity  #[doc = "Specifies the polarity of the pulses at the output terminal of the counter when Output Behavior is DAQmx_Val_Pulse. NI-DAQmx ignores this property if Output Behavior is DAQmx_Val_Toggle."]
// Exported_CtrOutEvent_Toggle_IdleState  #[doc = "Specifies the initial state of the output terminal of the counter when Output Behavior is DAQmx_Val_Toggle. The terminal enters this state when NI-DAQmx commits the task."]
// Exported_HshkEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Handshake Event."]
// Exported_HshkEvent_OutputBehavior  #[doc = "Specifies the output behavior of the Handshake Event."]
// Exported_HshkEvent_Delay  #[doc = "Specifies the number of seconds to delay after the Handshake Trigger deasserts before asserting the Handshake Event."]
// Exported_HshkEvent_Interlocked_AssertedLvl  #[doc = "Specifies the asserted level of the exported Handshake Event if Output Behavior is DAQmx_Val_Interlocked."]
// Exported_HshkEvent_Interlocked_AssertOnStart  #[doc = "Specifies to assert the Handshake Event when the task starts if Output Behavior is DAQmx_Val_Interlocked."]
// Exported_HshkEvent_Interlocked_DeassertDelay  #[doc = "Specifies in seconds the amount of time to wait after the Handshake Trigger asserts before deasserting the Handshake Event if Output Behavior is DAQmx_Val_Interlocked."]
// Exported_HshkEvent_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Handshake Event if Output Behavior is DAQmx_Val_Pulse."]
// Exported_HshkEvent_Pulse_Width  #[doc = "Specifies in seconds the pulse width of the exported Handshake Event if Output Behavior is DAQmx_Val_Pulse."]
// Exported_RdyForXferEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Ready for Transfer Event."]
// Exported_RdyForXferEvent_Lvl_ActiveLvl  #[doc = "Specifies the active level of the exported Ready for Transfer Event."]
// Exported_RdyForXferEvent_DeassertCond  #[doc = "Specifies when the ready for transfer event deasserts."]
// Exported_RdyForXferEvent_DeassertCondCustomThreshold  #[doc = "Specifies in samples the threshold below which the Ready for Transfer Event deasserts. This threshold is an amount of space available in the onboard memory of the device. Deassert Condition must be DAQmx_Val_OnbrdMemCustomThreshold to use a custom threshold."]
// Exported_DataActiveEvent_OutputTerm  #[doc = "Specifies the terminal to which to export the Data Active Event."]
// Exported_DataActiveEvent_Lvl_ActiveLvl  #[doc = "Specifies the polarity of the exported Data Active Event."]
// Exported_RdyForStartEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Ready for Start Event."]
// Exported_RdyForStartEvent_Lvl_ActiveLvl  #[doc = "Specifies the polarity of the exported Ready for Start Event."]
// Exported_SyncPulseEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Synchronization Pulse Event."]
// Exported_WatchdogExpiredEvent_OutputTerm  #[doc = "Specifies the terminal  to which to route the Watchdog Timer Expired Event."]
// PersistedChan_Author  #[doc = "Indicates the author of the global channel."]
// PersistedChan_AllowInteractiveEditing  #[doc = "Indicates whether the global channel can be edited in the DAQ Assistant."]
// PersistedChan_AllowInteractiveDeletion  #[doc = "Indicates whether the global channel can be deleted through MAX."]
// PersistedScale_Author  #[doc = "Indicates the author of the custom scale."]
// PersistedScale_AllowInteractiveEditing  #[doc = "Indicates whether the custom scale can be edited in the DAQ Assistant."]
// PersistedScale_AllowInteractiveDeletion  #[doc = "Indicates whether the custom scale can be deleted through MAX."]
// PersistedTask_Author  #[doc = "Indicates the author of the task."]
// PersistedTask_AllowInteractiveEditing  #[doc = "Indicates whether the task can be edited in the DAQ Assistant."]
// PersistedTask_AllowInteractiveDeletion  #[doc = "Indicates whether the task can be deleted through MAX."]
// PhysicalChan_AI_SupportedMeasTypes  #[doc = "Indicates the measurement types supported by the channel."]
// PhysicalChan_AI_TermCfgs  #[doc = "Indicates the list of terminal configurations supported by the channel."]
// PhysicalChan_AI_InputSrcs  #[doc = "Indicates the list of input sources supported by the channel. Channels may support using the signal from the I/O connector or one of several calibration signals."]
// PhysicalChan_AI_SensorPower_Types  #[doc = "Indicates the types of power supplied to the sensor supported by this channel."]
// PhysicalChan_AI_SensorPower_VoltageRangeVals  #[doc = "Indicates pairs of sensor power voltage ranges supported by this channel. Each pair consists of the low value followed by the high value."]
// PhysicalChan_AI_PowerControl_Voltage  #[doc = "Specifies the voltage level for the sensor's power supply."]
// PhysicalChan_AI_PowerControl_Enable  #[doc = "Specifies whether to turn on the sensor's power supply."]
// PhysicalChan_AI_PowerControl_Type  #[doc = "Specifies the type of power supplied to the sensor."]
// PhysicalChan_AI_SensorPower_OpenChan  #[doc = "Indicates whether there is an open channel or undercurrent condition on the channel."]
// PhysicalChan_AI_SensorPower_Overcurrent  #[doc = "Indicates whether there is an overcurrent condition on the channel."]
// PhysicalChan_AO_SupportedOutputTypes  #[doc = "Indicates the output types supported by the channel."]
// PhysicalChan_AO_SupportedPowerUpOutputTypes  #[doc = "Indicates the power up output types supported by the channel."]
// PhysicalChan_AO_TermCfgs  #[doc = "Indicates the list of terminal configurations supported by the channel."]
// PhysicalChan_AO_ManualControlEnable  #[doc = "Specifies if you can control the physical channel externally via a manual control located on the device. You cannot simultaneously control a channel manually and with NI-DAQmx."]
// PhysicalChan_AO_ManualControl_ShortDetected  #[doc = "Indicates whether the physical channel is currently disabled due to a short detected on the channel."]
// PhysicalChan_AO_ManualControlAmplitude  #[doc = "Indicates the current value of the front panel amplitude control for the physical channel in volts."]
// PhysicalChan_AO_ManualControlFreq  #[doc = "Indicates the current value of the front panel frequency control for the physical channel in hertz."]
// AO_PowerAmp_ChannelEnable  #[doc = "Specifies whether to enable or disable a channel for amplification. This property can also be used to check if a channel is enabled."]
// AO_PowerAmp_ScalingCoeff  #[doc = "Indicates the coefficients of a polynomial equation used to scale from pre-amplified values."]
// AO_PowerAmp_Overcurrent  #[doc = "Indicates if the channel detected an overcurrent condition."]
// AO_PowerAmp_Gain  #[doc = "Indicates the calibrated gain of the channel."]
// AO_PowerAmp_Offset  #[doc = "Indicates the calibrated offset of the channel in volts."]
// PhysicalChan_DI_PortWidth  #[doc = "Indicates in bits the width of digital input port."]
// PhysicalChan_DI_SampClkSupported  #[doc = "Indicates if the sample clock timing type is supported for the digital input physical channel."]
// PhysicalChan_DI_SampModes  #[doc = "Indicates the sample modes supported by devices that support sample clocked digital input."]
// PhysicalChan_DI_ChangeDetectSupported  #[doc = "Indicates if the change detection timing type is supported for the digital input physical channel."]
// PhysicalChan_DO_PortWidth  #[doc = "Indicates in bits the width of digital output port."]
// PhysicalChan_DO_SampClkSupported  #[doc = "Indicates if the sample clock timing type is supported for the digital output physical channel."]
// PhysicalChan_DO_SampModes  #[doc = "Indicates the sample modes supported by devices that support sample clocked digital output."]
// PhysicalChan_CI_SupportedMeasTypes  #[doc = "Indicates the measurement types supported by the channel."]
// PhysicalChan_CO_SupportedOutputTypes  #[doc = "Indicates the output types supported by the channel."]
// PhysicalChan_TEDS_MfgID  #[doc = "Indicates the manufacturer ID of the sensor."]
// PhysicalChan_TEDS_ModelNum  #[doc = "Indicates the model number of the sensor."]
// PhysicalChan_TEDS_SerialNum  #[doc = "Indicates the serial number of the sensor."]
// PhysicalChan_TEDS_VersionNum  #[doc = "Indicates the version number of the sensor."]
// PhysicalChan_TEDS_VersionLetter  #[doc = "Indicates the version letter of the sensor."]
// PhysicalChan_TEDS_BitStream  #[doc = "Indicates the TEDS binary bitstream without checksums."]
// PhysicalChan_TEDS_TemplateIDs  #[doc = "Indicates the IDs of the templates in the bitstream in BitStream."]
// Read_RelativeTo  #[doc = "Specifies the point in the buffer at which to begin a read operation. If you also specify an offset with Offset, the read operation begins at that offset relative to the point you select with this property. The default value is DAQmx_Val_CurrReadPos unless you configure a Reference Trigger for the task. If you configure a Reference Trigger, the default value is DAQmx_Val_FirstPretrigSamp."]
// Read_Offset  #[doc = "Specifies an offset in samples per channel at which to begin a read operation. This offset is relative to the location you specify with RelativeTo."]
// Read_ChannelsToRead  #[doc = "Specifies a subset of channels in the task from which to read."]
// Read_ReadAllAvailSamp  #[doc = "Specifies whether subsequent read operations read all samples currently available in the buffer or wait for the buffer to become full before reading. NI-DAQmx uses this setting for finite acquisitions and only when the number of samples to read is -1. For continuous acquisitions when the number of samples to read is -1, a read operation always reads all samples currently available in the buffer."]
// Read_AutoStart  #[doc = "Specifies if an NI-DAQmx Read function automatically starts the task  if you did not start the task explicitly by using DAQmxStartTask(). The default value is TRUE. When  an NI-DAQmx Read function starts a finite acquisition task, it also stops the task after reading the last sample."]
// Read_OverWrite  #[doc = "Specifies whether to overwrite samples in the buffer that you have not yet read."]
// Logging_FilePath  #[doc = "Specifies the path to the TDMS file to which you want to log data.  If the file path is changed while the task is running, this takes effect on the next sample interval (if Logging.SampsPerFile has been set) or when DAQmx Start New File is called. New file paths can be specified by ending with '\' or '/'. Files created after specifying a new file path retain the same name and numbering sequence."]
// Logging_Mode  #[doc = "Specifies whether to enable logging and whether to allow reading data while logging. Log mode allows for the best performance. However, you cannot read data while logging if you specify this mode. If you want to read data while logging, specify Log and Read mode."]
// Logging_TDMS_GroupName  #[doc = "Specifies the name of the group to create within the TDMS file for data from this task. If you append data to an existing file and the specified group already exists, NI-DAQmx appends a number symbol and a number to the group name, incrementing that number until finding a group name that does not exist. For example, if you specify a group name of Voltage Task, and that group already exists, NI-DAQmx assigns the gr..."]
// Logging_TDMS_Operation  #[doc = "Specifies how to open the TDMS file."]
// Logging_Pause  #[doc = "Specifies whether logging is paused while a task is executing. If Mode is set to Log and Read mode, this value is taken into consideration on the next call to DAQmx Read, where data is written to disk. If Mode is set to Log Only mode, this value is taken into consideration the next time that data is written to disk. A new TDMS group is written when logging is resumed from a paused state."]
// Logging_SampsPerFile  #[doc = "Specifies how many samples to write to each file. When the file reaches the number of samples specified, a new file is created with the naming convention of filename_####.tdms, where #### starts at 0001 and increments automatically with each new file. For example, if the file specified is path/to/folder/data.tdms, the next file name used is path/to/folder/data_0001.tdms. To disable file spanning behavior, set this attribute to ..."]
// Logging_FileWriteSize  #[doc = "Specifies the size, in samples, in which data will be written to disk.  The size must be evenly divisible by the volume sector size, in bytes."]
// Logging_FilePreallocationSize  #[doc = "Specifies a size in samples to be used to pre-allocate space on disk.  Pre-allocation can improve file I/O performance, especially in situations where multiple files are being written to disk.  For finite tasks, the default behavior is to pre-allocate the file based on the number of samples you configure the task to acquire."]
// Read_CurrReadPos  #[doc = "Indicates in samples per channel the current position in the buffer."]
// Read_AvailSampPerChan  #[doc = "Indicates the number of samples available to read per channel. This value is the same for all channels in the task."]
// Read_TotalSampPerChanAcquired  #[doc = "Indicates the total number of samples acquired by each channel. NI-DAQmx returns a single value because this value is the same for all channels. For retriggered acquisitions, this value is the cumulative number of samples across all retriggered acquisitions."]
// Read_CommonModeRangeErrorChansExist  #[doc = "Indicates if the device(s) detected a common mode range violation for any virtual channel in the task. Common mode range violation occurs when the voltage of either the positive terminal or negative terminal to ground are out of range. Reading this property clears the common mode range violation status for all channels in the task. You must read this property before you read Common Mode Range Error Channels. Other..."]
// Read_CommonModeRangeErrorChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected a common mode range violation. You must read Common Mode Range Error Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_ExcitFaultChansExist  #[doc = "Indicates if the device(s) detected an excitation fault condition for any virtual channel in the task. Reading this property clears the excitation fault status for all channels in the task. You must read this property before you read Excitation Fault Channels. Otherwise, you will receive an error."]
// Read_ExcitFaultChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an excitation fault condition. You must read Excitation Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_OvercurrentChansExist  #[doc = "Indicates if the device(s) detected an overcurrent condition for any virtual channel in the task. Reading this property clears the overcurrent status for all channels in the task. You must read this property before you read Overcurrent Channels. Otherwise, you will receive an error."]
// Read_OvercurrentChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an overcurrent condition. You must read Overcurrent Channels Exist before you read this property. Otherwise, you will receive an error. On some devices, you must restart the task for all overcurrent channels to recover."]
// Read_OvertemperatureChansExist  #[doc = "Indicates if the device(s) detected an overtemperature condition in any virtual channel in the task. Reading this property clears the overtemperature status for all channels in the task. You must read this property before you read Overtemperature Channels. Otherwise, you will receive an error."]
// Read_OvertemperatureChans  #[doc = "Indicates a list of names of any overtemperature virtual channels. You must read Overtemperature Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_OpenChansExist  #[doc = "Indicates if the device or devices detected an open channel condition in any virtual channel in the task. Reading this property clears the open channel status for all channels in this task. You must read this property before you read Open Channels. Otherwise, you will receive an error."]
// Read_OpenChans  #[doc = "Indicates a list of names of any open virtual channels. You must read Open Channels Exist before you read this property. Otherwise you will receive an error."]
// Read_OpenChansDetails  #[doc = "Indicates a list of details of any open virtual channels. You must read Open Channels Exist before you read this property. Otherwise you will receive an error."]
// Read_OpenCurrentLoopChansExist  #[doc = "Indicates if the device(s) detected an open current loop for any virtual channel in the task. Reading this property clears the open current loop status for all channels in the task. You must read this property before you read Open Current Loop Channels. Otherwise, you will receive an error."]
// Read_OpenCurrentLoopChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an open current loop. You must read Open Current Loop Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_OpenThrmcplChansExist  #[doc = "Indicates if the device(s) detected an open thermocouple connected to any virtual channel in the task. Reading this property clears the open thermocouple status for all channels in the task. You must read this property before you read Open Thermocouple Channels. Otherwise, you will receive an error."]
// Read_OpenThrmcplChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an open thermcouple. You must read Open Thermocouple Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_OverloadedChansExist  #[doc = "Indicates if the device(s) detected an overload in any virtual channel in the task. Reading this property clears the overload status for all channels in the task. You must read this property before you read Overloaded Channels. Otherwise, you will receive an error."]
// Read_OverloadedChans  #[doc = "Indicates a list of names of any overloaded virtual channels in the task. You must read Overloaded Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_InputLimitsFaultChansExist  #[doc = "Indicates if the device or devices detected a sample that was outside the upper or lower limits configured for each channel in the task. Reading this property clears the input limits fault channel status for all channels in the task. You must read this property before you read Input Limits Fault Channels. Otherwise, you will receive an error. Note: Fault detection applies to both positive and negative inputs. For ..."]
// Read_InputLimitsFaultChans  #[doc = "Indicates the virtual channels that have detected samples outside the upper or lower limits configured for each channel in the task. You must read Input Limits Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_PLL_UnlockedChansExist  #[doc = "Indicates whether the PLL is currently locked, or whether it became unlocked during the previous acquisition. Devices may report PLL Unlock either during acquisition or after acquisition."]
// Read_PLL_UnlockedChans  #[doc = "Indicates the channels that had their PLLs unlock."]
// Read_PowerSupplyFaultChansExist  #[doc = "Indicates if the device or devices detected a power supply fault condition in any virtual channel in the task. Reading this property clears the power supply fault status for all channels in this task. You must read this property before you read Power Supply Fault Channels. Otherwise, you will receive an error."]
// Read_PowerSupplyFaultChans  #[doc = "Indicates the virtual channels that have detected a power supply fault. You must read Power Supply Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// Read_Sync_UnlockedChansExist  #[doc = "Indicates whether the target is currently locked to the grand master. Devices may report PLL Unlock either during acquisition or after acquisition."]
// Read_Sync_UnlockedChans  #[doc = "Indicates the channels from devices in an unlocked target."]
// Read_AccessoryInsertionOrRemovalDetected  #[doc = "Indicates if any device(s) in the task detected the insertion or removal of an accessory since the task started. Reading this property clears the accessory change status for all channels in the task. You must read this property before you read Devices with Inserted or Removed Accessories. Otherwise, you will receive an error."]
// Read_DevsWithInsertedOrRemovedAccessories  #[doc = "Indicates the names of any devices that detected the insertion or removal of an accessory since the task started. You must read Accessory Insertion or Removal Detected before you read this property. Otherwise, you will receive an error."]
// RemoteSenseErrorChansExist  #[doc = "Indicates if the device(s) detected an error condition of the remote sense connection for any channel in the task. You must disable the output and resolve the hardware connection issue to clear the error condition. You must read this property before you read the Remote Sense Error Channels property. Otherwise, you will receive an error."]
// RemoteSenseErrorChans  #[doc = "Indicates a list of names of any virtual channels in the task for which a remote sense connection error condition has been detected. You must read Remote Sense Error Channels Exist before you read this property. Otherwise, you will receive an error."]
// AuxPowerErrorChansExist  #[doc = "Indicates if the device(s) detected an auxiliary power supply error condition for any channel in the task. Reading this property clears the error condition status for all channels in the task. You must read this property before you read the Aux Power Error Channels property. Otherwise, you will receive an error."]
// AuxPowerErrorChans  #[doc = "Indicates a list of names of any virtual channels in the task for which an auxiliary power supply error condition has been detected. You must read the Aux Power Error Channels Exist property before you read this property. Otherwise, you will receive an error."]
// Read_ChangeDetect_HasOverflowed  #[doc = "Indicates if samples were missed because change detection events occurred faster than the device could handle them. Some devices detect overflows differently than others."]
// Read_RawDataWidth  #[doc = "Indicates in bytes the size of a raw sample from the task."]
// Read_NumChans  #[doc = "Indicates the number of channels that an NI-DAQmx Read function reads from the task. This value is the number of channels in the task or the number of channels you specify with Channels to Read."]
// Read_DigitalLines_BytesPerChan  #[doc = "Indicates the number of bytes per channel that NI-DAQmx returns in a sample for line-based reads. If a channel has fewer lines than this number, the extra bytes are FALSE."]
// Read_WaitMode  #[doc = "Specifies how an NI-DAQmx Read function waits for samples to become available."]
// Read_SleepTime  #[doc = "Specifies in seconds the amount of time to sleep after checking for available samples if Wait Mode is DAQmx_Val_Sleep."]
// RealTime_ConvLateErrorsToWarnings  #[doc = "Specifies if DAQmxWaitForNextSampleClock(), an NI-DAQmx Read function, and an NI-DAQmx Write function convert late errors to warnings. NI-DAQmx returns no late warnings or errors until the number of warmup iterations you specify with Number Of Warmup Iterations execute."]
// RealTime_NumOfWarmupIters  #[doc = "Specifies the number of loop iterations that must occur before DAQmxWaitForNextSampleClock() and an NI-DAQmx Read function return any late warnings or errors. The system needs a number of iterations to stabilize. During this period, a large amount of jitter occurs, potentially causing reads and writes to be late. The default number of warmup iterations is 100. Specify a larger number if needed to stabilize the sys..."]
// RealTime_WaitForNextSampClkWaitMode  #[doc = "Specifies how DAQmxWaitForNextSampleClock() waits for the next Sample Clock pulse."]
// RealTime_ReportMissedSamp  #[doc = "Specifies whether an NI-DAQmx Read function returns lateness errors or warnings when it detects missed Sample Clock pulses. This setting does not affect DAQmxWaitForNextSampleClock(). Set this property to TRUE for applications that need to detect lateness without using DAQmxWaitForNextSampleClock()."]
// RealTime_WriteRecoveryMode  #[doc = "Specifies how NI-DAQmx attempts to recover after missing a Sample Clock pulse when performing counter writes."]
// Scale_Descr  #[doc = "Specifies a description for the scale."]
// Scale_ScaledUnits  #[doc = "Specifies the units to use for scaled values. You can use an arbitrary string."]
// Scale_PreScaledUnits  #[doc = "Specifies the units of the values that you want to scale."]
// Scale_Type  #[doc = "Indicates the method or equation form that the custom scale uses."]
// Scale_Lin_Slope  #[doc = "Specifies the slope, m, in the equation y=mx+b."]
// Scale_Lin_YIntercept  #[doc = "Specifies the y-intercept, b, in the equation y=mx+b."]
// Scale_Map_ScaledMax  #[doc = "Specifies the largest value in the range of scaled values. NI-DAQmx maps this value to Pre-Scaled Maximum Value. Reads coerce samples that are larger than this value to match this value. Writes generate errors for samples that are larger than this value."]
// Scale_Map_PreScaledMax  #[doc = "Specifies the largest value in the range of pre-scaled values. NI-DAQmx maps this value to Scaled Maximum Value."]
// Scale_Map_ScaledMin  #[doc = "Specifies the smallest value in the range of scaled values. NI-DAQmx maps this value to Pre-Scaled Minimum Value. Reads coerce samples that are smaller than this value to match this value. Writes generate errors for samples that are smaller than this value."]
// Scale_Map_PreScaledMin  #[doc = "Specifies the smallest value in the range of pre-scaled values. NI-DAQmx maps this value to Scaled Minimum Value."]
// Scale_Poly_ForwardCoeff  #[doc = "Specifies an array of coefficients for the polynomial that converts pre-scaled values to scaled values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9x^3."]
// Scale_Poly_ReverseCoeff  #[doc = "Specifies an array of coefficients for the polynomial that converts scaled values to pre-scaled values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9y^3."]
// Scale_Table_ScaledVals  #[doc = "Specifies an array of scaled values. These values map directly to the values in Pre-Scaled Values."]
// Scale_Table_PreScaledVals  #[doc = "Specifies an array of pre-scaled values. These values map directly to the values in Scaled Values."]
// SwitchChan_Usage  #[doc = "(Deprecated) Specifies how you can use the channel. Using this property acts as a safety mechanism to prevent you from connecting two source channels, for example."]
// SwitchChan_AnlgBusSharingEnable  #[doc = "(Deprecated) Specifies whether to enable sharing of an analog bus line so that multiple switch devices can connect to it simultaneously. For each device that will share the analog bus line, set this property to TRUE to enable sharing on the channel that connects to the analog bus line. Analog bus sharing is disabled by default."]
// SwitchChan_MaxACCarryCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum AC current that the device can carry."]
// SwitchChan_MaxACSwitchCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum AC current that the device can switch. This current is always against an RMS voltage level."]
// SwitchChan_MaxACCarryPwr  #[doc = "(Deprecated) Indicates in watts the maximum AC power that the device can carry."]
// SwitchChan_MaxACSwitchPwr  #[doc = "(Deprecated) Indicates in watts the maximum AC power that the device can switch."]
// SwitchChan_MaxDCCarryCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum DC current that the device can carry."]
// SwitchChan_MaxDCSwitchCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum DC current that the device can switch. This current is always against a DC voltage level."]
// SwitchChan_MaxDCCarryPwr  #[doc = "(Deprecated) Indicates in watts the maximum DC power that the device can carry."]
// SwitchChan_MaxDCSwitchPwr  #[doc = "(Deprecated) Indicates in watts the maximum DC power that the device can switch."]
// SwitchChan_MaxACVoltage  #[doc = "(Deprecated) Indicates in volts the maximum AC RMS voltage that the device can switch."]
// SwitchChan_MaxDCVoltage  #[doc = "(Deprecated) Indicates in volts the maximum DC voltage that the device can switch."]
// SwitchChan_WireMode  #[doc = "(Deprecated) Indicates the number of wires that the channel switches."]
// SwitchChan_Bandwidth  #[doc = "(Deprecated) Indicates in Hertz the maximum frequency of a signal that can pass through the switch without significant deterioration."]
// SwitchChan_Impedance  #[doc = "(Deprecated) Indicates in ohms the switch impedance. This value is important in the RF domain and should match the impedance of the sources and loads."]
// SwitchDev_SettlingTime  #[doc = "(Deprecated) Specifies in seconds the amount of time to wait for the switch to settle (or debounce). NI-DAQmx adds this time to the settling time of the motherboard. Modify this property only if the switch does not settle within the settling time of the motherboard. Refer to device documentation for supported settling times."]
// SwitchDev_AutoConnAnlgBus  #[doc = "(Deprecated) Specifies if NI-DAQmx routes multiplexed channels to the analog bus backplane. Only the SCXI-1127 and SCXI-1128 support this property."]
// SwitchDev_PwrDownLatchRelaysAfterSettling  #[doc = "(Deprecated) Specifies if DAQmxSwitchWaitForSettling() powers down latching relays after waiting for the device to settle."]
// SwitchDev_Settled  #[doc = "(Deprecated) Indicates when Settling Time expires."]
// SwitchDev_RelayList  #[doc = "(Deprecated) Indicates a comma-delimited list of relay names."]
// SwitchDev_NumRelays  #[doc = "(Deprecated) Indicates the number of relays on the device. This value matches the number of relay names in Relay List."]
// SwitchDev_SwitchChanList  #[doc = "(Deprecated) Indicates a comma-delimited list of channel names for the current topology of the device."]
// SwitchDev_NumSwitchChans  #[doc = "(Deprecated) Indicates the number of switch channels for the current topology of the device. This value matches the number of channel names in Switch Channel List."]
// SwitchDev_NumRows  #[doc = "(Deprecated) Indicates the number of rows on a device in a matrix switch topology. Indicates the number of multiplexed channels on a device in a mux topology."]
// SwitchDev_NumColumns  #[doc = "(Deprecated) Indicates the number of columns on a device in a matrix switch topology. This value is always 1 if the device is in a mux topology."]
// SwitchDev_Topology  #[doc = "(Deprecated) Indicates the current topology of the device. This value is one of the topology options in DAQmxSwitchSetTopologyAndReset()."]
// SwitchDev_Temperature  #[doc = "(Deprecated) Indicates the current temperature as read by the Switch module in degrees Celsius. Refer to your device documentation for more information."]
// SwitchScan_BreakMode  #[doc = "(Deprecated) Specifies the action to take between each entry in a scan list."]
// SwitchScan_RepeatMode  #[doc = "(Deprecated) Specifies if the task advances through the scan list multiple times."]
// SwitchScan_WaitingForAdv  #[doc = "(Deprecated) Indicates if the switch hardware is waiting for an  Advance Trigger. If the hardware is waiting, it completed the previous entry in the scan list."]
// Sys_GlobalChans  #[doc = "Indicates an array that contains the names of all global channels saved on the system."]
// Sys_Scales  #[doc = "Indicates an array that contains the names of all custom scales saved on the system."]
// Sys_Tasks  #[doc = "Indicates an array that contains the names of all tasks saved on the system."]
// Sys_DevNames  #[doc = "Indicates the names of all devices installed in the system."]
// Sys_NIDAQMajorVersion  #[doc = "Indicates the major portion of the installed version of NI-DAQmx, such as 7 for version 7.0."]
// Sys_NIDAQMinorVersion  #[doc = "Indicates the minor portion of the installed version of NI-DAQmx, such as 0 for version 7.0."]
// Sys_NIDAQUpdateVersion  #[doc = "Indicates the update portion of the installed version of NI-DAQmx, such as 1 for version 9.0.1."]
// Task_Name  #[doc = "Indicates the name of the task."]
// Task_Channels  #[doc = "Indicates the names of all virtual channels in the task."]
// Task_NumChans  #[doc = "Indicates the number of virtual channels in the task."]
// Task_Devices  #[doc = "Indicates an array containing the names of all devices in the task."]
// Task_NumDevices  #[doc = "Indicates the number of devices in the task."]
// Task_Complete  #[doc = "Indicates whether the task completed execution."]
// SampQuant_SampMode  #[doc = "Specifies if a task acquires or generates a finite number of samples or if it continuously acquires or generates samples."]
// SampQuant_SampPerChan  #[doc = "Specifies the number of samples to acquire or generate for each channel if Sample Mode is DAQmx_Val_FiniteSamps. If Sample Mode is DAQmx_Val_ContSamps, NI-DAQmx uses this value to determine the buffer size."]
// SampTimingType  #[doc = "Specifies the type of sample timing to use for the task."]
// SampClk_Rate  #[doc = "Specifies the sampling rate in samples per channel per second. If you use an external source for the Sample Clock, set this input to the maximum expected rate of that clock."]
// SampClk_MaxRate  #[doc = "Indicates the maximum Sample Clock rate supported by the task, based on other timing settings. For output tasks, the maximum Sample Clock rate is the maximum rate of the DAC. For input tasks, NI-DAQmx calculates the maximum sampling rate differently for multiplexed devices than simultaneous sampling devices."]
// SampClk_Src  #[doc = "Specifies the terminal of the signal to use as the Sample Clock."]
// SampClk_ActiveEdge  #[doc = "Specifies on which edge of a clock pulse sampling takes place. This property is useful primarily when the signal you use as the Sample Clock is not a periodic clock."]
// SampClk_OverrunBehavior  #[doc = "Specifies the action to take if Sample Clock edges occur faster than the device can handle them."]
// SampClk_UnderflowBehavior  #[doc = "Specifies the action to take when the onboard memory of the device becomes empty. In either case, the sample clock does not stop."]
// SampClk_TimebaseDiv  #[doc = "Specifies the number of Sample Clock Timebase pulses needed to produce a single Sample Clock pulse."]
// SampClk_Term  #[doc = "Indicates the name of the internal Sample Clock terminal for the task. This property does not return the name of the Sample Clock source terminal specified with Source."]
// SampClk_Timebase_Rate  #[doc = "Specifies the rate of the Sample Clock Timebase. Some applications require that you specify a rate when you use any signal other than the onboard Sample Clock Timebase. NI-DAQmx requires this rate to calculate other timing parameters."]
// SampClk_Timebase_Src  #[doc = "Specifies the terminal of the signal to use as the Sample Clock Timebase."]
// SampClk_Timebase_ActiveEdge  #[doc = "Specifies on which edge to recognize a Sample Clock Timebase pulse. This property is useful primarily when the signal you use as the Sample Clock Timebase is not a periodic clock."]
// SampClk_Timebase_MasterTimebaseDiv  #[doc = "Specifies the number of pulses of the Master Timebase needed to produce a single pulse of the Sample Clock Timebase."]
// SampClkTimebase_Term  #[doc = "Indicates the name of the internal Sample Clock Timebase terminal for the task. This property does not return the name of the Sample Clock Timebase source terminal specified with Source."]
// SampClk_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// SampClk_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// SampClk_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// SampClk_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// SampClk_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// SampClk_WriteWfm_UseInitialWfmDT  #[doc = "Specifies that the value of Rate will be determined by the dt component of the initial DAQmx Write waveform input for Output tasks."]
// Hshk_DelayAfterXfer  #[doc = "Specifies the number of seconds to wait after a handshake cycle before starting a new handshake cycle."]
// Hshk_StartCond  #[doc = "Specifies the point in the handshake cycle that the device is in when the task starts."]
// Hshk_SampleInputDataWhen  #[doc = "Specifies on which edge of the Handshake Trigger an input task latches the data from the peripheral device."]
// ChangeDetect_DI_RisingEdgePhysicalChans  #[doc = "Specifies the names of the digital lines or ports on which to detect rising edges. The lines or ports must be used by virtual channels in the task. You also can specify a string that contains a list or range of digital lines or ports."]
// ChangeDetect_DI_FallingEdgePhysicalChans  #[doc = "Specifies the names of the digital lines or ports on which to detect falling edges. The lines or ports must be used by virtual channels in the task. You also can specify a string that contains a list or range of digital lines or ports."]
// ChangeDetect_DI_Tristate  #[doc = "Specifies whether to tristate lines specified with Rising Edge Physical Channels and Falling Edge Physical Channels that are not in a virtual channel in the task. If you set this property to TRUE, NI-DAQmx tristates rising/falling edge lines that are not in a virtual channel in the task. If you set this property to FALSE, NI-DAQmx does not modify the configuration of rising/falling edge lines that are not in a vir..."]
// OnDemand_SimultaneousAOEnable  #[doc = "Specifies whether to update all channels in the task simultaneously, rather than updating channels independently when you write a sample to that channel."]
// Implicit_UnderflowBehavior  #[doc = "Specifies the action to take when the onboard memory of the device becomes empty."]
// AIConv_Rate  #[doc = "Specifies in Hertz the rate at which to clock the analog-to-digital converter. This clock is specific to the analog input section of multiplexed devices."]
// AIConv_MaxRate  #[doc = "Indicates the maximum convert rate supported by the task, given the current devices and channel count."]
// AIConv_Src  #[doc = "Specifies the terminal of the signal to use as the AI Convert Clock."]
// AIConv_ActiveEdge  #[doc = "Specifies on which edge of the clock pulse an analog-to-digital conversion takes place."]
// AIConv_TimebaseDiv  #[doc = "Specifies the number of AI Convert Clock Timebase pulses needed to produce a single AI Convert Clock pulse."]
// AIConv_Timebase_Src  #[doc = "Specifies the terminal  of the signal to use as the AI Convert Clock Timebase."]
// DelayFromSampClk_DelayUnits  #[doc = "Specifies the units of Delay."]
// DelayFromSampClk_Delay  #[doc = "Specifies the amount of time to wait after receiving a Sample Clock edge before beginning to acquire the sample. This value is in the units you specify with Delay Units."]
// AIConv_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the AI Convert Clock."]
// AIConv_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// AIConv_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AIConv_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AIConv_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// MasterTimebase_Rate  #[doc = "Specifies the rate of the Master Timebase."]
// MasterTimebase_Src  #[doc = "Specifies the terminal of the signal to use as the Master Timebase. On an E Series device, you can choose only between the onboard 20MHz Timebase or the RTSI7 terminal."]
// RefClk_Rate  #[doc = "Specifies the frequency of the Reference Clock."]
// RefClk_Src  #[doc = "Specifies the terminal of the signal to use as the Reference Clock."]
// SyncPulse_Type  #[doc = "Specifies the type of sync pulse used in the task."]
// SyncPulse_Src  #[doc = "Specifies the terminal of the signal to use as the synchronization pulse. The synchronization pulse resets the clock dividers and the ADCs/DACs on the device."]
// SyncPulse_Time_When  #[doc = "Specifies the start time of the sync pulse."]
// SyncPulse_Time_Timescale  #[doc = "Specifies the timescale to be used for timestamps for a sync pulse."]
// SyncPulse_SyncTime  #[doc = "Indicates in seconds the delay required to reset the ADCs/DACs after the device receives the synchronization pulse."]
// SyncPulse_MinDelayToStart  #[doc = "Specifies in seconds the amount of time that elapses after the master device issues the synchronization pulse before the task starts."]
// SyncPulse_ResetTime  #[doc = "Indicates in seconds the amount of time required for the ADCs or DACs on the device to reset. When synchronizing devices, query this property on all devices and note the largest reset time. Then, for each device, subtract the value of this property from the largest reset time and set Reset Delay to the resulting value."]
// SyncPulse_ResetDelay  #[doc = "Specifies in seconds the amount of time to wait after the Synchronization Pulse before resetting the ADCs or DACs on the device. When synchronizing devices, query Reset Time on all devices and note the largest reset time. Then, for each device, subtract the reset time from the largest reset time and set this property to the resulting value."]
// SyncPulse_Term  #[doc = "Indicates the name of the internal Synchronization Pulse terminal for the task. This property does not return the name of the source terminal."]
// SyncClk_Interval  #[doc = "Specifies the interval, in Sample Clock periods, between each internal Synchronization Clock pulse. NI-DAQmx uses this pulse for synchronization of triggers between multiple devices at different rates. Refer to device documentation for information about how to calculate this value."]
// SampTimingEngine  #[doc = "Specifies which timing engine to use for the task."]
// FirstSampTimestamp_Enable  #[doc = "Specifies whether to enable the first sample timestamp."]
// FirstSampTimestamp_Timescale  #[doc = "Specifies the timescale to be used for the first sample timestamp."]
// FirstSampTimestamp_Val  #[doc = "Indicates the timestamp of the first sample."]
// FirstSampClk_When  #[doc = "Specifies the time of the first sample clock pulse."]
// FirstSampClk_Timescale  #[doc = "Specifies the timescale to be used for the value of When."]
// FirstSampClk_Offset  #[doc = "Specifies, in seconds, the offset to apply to the When value. This offset modifies when the first sample clock occurs and is used to account for known delays in the signal path."]
// StartTrig_Type  #[doc = "Specifies the type of trigger to use to start a task."]
// StartTrig_Term  #[doc = "Indicates the name of the internal Start Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// DigEdge_StartTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Start Trigger."]
// DigEdge_StartTrig_Edge  #[doc = "Specifies on which edge of a digital pulse to start acquiring or generating samples."]
// DigEdge_StartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the trigger signal."]
// DigEdge_StartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// DigEdge_StartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// DigEdge_StartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// DigEdge_StartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device. If you set this property to TRUE, the device does not recognize and act upon the trigger until the next pulse of the internal timebase."]
// DigPattern_StartTrig_Src  #[doc = "Specifies the physical channels to use for pattern matching. The order of the physical channels determines the order of the pattern. If a port is included, the order of the physical channels within the port is in ascending order."]
// DigPattern_StartTrig_Pattern  #[doc = "Specifies the digital pattern that must be met for the Start Trigger to occur."]
// DigPattern_StartTrig_When  #[doc = "Specifies whether the Start Trigger occurs when the physical channels specified with Source match or differ from the digital pattern specified with Pattern."]
// AnlgEdge_StartTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Start Trigger."]
// AnlgEdge_StartTrig_Slope  #[doc = "Specifies on which slope of the trigger signal to start acquiring or generating samples."]
// AnlgEdge_StartTrig_Lvl  #[doc = "Specifies at what threshold in the units of the measurement or generation to start acquiring or generating samples. Use Slope to specify on which slope to trigger on this threshold."]
// AnlgEdge_StartTrig_Hyst  #[doc = "Specifies a hysteresis level in the units of the measurement or generation. If Slope is DAQmx_Val_RisingSlope, the trigger does not deassert until the source signal passes below  Level minus the hysteresis. If Slope is DAQmx_Val_FallingSlope, the trigger does not deassert until the source signal passes above Level plus the hysteresis. Hysteresis is always enabled. Set this property to a non-zero value to use hyste..."]
// AnlgEdge_StartTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// AnlgEdge_StartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay above or below the trigger level for the minimum pulse width before being recognized. Use filtering  for noisy trigger signals that transition in and out of the hysteresis window rapidly."]
// AnlgEdge_StartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// AnlgEdge_StartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AnlgEdge_StartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AnlgEdge_StartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// AnlgMultiEdge_StartTrig_Srcs  #[doc = "Specifies a list and/or range of analog sources that are going to be used for Analog triggering. Each source corresponds to an element in each of the Analog Multi Edge property arrays, if they are not empty."]
// AnlgMultiEdge_StartTrig_Slopes  #[doc = "Specifies an array of slopes on which to trigger task to start generating or acquiring samples. Each element of the array corresponds to a source in Start.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// AnlgMultiEdge_StartTrig_Lvls  #[doc = "Specifies an array of thresholds in the units of the measurement or generation to start acquiring or generating samples. Each element of the array corresponds to a source in Start.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// AnlgMultiEdge_StartTrig_Hysts  #[doc = "Specifies an array of hysteresis levels in the units of the measurement or generation. If the corresponding element of Start.AnlgMultiEdge.Slopes is Rising, the trigger does not deassert until the source signal passes below the corresponding element of Start.AnlgMultiEdge.Lvls minus the hysteresis. If Start.AnlgEdge.Slope is Falling, the trigger does not deassert until the source signal passes above Start.AnlgEdge..."]
// AnlgMultiEdge_StartTrig_Couplings  #[doc = "Specifies an array that describes the couplings for the corresponding source signal of the trigger if the source is a terminal rather than a virtual channel. Each element of the array corresponds to a source in Start.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// AnlgWin_StartTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Start Trigger."]
// AnlgWin_StartTrig_When  #[doc = "Specifies whether the task starts acquiring or generating samples when the signal enters or leaves the window you specify with Bottom and Top."]
// AnlgWin_StartTrig_Top  #[doc = "Specifies the upper limit of the window. Specify this value in the units of the measurement or generation."]
// AnlgWin_StartTrig_Btm  #[doc = "Specifies the lower limit of the window. Specify this value in the units of the measurement or generation."]
// AnlgWin_StartTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// AnlgWin_StartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay within the trigger window for the minimum pulse width before being recognized. Use filtering for noisy trigger signals that transition in and out of the window rapidly."]
// AnlgWin_StartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// AnlgWin_StartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AnlgWin_StartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AnlgWin_StartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// StartTrig_TrigWhen  #[doc = "Specifies when to trigger the start trigger."]
// StartTrig_Timescale  #[doc = "Specifies the timescale to be used for timestamps used in a time trigger."]
// StartTrig_TimestampEnable  #[doc = "Specifies whether the start trigger timestamp is enabled. If the timestamp is enabled but no resources are available, an error will be returned at run time."]
// StartTrig_TimestampTimescale  #[doc = "Specifies the start trigger timestamp timescale."]
// StartTrig_TimestampVal  #[doc = "Indicates the start trigger timestamp value."]
// StartTrig_Delay  #[doc = "Specifies an amount of time to wait after the Start Trigger is received before acquiring or generating the first sample. This value is in the units you specify with Delay Units."]
// StartTrig_DelayUnits  #[doc = "Specifies the units of Delay."]
// StartTrig_Retriggerable  #[doc = "Specifies whether a finite task resets and waits for another Start Trigger after the task completes. When you set this property to TRUE, the device performs a finite acquisition or generation each time the Start Trigger occurs until the task stops. The device ignores a trigger if it is in the process of acquiring or generating signals."]
// StartTrig_TrigWin  #[doc = "Specifies the period of time in seconds after the task starts during which the device may trigger. Once the window has expired, the device stops detecting triggers, and the task will finish after the device finishes acquiring post-trigger samples for any triggers detected. If no triggers are detected during the entire period, then no data will be returned. Ensure the period of time specified covers the entire time..."]
// StartTrig_RetriggerWin  #[doc = "Specifies the period of time in seconds after each trigger during which the device may trigger. Once the window has expired, the device stops detecting triggers, and the task will finish after the device finishes acquiring post-trigger samples that it already started. Ensure the period of time specified covers the entire time span desired for retrigger detection to avoid missed triggers. Specifying a Retrigger Win..."]
// StartTrig_MaxNumTrigsToDetect  #[doc = "Specifies the maximum number of times the task will detect a start trigger during the task. The number of times a trigger is detected and acted upon by the module may be less than the specified amount if the task stops early because of trigger/retrigger window expiration. Specifying the Maximum Number of Triggers to Detect to be 0 causes the driver to automatically set this value to the maximum possible number of ..."]
// RefTrig_Type  #[doc = "Specifies the type of trigger to use to mark a reference point for the measurement."]
// RefTrig_PretrigSamples  #[doc = "Specifies the minimum number of pretrigger samples to acquire from each channel before recognizing the reference trigger. Post-trigger samples per channel are equal to Samples Per Channel minus the number of pretrigger samples per channel."]
// RefTrig_Term  #[doc = "Indicates the name of the internal Reference Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// DigEdge_RefTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Reference Trigger."]
// DigEdge_RefTrig_Edge  #[doc = "Specifies on what edge of a digital pulse the Reference Trigger occurs."]
// DigEdge_RefTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the trigger signal."]
// DigEdge_RefTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// DigEdge_RefTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// DigEdge_RefTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// DigEdge_RefTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// DigPattern_RefTrig_Src  #[doc = "Specifies the physical channels to use for pattern matching. The order of the physical channels determines the order of the pattern. If a port is included, the order of the physical channels within the port is in ascending order."]
// DigPattern_RefTrig_Pattern  #[doc = "Specifies the digital pattern that must be met for the Reference Trigger to occur."]
// DigPattern_RefTrig_When  #[doc = "Specifies whether the Reference Trigger occurs when the physical channels specified with Source match or differ from the digital pattern specified with Pattern."]
// AnlgEdge_RefTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Reference Trigger."]
// AnlgEdge_RefTrig_Slope  #[doc = "Specifies on which slope of the source signal the Reference Trigger occurs."]
// AnlgEdge_RefTrig_Lvl  #[doc = "Specifies in the units of the measurement the threshold at which the Reference Trigger occurs.  Use Slope to specify on which slope to trigger at this threshold."]
// AnlgEdge_RefTrig_Hyst  #[doc = "Specifies a hysteresis level in the units of the measurement. If Slope is DAQmx_Val_RisingSlope, the trigger does not deassert until the source signal passes below Level minus the hysteresis. If Slope is DAQmx_Val_FallingSlope, the trigger does not deassert until the source signal passes above Level plus the hysteresis. Hysteresis is always enabled. Set this property to a non-zero value to use hysteresis."]
// AnlgEdge_RefTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// AnlgEdge_RefTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay above or below the trigger level for the minimum pulse width before being recognized. Use filtering  for noisy trigger signals that transition in and out of the hysteresis window rapidly."]
// AnlgEdge_RefTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width thefilter recognizes."]
// AnlgEdge_RefTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AnlgEdge_RefTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AnlgEdge_RefTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// AnlgMultiEdge_RefTrig_Srcs  #[doc = "Specifies a List and/or range of analog sources that are going to be used for Analog triggering. Each source corresponds to an element in each of the Analog Multi Edge property arrays, if they are not empty."]
// AnlgMultiEdge_RefTrig_Slopes  #[doc = "Specifies an array of slopes on which to trigger task to start generating or acquiring samples. Each element of the array corresponds to a source in Ref.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// AnlgMultiEdge_RefTrig_Lvls  #[doc = "Specifies an array of thresholds in the units of the measurement or generation to start acquiring or generating samples. Each element of the array corresponds to a source in Ref.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// AnlgMultiEdge_RefTrig_Hysts  #[doc = "Specifies an array of hysteresis levels in the units of the measurement or generation. If the corresponding element of Ref.AnlgMultiEdge.Slopes is Rising, the trigger does not deassert until the source signal passes below the corresponding element of Ref.AnlgMultiEdge.Lvls minus the hysteresis. If Ref.AnlgEdge.Slope is Falling, the trigger does not deassert until the source signal passes above Ref.AnlgEdge.Lvl plu..."]
// AnlgMultiEdge_RefTrig_Couplings  #[doc = "Specifies an array that describes the couplings for the corresponding source signal of the trigger if the source is a terminal rather than a virtual channel. Each element of the array corresponds to a source in Ref.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// AnlgWin_RefTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Reference Trigger."]
// AnlgWin_RefTrig_When  #[doc = "Specifies whether the Reference Trigger occurs when the source signal enters the window or when it leaves the window. Use Bottom and Top to specify the window."]
// AnlgWin_RefTrig_Top  #[doc = "Specifies the upper limit of the window. Specify this value in the units of the measurement."]
// AnlgWin_RefTrig_Btm  #[doc = "Specifies the lower limit of the window. Specify this value in the units of the measurement."]
// AnlgWin_RefTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// AnlgWin_RefTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay within the trigger window for the minimum pulse width before being recognized. Use filtering for noisy trigger signals that transition in and out of the window rapidly."]
// AnlgWin_RefTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// AnlgWin_RefTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AnlgWin_RefTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AnlgWin_RefTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// RefTrig_AutoTrigEnable  #[doc = "Specifies whether to send a software trigger to the device when a hardware trigger is no longer active in order to prevent a timeout."]
// RefTrig_AutoTriggered  #[doc = "Indicates whether a completed acquisition was triggered by the auto trigger. If an acquisition has not completed after the task starts, this property returns FALSE. This property is only applicable when Enable  is TRUE."]
// RefTrig_TimestampEnable  #[doc = "Specifies whether the reference trigger timestamp is enabled. If the timestamp is enabled but no resources are available, an error will be returned at run time."]
// RefTrig_TimestampTimescale  #[doc = "Specifies the reference trigger timestamp timescale."]
// RefTrig_TimestampVal  #[doc = "Indicates the reference trigger timestamp value."]
// RefTrig_Delay  #[doc = "Specifies in seconds the time to wait after the device receives the Reference Trigger before switching from pretrigger to posttrigger samples."]
// RefTrig_Retriggerable  #[doc = "Specifies whether a finite task resets, acquires pretrigger samples, and waits for another Reference Trigger after the task completes. When you set this property to TRUE, the device will acquire post-trigger samples, reset, and acquire pretrigger samples each time the Reference Trigger occurs until the task stops. The device ignores a trigger if it is in the process of acquiring signals."]
// RefTrig_TrigWin  #[doc = "Specifies the duration in seconds after the task starts during which the device may trigger. Once the window has passed, the device stops detecting triggers, and the task will stop after the device finishes acquiring post-trigger samples that it already started. If no triggers are detected during the entire period, then no data will be returned. Specifying a Trigger Window of -1 causes the window to be infinite."]
// RefTrig_RetriggerWin  #[doc = "Specifies the duration in seconds after each trigger during which the device may trigger. Once the window has passed, the device stops detecting triggers, and the task will stop after the device finishes acquiring post-trigger samples that it already started. Specifying a Retrigger Window of -1 causes the window to be infinite."]
// RefTrig_MaxNumTrigsToDetect  #[doc = "Specifies the maximum number of times the task will detect a reference trigger during the task. The number of times a trigger is detected and acted upon by the module may be less than the specified amount if the task stops early because of trigger/retrigger window expiration. Specifying the Maximum Number of Triggers to Detect to be 0 causes the driver to automatically set this value to the maximum possible number..."]
// AdvTrig_Type  #[doc = "(Deprecated) Specifies the type of trigger to use to advance to the next entry in a switch scan list."]
// DigEdge_AdvTrig_Src  #[doc = "(Deprecated) Specifies the name of a terminal where there is a digital signal to use as the source of the Advance Trigger."]
// DigEdge_AdvTrig_Edge  #[doc = "(Deprecated) Specifies on which edge of a digital signal to advance to the next entry in a scan list."]
// DigEdge_AdvTrig_DigFltr_Enable  #[doc = "(Deprecated) Specifies whether to apply the pulse width filter to the signal."]
// HshkTrig_Type  #[doc = "Specifies the type of Handshake Trigger to use."]
// Interlocked_HshkTrig_Src  #[doc = "Specifies the source terminal of the Handshake Trigger."]
// Interlocked_HshkTrig_AssertedLvl  #[doc = "Specifies the asserted level of the Handshake Trigger."]
// PauseTrig_Type  #[doc = "Specifies the type of trigger to use to pause a task."]
// PauseTrig_Term  #[doc = "Indicates the name of the internal Pause Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// AnlgLvl_PauseTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the trigger."]
// AnlgLvl_PauseTrig_When  #[doc = "Specifies whether the task pauses above or below the threshold you specify with Level."]
// AnlgLvl_PauseTrig_Lvl  #[doc = "Specifies the threshold at which to pause the task. Specify this value in the units of the measurement or generation. Use Pause When to specify whether the task pauses above or below this threshold."]
// AnlgLvl_PauseTrig_Hyst  #[doc = "Specifies a hysteresis level in the units of the measurement or generation. If Pause When is DAQmx_Val_AboveLvl, the trigger does not deassert until the source signal passes below Level minus the hysteresis. If Pause When is DAQmx_Val_BelowLvl, the trigger does not deassert until the source signal passes above Level plus the hysteresis. Hysteresis is always enabled. Set this property to a non-zero value to use hys..."]
// AnlgLvl_PauseTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// AnlgLvl_PauseTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay above or below the trigger level for the minimum pulse width before being recognized. Use filtering  for noisy trigger signals that transition in and out of the hysteresis window rapidly."]
// AnlgLvl_PauseTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// AnlgLvl_PauseTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AnlgLvl_PauseTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AnlgLvl_PauseTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// AnlgWin_PauseTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the trigger."]
// AnlgWin_PauseTrig_When  #[doc = "Specifies whether the task pauses while the trigger signal is inside or outside the window you specify with Bottom and Top."]
// AnlgWin_PauseTrig_Top  #[doc = "Specifies the upper limit of the window. Specify this value in the units of the measurement or generation."]
// AnlgWin_PauseTrig_Btm  #[doc = "Specifies the lower limit of the window. Specify this value in the units of the measurement or generation."]
// AnlgWin_PauseTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the terminal if the source is a terminal rather than a virtual channel."]
// AnlgWin_PauseTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay within the trigger window for the minimum pulse width before being recognized. Use filtering for noisy trigger signals that transition in and out of the window rapidly."]
// AnlgWin_PauseTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// AnlgWin_PauseTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// AnlgWin_PauseTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// AnlgWin_PauseTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// DigLvl_PauseTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Pause Trigger."]
// DigLvl_PauseTrig_When  #[doc = "Specifies whether the task pauses while the signal is high or low."]
// DigLvl_PauseTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the trigger signal."]
// DigLvl_PauseTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// DigLvl_PauseTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// DigLvl_PauseTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// DigLvl_PauseTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// DigPattern_PauseTrig_Src  #[doc = "Specifies the physical channels to use for pattern matching. The order of the physical channels determines the order of the pattern. If a port is included, the lines within the port are in ascending order."]
// DigPattern_PauseTrig_Pattern  #[doc = "Specifies the digital pattern that must be met for the Pause Trigger to occur."]
// DigPattern_PauseTrig_When  #[doc = "Specifies if the Pause Trigger occurs when the physical channels specified with Source match or differ from the digital pattern specified with Pattern."]
// ArmStartTrig_Type  #[doc = "Specifies the type of trigger to use to arm the task for a Start Trigger. If you configure an Arm Start Trigger, the task does not respond to a Start Trigger until the device receives the Arm Start Trigger."]
// ArmStart_Term  #[doc = "Indicates the name of the internal Arm Start Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// DigEdge_ArmStartTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Arm Start Trigger."]
// DigEdge_ArmStartTrig_Edge  #[doc = "Specifies on which edge of a digital signal to arm the task for a Start Trigger."]
// DigEdge_ArmStartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// DigEdge_ArmStartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// DigEdge_ArmStartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// DigEdge_ArmStartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// DigEdge_ArmStartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// ArmStartTrig_TrigWhen  #[doc = "Specifies when to trigger the arm start trigger."]
// ArmStartTrig_Timescale  #[doc = "Specifies the timescale to be used for timestamps used in an arm start time trigger."]
// ArmStartTrig_TimestampEnable  #[doc = "Specifies whether the arm start trigger timestamp is enabled. If the timestamp is enabled but no resources are available, an error will be returned at run time."]
// ArmStartTrig_TimestampTimescale  #[doc = "Specifies the arm start trigger timestamp timescale."]
// ArmStartTrig_TimestampVal  #[doc = "Indicates the arm start trigger timestamp value."]
// Trigger_SyncType  #[doc = "Specifies the role of the device in a synchronized system. Setting this value to  DAQmx_Val_Master or  DAQmx_Val_Slave enables trigger skew correction. If you enable trigger skew correction, set this property to DAQmx_Val_Master on only one device, and set this property to DAQmx_Val_Slave on the other devices."]
// Watchdog_Timeout  #[doc = "Specifies in seconds the amount of time until the watchdog timer expires. A value of -1 means the internal timer never expires. Set this input to -1 if you use an Expiration Trigger to expire the watchdog task."]
// WatchdogExpirTrig_Type  #[doc = "Specifies the type of trigger to use to expire a watchdog task."]
// WatchdogExpirTrig_TrigOnNetworkConnLoss  #[doc = "Specifies the watchdog timer behavior when the network connection is lost between the host and the chassis. If set to true, the watchdog timer expires when the chassis detects the loss of network connection."]
// DigEdge_WatchdogExpirTrig_Src  #[doc = "Specifies the name of a terminal where a digital signal exists to use as the source of the Expiration Trigger."]
// DigEdge_WatchdogExpirTrig_Edge  #[doc = "Specifies on which edge of a digital signal to expire the watchdog task."]
// Watchdog_DO_ExpirState  #[doc = "Specifies the state to which to set the digital physical channels when the watchdog task expires.  You cannot modify the expiration state of dedicated digital input physical channels."]
// Watchdog_AO_OutputType  #[doc = "Specifies the output type of the analog output physical channels when the watchdog task expires."]
// Watchdog_AO_ExpirState  #[doc = "Specifies the state to set the analog output physical channels when the watchdog task expires."]
// Watchdog_CO_ExpirState  #[doc = "Specifies the state to set the counter output channel terminal when the watchdog task expires."]
// Watchdog_HasExpired  #[doc = "Indicates if the watchdog timer expired. You can read this property only while the task is running."]
// Write_RelativeTo  #[doc = "Specifies the point in the buffer at which to write data. If you also specify an offset with Offset, the write operation begins at that offset relative to this point you select with this property."]
// Write_Offset  #[doc = "Specifies in samples per channel an offset at which a write operation begins. This offset is relative to the location you specify with Relative To."]
// Write_RegenMode  #[doc = "Specifies whether to allow NI-DAQmx to generate the same data multiple times."]
// Write_CurrWritePos  #[doc = "Indicates the position in the buffer of the next sample to generate. This value is identical for all channels in the task."]
// Write_OvercurrentChansExist  #[doc = "Indicates if the device(s) detected an overcurrent condition for any channel in the task. Reading this property clears the overcurrent status for all channels in the task. You must read this property before you read Overcurrent Channels. Otherwise, you will receive an error."]
// Write_OvercurrentChans  #[doc = "Indicates a list of names of any virtual channels in the task for which an overcurrent condition has been detected. You must read Overcurrent Channels Exist before you read this property. Otherwise, you will receive an error."]
// Write_OvertemperatureChansExist  #[doc = "Indicates if the device(s) detected an overtemperature condition in any virtual channel in the task. Reading this property clears the overtemperature status for all channels in the task. You must read this property before you read Overtemperature Channels. Otherwise, you will receive an error."]
// Write_OvertemperatureChans  #[doc = "Indicates a list of names of any overtemperature virtual channels. You must read Overtemperature Channels Exist before you read this property. Otherwise, you will receive an error. The list of names may be empty if the device cannot determine the source of the overtemperature."]
// Write_ExternalOvervoltageChansExist  #[doc = "Indicates if the device(s) detected an External Overvoltage condition for any channel in the task. Reading this property clears the External Overvoltage status for all channels in the task. You must read this property before you read External OvervoltageChans. Otherwise, you will receive an error."]
// Write_ExternalOvervoltageChans  #[doc = "Indicates a list of names of any virtual channels in the task for which an External Overvoltage condition has been detected. You must read External OvervoltageChansExist before you read this property. Otherwise, you will receive an error."]
// Write_OverloadedChansExist  #[doc = "Indicates if the device(s) detected an overload in any virtual channel in the task. Reading this property clears the overload status for all channels in the task. You must read this property before you read Overloaded Channels. Otherwise, you will receive an error."]
// Write_OverloadedChans  #[doc = "Indicates a list of names of any overloaded virtual channels in the task. You must read Overloaded Channels Exist before you read this property. Otherwise, you will receive an error."]
// Write_OpenCurrentLoopChansExist  #[doc = "Indicates if the device(s) detected an open current loop for any channel in the task. Reading this property clears the open current loop status for all channels in the task. You must read this property before you read Open Current Loop Channels. Otherwise, you will receive an error."]
// Write_OpenCurrentLoopChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an open current loop. You must read Open Current Loop Channels Exist before you read this property. Otherwise, you will receive an error."]
// Write_PowerSupplyFaultChansExist  #[doc = "Indicates if the device(s) detected a power supply fault for any channel in the task. Reading this property clears the power supply fault status for all channels in the task. You must read this property before you read Power Supply Fault Channels. Otherwise, you will receive an error."]
// Write_PowerSupplyFaultChans  #[doc = "Indicates a list of names of any virtual channels in the task that have a power supply fault. You must read Power Supply Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// Write_Sync_UnlockedChansExist  #[doc = "Indicates whether the target is currently locked to the grand master. Devices may report PLL Unlock either during acquisition or after acquisition."]
// Write_Sync_UnlockedChans  #[doc = "Indicates the channels from devices in an unlocked target."]
// Write_SpaceAvail  #[doc = "Indicates in samples per channel the amount of available space in the buffer."]
// Write_TotalSampPerChanGenerated  #[doc = "Indicates the total number of samples generated by each channel in the task. This value is identical for all channels in the task."]
// Write_AccessoryInsertionOrRemovalDetected  #[doc = "Indicates if any devices in the task detected the insertion or removal of an accessory since the task started. Reading this property clears the accessory change status for all channels in the task. You must read this property before you read Devices with Inserted or Removed Accessories. Otherwise, you will receive an error."]
// Write_DevsWithInsertedOrRemovedAccessories  #[doc = "Indicates the names of any devices that detected the insertion or removal of an accessory since the task started. You must read Accessory Insertion or Removal Detected before you read this property. Otherwise, you will receive an error."]
// Write_RawDataWidth  #[doc = "Indicates in bytes the required size of a raw sample to write to the task."]
// Write_NumChans  #[doc = "Indicates the number of channels that an NI-DAQmx Write function writes to the task. This value is the number of channels in the task."]
// Write_WaitMode  #[doc = "Specifies how an NI-DAQmx Write function waits for space to become available in the buffer."]
// Write_SleepTime  #[doc = "Specifies in seconds the amount of time to sleep after checking for available buffer space if Wait Mode is DAQmx_Val_Sleep."]
// Write_DigitalLines_BytesPerChan  #[doc = "Indicates the number of Boolean values expected per channel in a sample for line-based writes. This property is determined by the channel in the task with the most digital lines. If a channel has fewer lines than this number, NI-DAQmx ignores the extra Boolean values."]

// #define DAQmx_SelfCal_Supported  #[doc = "Indicates whether the device supports self-calibration."]
// #define DAQmx_SelfCal_LastTemp  #[doc = "Indicates in degrees Celsius the temperature of the device at the time of the last self-calibration. Compare this temperature to the current onboard temperature to determine if you should perform another calibration."]
// #define DAQmx_ExtCal_RecommendedInterval  #[doc = "Indicates in months the National Instruments recommended interval between each external calibration of the device."]
// #define DAQmx_ExtCal_LastTemp  #[doc = "Indicates in degrees Celsius the temperature of the device at the time of the last external calibration. Compare this temperature to the current onboard temperature to determine if you should perform another calibration."]
// #define DAQmx_Cal_UserDefinedInfo  #[doc = "Specifies a string that contains arbitrary, user-defined information. This number of characters in this string can be no more than Max Size."]
// #define DAQmx_Cal_UserDefinedInfo_MaxSize  #[doc = "Indicates the maximum length in characters of Information."]
// #define DAQmx_Cal_DevTemp  #[doc = "Indicates in degrees Celsius the current temperature of the device."]
// #define DAQmx_Cal_AccConnectionCount  #[doc = "Specifies the number of times a particular connection that results in tangible wear and tear of onboard components has been made on the accessory. This connection count is useful for tracking accessory life and usage."]
// #define DAQmx_Cal_RecommendedAccConnectionCountLimit  #[doc = "Indicates the recommended connection count limit for an accessory. If the accessory connection count exceeds this limit, the accessory could require maintenance."]
// #define DAQmx_AI_Max  #[doc = "Specifies the maximum value you expect to measure. This value is in the units you specify with a units property. When you query this property, it returns the coerced maximum value that the device can measure with the current settings."]
// #define DAQmx_AI_Min  #[doc = "Specifies the minimum value you expect to measure. This value is in the units you specify with a units property.  When you query this property, it returns the coerced minimum value that the device can measure with the current settings."]
// #define DAQmx_AI_CustomScaleName  #[doc = "Specifies the name of a custom scale for the channel."]
// #define DAQmx_AI_MeasType  #[doc = "Indicates the measurement to take with the analog input channel and in some cases, such as for temperature measurements, the sensor to use."]
// #define DAQmx_AI_Voltage_Units  #[doc = "Specifies the units to use to return voltage measurements from the channel."]
// #define DAQmx_AI_Voltage_dBRef  #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes."]
// #define DAQmx_AI_Voltage_ACRMS_Units  #[doc = "Specifies the units to use to return voltage RMS measurements from the channel."]
// #define DAQmx_AI_Temp_Units  #[doc = "Specifies the units to use to return temperature measurements from the channel."]
// #define DAQmx_AI_Thrmcpl_Type  #[doc = "Specifies the type of thermocouple connected to the channel. Thermocouple types differ in composition and measurement range."]
// #define DAQmx_AI_Thrmcpl_ScaleType  #[doc = "Specifies the method or equation form that the thermocouple scale uses."]
// #define DAQmx_AI_Thrmcpl_CJCSrc  #[doc = "Indicates the source of cold-junction compensation."]
// #define DAQmx_AI_Thrmcpl_CJCVal  #[doc = "Specifies the temperature of the cold junction if CJC Source is DAQmx_Val_ConstVal. Specify this value in the units of the measurement."]
// #define DAQmx_AI_Thrmcpl_CJCChan  #[doc = "Indicates the channel that acquires the temperature of the cold junction if CJC Source is DAQmx_Val_Chan. If the channel is a temperature channel, NI-DAQmx acquires the temperature in the correct units. Other channel types, such as a resistance channel with a custom sensor, must use a custom scale to scale values to degrees Celsius."]
// #define DAQmx_AI_RTD_Type  #[doc = "Specifies the type of RTD connected to the channel."]
// #define DAQmx_AI_RTD_R0  #[doc = "Specifies in ohms the sensor resistance at 0 deg C. The Callendar-Van Dusen equation requires this value. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_RTD_A  #[doc = "Specifies the 'A' constant of the Callendar-Van Dusen equation. NI-DAQmx requires this value when you use a custom RTD."]
// #define DAQmx_AI_RTD_B  #[doc = "Specifies the 'B' constant of the Callendar-Van Dusen equation. NI-DAQmx requires this value when you use a custom RTD."]
// #define DAQmx_AI_RTD_C  #[doc = "Specifies the 'C' constant of the Callendar-Van Dusen equation. NI-DAQmx requires this value when you use a custom RTD."]
// #define DAQmx_AI_Thrmstr_A  #[doc = "Specifies the 'A' constant of the Steinhart-Hart thermistor equation."]
// #define DAQmx_AI_Thrmstr_B  #[doc = "Specifies the 'B' constant of the Steinhart-Hart thermistor equation."]
// #define DAQmx_AI_Thrmstr_C  #[doc = "Specifies the 'C' constant of the Steinhart-Hart thermistor equation."]
// #define DAQmx_AI_Thrmstr_R1  #[doc = "Specifies in ohms the value of the reference resistor for the thermistor if you use voltage excitation. NI-DAQmx ignores this value for current excitation."]
// #define DAQmx_AI_ForceReadFromChan  #[doc = "Specifies whether to read from the channel if it is a cold-junction compensation channel. By default, an NI-DAQmx Read function does not return data from cold-junction compensation channels.  Setting this property to TRUE forces read operations to return the cold-junction compensation channel data with the other channels in the task."]
// #define DAQmx_AI_Current_Units  #[doc = "Specifies the units to use to return current measurements from the channel."]
// #define DAQmx_AI_Current_ACRMS_Units  #[doc = "Specifies the units to use to return current RMS measurements from the channel."]
// #define DAQmx_AI_Strain_Units  #[doc = "Specifies the units to use to return strain measurements from the channel."]
// #define DAQmx_AI_StrainGage_ForceReadFromChan  #[doc = "Specifies whether the data is returned by an NI-DAQmx Read function when set on a raw strain channel that is part of a rosette configuration."]
// #define DAQmx_AI_StrainGage_GageFactor  #[doc = "Specifies the sensitivity of the strain gage.  Gage factor relates the change in electrical resistance to the change in strain. Refer to the sensor documentation for this value."]
// #define DAQmx_AI_StrainGage_PoissonRatio  #[doc = "Specifies the ratio of lateral strain to axial strain in the material you are measuring."]
// #define DAQmx_AI_StrainGage_Cfg  #[doc = "Specifies the bridge configuration of the strain gages."]
// #define DAQmx_AI_RosetteStrainGage_RosetteType  #[doc = "Indicates the type of rosette gage."]
// #define DAQmx_AI_RosetteStrainGage_Orientation  #[doc = "Specifies gage orientation in degrees with respect to the X axis."]
// #define DAQmx_AI_RosetteStrainGage_StrainChans  #[doc = "Indicates the raw strain channels that comprise the strain rosette."]
// #define DAQmx_AI_RosetteStrainGage_RosetteMeasType  #[doc = "Specifies the type of rosette measurement."]
// #define DAQmx_AI_Resistance_Units  #[doc = "Specifies the units to use to return resistance measurements."]
// #define DAQmx_AI_Freq_Units  #[doc = "Specifies the units to use to return frequency measurements from the channel."]
// #define DAQmx_AI_Freq_ThreshVoltage  #[doc = "Specifies the voltage level at which to recognize waveform repetitions. You should select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
// #define DAQmx_AI_Freq_Hyst  #[doc = "Specifies in volts a window below Threshold Level. The input voltage must pass below Threshold Level minus this value before NI-DAQmx recognizes a waveform repetition at Threshold Level. Hysteresis can improve the measurement accuracy when the signal contains noise or jitter."]
// #define DAQmx_AI_LVDT_Units  #[doc = "Specifies the units to use to return linear position measurements from the channel."]
// #define DAQmx_AI_LVDT_Sensitivity  #[doc = "Specifies the sensitivity of the LVDT. This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_LVDT_SensitivityUnits  #[doc = "Specifies the units of Sensitivity."]
// #define DAQmx_AI_RVDT_Units  #[doc = "Specifies the units to use to return angular position measurements from the channel."]
// #define DAQmx_AI_RVDT_Sensitivity  #[doc = "Specifies the sensitivity of the RVDT. This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_RVDT_SensitivityUnits  #[doc = "Specifies the units of Sensitivity."]
// #define DAQmx_AI_EddyCurrentProxProbe_Units  #[doc = "Specifies the units to use to return proximity measurements from the channel."]
// #define DAQmx_AI_EddyCurrentProxProbe_Sensitivity  #[doc = "Specifies the sensitivity of the eddy current proximity probe . This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_EddyCurrentProxProbe_SensitivityUnits  #[doc = "Specifies the units of Sensitivity."]
// #define DAQmx_AI_SoundPressure_MaxSoundPressureLvl  #[doc = "Specifies the maximum instantaneous sound pressure level you expect to measure. This value is in decibels, referenced to 20 micropascals. NI-DAQmx uses the maximum sound pressure level to calculate values in pascals for Maximum Value and Minimum Value for the channel."]
// #define DAQmx_AI_SoundPressure_Units  #[doc = "Specifies the units to use to return sound pressure measurements from the channel."]
// #define DAQmx_AI_SoundPressure_dBRef  #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes. NI-DAQmx also uses the decibel reference level when converting Maximum Sound Pressure Level to a voltage level."]
// #define DAQmx_AI_Microphone_Sensitivity  #[doc = "Specifies the sensitivity of the microphone. This value is in mV/Pa. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_Accel_Units  #[doc = "Specifies the units to use to return acceleration measurements from the channel."]
// #define DAQmx_AI_Accel_dBRef  #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes."]
// #define DAQmx_AI_Accel_4WireDCVoltage_Sensitivity  #[doc = "Specifies the sensitivity of the 4 wire DC voltage acceleration sensor connected to the channel. This value is the units you specify with AI.Accel.4WireDCVoltage.SensitivityUnits. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_Accel_4WireDCVoltage_SensitivityUnits  #[doc = "Specifies the units of AI.Accel.4WireDCVoltage.Sensitivity."]
// #define DAQmx_AI_Accel_Sensitivity  #[doc = "Specifies the sensitivity of the accelerometer. This value is in the units you specify with Sensitivity Units. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_Accel_SensitivityUnits  #[doc = "Specifies the units of Sensitivity."]
// #define DAQmx_AI_Accel_Charge_Sensitivity  #[doc = "Specifies the sensitivity of the charge acceleration sensor connected to the channel. This value is the units you specify with AI.Accel.Charge.SensitivityUnits. Refer to the sensor documentation to determine this value."]
// #define DAQmx_AI_Accel_Charge_SensitivityUnits  #[doc = "Specifies the units of AI.Accel.Charge.Sensitivity."]
// #define DAQmx_AI_Velocity_Units  #[doc = "Specifies in which unit to return velocity measurements from the channel."]
// #define DAQmx_AI_Velocity_IEPESensor_dBRef  #[doc = "Specifies the decibel reference level in the units of the channel. When you read samples as a waveform, the decibel reference level is included in the waveform attributes."]
// #define DAQmx_AI_Velocity_IEPESensor_Sensitivity  #[doc = "Specifies the sensitivity of the IEPE velocity sensor connected to the channel. Specify this value in the unit indicated by Sensitivity Units."]
// #define DAQmx_AI_Velocity_IEPESensor_SensitivityUnits  #[doc = "Specifies the units for Sensitivity."]
// #define DAQmx_AI_Force_Units  #[doc = "Specifies in which unit to return force or load measurements from the channel."]
// #define DAQmx_AI_Force_IEPESensor_Sensitivity  #[doc = "Specifies the sensitivity of the IEPE force sensor connected to the channel. Specify this value in the unit indicated by Sensitivity Units."]
// #define DAQmx_AI_Force_IEPESensor_SensitivityUnits  #[doc = "Specifies the units for Sensitivity."]
// #define DAQmx_AI_Pressure_Units  #[doc = "Specifies  in which unit to return pressure measurements from the channel."]
// #define DAQmx_AI_Torque_Units  #[doc = "Specifies in which unit to return torque measurements from the channel."]
// #define DAQmx_AI_Bridge_Units  #[doc = "Specifies in which unit to return voltage ratios from the channel."]
// #define DAQmx_AI_Bridge_ElectricalUnits  #[doc = "Specifies from which electrical unit to scale data. Select  the same unit that the sensor data sheet or calibration certificate uses for electrical values."]
// #define DAQmx_AI_Bridge_PhysicalUnits  #[doc = "Specifies to which physical unit to scale electrical data. Select the same unit that the sensor data sheet or calibration certificate uses for physical values."]
// #define DAQmx_AI_Bridge_ScaleType  #[doc = "Specifies the scaling type to use when scaling electrical values from the sensor to physical units."]
// #define DAQmx_AI_Bridge_TwoPointLin_First_ElectricalVal  #[doc = "Specifies the first electrical value, corresponding to Physical Value. Specify this value in the unit indicated by Electrical Units."]
// #define DAQmx_AI_Bridge_TwoPointLin_First_PhysicalVal  #[doc = "Specifies the first physical value, corresponding to Electrical Value. Specify this value in the unit indicated by Physical Units."]
// #define DAQmx_AI_Bridge_TwoPointLin_Second_ElectricalVal  #[doc = "Specifies the second electrical value, corresponding to Physical Value. Specify this value in the unit indicated by Electrical Units."]
// #define DAQmx_AI_Bridge_TwoPointLin_Second_PhysicalVal  #[doc = "Specifies the second physical value, corresponding to Electrical Value. Specify this value in the unit indicated by Physical Units."]
// #define DAQmx_AI_Bridge_Table_ElectricalVals  #[doc = "Specifies the array of electrical values that map to the values in Physical Values. Specify this value in the unit indicated by Electrical Units."]
// #define DAQmx_AI_Bridge_Table_PhysicalVals  #[doc = "Specifies the array of physical values that map to the values in Electrical Values. Specify this value in the unit indicated by Physical Units."]
// #define DAQmx_AI_Bridge_Poly_ForwardCoeff  #[doc = "Specifies an array of coefficients for the polynomial that converts electrical values to physical values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9x^3."]
// #define DAQmx_AI_Bridge_Poly_ReverseCoeff  #[doc = "Specifies an array of coefficients for the polynomial that converts physical values to electrical values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9x^3."]
// #define DAQmx_AI_Charge_Units  #[doc = "Specifies the units to use to return charge measurements from the channel."]
// #define DAQmx_AI_Is_TEDS  #[doc = "Indicates if the virtual channel was initialized using a TEDS bitstream from the corresponding physical channel."]
// #define DAQmx_AI_TEDS_Units  #[doc = "Indicates the units defined by TEDS information associated with the channel."]
// #define DAQmx_AI_Coupling  #[doc = "Specifies the coupling for the channel."]
// #define DAQmx_AI_Impedance  #[doc = "Specifies the input impedance of the channel."]
// #define DAQmx_AI_TermCfg  #[doc = "Specifies the terminal configuration for the channel."]
// #define DAQmx_AI_InputSrc  #[doc = "Specifies the source of the channel. You can use the signal from the I/O connector or one of several calibration signals. Certain devices have a single calibration signal bus. For these devices, you must specify the same calibration signal for all channels you connect to a calibration signal."]
// #define DAQmx_AI_ResistanceCfg  #[doc = "Specifies the resistance configuration for the channel. NI-DAQmx uses this value for any resistance-based measurements, including temperature measurement using a thermistor or RTD."]
// #define DAQmx_AI_LeadWireResistance  #[doc = "Specifies in ohms the resistance of the wires that lead to the sensor."]
// #define DAQmx_AI_Bridge_Cfg  #[doc = "Specifies the type of Wheatstone bridge connected to the channel."]
// #define DAQmx_AI_Bridge_NomResistance  #[doc = "Specifies in ohms the resistance of the bridge while not under load."]
// #define DAQmx_AI_Bridge_InitialVoltage  #[doc = "Specifies in volts the output voltage of the bridge while not under load. NI-DAQmx subtracts this value from any measurements before applying scaling equations.  If you set Initial Bridge Ratio, NI-DAQmx coerces this property to Initial Bridge Ratio times Actual Excitation Value. This property is set by DAQmx Perform Bridge Offset Nulling Calibration. If you set this property, NI-DAQmx coerces Initial Bridge Ratio..."]
// #define DAQmx_AI_Bridge_InitialRatio  #[doc = "Specifies in volts per volt the ratio of output voltage from the bridge to excitation voltage supplied to the bridge while not under load. NI-DAQmx subtracts this value from any measurements before applying scaling equations. If you set Initial Bridge Voltage, NI-DAQmx coerces this property  to Initial Bridge Voltage divided by Actual Excitation Value. If you set this property, NI-DAQmx coerces Initial Bridge Volt..."]
// #define DAQmx_AI_Bridge_ShuntCal_Enable  #[doc = "Specifies whether to enable a shunt calibration switch. Use Shunt Cal Select to select the switch(es) to enable."]
// #define DAQmx_AI_Bridge_ShuntCal_Select  #[doc = "Specifies which shunt calibration switch(es) to enable.  Use Shunt Cal Enable to enable the switch(es) you specify with this property."]
// #define DAQmx_AI_Bridge_ShuntCal_ShuntCalASrc  #[doc = "Specifies whether to use internal or external shunt when Shunt Cal A is selected."]
// #define DAQmx_AI_Bridge_ShuntCal_GainAdjust  #[doc = "Specifies the result of a shunt calibration. This property is set by DAQmx Perform Shunt Calibration. NI-DAQmx multiplies data read from the channel by the value of this property. This value should be close to 1.0."]
// #define DAQmx_AI_Bridge_ShuntCal_ShuntCalAResistance  #[doc = "Specifies in ohms the desired value of the internal shunt calibration A resistor."]
// #define DAQmx_AI_Bridge_ShuntCal_ShuntCalAActualResistance  #[doc = "Specifies in ohms the actual value of the internal shunt calibration A resistor."]
// #define DAQmx_AI_Bridge_ShuntCal_ShuntCalBResistance  #[doc = "Specifies in ohms the desired value of the internal shunt calibration B resistor."]
// #define DAQmx_AI_Bridge_ShuntCal_ShuntCalBActualResistance  #[doc = "Specifies in ohms the actual value of the internal shunt calibration B resistor."]
// #define DAQmx_AI_Bridge_Balance_CoarsePot  #[doc = "Specifies by how much to compensate for offset in the signal. This value can be between 0 and 127."]
// #define DAQmx_AI_Bridge_Balance_FinePot  #[doc = "Specifies by how much to compensate for offset in the signal. This value can be between 0 and 4095."]
// #define DAQmx_AI_CurrentShunt_Loc  #[doc = "Specifies the shunt resistor location for current measurements."]
// #define DAQmx_AI_CurrentShunt_Resistance  #[doc = "Specifies in ohms the external shunt resistance for current measurements."]
// #define DAQmx_AI_Excit_Sense  #[doc = "Specifies whether to use local or remote sense to sense excitation."]
// #define DAQmx_AI_Excit_Src  #[doc = "Specifies the source of excitation."]
// #define DAQmx_AI_Excit_Val  #[doc = "Specifies the amount of excitation that the sensor requires. If Voltage or Current is  DAQmx_Val_Voltage, this value is in volts. If Voltage or Current is  DAQmx_Val_Current, this value is in amperes."]
// #define DAQmx_AI_Excit_UseForScaling  #[doc = "Specifies if NI-DAQmx divides the measurement by the excitation. You should typically set this property to TRUE for ratiometric transducers. If you set this property to TRUE, set Maximum Value and Minimum Value to reflect the scaling."]
// #define DAQmx_AI_Excit_UseMultiplexed  #[doc = "Specifies if the SCXI-1122 multiplexes the excitation to the upper half of the channels as it advances through the scan list."]
// #define DAQmx_AI_Excit_ActualVal  #[doc = "Specifies the actual amount of excitation supplied by an internal excitation source.  If you read an internal excitation source more precisely with an external device, set this property to the value you read.  NI-DAQmx ignores this value for external excitation. When performing shunt calibration, some devices set this property automatically."]
// #define DAQmx_AI_Excit_DCorAC  #[doc = "Specifies if the excitation supply is DC or AC."]
// #define DAQmx_AI_Excit_VoltageOrCurrent  #[doc = "Specifies if the channel uses current or voltage excitation."]
// #define DAQmx_AI_Excit_IdleOutputBehavior  #[doc = "Specifies whether this channel will disable excitation after the task is uncommitted. Setting this to Zero Volts or Amps disables excitation after task uncommit. Setting this attribute to Maintain Existing Value leaves the excitation on after task uncommit."]
// #define DAQmx_AI_ACExcit_Freq  #[doc = "Specifies the AC excitation frequency in Hertz."]
// #define DAQmx_AI_ACExcit_SyncEnable  #[doc = "Specifies whether to synchronize the AC excitation source of the channel to that of another channel. Synchronize the excitation sources of multiple channels to use multichannel sensors. Set this property to FALSE for the master channel and to TRUE for the slave channels."]
// #define DAQmx_AI_ACExcit_WireMode  #[doc = "Specifies the number of leads on the LVDT or RVDT. Some sensors require you to tie leads together to create a four- or five- wire sensor. Refer to the sensor documentation for more information."]
// #define DAQmx_AI_SensorPower_Voltage  #[doc = "Specifies the voltage level for the sensor's power supply."]
// #define DAQmx_AI_SensorPower_Cfg  #[doc = "Specifies whether to turn on the sensor's power supply or to leave the configuration unchanged."]
// #define DAQmx_AI_SensorPower_Type  #[doc = "Specifies the type of power supplied to the sensor."]
// #define DAQmx_AI_OpenThrmcplDetectEnable  #[doc = "Specifies whether to apply the open thermocouple detection bias voltage to the channel. Changing the value of this property on a channel may require settling time before the data returned is valid. To compensate for this settling time, discard unsettled data or add a delay between committing and starting the task. Refer to your device specifications for the required settling time. When open thermocouple detection ..."]
// #define DAQmx_AI_Thrmcpl_LeadOffsetVoltage  #[doc = "Specifies the lead offset nulling voltage to subtract from measurements on a device. This property is ignored if open thermocouple detection is disabled."]
// #define DAQmx_AI_Atten  #[doc = "Specifies the amount of attenuation to use."]
// #define DAQmx_AI_ProbeAtten  #[doc = "Specifies the amount of attenuation provided by the probe connected to the channel. Specify this attenuation as a ratio."]
// #define DAQmx_AI_Lowpass_Enable  #[doc = "Specifies whether to enable the lowpass filter of the channel."]
// #define DAQmx_AI_Lowpass_CutoffFreq  #[doc = "Specifies the frequency in Hertz that corresponds to the -3dB cutoff of the filter."]
// #define DAQmx_AI_Lowpass_SwitchCap_ClkSrc  #[doc = "Specifies the source of the filter clock. If you need a higher resolution for the filter, you can supply an external clock to increase the resolution. Refer to the SCXI-1141/1142/1143 User Manual for more information."]
// #define DAQmx_AI_Lowpass_SwitchCap_ExtClkFreq  #[doc = "Specifies the frequency of the external clock when you set Clock Source to DAQmx_Val_External.  NI-DAQmx uses this frequency to set the pre- and post- filters on the SCXI-1141, SCXI-1142, and SCXI-1143. On those devices, NI-DAQmx determines the filter cutoff by using the equation f/(100*n), where f is the external frequency, and n is the external clock divisor. Refer to the SCXI-1141/1142/1143 User Manual for more..."]
// #define DAQmx_AI_Lowpass_SwitchCap_ExtClkDiv  #[doc = "Specifies the divisor for the external clock when you set Clock Source to DAQmx_Val_External. On the SCXI-1141, SCXI-1142, and SCXI-1143, NI-DAQmx determines the filter cutoff by using the equation f/(100*n), where f is the external frequency, and n is the external clock divisor. Refer to the SCXI-1141/1142/1143 User Manual for more information."]
// #define DAQmx_AI_Lowpass_SwitchCap_OutClkDiv  #[doc = "Specifies the divisor for the output clock.  NI-DAQmx uses the cutoff frequency to determine the output clock frequency. Refer to the SCXI-1141/1142/1143 User Manual for more information."]
// #define DAQmx_AI_DigFltr_Enable  #[doc = "Specifies whether the digital filter is enabled or disabled."]
// #define DAQmx_AI_DigFltr_Type  #[doc = "Specifies the digital filter type."]
// #define DAQmx_AI_DigFltr_Response  #[doc = "Specifies the digital filter response."]
// #define DAQmx_AI_DigFltr_Order  #[doc = "Specifies the order of the digital filter."]
// #define DAQmx_AI_DigFltr_Lowpass_CutoffFreq  #[doc = "Specifies the lowpass cutoff frequency of the digital filter."]
// #define DAQmx_AI_DigFltr_Highpass_CutoffFreq  #[doc = "Specifies the highpass cutoff frequency of the digital filter."]
// #define DAQmx_AI_DigFltr_Bandpass_CenterFreq  #[doc = "Specifies the center frequency of the passband for the digital filter."]
// #define DAQmx_AI_DigFltr_Bandpass_Width  #[doc = "Specifies the width of the passband centered around the center frequency for the digital filter."]
// #define DAQmx_AI_DigFltr_Notch_CenterFreq  #[doc = "Specifies the center frequency of the stopband for the digital filter."]
// #define DAQmx_AI_DigFltr_Notch_Width  #[doc = "Specifies the width of the stopband centered around the center frequency for the digital filter."]
// #define DAQmx_AI_DigFltr_Coeff  #[doc = "Specifies the digital filter coefficients."]
// #define DAQmx_AI_Filter_Enable  #[doc = "Specifies the corresponding filter enable/disable state."]
// #define DAQmx_AI_Filter_Freq  #[doc = "Specifies the corresponding filter frequency (cutoff or center) of the filter response."]
// #define DAQmx_AI_Filter_Response  #[doc = "Specifies the corresponding filter response and defines the shape of the filter response."]
// #define DAQmx_AI_Filter_Order  #[doc = "Specifies the corresponding filter order and defines the slope of the filter response."]
// #define DAQmx_AI_FilterDelay  #[doc = "Indicates the amount of time between when the ADC samples data and when the sample is read by the host device. This value is in the units you specify with Filter Delay Units. You can adjust this amount of time using Filter Delay Adjustment."]
// #define DAQmx_AI_FilterDelayUnits  #[doc = "Specifies the units of Filter Delay and Filter Delay Adjustment."]
// #define DAQmx_AI_RemoveFilterDelay  #[doc = "Specifies if filter delay removal is enabled on the device."]
// #define DAQmx_AI_FilterDelayAdjustment  #[doc = "Specifies the amount of filter delay that gets removed if Remove Filter Delay is enabled. This delay adjustment is in addition to the value indicated by Filter Delay. This delay adjustment is in the units you specify with Filter Delay Units."]
// #define DAQmx_AI_AveragingWinSize  #[doc = "Specifies the number of samples to average while acquiring data. Increasing the number of samples to average reduces noise in your measurement."]
// #define DAQmx_AI_ResolutionUnits  #[doc = "Indicates the units of Resolution Value."]
// #define DAQmx_AI_Resolution  #[doc = "Indicates the resolution of the analog-to-digital converter of the channel. This value is in the units you specify with Resolution Units."]
// #define DAQmx_AI_RawSampSize  #[doc = "Indicates in bits the size of a raw sample from the device."]
// #define DAQmx_AI_RawSampJustification  #[doc = "Indicates the justification of a raw sample from the device."]
// #define DAQmx_AI_ADCTimingMode  #[doc = "Specifies the ADC timing mode, controlling the tradeoff between speed and effective resolution. Some ADC timing modes provide increased powerline noise rejection. On devices that have an AI Convert clock, this setting affects both the maximum and default values for Rate. You must use the same ADC timing mode for all channels on a device, but you can use different ADC timing modes for different devices in the same ..."]
// #define DAQmx_AI_ADCCustomTimingMode  #[doc = "Specifies the timing mode of the ADC when Timing Mode is DAQmx_Val_Custom."]
// #define DAQmx_AI_Dither_Enable  #[doc = "Specifies whether to enable dithering.  Dithering adds Gaussian noise to the input signal. You can use dithering to achieve higher resolution measurements by over sampling the input signal and averaging the results."]
// #define DAQmx_AI_ChanCal_HasValidCalInfo  #[doc = "Indicates if the channel has calibration information."]
// #define DAQmx_AI_ChanCal_EnableCal  #[doc = "Specifies whether to enable the channel calibration associated with the channel."]
// #define DAQmx_AI_ChanCal_ApplyCalIfExp  #[doc = "Specifies whether to apply the channel calibration to the channel after the expiration date has passed."]
// #define DAQmx_AI_ChanCal_ScaleType  #[doc = "Specifies the method or equation form that the calibration scale uses."]
// #define DAQmx_AI_ChanCal_Table_PreScaledVals  #[doc = "Specifies the reference values collected when calibrating the channel."]
// #define DAQmx_AI_ChanCal_Table_ScaledVals  #[doc = "Specifies the acquired values collected when calibrating the channel."]
// #define DAQmx_AI_ChanCal_Poly_ForwardCoeff  #[doc = "Specifies the forward polynomial values used for calibrating the channel."]
// #define DAQmx_AI_ChanCal_Poly_ReverseCoeff  #[doc = "Specifies the reverse polynomial values used for calibrating the channel."]
// #define DAQmx_AI_ChanCal_OperatorName  #[doc = "Specifies the name of the operator who performed the channel calibration."]
// #define DAQmx_AI_ChanCal_Desc  #[doc = "Specifies the description entered for the calibration of the channel."]
// #define DAQmx_AI_ChanCal_Verif_RefVals  #[doc = "Specifies the reference values collected when verifying the calibration. NI-DAQmx stores these values as a record of calibration accuracy and does not use them in the scaling process."]
// #define DAQmx_AI_ChanCal_Verif_AcqVals  #[doc = "Specifies the acquired values collected when verifying the calibration. NI-DAQmx stores these values as a record of calibration accuracy and does not use them in the scaling process."]
// #define DAQmx_AI_Rng_High  #[doc = "Specifies the upper limit of the input range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
// #define DAQmx_AI_Rng_Low  #[doc = "Specifies the lower limit of the input range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
// #define DAQmx_AI_DCOffset  #[doc = "Specifies the DC value to add to the input range of the device. Use High and Low to specify the input range. This offset is in the native units of the device ."]
// #define DAQmx_AI_Gain  #[doc = "Specifies a gain factor to apply to the channel."]
// #define DAQmx_AI_SampAndHold_Enable  #[doc = "Specifies whether to enable the sample and hold circuitry of the device. When you disable sample and hold circuitry, a small voltage offset might be introduced into the signal.  You can eliminate this offset by using Auto Zero Mode to perform an auto zero on the channel."]
// #define DAQmx_AI_AutoZeroMode  #[doc = "Specifies how often to measure ground. NI-DAQmx subtracts the measured ground voltage from every sample."]
// #define DAQmx_AI_ChopEnable  #[doc = "Specifies whether the device will chop its inputs. Chopping removes offset voltages and other low frequency errors."]
// #define DAQmx_AI_DataXferMaxRate  #[doc = "Specifies the rate in B/s to transfer data from the device. If this value is not set, then the device will transfer data at a rate based on the bus detected. Modify this value to affect performance under different combinations of operating system, configuration, and device."]
// #define DAQmx_AI_DataXferMech  #[doc = "Specifies the data transfer mode for the device."]
// #define DAQmx_AI_DataXferReqCond  #[doc = "Specifies under what condition to transfer data from the onboard memory of the device to the buffer."]
// #define DAQmx_AI_DataXferCustomThreshold  #[doc = "Specifies the number of samples that must be in the FIFO to transfer data from the device if Data Transfer Request Condition is DAQmx_Val_OnbrdMemCustomThreshold."]
// #define DAQmx_AI_UsbXferReqSize  #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_AI_UsbXferReqCount  #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_AI_MemMapEnable  #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
// #define DAQmx_AI_RawDataCompressionType  #[doc = "Specifies the type of compression to apply to raw samples returned from the device."]
// #define DAQmx_AI_LossyLSBRemoval_CompressedSampSize  #[doc = "Specifies the number of bits to return in a raw sample when Raw Data Compression Type is set to DAQmx_Val_LossyLSBRemoval."]
// #define DAQmx_AI_DevScalingCoeff  #[doc = "Indicates the coefficients of a polynomial equation that NI-DAQmx uses to scale values from the native format of the device to volts. Each element of the array corresponds to a term of the equation. For example, if index two of the array is 4, the third term of the equation is 4x^2. Scaling coefficients do not account for any custom scales or sensors contained by the channel."]
// #define DAQmx_AI_EnhancedAliasRejectionEnable  #[doc = "Specifies whether to enable enhanced alias rejection. Leave this property set to the default value for most applications."]
// #define DAQmx_AI_OpenChanDetectEnable  #[doc = "Specifies whether to enable open channel detection."]
// #define DAQmx_AI_InputLimitsFaultDetect_UpperLimit  #[doc = "Specifies the level of the upper limit for input limits detection. An input sample outside the upper and lower bounds causes a fault. Note: Fault detection applies to both positive and negative inputs. For instance, if you specify a lower limit of 2 mA and an upper limit of 12 mA, NI-DAQmx detects a fault at 15 mA and -15 mA, but not at -6 mA because it is in the range of -12 mA to -2 mA."]
// #define DAQmx_AI_InputLimitsFaultDetect_LowerLimit  #[doc = "Specifies the level of the lower limit for input limits detection. An input sample outside the upper and lower bounds causes a fault. Note: Fault detection applies to both positive and negative inputs. For instance, if you specify a lower limit of 2 mA and an upper limit of 12 mA, NI-DAQmx detects a fault at 15 mA and -15 mA, but not at -6 mA because it is in the range of -12 mA to -2 mA."]
// #define DAQmx_AI_InputLimitsFaultDetectEnable  #[doc = "Specifies whether to enable input limits fault detection."]
// #define DAQmx_AI_PowerSupplyFaultDetectEnable  #[doc = "Specifies whether to enable power supply fault detection."]
// #define DAQmx_AI_OvercurrentDetectEnable  #[doc = "Specifies whether to enable overcurrent detection."]
// #define DAQmx_AO_Max  #[doc = "Specifies the maximum value you expect to generate. The value is in the units you specify with a units property. If you try to write a value larger than the maximum value, NI-DAQmx generates an error. NI-DAQmx might coerce this value to a smaller value if other task settings restrict the device from generating the desired maximum."]
// #define DAQmx_AO_Min  #[doc = "Specifies the minimum value you expect to generate. The value is in the units you specify with a units property. If you try to write a value smaller than the minimum value, NI-DAQmx generates an error. NI-DAQmx might coerce this value to a larger value if other task settings restrict the device from generating the desired minimum."]
// #define DAQmx_AO_CustomScaleName  #[doc = "Specifies the name of a custom scale for the channel."]
// #define DAQmx_AO_OutputType  #[doc = "Indicates whether the channel generates voltage,  current, or a waveform."]
// #define DAQmx_AO_Voltage_Units  #[doc = "Specifies in what units to generate voltage on the channel. Write data to the channel in the units you select."]
// #define DAQmx_AO_Voltage_CurrentLimit  #[doc = "Specifies the current limit, in amperes, for the voltage channel."]
// #define DAQmx_AO_Current_Units  #[doc = "Specifies in what units to generate current on the channel. Write data to the channel in the units you select."]
// #define DAQmx_AO_FuncGen_Type  #[doc = "Specifies the kind of the waveform to generate."]
// #define DAQmx_AO_FuncGen_Freq  #[doc = "Specifies the frequency of the waveform to generate in hertz."]
// #define DAQmx_AO_FuncGen_Amplitude  #[doc = "Specifies the zero-to-peak amplitude of the waveform to generate in volts. Zero and negative values are valid."]
// #define DAQmx_AO_FuncGen_Offset  #[doc = "Specifies the voltage offset of the waveform to generate."]
// #define DAQmx_AO_FuncGen_StartPhase  #[doc = "Specifies the starting phase in degrees of the waveform to generate."]
// #define DAQmx_AO_FuncGen_Square_DutyCycle  #[doc = "Specifies the square wave duty cycle of the waveform to generate."]
// #define DAQmx_AO_FuncGen_ModulationType  #[doc = "Specifies if the device generates a modulated version of the waveform using the original waveform as a carrier and input from an external terminal as the signal."]
// #define DAQmx_AO_FuncGen_FMDeviation  #[doc = "Specifies the FM deviation in hertz per volt when Type is DAQmx_Val_FM."]
// #define DAQmx_AO_OutputImpedance  #[doc = "Specifies in ohms the impedance of the analog output stage of the device."]
// #define DAQmx_AO_LoadImpedance  #[doc = "Specifies in ohms the load impedance connected to the analog output channel."]
// #define DAQmx_AO_IdleOutputBehavior  #[doc = "Specifies the state of the channel when no generation is in progress."]
// #define DAQmx_AO_TermCfg  #[doc = "Specifies the terminal configuration of the channel."]
// #define DAQmx_AO_Common_Mode_Offset  #[doc = "Specifies the common-mode offset of the AO channel. Use the property only when Terminal Configuration is set to Differential."]
// #define DAQmx_AO_ResolutionUnits  #[doc = "Specifies the units of Resolution Value."]
// #define DAQmx_AO_Resolution  #[doc = "Indicates the resolution of the digital-to-analog converter of the channel. This value is in the units you specify with Resolution Units."]
// #define DAQmx_AO_DAC_Rng_High  #[doc = "Specifies the upper limit of the output range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
// #define DAQmx_AO_DAC_Rng_Low  #[doc = "Specifies the lower limit of the output range of the device. This value is in the native units of the device. On E Series devices, for example, the native units is volts."]
// #define DAQmx_AO_DAC_Ref_ConnToGnd  #[doc = "Specifies whether to ground the internal DAC reference. Grounding the internal DAC reference has the effect of grounding all analog output channels and stopping waveform generation across all analog output channels regardless of whether the channels belong to the current task. You can ground the internal DAC reference only when Source is DAQmx_Val_Internal and Allow Connecting DAC Reference to Ground at Runtime is..."]
// #define DAQmx_AO_DAC_Ref_AllowConnToGnd  #[doc = "Specifies whether to allow grounding the internal DAC reference at run time. You must set this property to TRUE and set Source to DAQmx_Val_Internal before you can set Connect DAC Reference to Ground to TRUE."]
// #define DAQmx_AO_DAC_Ref_Src  #[doc = "Specifies the source of the DAC reference voltage. The value of this voltage source determines the full-scale value of the DAC."]
// #define DAQmx_AO_DAC_Ref_ExtSrc  #[doc = "Specifies the source of the DAC reference voltage if Source is DAQmx_Val_External. The valid sources for this signal vary by device."]
// #define DAQmx_AO_DAC_Ref_Val  #[doc = "Specifies in volts the value of the DAC reference voltage. This voltage determines the full-scale range of the DAC. Smaller reference voltages result in smaller ranges, but increased resolution."]
// #define DAQmx_AO_DAC_Offset_Src  #[doc = "Specifies the source of the DAC offset voltage. The value of this voltage source determines the full-scale value of the DAC."]
// #define DAQmx_AO_DAC_Offset_ExtSrc  #[doc = "Specifies the source of the DAC offset voltage if Source is DAQmx_Val_External. The valid sources for this signal vary by device."]
// #define DAQmx_AO_DAC_Offset_Val  #[doc = "Specifies in volts the value of the DAC offset voltage. To achieve best accuracy, the DAC offset value should be hand calibrated."]
// #define DAQmx_AO_ReglitchEnable  #[doc = "Specifies whether to enable reglitching.  The output of a DAC normally glitches whenever the DAC is updated with a new value. The amount of glitching differs from code to code and is generally largest at major code transitions.  Reglitching generates uniform glitch energy at each code transition and provides for more uniform glitches.  Uniform glitch energy makes it easier to filter out the noise introduced from g..."]
// #define DAQmx_AO_FilterDelay  #[doc = "Specifies the amount of time between when the sample is written by the host device and when the sample is output by the DAC. This value is in the units you specify with Filter Delay Units."]
// #define DAQmx_AO_FilterDelayUnits  #[doc = "Specifies the units of Filter Delay and Filter Delay Adjustment."]
// #define DAQmx_AO_FilterDelayAdjustment  #[doc = "Specifies an additional amount of time to wait between when the sample is written by the host device and when the sample is output by the DAC. This delay adjustment is in addition to the value indicated by Filter Delay. This delay adjustment is in the units you specify with Filter Delay Units."]
// #define DAQmx_AO_Gain  #[doc = "Specifies in decibels the gain factor to apply to the channel."]
// #define DAQmx_AO_UseOnlyOnBrdMem  #[doc = "Specifies whether to write samples directly to the onboard memory of the device, bypassing the memory buffer. Generally, you cannot update onboard memory directly after you start the task. Onboard memory includes data FIFOs."]
// #define DAQmx_AO_DataXferMech  #[doc = "Specifies the data transfer mode for the device."]
// #define DAQmx_AO_DataXferReqCond  #[doc = "Specifies under what condition to transfer data from the buffer to the onboard memory of the device."]
// #define DAQmx_AO_UsbXferReqSize  #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_AO_UsbXferReqCount  #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_AO_MemMapEnable  #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
// #define DAQmx_AO_DevScalingCoeff  #[doc = "Indicates the coefficients of a linear equation that NI-DAQmx uses to scale values from a voltage to the native format of the device. Each element of the array corresponds to a term of the equation. The first element of the array corresponds to the y-intercept, and the second element corresponds to the slope. Scaling coefficients do not account for any custom scales that may be applied to the channel."]
// #define DAQmx_AO_EnhancedImageRejectionEnable  #[doc = "Specifies whether to enable the DAC interpolation filter. Disable the interpolation filter to improve DAC signal-to-noise ratio at the expense of degraded image rejection."]
// #define DAQmx_DI_InvertLines  #[doc = "Specifies whether to invert the lines in the channel. If you set this property to TRUE, the lines are at high logic when off and at low logic when on."]
// #define DAQmx_DI_NumLines  #[doc = "Indicates the number of digital lines in the channel."]
// #define DAQmx_DI_DigFltr_Enable  #[doc = "Specifies whether to enable the digital filter for the line(s) or port(s). You can enable the filter on a line-by-line basis. You do not have to enable the filter for all lines in a channel."]
// #define DAQmx_DI_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes as a valid high or low state transition."]
// #define DAQmx_DI_DigFltr_EnableBusMode  #[doc = "Specifies whether to enable bus mode for digital filtering. If you set this property to TRUE, NI-DAQmx treats all lines that use common filtering settings as a bus. If any line in the bus has jitter, all lines in the bus hold state until the entire bus stabilizes, or until 2 times the minimum pulse width elapses. If you set this property to FALSE, NI-DAQmx filters all lines individually. Jitter in one line does no..."]
// #define DAQmx_DI_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_DI_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_DI_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_DI_Tristate  #[doc = "Specifies whether to tristate the lines in the channel. If you set this property to TRUE, NI-DAQmx tristates the lines in the channel. If you set this property to FALSE, NI-DAQmx does not modify the configuration of the lines even if the lines were previously tristated. Set this property to FALSE to read lines in other tasks or to read output-only lines."]
// #define DAQmx_DI_LogicFamily  #[doc = "Specifies the logic family to use for acquisition. A logic family corresponds to voltage thresholds that are compatible with a group of voltage standards. Refer to the device documentation for information on the logic high and logic low voltages for these logic families."]
// #define DAQmx_DI_DataXferMech  #[doc = "Specifies the data transfer mode for the device."]
// #define DAQmx_DI_DataXferReqCond  #[doc = "Specifies under what condition to transfer data from the onboard memory of the device to the buffer."]
// #define DAQmx_DI_UsbXferReqSize  #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_DI_UsbXferReqCount  #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_DI_MemMapEnable  #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
// #define DAQmx_DI_AcquireOn  #[doc = "Specifies on which edge of the sample clock to acquire samples."]
// #define DAQmx_DO_OutputDriveType  #[doc = "Specifies the drive type for digital output channels."]
// #define DAQmx_DO_InvertLines  #[doc = "Specifies whether to invert the lines in the channel. If you set this property to TRUE, the lines are at high logic when off and at low logic when on."]
// #define DAQmx_DO_NumLines  #[doc = "Indicates the number of digital lines in the channel."]
// #define DAQmx_DO_Tristate  #[doc = "Specifies whether to stop driving the channel and set it to a high-impedance state. You must commit the task for this setting to take effect."]
// #define DAQmx_DO_LineStates_StartState  #[doc = "Specifies the state of the lines in a digital output task when the task starts."]
// #define DAQmx_DO_LineStates_PausedState  #[doc = "Specifies the state of the lines in a digital output task when the task pauses."]
// #define DAQmx_DO_LineStates_DoneState  #[doc = "Specifies the state of the lines in a digital output task when the task completes execution."]
// #define DAQmx_DO_LogicFamily  #[doc = "Specifies the logic family to use for generation. A logic family corresponds to voltage thresholds that are compatible with a group of voltage standards. Refer to the device documentation for information on the logic high and logic low voltages for these logic families."]
// #define DAQmx_DO_Overcurrent_Limit  #[doc = "Specifies the current threshold in Amperes for the channel. A value of 0 means the channel observes no limit. Devices can monitor only a finite number of current thresholds simultaneously. If you attempt to monitor additional thresholds, NI-DAQmx returns an error."]
// #define DAQmx_DO_Overcurrent_AutoReenable  #[doc = "Specifies whether to automatically reenable channels after they no longer exceed the current limit specified by Current Limit."]
// #define DAQmx_DO_Overcurrent_ReenablePeriod  #[doc = "Specifies the delay in seconds between the time a channel no longer exceeds the current limit and the reactivation of that channel, if Automatic Re-enable is TRUE."]
// #define DAQmx_DO_UseOnlyOnBrdMem  #[doc = "Specifies whether to write samples directly to the onboard memory of the device, bypassing the memory buffer. Generally, you cannot update onboard memory after you start the task. Onboard memory includes data FIFOs."]
// #define DAQmx_DO_DataXferMech  #[doc = "Specifies the data transfer mode for the device."]
// #define DAQmx_DO_DataXferReqCond  #[doc = "Specifies under what condition to transfer data from the buffer to the onboard memory of the device."]
// #define DAQmx_DO_UsbXferReqSize  #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_DO_UsbXferReqCount  #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_DO_MemMapEnable  #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
// #define DAQmx_DO_GenerateOn  #[doc = "Specifies on which edge of the sample clock to generate samples."]
// #define DAQmx_CI_Max  #[doc = "Specifies the maximum value you expect to measure. This value is in the units you specify with a units property. When you query this property, it returns the coerced maximum value that the hardware can measure with the current settings."]
// #define DAQmx_CI_Min  #[doc = "Specifies the minimum value you expect to measure. This value is in the units you specify with a units property. When you query this property, it returns the coerced minimum value that the hardware can measure with the current settings."]
// #define DAQmx_CI_CustomScaleName  #[doc = "Specifies the name of a custom scale for the channel."]
// #define DAQmx_CI_MeasType  #[doc = "Indicates the measurement to take with the channel."]
// #define DAQmx_CI_Freq_Units  #[doc = "Specifies the units to use to return frequency measurements."]
// #define DAQmx_CI_Freq_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_Freq_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Freq_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_Freq_ThreshVoltage  #[doc = "Specifies the voltage level at which to recognize waveform repetitions. Select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
// #define DAQmx_CI_Freq_Hyst  #[doc = "Specifies a hysteresis level to apply to Threshold Level. When Starting Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Starting Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
// #define DAQmx_CI_Freq_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Freq_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Freq_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Freq_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Freq_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Freq_StartingEdge  #[doc = "Specifies between which edges to measure the frequency of the signal."]
// #define DAQmx_CI_Freq_MeasMeth  #[doc = "Specifies the method to use to measure the frequency of the signal."]
// #define DAQmx_CI_Freq_EnableAveraging  #[doc = "Specifies whether to enable averaging mode for Sample Clock-timed frequency measurements."]
// #define DAQmx_CI_Freq_MeasTime  #[doc = "Specifies in seconds the length of time to measure the frequency of the signal if Method is DAQmx_Val_HighFreq2Ctr. Measurement accuracy increases with increased measurement time and with increased signal frequency. If you measure a high-frequency signal for too long, however, the count register could roll over, which results in an incorrect measurement."]
// #define DAQmx_CI_Freq_Div  #[doc = "Specifies the value by which to divide the input signal if  Method is DAQmx_Val_LargeRng2Ctr. The larger the divisor, the more accurate the measurement. However, too large a value could cause the count register to roll over, which results in an incorrect measurement."]
// #define DAQmx_CI_Period_Units  #[doc = "Specifies the unit to use to return period measurements."]
// #define DAQmx_CI_Period_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_Period_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Period_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_Period_ThreshVoltage  #[doc = "Specifies the voltage level at which to recognize waveform repetitions. Select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
// #define DAQmx_CI_Period_Hyst  #[doc = "Specifies a hysteresis level to apply to Threshold Level. When Starting Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Starting Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
// #define DAQmx_CI_Period_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Period_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Period_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Period_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Period_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Period_StartingEdge  #[doc = "Specifies between which edges to measure the period of the signal."]
// #define DAQmx_CI_Period_MeasMeth  #[doc = "Specifies the method to use to measure the period of the signal."]
// #define DAQmx_CI_Period_EnableAveraging  #[doc = "Specifies whether to enable averaging mode for Sample Clock-timed period measurements."]
// #define DAQmx_CI_Period_MeasTime  #[doc = "Specifies in seconds the length of time to measure the period of the signal if Method is DAQmx_Val_HighFreq2Ctr. Measurement accuracy increases with increased measurement time and with increased signal frequency. If you measure a high-frequency signal for too long, however, the count register could roll over, which results in an incorrect measurement."]
// #define DAQmx_CI_Period_Div  #[doc = "Specifies the value by which to divide the input signal if Method is DAQmx_Val_LargeRng2Ctr. The larger the divisor, the more accurate the measurement. However, too large a value could cause the count register to roll over, which results in an incorrect measurement."]
// #define DAQmx_CI_CountEdges_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_CountEdges_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_CountEdges_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_CountEdges_ThreshVoltage  #[doc = "Specifies the voltage level at which to recognize waveform repetitions. Select a voltage level that occurs only once within the entire period of a waveform. You also can select a voltage that occurs only once while the voltage rises or falls."]
// #define DAQmx_CI_CountEdges_Hyst  #[doc = "Specifies a hysteresis level to apply to Threshold Level. When Active Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Active Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
// #define DAQmx_CI_CountEdges_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_CountEdges_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_CountEdges_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_CountEdges_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_CountEdges_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_CountEdges_Dir  #[doc = "Specifies whether to increment or decrement the counter on each edge."]
// #define DAQmx_CI_CountEdges_DirTerm  #[doc = "Specifies the source terminal of the digital signal that controls the count direction if Direction is DAQmx_Val_ExtControlled."]
// #define DAQmx_CI_CountEdges_CountDir_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_CountEdges_CountDir_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_CountEdges_CountDir_ThreshVoltage  #[doc = "Specifies the voltage level applied to the Count Direction terminal. When the signal is above this threshold, the counter counts up. When the signal is below this threshold, the counter counts down."]
// #define DAQmx_CI_CountEdges_CountDir_Hyst  #[doc = "Specifies a hysteresis level applied to the Threshold Level. The source signal must fall below Threshold Level minus the hysteresis before a change in count direction occurs."]
// #define DAQmx_CI_CountEdges_CountDir_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_CountEdges_CountDir_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_CountEdges_CountDir_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_CountEdges_CountDir_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_CountEdges_CountDir_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_CountEdges_InitialCnt  #[doc = "Specifies the starting value from which to count."]
// #define DAQmx_CI_CountEdges_ActiveEdge  #[doc = "Specifies on which edges to increment or decrement the counter."]
// #define DAQmx_CI_CountEdges_CountReset_Enable  #[doc = "Specifies whether to reset the count on the active edge specified with Terminal."]
// #define DAQmx_CI_CountEdges_CountReset_ResetCount  #[doc = "Specifies the value to reset the count to."]
// #define DAQmx_CI_CountEdges_CountReset_Term  #[doc = "Specifies the input terminal of the signal to reset the count."]
// #define DAQmx_CI_CountEdges_CountReset_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_CountEdges_CountReset_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_CountEdges_CountReset_ThreshVoltage  #[doc = "Specifies the voltage level at which to recognize the counter reset event."]
// #define DAQmx_CI_CountEdges_CountReset_Hyst  #[doc = "Specifies a hysteresis level applied to Threshold Level. When Active Edge is rising, the source signal must first fall below Threshold Level minus the hysteresis before a rising edge is detected at Threshold Level. When Active Edge is falling, the source signal must first rise above Threshold Level plus the hysteresis before a falling edge is detected at Threshold Level."]
// #define DAQmx_CI_CountEdges_CountReset_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_CountEdges_CountReset_DigFltr_MinPulseWidth  #[doc = "Specifies the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_CountEdges_CountReset_DigFltr_TimebaseSrc  #[doc = "Specifies the input of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_CountEdges_CountReset_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_CountEdges_CountReset_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_CountEdges_CountReset_ActiveEdge  #[doc = "Specifies on which edge of the signal to reset the count."]
// #define DAQmx_CI_CountEdges_Gate_Enable  #[doc = "Specifies whether to enable the functionality to gate the counter input signal for a count edges measurement."]
// #define DAQmx_CI_CountEdges_Gate_Term  #[doc = "Specifies the gate terminal."]
// #define DAQmx_CI_CountEdges_Gate_TermCfg  #[doc = "Specifies the gate terminal configuration."]
// #define DAQmx_CI_CountEdges_Gate_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the gate input line."]
// #define DAQmx_CI_CountEdges_Gate_ThreshVoltage  #[doc = "Specifies the voltage level at which to recognize the counter gate signal."]
// #define DAQmx_CI_CountEdges_Gate_Hyst  #[doc = "Specifies a hysteresis level applied to the Threshold Level. When Pause When is High, the source signal must fall below Threshold Level minus the hysteresis before the counter resumes counting. When Pause When is Low, the source signal must rise above Threshold Level plus the hysteresis before the counter resumes counting."]
// #define DAQmx_CI_CountEdges_Gate_DigFltrEnable  #[doc = "Specifies whether to apply the pulse width filter to the gate input signal."]
// #define DAQmx_CI_CountEdges_Gate_DigFltrMinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
// #define DAQmx_CI_CountEdges_Gate_DigFltrTimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_CountEdges_Gate_DigFltrTimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_CountEdges_GateWhen  #[doc = "Specifies whether the counter gates input pulses while the signal is high or low."]
// #define DAQmx_CI_DutyCycle_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_DutyCycle_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_DutyCycle_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_DutyCycle_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_DutyCycle_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
// #define DAQmx_CI_DutyCycle_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_DutyCycle_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_DutyCycle_StartingEdge  #[doc = "Specifies which edge of the input signal to begin the duty cycle measurement."]
// #define DAQmx_CI_AngEncoder_Units  #[doc = "Specifies the units to use to return angular position measurements from the channel."]
// #define DAQmx_CI_AngEncoder_PulsesPerRev  #[doc = "Specifies the number of pulses the encoder generates per revolution. This value is the number of pulses on either signal A or signal B, not the total number of pulses on both signal A and signal B."]
// #define DAQmx_CI_AngEncoder_InitialAngle  #[doc = "Specifies the starting angle of the encoder. This value is in the units you specify with Units."]
// #define DAQmx_CI_LinEncoder_Units  #[doc = "Specifies the units to use to return linear encoder measurements from the channel."]
// #define DAQmx_CI_LinEncoder_DistPerPulse  #[doc = "Specifies the distance to measure for each pulse the encoder generates on signal A or signal B. This value is in the units you specify with Units."]
// #define DAQmx_CI_LinEncoder_InitialPos  #[doc = "Specifies the position of the encoder when the measurement begins. This value is in the units you specify with Units."]
// #define DAQmx_CI_Encoder_DecodingType  #[doc = "Specifies how to count and interpret the pulses the encoder generates on signal A and signal B. DAQmx_Val_X1, DAQmx_Val_X2, and DAQmx_Val_X4 are valid for quadrature encoders only. DAQmx_Val_TwoPulseCounting is valid for two-pulse encoders only."]
// #define DAQmx_CI_Encoder_AInputTerm  #[doc = "Specifies the terminal to which signal A is connected."]
// #define DAQmx_CI_Encoder_AInputTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Encoder_AInputLogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_Encoder_AInput_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Encoder_AInput_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Encoder_AInput_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Encoder_AInput_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Encoder_AInput_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Encoder_BInputTerm  #[doc = "Specifies the terminal to which signal B is connected."]
// #define DAQmx_CI_Encoder_BInputTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Encoder_BInputLogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_Encoder_BInput_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Encoder_BInput_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Encoder_BInput_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Encoder_BInput_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Encoder_BInput_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Encoder_ZInputTerm  #[doc = "Specifies the terminal to which signal Z is connected."]
// #define DAQmx_CI_Encoder_ZInputTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Encoder_ZInputLogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_Encoder_ZInput_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Encoder_ZInput_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Encoder_ZInput_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Encoder_ZInput_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Encoder_ZInput_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Encoder_ZIndexEnable  #[doc = "Specifies whether to use Z indexing for the channel."]
// #define DAQmx_CI_Encoder_ZIndexVal  #[doc = "Specifies the value to which to reset the measurement when signal Z is high and signal A and signal B are at the states you specify with Z Index Phase. Specify this value in the units of the measurement."]
// #define DAQmx_CI_Encoder_ZIndexPhase  #[doc = "Specifies the states at which signal A and signal B must be while signal Z is high for NI-DAQmx to reset the measurement. If signal Z is never high while signal A and signal B are high, for example, you must choose a phase other than DAQmx_Val_AHighBHigh."]
// #define DAQmx_CI_PulseWidth_Units  #[doc = "Specifies the units to use to return pulse width measurements."]
// #define DAQmx_CI_PulseWidth_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_PulseWidth_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_PulseWidth_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_PulseWidth_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_PulseWidth_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_PulseWidth_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_PulseWidth_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_PulseWidth_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_PulseWidth_StartingEdge  #[doc = "Specifies on which edge of the input signal to begin each pulse width measurement."]
// #define DAQmx_CI_Timestamp_Units  #[doc = "Specifies the units to use to return timestamp measurements."]
// #define DAQmx_CI_Timestamp_InitialSeconds  #[doc = "Specifies the number of seconds that elapsed since the beginning of the current year. This value is ignored if  Synchronization Method is DAQmx_Val_IRIGB."]
// #define DAQmx_CI_GPS_SyncMethod  #[doc = "Specifies the method to use to synchronize the counter to a GPS receiver."]
// #define DAQmx_CI_GPS_SyncSrc  #[doc = "Specifies the terminal to which the GPS synchronization signal is connected."]
// #define DAQmx_CI_Velocity_AngEncoder_Units  #[doc = "Specifies the units to use to return angular velocity counter measurements."]
// #define DAQmx_CI_Velocity_AngEncoder_PulsesPerRev  #[doc = "Specifies the number of pulses the encoder generates per revolution. This value is the number of pulses on either signal A or signal B, not the total number of pulses on both signal A and signal B."]
// #define DAQmx_CI_Velocity_LinEncoder_Units  #[doc = "Specifies the units to use to return linear encoder velocity measurements from the channel."]
// #define DAQmx_CI_Velocity_LinEncoder_DistPerPulse  #[doc = "Specifies the distance to measure for each pulse the encoder generates on signal A or signal B. This value is in the units you specify in CI.Velocity.LinEncoder.DistUnits."]
// #define DAQmx_CI_Velocity_Encoder_DecodingType  #[doc = "Specifies how to count and interpret the pulses the encoder generates on signal A and signal B. X1, X2, and X4 are valid for quadrature encoders only. Two Pulse Counting is valid for two-pulse encoders only."]
// #define DAQmx_CI_Velocity_Encoder_AInputTerm  #[doc = "Specifies the terminal to which signal A is connected."]
// #define DAQmx_CI_Velocity_Encoder_AInputTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Velocity_Encoder_AInputLogicLvlBehavior  #[doc = "Specifies the logic level behavior of the input terminal."]
// #define DAQmx_CI_Velocity_Encoder_AInputDigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Velocity_Encoder_AInputDigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
// #define DAQmx_CI_Velocity_Encoder_AInputDigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Velocity_Encoder_AInputDigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Velocity_Encoder_BInputTerm  #[doc = "Specifies the terminal to which signal B is connected."]
// #define DAQmx_CI_Velocity_Encoder_BInputTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Velocity_Encoder_BInputLogicLvlBehavior  #[doc = "Specifies the logic level behavior of the input terminal."]
// #define DAQmx_CI_Velocity_Encoder_BInputDigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_Velocity_Encoder_BInputDigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the digital filter recognizes."]
// #define DAQmx_CI_Velocity_Encoder_BInputDigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_Velocity_Encoder_BInputDigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Velocity_MeasTime  #[doc = "Specifies in seconds the length of time to measure the velocity of the signal."]
// #define DAQmx_CI_Velocity_Div  #[doc = "Specifies the value by which to divide the input signal."]
// #define DAQmx_CI_TwoEdgeSep_Units  #[doc = "Specifies the units to use to return two-edge separation measurements from the channel."]
// #define DAQmx_CI_TwoEdgeSep_FirstTerm  #[doc = "Specifies the source terminal of the digital signal that starts each measurement."]
// #define DAQmx_CI_TwoEdgeSep_FirstTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_TwoEdgeSep_FirstLogicLvlBehavior  #[doc = "Specifies the logic level behavior on the input line."]
// #define DAQmx_CI_TwoEdgeSep_First_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_TwoEdgeSep_First_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_TwoEdgeSep_First_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_TwoEdgeSep_First_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_TwoEdgeSep_First_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_TwoEdgeSep_FirstEdge  #[doc = "Specifies on which edge of the first signal to start each measurement."]
// #define DAQmx_CI_TwoEdgeSep_SecondTerm  #[doc = "Specifies the source terminal of the digital signal that stops each measurement."]
// #define DAQmx_CI_TwoEdgeSep_SecondTermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_TwoEdgeSep_SecondLogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_TwoEdgeSep_Second_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_TwoEdgeSep_Second_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_TwoEdgeSep_Second_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_TwoEdgeSep_Second_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_TwoEdgeSep_Second_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_TwoEdgeSep_SecondEdge  #[doc = "Specifies on which edge of the second signal to stop each measurement."]
// #define DAQmx_CI_SemiPeriod_Units  #[doc = "Specifies the units to use to return semi-period measurements."]
// #define DAQmx_CI_SemiPeriod_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_SemiPeriod_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_SemiPeriod_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_SemiPeriod_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_SemiPeriod_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_SemiPeriod_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_SemiPeriod_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_SemiPeriod_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_SemiPeriod_StartingEdge  #[doc = "Specifies on which edge of the input signal to begin semi-period measurement. Semi-period measurements alternate between high time and low time, starting on this edge."]
// #define DAQmx_CI_Pulse_Freq_Units  #[doc = "Specifies the units to use to return pulse specifications in terms of frequency."]
// #define DAQmx_CI_Pulse_Freq_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_Pulse_Freq_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Pulse_Freq_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_Pulse_Freq_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the signal to measure."]
// #define DAQmx_CI_Pulse_Freq_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Pulse_Freq_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_CI_Pulse_Freq_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Pulse_Freq_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Pulse_Freq_Start_Edge  #[doc = "Specifies on which edge of the input signal to begin pulse measurement."]
// #define DAQmx_CI_Pulse_Time_Units  #[doc = "Specifies the units to use to return pulse specifications in terms of high time and low time."]
// #define DAQmx_CI_Pulse_Time_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_Pulse_Time_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Pulse_Time_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_Pulse_Time_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the signal to measure."]
// #define DAQmx_CI_Pulse_Time_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Pulse_Time_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_CI_Pulse_Time_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Pulse_Time_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Pulse_Time_StartEdge  #[doc = "Specifies on which edge of the input signal to begin pulse measurement."]
// #define DAQmx_CI_Pulse_Ticks_Term  #[doc = "Specifies the input terminal of the signal to measure."]
// #define DAQmx_CI_Pulse_Ticks_TermCfg  #[doc = "Specifies the input terminal configuration."]
// #define DAQmx_CI_Pulse_Ticks_LogicLvlBehavior  #[doc = "Specifies the logic level behavior on the count reset line."]
// #define DAQmx_CI_Pulse_Ticks_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the signal to measure."]
// #define DAQmx_CI_Pulse_Ticks_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_Pulse_Ticks_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_CI_Pulse_Ticks_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_Pulse_Ticks_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_Pulse_Ticks_StartEdge  #[doc = "Specifies on which edge of the input signal to begin pulse measurement."]
// #define DAQmx_CI_CtrTimebaseSrc  #[doc = "Specifies the terminal of the timebase to use for the counter."]
// #define DAQmx_CI_CtrTimebaseRate  #[doc = "Specifies in Hertz the frequency of the counter timebase. Specifying the rate of a counter timebase allows you to take measurements in terms of time or frequency rather than in ticks of the timebase. If you use an external timebase and do not specify the rate, you can take measurements only in terms of ticks of the timebase."]
// #define DAQmx_CI_CtrTimebaseActiveEdge  #[doc = "Specifies whether a timebase cycle is from rising edge to rising edge or from falling edge to falling edge."]
// #define DAQmx_CI_CtrTimebase_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CI_CtrTimebase_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CI_CtrTimebase_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CI_CtrTimebase_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CI_CtrTimebase_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CI_ThreshVoltage  #[doc = "Specifies the digital threshold value in Volts for high and low input transitions. Some devices do not support this for differential channels."]
// #define DAQmx_CI_Filter_Enable  #[doc = "Specifies the corresponding filter enable/disable state."]
// #define DAQmx_CI_Filter_Freq  #[doc = "Specifies the corresponding filter frequency (cutoff or center) of the filter response."]
// #define DAQmx_CI_Filter_Response  #[doc = "Specifies the corresponding filter response and defines the shape of the filter response."]
// #define DAQmx_CI_Filter_Order  #[doc = "Specifies the corresponding filter order and defines the slope of the filter response."]
// #define DAQmx_CI_FilterDelay  #[doc = "Indicates the amount of time between when the input signal transitions and when the filtered sample is read by the host device. This value is in the units specified with Filter Delay Units."]
// #define DAQmx_CI_FilterDelayUnits  #[doc = "Specifies the units of Filter Delay."]
// #define DAQmx_CI_Count  #[doc = "Indicates the current value of the count register."]
// #define DAQmx_CI_OutputState  #[doc = "Indicates the current state of the out terminal of the counter."]
// #define DAQmx_CI_TCReached  #[doc = "Indicates whether the counter rolled over. When you query this property, NI-DAQmx resets it to FALSE."]
// #define DAQmx_CI_CtrTimebaseMasterTimebaseDiv  #[doc = "Specifies the divisor for an external counter timebase. You can divide the counter timebase in order to measure slower signals without causing the count register to roll over."]
// #define DAQmx_CI_SampClkOverrunBehavior  #[doc = "Specifies the counter behavior when data is read but a new value was not detected during a sample clock."]
// #define DAQmx_CI_SampClkOverrunSentinelVal  #[doc = "Specifies the sentinel value returned when the No New Sample Behavior is set to Sentinel Value."]
// #define DAQmx_CI_DataXferMech  #[doc = "Specifies the data transfer mode for the channel."]
// #define DAQmx_CI_DataXferReqCond  #[doc = "Specifies under what condition to transfer data from the onboard memory of the device to the buffer."]
// #define DAQmx_CI_UsbXferReqSize  #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_CI_UsbXferReqCount  #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_CI_MemMapEnable  #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
// #define DAQmx_CI_NumPossiblyInvalidSamps  #[doc = "Indicates the number of samples that the device might have overwritten before it could transfer them to the buffer."]
// #define DAQmx_CI_DupCountPrevent  #[doc = "Specifies whether to enable duplicate count prevention for the channel. Duplicate count prevention is enabled by default. Setting  Prescaler disables duplicate count prevention unless you explicitly enable it."]
// #define DAQmx_CI_Prescaler  #[doc = "Specifies the divisor to apply to the signal you connect to the counter source terminal. Scaled data that you read takes this setting into account. You should use a prescaler only when you connect an external signal to the counter source terminal and when that signal has a higher frequency than the fastest onboard timebase. Setting this value disables duplicate count prevention unless you explicitly set Duplicate ..."]
// #define DAQmx_CI_MaxMeasPeriod  #[doc = "Specifies the maximum period (in seconds) in which the device will recognize signals. For frequency measurements, a signal with a higher period than the one set in this property will return 0 Hz. For duty cycle, the device will return 0 or 1 depending on the state of the line during the max defined period of time. Period measurements will return NaN. Pulse width measurement will return zero."]
// #define DAQmx_CO_OutputType  #[doc = "Indicates how to define pulses generated on the channel."]
// #define DAQmx_CO_Pulse_IdleState  #[doc = "Specifies the resting state of the output terminal."]
// #define DAQmx_CO_Pulse_Term  #[doc = "Specifies on which terminal to generate pulses."]
// #define DAQmx_CO_Pulse_Time_Units  #[doc = "Specifies the units in which to define high and low pulse time."]
// #define DAQmx_CO_Pulse_HighTime  #[doc = "Specifies the amount of time that the pulse is at a high voltage. This value is in the units you specify with Units or when you create the channel."]
// #define DAQmx_CO_Pulse_LowTime  #[doc = "Specifies the amount of time that the pulse is at a low voltage. This value is in the units you specify with Units or when you create the channel."]
// #define DAQmx_CO_Pulse_Time_InitialDelay  #[doc = "Specifies in seconds the amount of time to wait before generating the first pulse."]
// #define DAQmx_CO_Pulse_DutyCyc  #[doc = "Specifies the duty cycle of the pulses. The duty cycle of a signal is the width of the pulse divided by period. NI-DAQmx uses this ratio and the pulse frequency to determine the width of the pulses and the delay between pulses."]
// #define DAQmx_CO_Pulse_Freq_Units  #[doc = "Specifies the units in which to define pulse frequency."]
// #define DAQmx_CO_Pulse_Freq  #[doc = "Specifies the frequency of the pulses to generate. This value is in the units you specify with Units or when you create the channel."]
// #define DAQmx_CO_Pulse_Freq_InitialDelay  #[doc = "Specifies in seconds the amount of time to wait before generating the first pulse."]
// #define DAQmx_CO_Pulse_HighTicks  #[doc = "Specifies the number of ticks the pulse is high."]
// #define DAQmx_CO_Pulse_LowTicks  #[doc = "Specifies the number of ticks the pulse is low."]
// #define DAQmx_CO_Pulse_Ticks_InitialDelay  #[doc = "Specifies the number of ticks to wait before generating the first pulse."]
// #define DAQmx_CO_CtrTimebaseSrc  #[doc = "Specifies the terminal of the timebase to use for the counter. Typically, NI-DAQmx uses one of the internal counter timebases when generating pulses. Use this property to specify an external timebase and produce custom pulse widths that are not possible using the internal timebases."]
// #define DAQmx_CO_CtrTimebaseRate  #[doc = "Specifies in Hertz the frequency of the counter timebase. Specifying the rate of a counter timebase allows you to define output pulses in seconds rather than in ticks of the timebase. If you use an external timebase and do not specify the rate, you can define output pulses only in ticks of the timebase."]
// #define DAQmx_CO_CtrTimebaseActiveEdge  #[doc = "Specifies whether a timebase cycle is from rising edge to rising edge or from falling edge to falling edge."]
// #define DAQmx_CO_CtrTimebase_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_CO_CtrTimebase_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_CO_CtrTimebase_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_CO_CtrTimebase_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_CO_CtrTimebase_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_CO_Count  #[doc = "Indicates the current value of the count register."]
// #define DAQmx_CO_OutputState  #[doc = "Indicates the current state of the output terminal of the counter."]
// #define DAQmx_CO_AutoIncrCnt  #[doc = "Specifies a number of timebase ticks by which to increase the time spent in the idle state for each successive pulse."]
// #define DAQmx_CO_CtrTimebaseMasterTimebaseDiv  #[doc = "Specifies the divisor for an external counter timebase. You can divide the counter timebase in order to generate slower signals without causing the count register to roll over."]
// #define DAQmx_CO_PulseDone  #[doc = "Indicates if the task completed pulse generation. Use this value for retriggerable pulse generation when you need to determine if the device generated the current pulse. For retriggerable tasks, when you query this property, NI-DAQmx resets it to FALSE."]
// #define DAQmx_CO_EnableInitialDelayOnRetrigger  #[doc = "Specifies whether to apply the initial delay to retriggered pulse trains."]
// #define DAQmx_CO_ConstrainedGenMode  #[doc = "Specifies constraints to apply when the counter generates pulses. Constraining the counter reduces the device resources required for counter operation. Constraining the counter can also allow additional analog or counter tasks on the device to run concurrently. For continuous counter tasks, NI-DAQmx consumes no device resources when the counter is constrained. For finite counter tasks, resource use increases with ..."]
// #define DAQmx_CO_UseOnlyOnBrdMem  #[doc = "Specifies whether to write samples directly to the onboard memory of the device, bypassing the memory buffer. Generally, you cannot update onboard memory directly after you start the task. Onboard memory includes data FIFOs."]
// #define DAQmx_CO_DataXferMech  #[doc = "Specifies the data transfer mode for the device. For buffered operations, use DMA or USB Bulk. For non-buffered operations, use Polled."]
// #define DAQmx_CO_DataXferReqCond  #[doc = "Specifies under what condition to transfer data from the buffer to the onboard memory of the device."]
// #define DAQmx_CO_UsbXferReqSize  #[doc = "Specifies the maximum size of a USB transfer request in bytes. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_CO_UsbXferReqCount  #[doc = "Specifies the maximum number of simultaneous USB transfers used to stream data. Modify this value to affect performance under different combinations of operating system and device."]
// #define DAQmx_CO_MemMapEnable  #[doc = "Specifies for NI-DAQmx to map hardware registers to the memory space of the application, if possible. Normally, NI-DAQmx maps hardware registers to memory accessible only to the kernel. Mapping the registers to the memory space of the application increases performance. However, if the application accesses the memory space mapped to the registers, it can adversely affect the operation of the device and possibly res..."]
// #define DAQmx_CO_Prescaler  #[doc = "Specifies the divisor to apply to the signal you connect to the counter source terminal. Pulse generations defined by frequency or time take this setting into account, but pulse generations defined by ticks do not. You should use a prescaler only when you connect an external signal to the counter source terminal and when that signal has a higher frequency than the fastest onboard timebase."]
// #define DAQmx_CO_RdyForNewVal  #[doc = "Indicates whether the counter is ready for new continuous pulse train values."]
// #define DAQmx_Pwr_Voltage_Setpoint  #[doc = "Specifies the constant output voltage, in volts. Can be set while a task is running. Can be read at any time during a task."]
// #define DAQmx_Pwr_Voltage_DevScalingCoeff  #[doc = "Indicates the coefficients of the polynomial equation that NI-DAQmx uses to scale values from the native format of the device to volts. Can be read at any time during a task."]
// #define DAQmx_Pwr_Current_Setpoint  #[doc = "Specifies the output current, in amperes. If the load draws current greater than the specified value, the device will operate in Constant Current mode."]
// #define DAQmx_Pwr_Current_DevScalingCoeff  #[doc = "Indicates the coefficients of the polynomial equation that NI-DAQmx uses to scale values from the native format of the device to amperes. Can be read at any time during a task."]
// #define DAQmx_Pwr_OutputEnable  #[doc = "Specifies whether to enable or disable power module output. Can be set while a task is running. Can be read at any time during a task. When a task is running, the output is enabled immediately. Otherwise, the output is not enabled until the task enters the Committed state."]
// #define DAQmx_Pwr_OutputState  #[doc = "Indicates power channel operating state. Can be read at any time during a task."]
// #define DAQmx_Pwr_IdleOutputBehavior  #[doc = "Specifies whether to disable the output or maintain the existing value after the task is uncommitted."]
// #define DAQmx_Pwr_RemoteSense  #[doc = "Specifies whether to use local or remote sense to sense the output voltage. DAQmx Read (Power) will return remote or local voltage based on the Remote Sense attribute value. Reading this property will return the user-defined value."]
// #define DAQmx_ChanType  #[doc = "Indicates the type of the virtual channel."]
// #define DAQmx_PhysicalChanName  #[doc = "Specifies the name of the physical channel upon which this virtual channel is based."]
// #define DAQmx_ChanDescr  #[doc = "Specifies a user-defined description for the channel."]
// #define DAQmx_ChanIsGlobal  #[doc = "Indicates whether the channel is a global channel."]
// #define DAQmx_Chan_SyncUnlockBehavior  #[doc = "Specifies the action to take if the target loses its synchronization to the grand master."]
// #define DAQmx_Dev_IsSimulated  #[doc = "Indicates if the device is a simulated device."]
// #define DAQmx_Dev_ProductCategory  #[doc = "Indicates the product category of the device. This category corresponds to the category displayed in MAX when creating NI-DAQmx simulated devices."]
// #define DAQmx_Dev_ProductType  #[doc = "Indicates the product name of the device."]
// #define DAQmx_Dev_ProductNum  #[doc = "Indicates the unique hardware identification number for the device."]
// #define DAQmx_Dev_SerialNum  #[doc = "Indicates the serial number of the device. This value is zero if the device does not have a serial number."]
// #define DAQmx_Dev_Accessory_ProductTypes  #[doc = "Indicates the model names of accessories connected to the device. Each array element corresponds to a connector. For example, index 0 corresponds to connector 0. The array contains an empty string for each connector with no accessory connected."]
// #define DAQmx_Dev_Accessory_ProductNums  #[doc = "Indicates the unique hardware identification number for accessories connected to the device. Each array element corresponds to a connector. For example, index 0 corresponds to connector 0. The array contains 0 for each connector with no accessory connected."]
// #define DAQmx_Dev_Accessory_SerialNums  #[doc = "Indicates the serial number for accessories connected to the device. Each array element corresponds to a connector. For example, index 0 corresponds to connector 0. The array contains 0 for each connector with no accessory connected."]
// #define DAQmx_Carrier_SerialNum  #[doc = "Indicates the serial number of the device carrier. This value is zero if the carrier does not have a serial number."]
// #define DAQmx_FieldDAQ_DevName  #[doc = "Indicates the parent device which this bank is located in."]
// #define DAQmx_FieldDAQ_BankDevNames  #[doc = "Indicates an array containing the names of the banks in the FieldDAQ."]
// #define DAQmx_Dev_Chassis_ModuleDevNames  #[doc = "Indicates an array containing the names of the modules in the chassis."]
// #define DAQmx_Dev_AnlgTrigSupported  #[doc = "Indicates if the device supports analog triggering."]
// #define DAQmx_Dev_DigTrigSupported  #[doc = "Indicates if the device supports digital triggering."]
// #define DAQmx_Dev_TimeTrigSupported  #[doc = "Indicates whether the device supports time triggering."]
// #define DAQmx_Dev_AI_PhysicalChans  #[doc = "Indicates an array containing the names of the analog input physical channels available on the device."]
// #define DAQmx_Dev_AI_SupportedMeasTypes  #[doc = "Indicates the measurement types supported by the physical channels of the device. Refer to Measurement Types for information on specific channels."]
// #define DAQmx_Dev_AI_MaxSingleChanRate  #[doc = "Indicates the maximum rate for an analog input task if the task contains only a single channel from this device."]
// #define DAQmx_Dev_AI_MaxMultiChanRate  #[doc = "Indicates the maximum sampling rate for an analog input task from this device. To find the maximum rate for the task, take the minimum of Maximum Single Channel Rate or the indicated sampling rate of this device divided by the number of channels to acquire data from (including cold-junction compensation and autozero channels)."]
// #define DAQmx_Dev_AI_MinRate  #[doc = "Indicates the minimum rate for an analog input task on this device. NI-DAQmx returns a warning or error if you attempt to sample at a slower rate."]
// #define DAQmx_Dev_AI_SimultaneousSamplingSupported  #[doc = "Indicates if the device supports simultaneous sampling."]
// #define DAQmx_Dev_AI_NumSampTimingEngines  #[doc = "Indicates the number of Analog Input sample timing engines supported by the device."]
// #define DAQmx_Dev_AI_SampModes  #[doc = "Indicates sample modes supported by devices that support sample clocked analog input."]
// #define DAQmx_Dev_AI_NumSyncPulseSrcs  #[doc = "Indicates the number of Analog Input synchronization pulse sources supported by the device."]
// #define DAQmx_Dev_AI_TrigUsage  #[doc = "Indicates the triggers supported by this device for an analog input task."]
// #define DAQmx_Dev_AI_VoltageRngs  #[doc = "Indicates pairs of input voltage ranges supported by this device. Each pair consists of the low value, followed by the high value."]
// #define DAQmx_Dev_AI_VoltageIntExcitDiscreteVals  #[doc = "Indicates the set of discrete internal voltage excitation values supported by this device. If the device supports ranges of internal excitation values, use Range Values to determine supported excitation values."]
// #define DAQmx_Dev_AI_VoltageIntExcitRangeVals  #[doc = "Indicates pairs of internal voltage excitation ranges supported by this device. Each pair consists of the low value, followed by the high value. If the device supports a set of discrete internal excitation values, use Discrete Values to determine the supported excitation values."]
// #define DAQmx_Dev_AI_ChargeRngs  #[doc = "Indicates in coulombs pairs of input charge ranges for the device. Each pair consists of the low value followed by the high value."]
// #define DAQmx_Dev_AI_CurrentRngs  #[doc = "Indicates the pairs of current input ranges supported by this device. Each pair consists of the low value, followed by the high value."]
// #define DAQmx_Dev_AI_CurrentIntExcitDiscreteVals  #[doc = "Indicates the set of discrete internal current excitation values supported by this device."]
// #define DAQmx_Dev_AI_BridgeRngs  #[doc = "Indicates pairs of input voltage ratio ranges, in volts per volt, supported by devices that acquire using ratiometric measurements. Each pair consists of the low value followed by the high value."]
// #define DAQmx_Dev_AI_ResistanceRngs  #[doc = "Indicates pairs of input resistance ranges, in ohms, supported by devices that have the necessary signal conditioning to measure resistances. Each pair consists of the low value followed by the high value."]
// #define DAQmx_Dev_AI_FreqRngs  #[doc = "Indicates the pairs of frequency input ranges supported by this device. Each pair consists of the low value, followed by the high value."]
// #define DAQmx_Dev_AI_Gains  #[doc = "Indicates the input gain settings supported by this device."]
// #define DAQmx_Dev_AI_Couplings  #[doc = "Indicates the coupling types supported by this device."]
// #define DAQmx_Dev_AI_LowpassCutoffFreqDiscreteVals  #[doc = "Indicates the set of discrete lowpass cutoff frequencies supported by this device. If the device supports ranges of lowpass cutoff frequencies, use Range Values to determine supported frequencies."]
// #define DAQmx_Dev_AI_LowpassCutoffFreqRangeVals  #[doc = "Indicates pairs of lowpass cutoff frequency ranges supported by this device. Each pair consists of the low value, followed by the high value. If the device supports a set of discrete lowpass cutoff frequencies, use Discrete Values to determine the supported  frequencies."]
// #define DAQmx_AI_DigFltr_Types  #[doc = "Indicates the AI digital filter types supported by the device."]
// #define DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqDiscreteVals  #[doc = "Indicates the set of discrete lowpass cutoff frequencies supported by this device. If the device supports ranges of lowpass cutoff frequencies, use AI.DigFltr.Lowpass.CutoffFreq.RangeVals to determine supported frequencies."]
// #define DAQmx_Dev_AI_DigFltr_LowpassCutoffFreqRangeVals  #[doc = "Indicates pairs of lowpass cutoff frequency ranges supported by this device. Each pair consists of the low value, followed by the high value. If the device supports a set of discrete lowpass cutoff frequencies, use AI.DigFltr.Lowpass.CutoffFreq.DiscreteVals to determine the supported frequencies."]
// #define DAQmx_Dev_AO_PhysicalChans  #[doc = "Indicates an array containing the names of the analog output physical channels available on the device."]
// #define DAQmx_Dev_AO_SupportedOutputTypes  #[doc = "Indicates the generation types supported by the physical channels of the device. Refer to Output Types for information on specific channels."]
// #define DAQmx_Dev_AO_MaxRate  #[doc = "Indicates the maximum analog output rate of the device."]
// #define DAQmx_Dev_AO_MinRate  #[doc = "Indicates the minimum analog output rate of the device."]
// #define DAQmx_Dev_AO_SampClkSupported  #[doc = "Indicates if the device supports the sample clock timing  type for analog output tasks."]
// #define DAQmx_Dev_AO_NumSampTimingEngines  #[doc = "Indicates the number of Analog Output sample timing engines supported by the device."]
// #define DAQmx_Dev_AO_SampModes  #[doc = "Indicates sample modes supported by devices that support sample clocked analog output."]
// #define DAQmx_Dev_AO_NumSyncPulseSrcs  #[doc = "Indicates the number of Analog Output synchronization pulse sources supported by the device."]
// #define DAQmx_Dev_AO_TrigUsage  #[doc = "Indicates the triggers supported by this device for analog output tasks."]
// #define DAQmx_Dev_AO_VoltageRngs  #[doc = "Indicates pairs of output voltage ranges supported by this device. Each pair consists of the low value, followed by the high value."]
// #define DAQmx_Dev_AO_CurrentRngs  #[doc = "Indicates pairs of output current ranges supported by this device. Each pair consists of the low value, followed by the high value."]
// #define DAQmx_Dev_AO_Gains  #[doc = "Indicates the output gain settings supported by this device."]
// #define DAQmx_Dev_DI_Lines  #[doc = "Indicates an array containing the names of the digital input lines available on the device."]
// #define DAQmx_Dev_DI_Ports  #[doc = "Indicates an array containing the names of the digital input ports available on the device."]
// #define DAQmx_Dev_DI_MaxRate  #[doc = "Indicates the maximum digital input rate of the device."]
// #define DAQmx_Dev_DI_NumSampTimingEngines  #[doc = "Indicates the number of Digital Input sample timing engines supported by the device."]
// #define DAQmx_Dev_DI_TrigUsage  #[doc = "Indicates the triggers supported by this device for digital input tasks."]
// #define DAQmx_Dev_DO_Lines  #[doc = "Indicates an array containing the names of the digital output lines available on the device."]
// #define DAQmx_Dev_DO_Ports  #[doc = "Indicates an array containing the names of the digital output ports available on the device."]
// #define DAQmx_Dev_DO_MaxRate  #[doc = "Indicates the maximum digital output rate of the device."]
// #define DAQmx_Dev_DO_NumSampTimingEngines  #[doc = "Indicates the number of Digital Output synchronization pulse sources supported by the device."]
// #define DAQmx_Dev_DO_TrigUsage  #[doc = "Indicates the triggers supported by this device for digital output tasks."]
// #define DAQmx_Dev_CI_PhysicalChans  #[doc = "Indicates an array containing the names of the counter input physical channels available on the device."]
// #define DAQmx_Dev_CI_SupportedMeasTypes  #[doc = "Indicates the measurement types supported by the physical channels of the device. Refer to Measurement Types for information on specific channels."]
// #define DAQmx_Dev_CI_TrigUsage  #[doc = "Indicates the triggers supported by this device for counter input tasks."]
// #define DAQmx_Dev_CI_SampClkSupported  #[doc = "Indicates if the device supports the sample clock timing type for counter input tasks."]
// #define DAQmx_Dev_CI_SampModes  #[doc = "Indicates sample modes supported by devices that support sample clocked counter input."]
// #define DAQmx_Dev_CI_MaxSize  #[doc = "Indicates in bits the size of the counters on the device."]
// #define DAQmx_Dev_CI_MaxTimebase  #[doc = "Indicates in hertz the maximum counter timebase frequency."]
// #define DAQmx_Dev_CO_PhysicalChans  #[doc = "Indicates an array containing the names of the counter output physical channels available on the device."]
// #define DAQmx_Dev_CO_SupportedOutputTypes  #[doc = "Indicates the generation types supported by the physical channels of the device. Refer to Output Types for information on specific channels."]
// #define DAQmx_Dev_CO_SampClkSupported  #[doc = "Indicates if the device supports Sample Clock timing for counter output tasks."]
// #define DAQmx_Dev_CO_SampModes  #[doc = "Indicates sample modes supported by devices that support sample clocked counter output."]
// #define DAQmx_Dev_CO_TrigUsage  #[doc = "Indicates the triggers supported by this device for counter output tasks."]
// #define DAQmx_Dev_CO_MaxSize  #[doc = "Indicates in bits the size of the counters on the device."]
// #define DAQmx_Dev_CO_MaxTimebase  #[doc = "Indicates in hertz the maximum counter timebase frequency."]
// #define DAQmx_Dev_TEDS_HWTEDSSupported  #[doc = "Indicates whether the device supports hardware TEDS."]
// #define DAQmx_Dev_NumDMAChans  #[doc = "Indicates the number of DMA channels on the device."]
// #define DAQmx_Dev_BusType  #[doc = "Indicates the bus type of the device."]
// #define DAQmx_Dev_PCI_BusNum  #[doc = "Indicates the PCI bus number of the device."]
// #define DAQmx_Dev_PCI_DevNum  #[doc = "Indicates the PCI slot number of the device."]
// #define DAQmx_Dev_PXI_ChassisNum  #[doc = "Indicates the PXI chassis number of the device, as identified in MAX."]
// #define DAQmx_Dev_PXI_SlotNum  #[doc = "Indicates the PXI slot number of the device."]
// #define DAQmx_Dev_CompactDAQ_ChassisDevName  #[doc = "Indicates the name of the CompactDAQ chassis that contains this module."]
// #define DAQmx_Dev_CompactDAQ_SlotNum  #[doc = "Indicates the slot number in which this module is located in the CompactDAQ chassis."]
// #define DAQmx_Dev_CompactRIO_ChassisDevName  #[doc = "Indicates the name of the CompactRIO chassis that contains this module."]
// #define DAQmx_Dev_CompactRIO_SlotNum  #[doc = "Indicates the slot number of the CompactRIO chassis where this module is located."]
// #define DAQmx_Dev_TCPIP_Hostname  #[doc = "Indicates the IPv4 hostname of the device."]
// #define DAQmx_Dev_TCPIP_EthernetIP  #[doc = "Indicates the IPv4 address of the Ethernet interface in dotted decimal format. This property returns 0.0.0.0 if the Ethernet interface cannot acquire an address."]
// #define DAQmx_Dev_TCPIP_WirelessIP  #[doc = "Indicates the IPv4 address of the 802.11 wireless interface in dotted decimal format. This property returns 0.0.0.0 if the wireless interface cannot acquire an address."]
// #define DAQmx_Dev_Terminals  #[doc = "Indicates a list of all terminals on the device."]
// #define DAQmx_Dev_NumTimeTrigs  #[doc = "Indicates the number of time triggers available on the device."]
// #define DAQmx_Dev_NumTimestampEngines  #[doc = "Indicates the number of timestamp engines available on the device."]
// #define DAQmx_Exported_AIConvClk_OutputTerm  #[doc = "Specifies the terminal to which to route the AI Convert Clock."]
// #define DAQmx_Exported_AIConvClk_Pulse_Polarity  #[doc = "Indicates the polarity of the exported AI Convert Clock. The polarity is fixed and independent of the active edge of the source of the AI Convert Clock."]
// #define DAQmx_Exported_10MHzRefClk_OutputTerm  #[doc = "Specifies the terminal to which to route the 10MHz Clock."]
// #define DAQmx_Exported_20MHzTimebase_OutputTerm  #[doc = "Specifies the terminal to which to route the 20MHz Timebase."]
// #define DAQmx_Exported_SampClk_OutputBehavior  #[doc = "Specifies whether the exported Sample Clock issues a pulse at the beginning of a sample or changes to a high state for the duration of the sample."]
// #define DAQmx_Exported_SampClk_OutputTerm  #[doc = "Specifies the terminal to which to route the Sample Clock."]
// #define DAQmx_Exported_SampClk_DelayOffset  #[doc = "Specifies in seconds the amount of time to offset the exported Sample clock.  Refer to timing diagrams for generation applications in the device documentation for more information about this value."]
// #define DAQmx_Exported_SampClk_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Sample Clock if Output Behavior is DAQmx_Val_Pulse."]
// #define DAQmx_Exported_SampClkTimebase_OutputTerm  #[doc = "Specifies the terminal to which to route the Sample Clock Timebase."]
// #define DAQmx_Exported_DividedSampClkTimebase_OutputTerm  #[doc = "Specifies the terminal to which to route the Divided Sample Clock Timebase."]
// #define DAQmx_Exported_AdvTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Advance Trigger."]
// #define DAQmx_Exported_AdvTrig_Pulse_Polarity  #[doc = "Indicates the polarity of the exported Advance Trigger."]
// #define DAQmx_Exported_AdvTrig_Pulse_WidthUnits  #[doc = "Specifies the units of Width Value."]
// #define DAQmx_Exported_AdvTrig_Pulse_Width  #[doc = "Specifies the width of an exported Advance Trigger pulse. Specify this value in the units you specify with Width Units."]
// #define DAQmx_Exported_PauseTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Pause Trigger."]
// #define DAQmx_Exported_PauseTrig_Lvl_ActiveLvl  #[doc = "Specifies the active level of the exported Pause Trigger."]
// #define DAQmx_Exported_RefTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Reference Trigger."]
// #define DAQmx_Exported_RefTrig_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Reference Trigger."]
// #define DAQmx_Exported_StartTrig_OutputTerm  #[doc = "Specifies the terminal to which to route the Start Trigger."]
// #define DAQmx_Exported_StartTrig_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Start Trigger."]
// #define DAQmx_Exported_AdvCmpltEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Advance Complete Event."]
// #define DAQmx_Exported_AdvCmpltEvent_Delay  #[doc = "Specifies the output signal delay in periods of the sample clock."]
// #define DAQmx_Exported_AdvCmpltEvent_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Advance Complete Event."]
// #define DAQmx_Exported_AdvCmpltEvent_Pulse_Width  #[doc = "Specifies the width of the exported Advance Complete Event pulse."]
// #define DAQmx_Exported_AIHoldCmpltEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the AI Hold Complete Event."]
// #define DAQmx_Exported_AIHoldCmpltEvent_PulsePolarity  #[doc = "Specifies the polarity of an exported AI Hold Complete Event pulse."]
// #define DAQmx_Exported_ChangeDetectEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Change Detection Event."]
// #define DAQmx_Exported_ChangeDetectEvent_Pulse_Polarity  #[doc = "Specifies the polarity of an exported Change Detection Event pulse."]
// #define DAQmx_Exported_CtrOutEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Counter Output Event."]
// #define DAQmx_Exported_CtrOutEvent_OutputBehavior  #[doc = "Specifies whether the exported Counter Output Event pulses or changes from one state to the other when the counter reaches terminal count."]
// #define DAQmx_Exported_CtrOutEvent_Pulse_Polarity  #[doc = "Specifies the polarity of the pulses at the output terminal of the counter when Output Behavior is DAQmx_Val_Pulse. NI-DAQmx ignores this property if Output Behavior is DAQmx_Val_Toggle."]
// #define DAQmx_Exported_CtrOutEvent_Toggle_IdleState  #[doc = "Specifies the initial state of the output terminal of the counter when Output Behavior is DAQmx_Val_Toggle. The terminal enters this state when NI-DAQmx commits the task."]
// #define DAQmx_Exported_HshkEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Handshake Event."]
// #define DAQmx_Exported_HshkEvent_OutputBehavior  #[doc = "Specifies the output behavior of the Handshake Event."]
// #define DAQmx_Exported_HshkEvent_Delay  #[doc = "Specifies the number of seconds to delay after the Handshake Trigger deasserts before asserting the Handshake Event."]
// #define DAQmx_Exported_HshkEvent_Interlocked_AssertedLvl  #[doc = "Specifies the asserted level of the exported Handshake Event if Output Behavior is DAQmx_Val_Interlocked."]
// #define DAQmx_Exported_HshkEvent_Interlocked_AssertOnStart  #[doc = "Specifies to assert the Handshake Event when the task starts if Output Behavior is DAQmx_Val_Interlocked."]
// #define DAQmx_Exported_HshkEvent_Interlocked_DeassertDelay  #[doc = "Specifies in seconds the amount of time to wait after the Handshake Trigger asserts before deasserting the Handshake Event if Output Behavior is DAQmx_Val_Interlocked."]
// #define DAQmx_Exported_HshkEvent_Pulse_Polarity  #[doc = "Specifies the polarity of the exported Handshake Event if Output Behavior is DAQmx_Val_Pulse."]
// #define DAQmx_Exported_HshkEvent_Pulse_Width  #[doc = "Specifies in seconds the pulse width of the exported Handshake Event if Output Behavior is DAQmx_Val_Pulse."]
// #define DAQmx_Exported_RdyForXferEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Ready for Transfer Event."]
// #define DAQmx_Exported_RdyForXferEvent_Lvl_ActiveLvl  #[doc = "Specifies the active level of the exported Ready for Transfer Event."]
// #define DAQmx_Exported_RdyForXferEvent_DeassertCond  #[doc = "Specifies when the ready for transfer event deasserts."]
// #define DAQmx_Exported_RdyForXferEvent_DeassertCondCustomThreshold  #[doc = "Specifies in samples the threshold below which the Ready for Transfer Event deasserts. This threshold is an amount of space available in the onboard memory of the device. Deassert Condition must be DAQmx_Val_OnbrdMemCustomThreshold to use a custom threshold."]
// #define DAQmx_Exported_DataActiveEvent_OutputTerm  #[doc = "Specifies the terminal to which to export the Data Active Event."]
// #define DAQmx_Exported_DataActiveEvent_Lvl_ActiveLvl  #[doc = "Specifies the polarity of the exported Data Active Event."]
// #define DAQmx_Exported_RdyForStartEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Ready for Start Event."]
// #define DAQmx_Exported_RdyForStartEvent_Lvl_ActiveLvl  #[doc = "Specifies the polarity of the exported Ready for Start Event."]
// #define DAQmx_Exported_SyncPulseEvent_OutputTerm  #[doc = "Specifies the terminal to which to route the Synchronization Pulse Event."]
// #define DAQmx_Exported_WatchdogExpiredEvent_OutputTerm  #[doc = "Specifies the terminal  to which to route the Watchdog Timer Expired Event."]
// #define DAQmx_PersistedChan_Author  #[doc = "Indicates the author of the global channel."]
// #define DAQmx_PersistedChan_AllowInteractiveEditing  #[doc = "Indicates whether the global channel can be edited in the DAQ Assistant."]
// #define DAQmx_PersistedChan_AllowInteractiveDeletion  #[doc = "Indicates whether the global channel can be deleted through MAX."]
// #define DAQmx_PersistedScale_Author  #[doc = "Indicates the author of the custom scale."]
// #define DAQmx_PersistedScale_AllowInteractiveEditing  #[doc = "Indicates whether the custom scale can be edited in the DAQ Assistant."]
// #define DAQmx_PersistedScale_AllowInteractiveDeletion  #[doc = "Indicates whether the custom scale can be deleted through MAX."]
// #define DAQmx_PersistedTask_Author  #[doc = "Indicates the author of the task."]
// #define DAQmx_PersistedTask_AllowInteractiveEditing  #[doc = "Indicates whether the task can be edited in the DAQ Assistant."]
// #define DAQmx_PersistedTask_AllowInteractiveDeletion  #[doc = "Indicates whether the task can be deleted through MAX."]
// #define DAQmx_PhysicalChan_AI_SupportedMeasTypes  #[doc = "Indicates the measurement types supported by the channel."]
// #define DAQmx_PhysicalChan_AI_TermCfgs  #[doc = "Indicates the list of terminal configurations supported by the channel."]
// #define DAQmx_PhysicalChan_AI_InputSrcs  #[doc = "Indicates the list of input sources supported by the channel. Channels may support using the signal from the I/O connector or one of several calibration signals."]
// #define DAQmx_PhysicalChan_AI_SensorPower_Types  #[doc = "Indicates the types of power supplied to the sensor supported by this channel."]
// #define DAQmx_PhysicalChan_AI_SensorPower_VoltageRangeVals  #[doc = "Indicates pairs of sensor power voltage ranges supported by this channel. Each pair consists of the low value followed by the high value."]
// #define DAQmx_PhysicalChan_AI_PowerControl_Voltage  #[doc = "Specifies the voltage level for the sensor's power supply."]
// #define DAQmx_PhysicalChan_AI_PowerControl_Enable  #[doc = "Specifies whether to turn on the sensor's power supply."]
// #define DAQmx_PhysicalChan_AI_PowerControl_Type  #[doc = "Specifies the type of power supplied to the sensor."]
// #define DAQmx_PhysicalChan_AI_SensorPower_OpenChan  #[doc = "Indicates whether there is an open channel or undercurrent condition on the channel."]
// #define DAQmx_PhysicalChan_AI_SensorPower_Overcurrent  #[doc = "Indicates whether there is an overcurrent condition on the channel."]
// #define DAQmx_PhysicalChan_AO_SupportedOutputTypes  #[doc = "Indicates the output types supported by the channel."]
// #define DAQmx_PhysicalChan_AO_SupportedPowerUpOutputTypes  #[doc = "Indicates the power up output types supported by the channel."]
// #define DAQmx_PhysicalChan_AO_TermCfgs  #[doc = "Indicates the list of terminal configurations supported by the channel."]
// #define DAQmx_PhysicalChan_AO_ManualControlEnable  #[doc = "Specifies if you can control the physical channel externally via a manual control located on the device. You cannot simultaneously control a channel manually and with NI-DAQmx."]
// #define DAQmx_PhysicalChan_AO_ManualControl_ShortDetected  #[doc = "Indicates whether the physical channel is currently disabled due to a short detected on the channel."]
// #define DAQmx_PhysicalChan_AO_ManualControlAmplitude  #[doc = "Indicates the current value of the front panel amplitude control for the physical channel in volts."]
// #define DAQmx_PhysicalChan_AO_ManualControlFreq  #[doc = "Indicates the current value of the front panel frequency control for the physical channel in hertz."]
// #define DAQmx_AO_PowerAmp_ChannelEnable  #[doc = "Specifies whether to enable or disable a channel for amplification. This property can also be used to check if a channel is enabled."]
// #define DAQmx_AO_PowerAmp_ScalingCoeff  #[doc = "Indicates the coefficients of a polynomial equation used to scale from pre-amplified values."]
// #define DAQmx_AO_PowerAmp_Overcurrent  #[doc = "Indicates if the channel detected an overcurrent condition."]
// #define DAQmx_AO_PowerAmp_Gain  #[doc = "Indicates the calibrated gain of the channel."]
// #define DAQmx_AO_PowerAmp_Offset  #[doc = "Indicates the calibrated offset of the channel in volts."]
// #define DAQmx_PhysicalChan_DI_PortWidth  #[doc = "Indicates in bits the width of digital input port."]
// #define DAQmx_PhysicalChan_DI_SampClkSupported  #[doc = "Indicates if the sample clock timing type is supported for the digital input physical channel."]
// #define DAQmx_PhysicalChan_DI_SampModes  #[doc = "Indicates the sample modes supported by devices that support sample clocked digital input."]
// #define DAQmx_PhysicalChan_DI_ChangeDetectSupported  #[doc = "Indicates if the change detection timing type is supported for the digital input physical channel."]
// #define DAQmx_PhysicalChan_DO_PortWidth  #[doc = "Indicates in bits the width of digital output port."]
// #define DAQmx_PhysicalChan_DO_SampClkSupported  #[doc = "Indicates if the sample clock timing type is supported for the digital output physical channel."]
// #define DAQmx_PhysicalChan_DO_SampModes  #[doc = "Indicates the sample modes supported by devices that support sample clocked digital output."]
// #define DAQmx_PhysicalChan_CI_SupportedMeasTypes  #[doc = "Indicates the measurement types supported by the channel."]
// #define DAQmx_PhysicalChan_CO_SupportedOutputTypes  #[doc = "Indicates the output types supported by the channel."]
// #define DAQmx_PhysicalChan_TEDS_MfgID  #[doc = "Indicates the manufacturer ID of the sensor."]
// #define DAQmx_PhysicalChan_TEDS_ModelNum  #[doc = "Indicates the model number of the sensor."]
// #define DAQmx_PhysicalChan_TEDS_SerialNum  #[doc = "Indicates the serial number of the sensor."]
// #define DAQmx_PhysicalChan_TEDS_VersionNum  #[doc = "Indicates the version number of the sensor."]
// #define DAQmx_PhysicalChan_TEDS_VersionLetter  #[doc = "Indicates the version letter of the sensor."]
// #define DAQmx_PhysicalChan_TEDS_BitStream  #[doc = "Indicates the TEDS binary bitstream without checksums."]
// #define DAQmx_PhysicalChan_TEDS_TemplateIDs  #[doc = "Indicates the IDs of the templates in the bitstream in BitStream."]
// #define DAQmx_Read_RelativeTo  #[doc = "Specifies the point in the buffer at which to begin a read operation. If you also specify an offset with Offset, the read operation begins at that offset relative to the point you select with this property. The default value is DAQmx_Val_CurrReadPos unless you configure a Reference Trigger for the task. If you configure a Reference Trigger, the default value is DAQmx_Val_FirstPretrigSamp."]
// #define DAQmx_Read_Offset  #[doc = "Specifies an offset in samples per channel at which to begin a read operation. This offset is relative to the location you specify with RelativeTo."]
// #define DAQmx_Read_ChannelsToRead  #[doc = "Specifies a subset of channels in the task from which to read."]
// #define DAQmx_Read_ReadAllAvailSamp  #[doc = "Specifies whether subsequent read operations read all samples currently available in the buffer or wait for the buffer to become full before reading. NI-DAQmx uses this setting for finite acquisitions and only when the number of samples to read is -1. For continuous acquisitions when the number of samples to read is -1, a read operation always reads all samples currently available in the buffer."]
// #define DAQmx_Read_AutoStart  #[doc = "Specifies if an NI-DAQmx Read function automatically starts the task  if you did not start the task explicitly by using DAQmxStartTask(). The default value is TRUE. When  an NI-DAQmx Read function starts a finite acquisition task, it also stops the task after reading the last sample."]
// #define DAQmx_Read_OverWrite  #[doc = "Specifies whether to overwrite samples in the buffer that you have not yet read."]
// #define DAQmx_Logging_FilePath  #[doc = "Specifies the path to the TDMS file to which you want to log data.  If the file path is changed while the task is running, this takes effect on the next sample interval (if Logging.SampsPerFile has been set) or when DAQmx Start New File is called. New file paths can be specified by ending with '\' or '/'. Files created after specifying a new file path retain the same name and numbering sequence."]
// #define DAQmx_Logging_Mode  #[doc = "Specifies whether to enable logging and whether to allow reading data while logging. Log mode allows for the best performance. However, you cannot read data while logging if you specify this mode. If you want to read data while logging, specify Log and Read mode."]
// #define DAQmx_Logging_TDMS_GroupName  #[doc = "Specifies the name of the group to create within the TDMS file for data from this task. If you append data to an existing file and the specified group already exists, NI-DAQmx appends a number symbol and a number to the group name, incrementing that number until finding a group name that does not exist. For example, if you specify a group name of Voltage Task, and that group already exists, NI-DAQmx assigns the gr..."]
// #define DAQmx_Logging_TDMS_Operation  #[doc = "Specifies how to open the TDMS file."]
// #define DAQmx_Logging_Pause  #[doc = "Specifies whether logging is paused while a task is executing. If Mode is set to Log and Read mode, this value is taken into consideration on the next call to DAQmx Read, where data is written to disk. If Mode is set to Log Only mode, this value is taken into consideration the next time that data is written to disk. A new TDMS group is written when logging is resumed from a paused state."]
// #define DAQmx_Logging_SampsPerFile  #[doc = "Specifies how many samples to write to each file. When the file reaches the number of samples specified, a new file is created with the naming convention of filename_####.tdms, where #### starts at 0001 and increments automatically with each new file. For example, if the file specified is path/to/folder/data.tdms, the next file name used is path/to/folder/data_0001.tdms. To disable file spanning behavior, set this attribute to ..."]
// #define DAQmx_Logging_FileWriteSize  #[doc = "Specifies the size, in samples, in which data will be written to disk.  The size must be evenly divisible by the volume sector size, in bytes."]
// #define DAQmx_Logging_FilePreallocationSize  #[doc = "Specifies a size in samples to be used to pre-allocate space on disk.  Pre-allocation can improve file I/O performance, especially in situations where multiple files are being written to disk.  For finite tasks, the default behavior is to pre-allocate the file based on the number of samples you configure the task to acquire."]
// #define DAQmx_Read_CurrReadPos  #[doc = "Indicates in samples per channel the current position in the buffer."]
// #define DAQmx_Read_AvailSampPerChan  #[doc = "Indicates the number of samples available to read per channel. This value is the same for all channels in the task."]
// #define DAQmx_Read_TotalSampPerChanAcquired  #[doc = "Indicates the total number of samples acquired by each channel. NI-DAQmx returns a single value because this value is the same for all channels. For retriggered acquisitions, this value is the cumulative number of samples across all retriggered acquisitions."]
// #define DAQmx_Read_CommonModeRangeErrorChansExist  #[doc = "Indicates if the device(s) detected a common mode range violation for any virtual channel in the task. Common mode range violation occurs when the voltage of either the positive terminal or negative terminal to ground are out of range. Reading this property clears the common mode range violation status for all channels in the task. You must read this property before you read Common Mode Range Error Channels. Other..."]
// #define DAQmx_Read_CommonModeRangeErrorChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected a common mode range violation. You must read Common Mode Range Error Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_ExcitFaultChansExist  #[doc = "Indicates if the device(s) detected an excitation fault condition for any virtual channel in the task. Reading this property clears the excitation fault status for all channels in the task. You must read this property before you read Excitation Fault Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_ExcitFaultChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an excitation fault condition. You must read Excitation Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_OvercurrentChansExist  #[doc = "Indicates if the device(s) detected an overcurrent condition for any virtual channel in the task. Reading this property clears the overcurrent status for all channels in the task. You must read this property before you read Overcurrent Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_OvercurrentChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an overcurrent condition. You must read Overcurrent Channels Exist before you read this property. Otherwise, you will receive an error. On some devices, you must restart the task for all overcurrent channels to recover."]
// #define DAQmx_Read_OvertemperatureChansExist  #[doc = "Indicates if the device(s) detected an overtemperature condition in any virtual channel in the task. Reading this property clears the overtemperature status for all channels in the task. You must read this property before you read Overtemperature Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_OvertemperatureChans  #[doc = "Indicates a list of names of any overtemperature virtual channels. You must read Overtemperature Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_OpenChansExist  #[doc = "Indicates if the device or devices detected an open channel condition in any virtual channel in the task. Reading this property clears the open channel status for all channels in this task. You must read this property before you read Open Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_OpenChans  #[doc = "Indicates a list of names of any open virtual channels. You must read Open Channels Exist before you read this property. Otherwise you will receive an error."]
// #define DAQmx_Read_OpenChansDetails  #[doc = "Indicates a list of details of any open virtual channels. You must read Open Channels Exist before you read this property. Otherwise you will receive an error."]
// #define DAQmx_Read_OpenCurrentLoopChansExist  #[doc = "Indicates if the device(s) detected an open current loop for any virtual channel in the task. Reading this property clears the open current loop status for all channels in the task. You must read this property before you read Open Current Loop Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_OpenCurrentLoopChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an open current loop. You must read Open Current Loop Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_OpenThrmcplChansExist  #[doc = "Indicates if the device(s) detected an open thermocouple connected to any virtual channel in the task. Reading this property clears the open thermocouple status for all channels in the task. You must read this property before you read Open Thermocouple Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_OpenThrmcplChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an open thermcouple. You must read Open Thermocouple Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_OverloadedChansExist  #[doc = "Indicates if the device(s) detected an overload in any virtual channel in the task. Reading this property clears the overload status for all channels in the task. You must read this property before you read Overloaded Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_OverloadedChans  #[doc = "Indicates a list of names of any overloaded virtual channels in the task. You must read Overloaded Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_InputLimitsFaultChansExist  #[doc = "Indicates if the device or devices detected a sample that was outside the upper or lower limits configured for each channel in the task. Reading this property clears the input limits fault channel status for all channels in the task. You must read this property before you read Input Limits Fault Channels. Otherwise, you will receive an error. Note: Fault detection applies to both positive and negative inputs. For ..."]
// #define DAQmx_Read_InputLimitsFaultChans  #[doc = "Indicates the virtual channels that have detected samples outside the upper or lower limits configured for each channel in the task. You must read Input Limits Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_PLL_UnlockedChansExist  #[doc = "Indicates whether the PLL is currently locked, or whether it became unlocked during the previous acquisition. Devices may report PLL Unlock either during acquisition or after acquisition."]
// #define DAQmx_Read_PLL_UnlockedChans  #[doc = "Indicates the channels that had their PLLs unlock."]
// #define DAQmx_Read_PowerSupplyFaultChansExist  #[doc = "Indicates if the device or devices detected a power supply fault condition in any virtual channel in the task. Reading this property clears the power supply fault status for all channels in this task. You must read this property before you read Power Supply Fault Channels. Otherwise, you will receive an error."]
// #define DAQmx_Read_PowerSupplyFaultChans  #[doc = "Indicates the virtual channels that have detected a power supply fault. You must read Power Supply Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_Sync_UnlockedChansExist  #[doc = "Indicates whether the target is currently locked to the grand master. Devices may report PLL Unlock either during acquisition or after acquisition."]
// #define DAQmx_Read_Sync_UnlockedChans  #[doc = "Indicates the channels from devices in an unlocked target."]
// #define DAQmx_Read_AccessoryInsertionOrRemovalDetected  #[doc = "Indicates if any device(s) in the task detected the insertion or removal of an accessory since the task started. Reading this property clears the accessory change status for all channels in the task. You must read this property before you read Devices with Inserted or Removed Accessories. Otherwise, you will receive an error."]
// #define DAQmx_Read_DevsWithInsertedOrRemovedAccessories  #[doc = "Indicates the names of any devices that detected the insertion or removal of an accessory since the task started. You must read Accessory Insertion or Removal Detected before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_RemoteSenseErrorChansExist  #[doc = "Indicates if the device(s) detected an error condition of the remote sense connection for any channel in the task. You must disable the output and resolve the hardware connection issue to clear the error condition. You must read this property before you read the Remote Sense Error Channels property. Otherwise, you will receive an error."]
// #define DAQmx_RemoteSenseErrorChans  #[doc = "Indicates a list of names of any virtual channels in the task for which a remote sense connection error condition has been detected. You must read Remote Sense Error Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_AuxPowerErrorChansExist  #[doc = "Indicates if the device(s) detected an auxiliary power supply error condition for any channel in the task. Reading this property clears the error condition status for all channels in the task. You must read this property before you read the Aux Power Error Channels property. Otherwise, you will receive an error."]
// #define DAQmx_AuxPowerErrorChans  #[doc = "Indicates a list of names of any virtual channels in the task for which an auxiliary power supply error condition has been detected. You must read the Aux Power Error Channels Exist property before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Read_ChangeDetect_HasOverflowed  #[doc = "Indicates if samples were missed because change detection events occurred faster than the device could handle them. Some devices detect overflows differently than others."]
// #define DAQmx_Read_RawDataWidth  #[doc = "Indicates in bytes the size of a raw sample from the task."]
// #define DAQmx_Read_NumChans  #[doc = "Indicates the number of channels that an NI-DAQmx Read function reads from the task. This value is the number of channels in the task or the number of channels you specify with Channels to Read."]
// #define DAQmx_Read_DigitalLines_BytesPerChan  #[doc = "Indicates the number of bytes per channel that NI-DAQmx returns in a sample for line-based reads. If a channel has fewer lines than this number, the extra bytes are FALSE."]
// #define DAQmx_Read_WaitMode  #[doc = "Specifies how an NI-DAQmx Read function waits for samples to become available."]
// #define DAQmx_Read_SleepTime  #[doc = "Specifies in seconds the amount of time to sleep after checking for available samples if Wait Mode is DAQmx_Val_Sleep."]
// #define DAQmx_RealTime_ConvLateErrorsToWarnings  #[doc = "Specifies if DAQmxWaitForNextSampleClock(), an NI-DAQmx Read function, and an NI-DAQmx Write function convert late errors to warnings. NI-DAQmx returns no late warnings or errors until the number of warmup iterations you specify with Number Of Warmup Iterations execute."]
// #define DAQmx_RealTime_NumOfWarmupIters  #[doc = "Specifies the number of loop iterations that must occur before DAQmxWaitForNextSampleClock() and an NI-DAQmx Read function return any late warnings or errors. The system needs a number of iterations to stabilize. During this period, a large amount of jitter occurs, potentially causing reads and writes to be late. The default number of warmup iterations is 100. Specify a larger number if needed to stabilize the sys..."]
// #define DAQmx_RealTime_WaitForNextSampClkWaitMode  #[doc = "Specifies how DAQmxWaitForNextSampleClock() waits for the next Sample Clock pulse."]
// #define DAQmx_RealTime_ReportMissedSamp  #[doc = "Specifies whether an NI-DAQmx Read function returns lateness errors or warnings when it detects missed Sample Clock pulses. This setting does not affect DAQmxWaitForNextSampleClock(). Set this property to TRUE for applications that need to detect lateness without using DAQmxWaitForNextSampleClock()."]
// #define DAQmx_RealTime_WriteRecoveryMode  #[doc = "Specifies how NI-DAQmx attempts to recover after missing a Sample Clock pulse when performing counter writes."]
// #define DAQmx_Scale_Descr  #[doc = "Specifies a description for the scale."]
// #define DAQmx_Scale_ScaledUnits  #[doc = "Specifies the units to use for scaled values. You can use an arbitrary string."]
// #define DAQmx_Scale_PreScaledUnits  #[doc = "Specifies the units of the values that you want to scale."]
// #define DAQmx_Scale_Type  #[doc = "Indicates the method or equation form that the custom scale uses."]
// #define DAQmx_Scale_Lin_Slope  #[doc = "Specifies the slope, m, in the equation y=mx+b."]
// #define DAQmx_Scale_Lin_YIntercept  #[doc = "Specifies the y-intercept, b, in the equation y=mx+b."]
// #define DAQmx_Scale_Map_ScaledMax  #[doc = "Specifies the largest value in the range of scaled values. NI-DAQmx maps this value to Pre-Scaled Maximum Value. Reads coerce samples that are larger than this value to match this value. Writes generate errors for samples that are larger than this value."]
// #define DAQmx_Scale_Map_PreScaledMax  #[doc = "Specifies the largest value in the range of pre-scaled values. NI-DAQmx maps this value to Scaled Maximum Value."]
// #define DAQmx_Scale_Map_ScaledMin  #[doc = "Specifies the smallest value in the range of scaled values. NI-DAQmx maps this value to Pre-Scaled Minimum Value. Reads coerce samples that are smaller than this value to match this value. Writes generate errors for samples that are smaller than this value."]
// #define DAQmx_Scale_Map_PreScaledMin  #[doc = "Specifies the smallest value in the range of pre-scaled values. NI-DAQmx maps this value to Scaled Minimum Value."]
// #define DAQmx_Scale_Poly_ForwardCoeff  #[doc = "Specifies an array of coefficients for the polynomial that converts pre-scaled values to scaled values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9x^3."]
// #define DAQmx_Scale_Poly_ReverseCoeff  #[doc = "Specifies an array of coefficients for the polynomial that converts scaled values to pre-scaled values. Each element of the array corresponds to a term of the equation. For example, if index three of the array is 9, the fourth term of the equation is 9y^3."]
// #define DAQmx_Scale_Table_ScaledVals  #[doc = "Specifies an array of scaled values. These values map directly to the values in Pre-Scaled Values."]
// #define DAQmx_Scale_Table_PreScaledVals  #[doc = "Specifies an array of pre-scaled values. These values map directly to the values in Scaled Values."]
// #define DAQmx_SwitchChan_Usage  #[doc = "(Deprecated) Specifies how you can use the channel. Using this property acts as a safety mechanism to prevent you from connecting two source channels, for example."]
// #define DAQmx_SwitchChan_AnlgBusSharingEnable  #[doc = "(Deprecated) Specifies whether to enable sharing of an analog bus line so that multiple switch devices can connect to it simultaneously. For each device that will share the analog bus line, set this property to TRUE to enable sharing on the channel that connects to the analog bus line. Analog bus sharing is disabled by default."]
// #define DAQmx_SwitchChan_MaxACCarryCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum AC current that the device can carry."]
// #define DAQmx_SwitchChan_MaxACSwitchCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum AC current that the device can switch. This current is always against an RMS voltage level."]
// #define DAQmx_SwitchChan_MaxACCarryPwr  #[doc = "(Deprecated) Indicates in watts the maximum AC power that the device can carry."]
// #define DAQmx_SwitchChan_MaxACSwitchPwr  #[doc = "(Deprecated) Indicates in watts the maximum AC power that the device can switch."]
// #define DAQmx_SwitchChan_MaxDCCarryCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum DC current that the device can carry."]
// #define DAQmx_SwitchChan_MaxDCSwitchCurrent  #[doc = "(Deprecated) Indicates in amperes the maximum DC current that the device can switch. This current is always against a DC voltage level."]
// #define DAQmx_SwitchChan_MaxDCCarryPwr  #[doc = "(Deprecated) Indicates in watts the maximum DC power that the device can carry."]
// #define DAQmx_SwitchChan_MaxDCSwitchPwr  #[doc = "(Deprecated) Indicates in watts the maximum DC power that the device can switch."]
// #define DAQmx_SwitchChan_MaxACVoltage  #[doc = "(Deprecated) Indicates in volts the maximum AC RMS voltage that the device can switch."]
// #define DAQmx_SwitchChan_MaxDCVoltage  #[doc = "(Deprecated) Indicates in volts the maximum DC voltage that the device can switch."]
// #define DAQmx_SwitchChan_WireMode  #[doc = "(Deprecated) Indicates the number of wires that the channel switches."]
// #define DAQmx_SwitchChan_Bandwidth  #[doc = "(Deprecated) Indicates in Hertz the maximum frequency of a signal that can pass through the switch without significant deterioration."]
// #define DAQmx_SwitchChan_Impedance  #[doc = "(Deprecated) Indicates in ohms the switch impedance. This value is important in the RF domain and should match the impedance of the sources and loads."]
// #define DAQmx_SwitchDev_SettlingTime  #[doc = "(Deprecated) Specifies in seconds the amount of time to wait for the switch to settle (or debounce). NI-DAQmx adds this time to the settling time of the motherboard. Modify this property only if the switch does not settle within the settling time of the motherboard. Refer to device documentation for supported settling times."]
// #define DAQmx_SwitchDev_AutoConnAnlgBus  #[doc = "(Deprecated) Specifies if NI-DAQmx routes multiplexed channels to the analog bus backplane. Only the SCXI-1127 and SCXI-1128 support this property."]
// #define DAQmx_SwitchDev_PwrDownLatchRelaysAfterSettling  #[doc = "(Deprecated) Specifies if DAQmxSwitchWaitForSettling() powers down latching relays after waiting for the device to settle."]
// #define DAQmx_SwitchDev_Settled  #[doc = "(Deprecated) Indicates when Settling Time expires."]
// #define DAQmx_SwitchDev_RelayList  #[doc = "(Deprecated) Indicates a comma-delimited list of relay names."]
// #define DAQmx_SwitchDev_NumRelays  #[doc = "(Deprecated) Indicates the number of relays on the device. This value matches the number of relay names in Relay List."]
// #define DAQmx_SwitchDev_SwitchChanList  #[doc = "(Deprecated) Indicates a comma-delimited list of channel names for the current topology of the device."]
// #define DAQmx_SwitchDev_NumSwitchChans  #[doc = "(Deprecated) Indicates the number of switch channels for the current topology of the device. This value matches the number of channel names in Switch Channel List."]
// #define DAQmx_SwitchDev_NumRows  #[doc = "(Deprecated) Indicates the number of rows on a device in a matrix switch topology. Indicates the number of multiplexed channels on a device in a mux topology."]
// #define DAQmx_SwitchDev_NumColumns  #[doc = "(Deprecated) Indicates the number of columns on a device in a matrix switch topology. This value is always 1 if the device is in a mux topology."]
// #define DAQmx_SwitchDev_Topology  #[doc = "(Deprecated) Indicates the current topology of the device. This value is one of the topology options in DAQmxSwitchSetTopologyAndReset()."]
// #define DAQmx_SwitchDev_Temperature  #[doc = "(Deprecated) Indicates the current temperature as read by the Switch module in degrees Celsius. Refer to your device documentation for more information."]
// #define DAQmx_SwitchScan_BreakMode  #[doc = "(Deprecated) Specifies the action to take between each entry in a scan list."]
// #define DAQmx_SwitchScan_RepeatMode  #[doc = "(Deprecated) Specifies if the task advances through the scan list multiple times."]
// #define DAQmx_SwitchScan_WaitingForAdv  #[doc = "(Deprecated) Indicates if the switch hardware is waiting for an  Advance Trigger. If the hardware is waiting, it completed the previous entry in the scan list."]
// #define DAQmx_Sys_GlobalChans  #[doc = "Indicates an array that contains the names of all global channels saved on the system."]
// #define DAQmx_Sys_Scales  #[doc = "Indicates an array that contains the names of all custom scales saved on the system."]
// #define DAQmx_Sys_Tasks  #[doc = "Indicates an array that contains the names of all tasks saved on the system."]
// #define DAQmx_Sys_DevNames  #[doc = "Indicates the names of all devices installed in the system."]
// #define DAQmx_Sys_NIDAQMajorVersion  #[doc = "Indicates the major portion of the installed version of NI-DAQmx, such as 7 for version 7.0."]
// #define DAQmx_Sys_NIDAQMinorVersion  #[doc = "Indicates the minor portion of the installed version of NI-DAQmx, such as 0 for version 7.0."]
// #define DAQmx_Sys_NIDAQUpdateVersion  #[doc = "Indicates the update portion of the installed version of NI-DAQmx, such as 1 for version 9.0.1."]
// #define DAQmx_Task_Name  #[doc = "Indicates the name of the task."]
// #define DAQmx_Task_Channels  #[doc = "Indicates the names of all virtual channels in the task."]
// #define DAQmx_Task_NumChans  #[doc = "Indicates the number of virtual channels in the task."]
// #define DAQmx_Task_Devices  #[doc = "Indicates an array containing the names of all devices in the task."]
// #define DAQmx_Task_NumDevices  #[doc = "Indicates the number of devices in the task."]
// #define DAQmx_Task_Complete  #[doc = "Indicates whether the task completed execution."]
// #define DAQmx_SampQuant_SampMode  #[doc = "Specifies if a task acquires or generates a finite number of samples or if it continuously acquires or generates samples."]
// #define DAQmx_SampQuant_SampPerChan  #[doc = "Specifies the number of samples to acquire or generate for each channel if Sample Mode is DAQmx_Val_FiniteSamps. If Sample Mode is DAQmx_Val_ContSamps, NI-DAQmx uses this value to determine the buffer size."]
// #define DAQmx_SampTimingType  #[doc = "Specifies the type of sample timing to use for the task."]
// #define DAQmx_SampClk_Rate  #[doc = "Specifies the sampling rate in samples per channel per second. If you use an external source for the Sample Clock, set this input to the maximum expected rate of that clock."]
// #define DAQmx_SampClk_MaxRate  #[doc = "Indicates the maximum Sample Clock rate supported by the task, based on other timing settings. For output tasks, the maximum Sample Clock rate is the maximum rate of the DAC. For input tasks, NI-DAQmx calculates the maximum sampling rate differently for multiplexed devices than simultaneous sampling devices."]
// #define DAQmx_SampClk_Src  #[doc = "Specifies the terminal of the signal to use as the Sample Clock."]
// #define DAQmx_SampClk_ActiveEdge  #[doc = "Specifies on which edge of a clock pulse sampling takes place. This property is useful primarily when the signal you use as the Sample Clock is not a periodic clock."]
// #define DAQmx_SampClk_OverrunBehavior  #[doc = "Specifies the action to take if Sample Clock edges occur faster than the device can handle them."]
// #define DAQmx_SampClk_UnderflowBehavior  #[doc = "Specifies the action to take when the onboard memory of the device becomes empty. In either case, the sample clock does not stop."]
// #define DAQmx_SampClk_TimebaseDiv  #[doc = "Specifies the number of Sample Clock Timebase pulses needed to produce a single Sample Clock pulse."]
// #define DAQmx_SampClk_Term  #[doc = "Indicates the name of the internal Sample Clock terminal for the task. This property does not return the name of the Sample Clock source terminal specified with Source."]
// #define DAQmx_SampClk_Timebase_Rate  #[doc = "Specifies the rate of the Sample Clock Timebase. Some applications require that you specify a rate when you use any signal other than the onboard Sample Clock Timebase. NI-DAQmx requires this rate to calculate other timing parameters."]
// #define DAQmx_SampClk_Timebase_Src  #[doc = "Specifies the terminal of the signal to use as the Sample Clock Timebase."]
// #define DAQmx_SampClk_Timebase_ActiveEdge  #[doc = "Specifies on which edge to recognize a Sample Clock Timebase pulse. This property is useful primarily when the signal you use as the Sample Clock Timebase is not a periodic clock."]
// #define DAQmx_SampClk_Timebase_MasterTimebaseDiv  #[doc = "Specifies the number of pulses of the Master Timebase needed to produce a single pulse of the Sample Clock Timebase."]
// #define DAQmx_SampClkTimebase_Term  #[doc = "Indicates the name of the internal Sample Clock Timebase terminal for the task. This property does not return the name of the Sample Clock Timebase source terminal specified with Source."]
// #define DAQmx_SampClk_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_SampClk_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_SampClk_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_SampClk_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_SampClk_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_SampClk_WriteWfm_UseInitialWfmDT  #[doc = "Specifies that the value of Rate will be determined by the dt component of the initial DAQmx Write waveform input for Output tasks."]
// #define DAQmx_Hshk_DelayAfterXfer  #[doc = "Specifies the number of seconds to wait after a handshake cycle before starting a new handshake cycle."]
// #define DAQmx_Hshk_StartCond  #[doc = "Specifies the point in the handshake cycle that the device is in when the task starts."]
// #define DAQmx_Hshk_SampleInputDataWhen  #[doc = "Specifies on which edge of the Handshake Trigger an input task latches the data from the peripheral device."]
// #define DAQmx_ChangeDetect_DI_RisingEdgePhysicalChans  #[doc = "Specifies the names of the digital lines or ports on which to detect rising edges. The lines or ports must be used by virtual channels in the task. You also can specify a string that contains a list or range of digital lines or ports."]
// #define DAQmx_ChangeDetect_DI_FallingEdgePhysicalChans  #[doc = "Specifies the names of the digital lines or ports on which to detect falling edges. The lines or ports must be used by virtual channels in the task. You also can specify a string that contains a list or range of digital lines or ports."]
// #define DAQmx_ChangeDetect_DI_Tristate  #[doc = "Specifies whether to tristate lines specified with Rising Edge Physical Channels and Falling Edge Physical Channels that are not in a virtual channel in the task. If you set this property to TRUE, NI-DAQmx tristates rising/falling edge lines that are not in a virtual channel in the task. If you set this property to FALSE, NI-DAQmx does not modify the configuration of rising/falling edge lines that are not in a vir..."]
// #define DAQmx_OnDemand_SimultaneousAOEnable  #[doc = "Specifies whether to update all channels in the task simultaneously, rather than updating channels independently when you write a sample to that channel."]
// #define DAQmx_Implicit_UnderflowBehavior  #[doc = "Specifies the action to take when the onboard memory of the device becomes empty."]
// #define DAQmx_AIConv_Rate  #[doc = "Specifies in Hertz the rate at which to clock the analog-to-digital converter. This clock is specific to the analog input section of multiplexed devices."]
// #define DAQmx_AIConv_MaxRate  #[doc = "Indicates the maximum convert rate supported by the task, given the current devices and channel count."]
// #define DAQmx_AIConv_Src  #[doc = "Specifies the terminal of the signal to use as the AI Convert Clock."]
// #define DAQmx_AIConv_ActiveEdge  #[doc = "Specifies on which edge of the clock pulse an analog-to-digital conversion takes place."]
// #define DAQmx_AIConv_TimebaseDiv  #[doc = "Specifies the number of AI Convert Clock Timebase pulses needed to produce a single AI Convert Clock pulse."]
// #define DAQmx_AIConv_Timebase_Src  #[doc = "Specifies the terminal  of the signal to use as the AI Convert Clock Timebase."]
// #define DAQmx_DelayFromSampClk_DelayUnits  #[doc = "Specifies the units of Delay."]
// #define DAQmx_DelayFromSampClk_Delay  #[doc = "Specifies the amount of time to wait after receiving a Sample Clock edge before beginning to acquire the sample. This value is in the units you specify with Delay Units."]
// #define DAQmx_AIConv_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the AI Convert Clock."]
// #define DAQmx_AIConv_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_AIConv_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AIConv_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AIConv_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_MasterTimebase_Rate  #[doc = "Specifies the rate of the Master Timebase."]
// #define DAQmx_MasterTimebase_Src  #[doc = "Specifies the terminal of the signal to use as the Master Timebase. On an E Series device, you can choose only between the onboard 20MHz Timebase or the RTSI7 terminal."]
// #define DAQmx_RefClk_Rate  #[doc = "Specifies the frequency of the Reference Clock."]
// #define DAQmx_RefClk_Src  #[doc = "Specifies the terminal of the signal to use as the Reference Clock."]
// #define DAQmx_SyncPulse_Type  #[doc = "Specifies the type of sync pulse used in the task."]
// #define DAQmx_SyncPulse_Src  #[doc = "Specifies the terminal of the signal to use as the synchronization pulse. The synchronization pulse resets the clock dividers and the ADCs/DACs on the device."]
// #define DAQmx_SyncPulse_Time_When  #[doc = "Specifies the start time of the sync pulse."]
// #define DAQmx_SyncPulse_Time_Timescale  #[doc = "Specifies the timescale to be used for timestamps for a sync pulse."]
// #define DAQmx_SyncPulse_SyncTime  #[doc = "Indicates in seconds the delay required to reset the ADCs/DACs after the device receives the synchronization pulse."]
// #define DAQmx_SyncPulse_MinDelayToStart  #[doc = "Specifies in seconds the amount of time that elapses after the master device issues the synchronization pulse before the task starts."]
// #define DAQmx_SyncPulse_ResetTime  #[doc = "Indicates in seconds the amount of time required for the ADCs or DACs on the device to reset. When synchronizing devices, query this property on all devices and note the largest reset time. Then, for each device, subtract the value of this property from the largest reset time and set Reset Delay to the resulting value."]
// #define DAQmx_SyncPulse_ResetDelay  #[doc = "Specifies in seconds the amount of time to wait after the Synchronization Pulse before resetting the ADCs or DACs on the device. When synchronizing devices, query Reset Time on all devices and note the largest reset time. Then, for each device, subtract the reset time from the largest reset time and set this property to the resulting value."]
// #define DAQmx_SyncPulse_Term  #[doc = "Indicates the name of the internal Synchronization Pulse terminal for the task. This property does not return the name of the source terminal."]
// #define DAQmx_SyncClk_Interval  #[doc = "Specifies the interval, in Sample Clock periods, between each internal Synchronization Clock pulse. NI-DAQmx uses this pulse for synchronization of triggers between multiple devices at different rates. Refer to device documentation for information about how to calculate this value."]
// #define DAQmx_SampTimingEngine  #[doc = "Specifies which timing engine to use for the task."]
// #define DAQmx_FirstSampTimestamp_Enable  #[doc = "Specifies whether to enable the first sample timestamp."]
// #define DAQmx_FirstSampTimestamp_Timescale  #[doc = "Specifies the timescale to be used for the first sample timestamp."]
// #define DAQmx_FirstSampTimestamp_Val  #[doc = "Indicates the timestamp of the first sample."]
// #define DAQmx_FirstSampClk_When  #[doc = "Specifies the time of the first sample clock pulse."]
// #define DAQmx_FirstSampClk_Timescale  #[doc = "Specifies the timescale to be used for the value of When."]
// #define DAQmx_FirstSampClk_Offset  #[doc = "Specifies, in seconds, the offset to apply to the When value. This offset modifies when the first sample clock occurs and is used to account for known delays in the signal path."]
// #define DAQmx_StartTrig_Type  #[doc = "Specifies the type of trigger to use to start a task."]
// #define DAQmx_StartTrig_Term  #[doc = "Indicates the name of the internal Start Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// #define DAQmx_DigEdge_StartTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Start Trigger."]
// #define DAQmx_DigEdge_StartTrig_Edge  #[doc = "Specifies on which edge of a digital pulse to start acquiring or generating samples."]
// #define DAQmx_DigEdge_StartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the trigger signal."]
// #define DAQmx_DigEdge_StartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_DigEdge_StartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_DigEdge_StartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_DigEdge_StartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device. If you set this property to TRUE, the device does not recognize and act upon the trigger until the next pulse of the internal timebase."]
// #define DAQmx_DigPattern_StartTrig_Src  #[doc = "Specifies the physical channels to use for pattern matching. The order of the physical channels determines the order of the pattern. If a port is included, the order of the physical channels within the port is in ascending order."]
// #define DAQmx_DigPattern_StartTrig_Pattern  #[doc = "Specifies the digital pattern that must be met for the Start Trigger to occur."]
// #define DAQmx_DigPattern_StartTrig_When  #[doc = "Specifies whether the Start Trigger occurs when the physical channels specified with Source match or differ from the digital pattern specified with Pattern."]
// #define DAQmx_AnlgEdge_StartTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Start Trigger."]
// #define DAQmx_AnlgEdge_StartTrig_Slope  #[doc = "Specifies on which slope of the trigger signal to start acquiring or generating samples."]
// #define DAQmx_AnlgEdge_StartTrig_Lvl  #[doc = "Specifies at what threshold in the units of the measurement or generation to start acquiring or generating samples. Use Slope to specify on which slope to trigger on this threshold."]
// #define DAQmx_AnlgEdge_StartTrig_Hyst  #[doc = "Specifies a hysteresis level in the units of the measurement or generation. If Slope is DAQmx_Val_RisingSlope, the trigger does not deassert until the source signal passes below  Level minus the hysteresis. If Slope is DAQmx_Val_FallingSlope, the trigger does not deassert until the source signal passes above Level plus the hysteresis. Hysteresis is always enabled. Set this property to a non-zero value to use hyste..."]
// #define DAQmx_AnlgEdge_StartTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// #define DAQmx_AnlgEdge_StartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay above or below the trigger level for the minimum pulse width before being recognized. Use filtering  for noisy trigger signals that transition in and out of the hysteresis window rapidly."]
// #define DAQmx_AnlgEdge_StartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_AnlgEdge_StartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AnlgEdge_StartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AnlgEdge_StartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_AnlgMultiEdge_StartTrig_Srcs  #[doc = "Specifies a list and/or range of analog sources that are going to be used for Analog triggering. Each source corresponds to an element in each of the Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgMultiEdge_StartTrig_Slopes  #[doc = "Specifies an array of slopes on which to trigger task to start generating or acquiring samples. Each element of the array corresponds to a source in Start.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgMultiEdge_StartTrig_Lvls  #[doc = "Specifies an array of thresholds in the units of the measurement or generation to start acquiring or generating samples. Each element of the array corresponds to a source in Start.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgMultiEdge_StartTrig_Hysts  #[doc = "Specifies an array of hysteresis levels in the units of the measurement or generation. If the corresponding element of Start.AnlgMultiEdge.Slopes is Rising, the trigger does not deassert until the source signal passes below the corresponding element of Start.AnlgMultiEdge.Lvls minus the hysteresis. If Start.AnlgEdge.Slope is Falling, the trigger does not deassert until the source signal passes above Start.AnlgEdge..."]
// #define DAQmx_AnlgMultiEdge_StartTrig_Couplings  #[doc = "Specifies an array that describes the couplings for the corresponding source signal of the trigger if the source is a terminal rather than a virtual channel. Each element of the array corresponds to a source in Start.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgWin_StartTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Start Trigger."]
// #define DAQmx_AnlgWin_StartTrig_When  #[doc = "Specifies whether the task starts acquiring or generating samples when the signal enters or leaves the window you specify with Bottom and Top."]
// #define DAQmx_AnlgWin_StartTrig_Top  #[doc = "Specifies the upper limit of the window. Specify this value in the units of the measurement or generation."]
// #define DAQmx_AnlgWin_StartTrig_Btm  #[doc = "Specifies the lower limit of the window. Specify this value in the units of the measurement or generation."]
// #define DAQmx_AnlgWin_StartTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// #define DAQmx_AnlgWin_StartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay within the trigger window for the minimum pulse width before being recognized. Use filtering for noisy trigger signals that transition in and out of the window rapidly."]
// #define DAQmx_AnlgWin_StartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_AnlgWin_StartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AnlgWin_StartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AnlgWin_StartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_StartTrig_TrigWhen  #[doc = "Specifies when to trigger the start trigger."]
// #define DAQmx_StartTrig_Timescale  #[doc = "Specifies the timescale to be used for timestamps used in a time trigger."]
// #define DAQmx_StartTrig_TimestampEnable  #[doc = "Specifies whether the start trigger timestamp is enabled. If the timestamp is enabled but no resources are available, an error will be returned at run time."]
// #define DAQmx_StartTrig_TimestampTimescale  #[doc = "Specifies the start trigger timestamp timescale."]
// #define DAQmx_StartTrig_TimestampVal  #[doc = "Indicates the start trigger timestamp value."]
// #define DAQmx_StartTrig_Delay  #[doc = "Specifies an amount of time to wait after the Start Trigger is received before acquiring or generating the first sample. This value is in the units you specify with Delay Units."]
// #define DAQmx_StartTrig_DelayUnits  #[doc = "Specifies the units of Delay."]
// #define DAQmx_StartTrig_Retriggerable  #[doc = "Specifies whether a finite task resets and waits for another Start Trigger after the task completes. When you set this property to TRUE, the device performs a finite acquisition or generation each time the Start Trigger occurs until the task stops. The device ignores a trigger if it is in the process of acquiring or generating signals."]
// #define DAQmx_StartTrig_TrigWin  #[doc = "Specifies the period of time in seconds after the task starts during which the device may trigger. Once the window has expired, the device stops detecting triggers, and the task will finish after the device finishes acquiring post-trigger samples for any triggers detected. If no triggers are detected during the entire period, then no data will be returned. Ensure the period of time specified covers the entire time..."]
// #define DAQmx_StartTrig_RetriggerWin  #[doc = "Specifies the period of time in seconds after each trigger during which the device may trigger. Once the window has expired, the device stops detecting triggers, and the task will finish after the device finishes acquiring post-trigger samples that it already started. Ensure the period of time specified covers the entire time span desired for retrigger detection to avoid missed triggers. Specifying a Retrigger Win..."]
// #define DAQmx_StartTrig_MaxNumTrigsToDetect  #[doc = "Specifies the maximum number of times the task will detect a start trigger during the task. The number of times a trigger is detected and acted upon by the module may be less than the specified amount if the task stops early because of trigger/retrigger window expiration. Specifying the Maximum Number of Triggers to Detect to be 0 causes the driver to automatically set this value to the maximum possible number of ..."]
// #define DAQmx_RefTrig_Type  #[doc = "Specifies the type of trigger to use to mark a reference point for the measurement."]
// #define DAQmx_RefTrig_PretrigSamples  #[doc = "Specifies the minimum number of pretrigger samples to acquire from each channel before recognizing the reference trigger. Post-trigger samples per channel are equal to Samples Per Channel minus the number of pretrigger samples per channel."]
// #define DAQmx_RefTrig_Term  #[doc = "Indicates the name of the internal Reference Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// #define DAQmx_DigEdge_RefTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Reference Trigger."]
// #define DAQmx_DigEdge_RefTrig_Edge  #[doc = "Specifies on what edge of a digital pulse the Reference Trigger occurs."]
// #define DAQmx_DigEdge_RefTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the trigger signal."]
// #define DAQmx_DigEdge_RefTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_DigEdge_RefTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_DigEdge_RefTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_DigEdge_RefTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_DigPattern_RefTrig_Src  #[doc = "Specifies the physical channels to use for pattern matching. The order of the physical channels determines the order of the pattern. If a port is included, the order of the physical channels within the port is in ascending order."]
// #define DAQmx_DigPattern_RefTrig_Pattern  #[doc = "Specifies the digital pattern that must be met for the Reference Trigger to occur."]
// #define DAQmx_DigPattern_RefTrig_When  #[doc = "Specifies whether the Reference Trigger occurs when the physical channels specified with Source match or differ from the digital pattern specified with Pattern."]
// #define DAQmx_AnlgEdge_RefTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Reference Trigger."]
// #define DAQmx_AnlgEdge_RefTrig_Slope  #[doc = "Specifies on which slope of the source signal the Reference Trigger occurs."]
// #define DAQmx_AnlgEdge_RefTrig_Lvl  #[doc = "Specifies in the units of the measurement the threshold at which the Reference Trigger occurs.  Use Slope to specify on which slope to trigger at this threshold."]
// #define DAQmx_AnlgEdge_RefTrig_Hyst  #[doc = "Specifies a hysteresis level in the units of the measurement. If Slope is DAQmx_Val_RisingSlope, the trigger does not deassert until the source signal passes below Level minus the hysteresis. If Slope is DAQmx_Val_FallingSlope, the trigger does not deassert until the source signal passes above Level plus the hysteresis. Hysteresis is always enabled. Set this property to a non-zero value to use hysteresis."]
// #define DAQmx_AnlgEdge_RefTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// #define DAQmx_AnlgEdge_RefTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay above or below the trigger level for the minimum pulse width before being recognized. Use filtering  for noisy trigger signals that transition in and out of the hysteresis window rapidly."]
// #define DAQmx_AnlgEdge_RefTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width thefilter recognizes."]
// #define DAQmx_AnlgEdge_RefTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AnlgEdge_RefTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AnlgEdge_RefTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_AnlgMultiEdge_RefTrig_Srcs  #[doc = "Specifies a List and/or range of analog sources that are going to be used for Analog triggering. Each source corresponds to an element in each of the Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgMultiEdge_RefTrig_Slopes  #[doc = "Specifies an array of slopes on which to trigger task to start generating or acquiring samples. Each element of the array corresponds to a source in Ref.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgMultiEdge_RefTrig_Lvls  #[doc = "Specifies an array of thresholds in the units of the measurement or generation to start acquiring or generating samples. Each element of the array corresponds to a source in Ref.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgMultiEdge_RefTrig_Hysts  #[doc = "Specifies an array of hysteresis levels in the units of the measurement or generation. If the corresponding element of Ref.AnlgMultiEdge.Slopes is Rising, the trigger does not deassert until the source signal passes below the corresponding element of Ref.AnlgMultiEdge.Lvls minus the hysteresis. If Ref.AnlgEdge.Slope is Falling, the trigger does not deassert until the source signal passes above Ref.AnlgEdge.Lvl plu..."]
// #define DAQmx_AnlgMultiEdge_RefTrig_Couplings  #[doc = "Specifies an array that describes the couplings for the corresponding source signal of the trigger if the source is a terminal rather than a virtual channel. Each element of the array corresponds to a source in Ref.AnlgMultiEdge.Srcs and an element in each of the other Analog Multi Edge property arrays, if they are not empty."]
// #define DAQmx_AnlgWin_RefTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the Reference Trigger."]
// #define DAQmx_AnlgWin_RefTrig_When  #[doc = "Specifies whether the Reference Trigger occurs when the source signal enters the window or when it leaves the window. Use Bottom and Top to specify the window."]
// #define DAQmx_AnlgWin_RefTrig_Top  #[doc = "Specifies the upper limit of the window. Specify this value in the units of the measurement."]
// #define DAQmx_AnlgWin_RefTrig_Btm  #[doc = "Specifies the lower limit of the window. Specify this value in the units of the measurement."]
// #define DAQmx_AnlgWin_RefTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// #define DAQmx_AnlgWin_RefTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay within the trigger window for the minimum pulse width before being recognized. Use filtering for noisy trigger signals that transition in and out of the window rapidly."]
// #define DAQmx_AnlgWin_RefTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_AnlgWin_RefTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AnlgWin_RefTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AnlgWin_RefTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_RefTrig_AutoTrigEnable  #[doc = "Specifies whether to send a software trigger to the device when a hardware trigger is no longer active in order to prevent a timeout."]
// #define DAQmx_RefTrig_AutoTriggered  #[doc = "Indicates whether a completed acquisition was triggered by the auto trigger. If an acquisition has not completed after the task starts, this property returns FALSE. This property is only applicable when Enable  is TRUE."]
// #define DAQmx_RefTrig_TimestampEnable  #[doc = "Specifies whether the reference trigger timestamp is enabled. If the timestamp is enabled but no resources are available, an error will be returned at run time."]
// #define DAQmx_RefTrig_TimestampTimescale  #[doc = "Specifies the reference trigger timestamp timescale."]
// #define DAQmx_RefTrig_TimestampVal  #[doc = "Indicates the reference trigger timestamp value."]
// #define DAQmx_RefTrig_Delay  #[doc = "Specifies in seconds the time to wait after the device receives the Reference Trigger before switching from pretrigger to posttrigger samples."]
// #define DAQmx_RefTrig_Retriggerable  #[doc = "Specifies whether a finite task resets, acquires pretrigger samples, and waits for another Reference Trigger after the task completes. When you set this property to TRUE, the device will acquire post-trigger samples, reset, and acquire pretrigger samples each time the Reference Trigger occurs until the task stops. The device ignores a trigger if it is in the process of acquiring signals."]
// #define DAQmx_RefTrig_TrigWin  #[doc = "Specifies the duration in seconds after the task starts during which the device may trigger. Once the window has passed, the device stops detecting triggers, and the task will stop after the device finishes acquiring post-trigger samples that it already started. If no triggers are detected during the entire period, then no data will be returned. Specifying a Trigger Window of -1 causes the window to be infinite."]
// #define DAQmx_RefTrig_RetriggerWin  #[doc = "Specifies the duration in seconds after each trigger during which the device may trigger. Once the window has passed, the device stops detecting triggers, and the task will stop after the device finishes acquiring post-trigger samples that it already started. Specifying a Retrigger Window of -1 causes the window to be infinite."]
// #define DAQmx_RefTrig_MaxNumTrigsToDetect  #[doc = "Specifies the maximum number of times the task will detect a reference trigger during the task. The number of times a trigger is detected and acted upon by the module may be less than the specified amount if the task stops early because of trigger/retrigger window expiration. Specifying the Maximum Number of Triggers to Detect to be 0 causes the driver to automatically set this value to the maximum possible number..."]
// #define DAQmx_AdvTrig_Type  #[doc = "(Deprecated) Specifies the type of trigger to use to advance to the next entry in a switch scan list."]
// #define DAQmx_DigEdge_AdvTrig_Src  #[doc = "(Deprecated) Specifies the name of a terminal where there is a digital signal to use as the source of the Advance Trigger."]
// #define DAQmx_DigEdge_AdvTrig_Edge  #[doc = "(Deprecated) Specifies on which edge of a digital signal to advance to the next entry in a scan list."]
// #define DAQmx_DigEdge_AdvTrig_DigFltr_Enable  #[doc = "(Deprecated) Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_HshkTrig_Type  #[doc = "Specifies the type of Handshake Trigger to use."]
// #define DAQmx_Interlocked_HshkTrig_Src  #[doc = "Specifies the source terminal of the Handshake Trigger."]
// #define DAQmx_Interlocked_HshkTrig_AssertedLvl  #[doc = "Specifies the asserted level of the Handshake Trigger."]
// #define DAQmx_PauseTrig_Type  #[doc = "Specifies the type of trigger to use to pause a task."]
// #define DAQmx_PauseTrig_Term  #[doc = "Indicates the name of the internal Pause Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// #define DAQmx_AnlgLvl_PauseTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the trigger."]
// #define DAQmx_AnlgLvl_PauseTrig_When  #[doc = "Specifies whether the task pauses above or below the threshold you specify with Level."]
// #define DAQmx_AnlgLvl_PauseTrig_Lvl  #[doc = "Specifies the threshold at which to pause the task. Specify this value in the units of the measurement or generation. Use Pause When to specify whether the task pauses above or below this threshold."]
// #define DAQmx_AnlgLvl_PauseTrig_Hyst  #[doc = "Specifies a hysteresis level in the units of the measurement or generation. If Pause When is DAQmx_Val_AboveLvl, the trigger does not deassert until the source signal passes below Level minus the hysteresis. If Pause When is DAQmx_Val_BelowLvl, the trigger does not deassert until the source signal passes above Level plus the hysteresis. Hysteresis is always enabled. Set this property to a non-zero value to use hys..."]
// #define DAQmx_AnlgLvl_PauseTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the trigger if the source is a terminal rather than a virtual channel."]
// #define DAQmx_AnlgLvl_PauseTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay above or below the trigger level for the minimum pulse width before being recognized. Use filtering  for noisy trigger signals that transition in and out of the hysteresis window rapidly."]
// #define DAQmx_AnlgLvl_PauseTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_AnlgLvl_PauseTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AnlgLvl_PauseTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AnlgLvl_PauseTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_AnlgWin_PauseTrig_Src  #[doc = "Specifies the name of a virtual channel or terminal where there is an analog signal to use as the source of the trigger."]
// #define DAQmx_AnlgWin_PauseTrig_When  #[doc = "Specifies whether the task pauses while the trigger signal is inside or outside the window you specify with Bottom and Top."]
// #define DAQmx_AnlgWin_PauseTrig_Top  #[doc = "Specifies the upper limit of the window. Specify this value in the units of the measurement or generation."]
// #define DAQmx_AnlgWin_PauseTrig_Btm  #[doc = "Specifies the lower limit of the window. Specify this value in the units of the measurement or generation."]
// #define DAQmx_AnlgWin_PauseTrig_Coupling  #[doc = "Specifies the coupling for the source signal of the terminal if the source is a terminal rather than a virtual channel."]
// #define DAQmx_AnlgWin_PauseTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the digital output of the analog triggering circuitry (the Analog Comparison Event). When enabled, the analog signal must stay within the trigger window for the minimum pulse width before being recognized. Use filtering for noisy trigger signals that transition in and out of the window rapidly."]
// #define DAQmx_AnlgWin_PauseTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_AnlgWin_PauseTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the terminal of the signal to use as the timebase of the digital filter."]
// #define DAQmx_AnlgWin_PauseTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the digital filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_AnlgWin_PauseTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_DigLvl_PauseTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Pause Trigger."]
// #define DAQmx_DigLvl_PauseTrig_When  #[doc = "Specifies whether the task pauses while the signal is high or low."]
// #define DAQmx_DigLvl_PauseTrig_DigFltr_Enable  #[doc = "Specifies whether to apply a digital filter to the trigger signal."]
// #define DAQmx_DigLvl_PauseTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_DigLvl_PauseTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_DigLvl_PauseTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_DigLvl_PauseTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_DigPattern_PauseTrig_Src  #[doc = "Specifies the physical channels to use for pattern matching. The order of the physical channels determines the order of the pattern. If a port is included, the lines within the port are in ascending order."]
// #define DAQmx_DigPattern_PauseTrig_Pattern  #[doc = "Specifies the digital pattern that must be met for the Pause Trigger to occur."]
// #define DAQmx_DigPattern_PauseTrig_When  #[doc = "Specifies if the Pause Trigger occurs when the physical channels specified with Source match or differ from the digital pattern specified with Pattern."]
// #define DAQmx_ArmStartTrig_Type  #[doc = "Specifies the type of trigger to use to arm the task for a Start Trigger. If you configure an Arm Start Trigger, the task does not respond to a Start Trigger until the device receives the Arm Start Trigger."]
// #define DAQmx_ArmStart_Term  #[doc = "Indicates the name of the internal Arm Start Trigger terminal for the task. This property does not return the name of the trigger source terminal."]
// #define DAQmx_DigEdge_ArmStartTrig_Src  #[doc = "Specifies the name of a terminal where there is a digital signal to use as the source of the Arm Start Trigger."]
// #define DAQmx_DigEdge_ArmStartTrig_Edge  #[doc = "Specifies on which edge of a digital signal to arm the task for a Start Trigger."]
// #define DAQmx_DigEdge_ArmStartTrig_DigFltr_Enable  #[doc = "Specifies whether to apply the pulse width filter to the signal."]
// #define DAQmx_DigEdge_ArmStartTrig_DigFltr_MinPulseWidth  #[doc = "Specifies in seconds the minimum pulse width the filter recognizes."]
// #define DAQmx_DigEdge_ArmStartTrig_DigFltr_TimebaseSrc  #[doc = "Specifies the input terminal of the signal to use as the timebase of the pulse width filter."]
// #define DAQmx_DigEdge_ArmStartTrig_DigFltr_TimebaseRate  #[doc = "Specifies in hertz the rate of the pulse width filter timebase. NI-DAQmx uses this value to compute settings for the filter."]
// #define DAQmx_DigEdge_ArmStartTrig_DigSync_Enable  #[doc = "Specifies whether to synchronize recognition of transitions in the signal to the internal timebase of the device."]
// #define DAQmx_ArmStartTrig_TrigWhen  #[doc = "Specifies when to trigger the arm start trigger."]
// #define DAQmx_ArmStartTrig_Timescale  #[doc = "Specifies the timescale to be used for timestamps used in an arm start time trigger."]
// #define DAQmx_ArmStartTrig_TimestampEnable  #[doc = "Specifies whether the arm start trigger timestamp is enabled. If the timestamp is enabled but no resources are available, an error will be returned at run time."]
// #define DAQmx_ArmStartTrig_TimestampTimescale  #[doc = "Specifies the arm start trigger timestamp timescale."]
// #define DAQmx_ArmStartTrig_TimestampVal  #[doc = "Indicates the arm start trigger timestamp value."]
// #define DAQmx_Trigger_SyncType  #[doc = "Specifies the role of the device in a synchronized system. Setting this value to  DAQmx_Val_Master or  DAQmx_Val_Slave enables trigger skew correction. If you enable trigger skew correction, set this property to DAQmx_Val_Master on only one device, and set this property to DAQmx_Val_Slave on the other devices."]
// #define DAQmx_Watchdog_Timeout  #[doc = "Specifies in seconds the amount of time until the watchdog timer expires. A value of -1 means the internal timer never expires. Set this input to -1 if you use an Expiration Trigger to expire the watchdog task."]
// #define DAQmx_WatchdogExpirTrig_Type  #[doc = "Specifies the type of trigger to use to expire a watchdog task."]
// #define DAQmx_WatchdogExpirTrig_TrigOnNetworkConnLoss  #[doc = "Specifies the watchdog timer behavior when the network connection is lost between the host and the chassis. If set to true, the watchdog timer expires when the chassis detects the loss of network connection."]
// #define DAQmx_DigEdge_WatchdogExpirTrig_Src  #[doc = "Specifies the name of a terminal where a digital signal exists to use as the source of the Expiration Trigger."]
// #define DAQmx_DigEdge_WatchdogExpirTrig_Edge  #[doc = "Specifies on which edge of a digital signal to expire the watchdog task."]
// #define DAQmx_Watchdog_DO_ExpirState  #[doc = "Specifies the state to which to set the digital physical channels when the watchdog task expires.  You cannot modify the expiration state of dedicated digital input physical channels."]
// #define DAQmx_Watchdog_AO_OutputType  #[doc = "Specifies the output type of the analog output physical channels when the watchdog task expires."]
// #define DAQmx_Watchdog_AO_ExpirState  #[doc = "Specifies the state to set the analog output physical channels when the watchdog task expires."]
// #define DAQmx_Watchdog_CO_ExpirState  #[doc = "Specifies the state to set the counter output channel terminal when the watchdog task expires."]
// #define DAQmx_Watchdog_HasExpired  #[doc = "Indicates if the watchdog timer expired. You can read this property only while the task is running."]
// #define DAQmx_Write_RelativeTo  #[doc = "Specifies the point in the buffer at which to write data. If you also specify an offset with Offset, the write operation begins at that offset relative to this point you select with this property."]
// #define DAQmx_Write_Offset  #[doc = "Specifies in samples per channel an offset at which a write operation begins. This offset is relative to the location you specify with Relative To."]
// #define DAQmx_Write_RegenMode  #[doc = "Specifies whether to allow NI-DAQmx to generate the same data multiple times."]
// #define DAQmx_Write_CurrWritePos  #[doc = "Indicates the position in the buffer of the next sample to generate. This value is identical for all channels in the task."]
// #define DAQmx_Write_OvercurrentChansExist  #[doc = "Indicates if the device(s) detected an overcurrent condition for any channel in the task. Reading this property clears the overcurrent status for all channels in the task. You must read this property before you read Overcurrent Channels. Otherwise, you will receive an error."]
// #define DAQmx_Write_OvercurrentChans  #[doc = "Indicates a list of names of any virtual channels in the task for which an overcurrent condition has been detected. You must read Overcurrent Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Write_OvertemperatureChansExist  #[doc = "Indicates if the device(s) detected an overtemperature condition in any virtual channel in the task. Reading this property clears the overtemperature status for all channels in the task. You must read this property before you read Overtemperature Channels. Otherwise, you will receive an error."]
// #define DAQmx_Write_OvertemperatureChans  #[doc = "Indicates a list of names of any overtemperature virtual channels. You must read Overtemperature Channels Exist before you read this property. Otherwise, you will receive an error. The list of names may be empty if the device cannot determine the source of the overtemperature."]
// #define DAQmx_Write_ExternalOvervoltageChansExist  #[doc = "Indicates if the device(s) detected an External Overvoltage condition for any channel in the task. Reading this property clears the External Overvoltage status for all channels in the task. You must read this property before you read External OvervoltageChans. Otherwise, you will receive an error."]
// #define DAQmx_Write_ExternalOvervoltageChans  #[doc = "Indicates a list of names of any virtual channels in the task for which an External Overvoltage condition has been detected. You must read External OvervoltageChansExist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Write_OverloadedChansExist  #[doc = "Indicates if the device(s) detected an overload in any virtual channel in the task. Reading this property clears the overload status for all channels in the task. You must read this property before you read Overloaded Channels. Otherwise, you will receive an error."]
// #define DAQmx_Write_OverloadedChans  #[doc = "Indicates a list of names of any overloaded virtual channels in the task. You must read Overloaded Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Write_OpenCurrentLoopChansExist  #[doc = "Indicates if the device(s) detected an open current loop for any channel in the task. Reading this property clears the open current loop status for all channels in the task. You must read this property before you read Open Current Loop Channels. Otherwise, you will receive an error."]
// #define DAQmx_Write_OpenCurrentLoopChans  #[doc = "Indicates a list of names of any virtual channels in the task for which the device(s) detected an open current loop. You must read Open Current Loop Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Write_PowerSupplyFaultChansExist  #[doc = "Indicates if the device(s) detected a power supply fault for any channel in the task. Reading this property clears the power supply fault status for all channels in the task. You must read this property before you read Power Supply Fault Channels. Otherwise, you will receive an error."]
// #define DAQmx_Write_PowerSupplyFaultChans  #[doc = "Indicates a list of names of any virtual channels in the task that have a power supply fault. You must read Power Supply Fault Channels Exist before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Write_Sync_UnlockedChansExist  #[doc = "Indicates whether the target is currently locked to the grand master. Devices may report PLL Unlock either during acquisition or after acquisition."]
// #define DAQmx_Write_Sync_UnlockedChans  #[doc = "Indicates the channels from devices in an unlocked target."]
// #define DAQmx_Write_SpaceAvail  #[doc = "Indicates in samples per channel the amount of available space in the buffer."]
// #define DAQmx_Write_TotalSampPerChanGenerated  #[doc = "Indicates the total number of samples generated by each channel in the task. This value is identical for all channels in the task."]
// #define DAQmx_Write_AccessoryInsertionOrRemovalDetected  #[doc = "Indicates if any devices in the task detected the insertion or removal of an accessory since the task started. Reading this property clears the accessory change status for all channels in the task. You must read this property before you read Devices with Inserted or Removed Accessories. Otherwise, you will receive an error."]
// #define DAQmx_Write_DevsWithInsertedOrRemovedAccessories  #[doc = "Indicates the names of any devices that detected the insertion or removal of an accessory since the task started. You must read Accessory Insertion or Removal Detected before you read this property. Otherwise, you will receive an error."]
// #define DAQmx_Write_RawDataWidth  #[doc = "Indicates in bytes the required size of a raw sample to write to the task."]
// #define DAQmx_Write_NumChans  #[doc = "Indicates the number of channels that an NI-DAQmx Write function writes to the task. This value is the number of channels in the task."]
// #define DAQmx_Write_WaitMode  #[doc = "Specifies how an NI-DAQmx Write function waits for space to become available in the buffer."]
// #define DAQmx_Write_SleepTime  #[doc = "Specifies in seconds the amount of time to sleep after checking for available buffer space if Wait Mode is DAQmx_Val_Sleep."]
// #define DAQmx_Write_DigitalLines_BytesPerChan  #[doc = "Indicates the number of Boolean values expected per channel in a sample for line-based writes. This property is determined by the channel in the task with the most digital lines. If a channel has fewer lines than this number, NI-DAQmx ignores the extra Boolean values."]
