#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use anyhow::Result;
use nidaqmx_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};
use thiserror::Error;

pub fn daqmx_check(retcode: i32) -> Result<DaqmxStatus, DaqmxStatus> {
  match retcode {
    x if x == 0 => Ok(DaqmxStatus::Success),
    x if x > 0 => Ok(DaqmxStatus::from(x)),
    x if x < 0 => Err(DaqmxStatus::from(x)),
    _ => Err(DaqmxStatus::Invalid),
  }
}

#[derive(Error, Clone, Debug, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DaqmxStatus {
  #[error("Invalid Retcode!")]
  #[default]
  Invalid = 32324,
  #[error("Success")]
  Success = DAQmxSuccess,
  #[error("Absolute timestamp counter has rolled over.")]
  WarningTimestampCounterRolledOver = DAQmxWarningTimestampCounterRolledOver,
  #[error("Input termination resistor for one or more channels was overloaded. Your data might be invalid.")]
  WarningInputTerminationOverloaded = DAQmxWarningInputTerminationOverloaded,
  #[error(
    "ADC for one or more channels was overloaded. Your data might be invalid."
  )]
  WarningADCOverloaded = DAQmxWarningADCOverloaded,
  #[error("PLL was unlocked. Your data might be invalid.")]
  WarningPLLUnlocked = DAQmxWarningPLLUnlocked,
  #[error("Counter 0 DMA acquisition started while starting, committing, stopping, or uncommitting an analog input (AI) task. This could cause the counter acquisition to stop. If possible, use counter 1 instead of counter 0. Otherwise, start/commit the AI task before starting the counter 0 DMA acquisition, and stop/uncommit the AI task after stopping the counter 0 DMA acquisition.")]
  WarningCounter0DMADuringAIConflict = DAQmxWarningCounter0DMADuringAIConflict,
  #[error("Counter 1 DMA acquisition started while starting, committing, stopping, or uncommitting an analog output (AO) task. This could cause the counter acquisition to stop. If possible, use counter 0 instead of counter 1. Otherwise, start/commit the AO task before starting the counter 1 DMA acquisition, and stop/uncommit the AO task after stopping the counter 1 DMA acquisition.")]
  WarningCounter1DMADuringAOConflict = DAQmxWarningCounter1DMADuringAOConflict,
  #[error("Finite acquisition or generation has been stopped before the requested number of samples were acquired or generated.")]
  WarningStoppedBeforeDone = DAQmxWarningStoppedBeforeDone,
  #[error("Clock rate specified is so high that it violates the settling time requirements for the acquisition. Reduce the clock rate, or the accuracy of the measurement might be compromised.")]
  WarningRateViolatesSettlingTime = DAQmxWarningRateViolatesSettlingTime,
  #[error("Clock rate specified exceeds the maximum conversion rate of the ADC. ADC overrun errors are likely.")]
  WarningRateViolatesMaxADCRate = DAQmxWarningRateViolatesMaxADCRate,
  #[error("User-defined information string entered exceeds the maximum allowable string length. The string will be truncated to its maximum allowable length.")]
  WarningUserDefInfoStringTooLong = DAQmxWarningUserDefInfoStringTooLong,
  #[error("The combination of sample rate and buffer size settings could result in a large number of interrupts, causing the system to hang. Decrease your sample rate, or increase your buffer size. For acquisitions without a reference trigger, you can disallow buffer overwrites. For generations, you can disable the regeneration of old samples.")]
  WarningTooManyInterruptsPerSecond = DAQmxWarningTooManyInterruptsPerSecond,
  #[error("While writing to the buffer during a regeneration, the actual data generated might have alternated between old data and new data. That is, while the driver was replacing the old pattern in the buffer with the new pattern, the device might have generated a portion of new data, then a portion of old data, and then a portion of new data again. Reduce the sample rate, use a larger buffer, or refer to documentation about DAQmx Write for information about other ways to avoid this warning.")]
  WarningPotentialGlitchDuringWrite = DAQmxWarningPotentialGlitchDuringWrite,
  #[error("An attempt has been made to query the date/time of the last self calibration of a device that has never been self-calibrated using the NI-DAQmx API, so the date/time is invalid. Self-calibrate the board using the NI-DAQmx API.")]
  WarningDevNotSelfCalibratedWithDAQmx =
    DAQmxWarningDevNotSelfCalibratedWithDAQmx,
  #[error("Specified sample rate is lower than the lowest rate that can be generated using the onboard clock. The rate was coerced to the slowest possible sample rate. For slower rates, use an external sample clock or an external sample clock timebase.")]
  WarningAISampRateTooLow = DAQmxWarningAISampRateTooLow,
  #[error("Specified convert rate is too low to be generated using the onboard AI convert clock with the given timebase. The rate was coerced to the slowest possible convert rate. For slower rates, you must use an external convert clock or an external convert clock timebase.")]
  WarningAIConvRateTooLow = DAQmxWarningAIConvRateTooLow,
  #[error("Requested read offset could not be configured, so the offset was coerced to the minimum supported value.")]
  WarningReadOffsetCoercion = DAQmxWarningReadOffsetCoercion,
  #[error("Requested number of pretrigger samples per channel could not be configured, so it was coerced to the minimum supported value.")]
  WarningPretrigCoercion = DAQmxWarningPretrigCoercion,
  #[error("Attempted writing a sample value that was too large. The driver automatically coerced it to the maximum supported value.")]
  WarningSampValCoercedToMax = DAQmxWarningSampValCoercedToMax,
  #[error("Attempted writing a sample value that was too small.  The driver automatically coerced it to the minimum supported value.")]
  WarningSampValCoercedToMin = DAQmxWarningSampValCoercedToMin,
  #[error("One of more of the properties saved with a later version of NI-DAQ are not supported by the installed version of NI-DAQ and are ignored. Upgrade the installed version of NI-DAQ to a version compatible with the version used when saving the properties to take advantage of all the saved properties.")]
  WarningPropertyVersionNew = DAQmxWarningPropertyVersionNew,
  #[error("User-defined information to be stored in the EEPROM is too long. Only the leading portion was saved. Refer to documentation for information about the maximum length allowed for user-defined information.")]
  WarningUserDefinedInfoTooLong = DAQmxWarningUserDefinedInfoTooLong,
  #[error("Requested string could not fit into the given buffer. Only the first part of the string was copied into the buffer.  To allow for the terminating NULL character, the number of characters copied into the buffer is equal to the size of the buffer minus one. Call the function twice. Call the function initially to determine the string size. Use the second function call to get the full string value. In the first function call, pass NULL for the buffer and zero for the buffer size. The positive return value of the function is the string size (which includes the terminating NULL). Use this value to allocate a buffer of sufficient size, then use this buffer in the second function call.")]
  WarningCAPIStringTruncatedToFitBuffer =
    DAQmxWarningCAPIStringTruncatedToFitBuffer,
  #[error("Sample clock rate has been coerced to the minimum supported value because the specified value was too low. For lower sample clock rates, use an external sample clock or an external sample clock timebase.")]
  WarningSampClkRateTooLow = DAQmxWarningSampClkRateTooLow,
  #[error("Some of the last samples acquired during the finite DMA acquisition are possibly invalid due to counter limitations. Get the Number of Possibly Invalid Samples property to see how many samples might be invalid. Use continuous sample mode for DMA acquisitions with this type of counter.")]
  WarningPossiblyInvalidCTRSampsInFiniteDMAAcq =
    DAQmxWarningPossiblyInvalidCTRSampsInFiniteDMAAcq,
  #[error("RIS acquisition was completed, but some of the bins were not filled with a sufficient number of samples to perform the requested RIS averaging. Data for those bins was computed from the available samples. Consider increasing the timeout for the operation. Refer to documentation for details about RIS acquisitions.")]
  WarningRISAcqCompletedSomeBinsNotFilled =
    DAQmxWarningRISAcqCompletedSomeBinsNotFilled,
  #[error("A sensor on the device detected a temperature approaching the device's maximum recommended operating temperature. The device will shut down if its temperature exceeds the maximum recommended operating temperature. To avoid the shutdown, ensure the device temperature does not get above the maximum recommended operating temperature. You can often prevent the shutdown by periodically cleaning fan filters. Refer to user documentation for more information.")]
  WarningPXIDevTempExceedsMaxOpTemp = DAQmxWarningPXIDevTempExceedsMaxOpTemp,
  #[error("Output gain was coerced to the nearest acceptable value, because the original value was too low given the RF Frequency.")]
  WarningOutputGainTooLowForRFFreq = DAQmxWarningOutputGainTooLowForRFFreq,
  #[error("Output gain was coerced to the nearest acceptable value, because the original value was too high given the RF Frequency.")]
  WarningOutputGainTooHighForRFFreq = DAQmxWarningOutputGainTooHighForRFFreq,
  #[error("DAQmx Write was invoked more than once between two consecutive sample clocks. Only the last DAQmx Write took effect. To eliminate this warning, invoke DAQmx Write only once between two consecutive sample clocks.")]
  WarningMultipleWritesBetweenSampClks =
    DAQmxWarningMultipleWritesBetweenSampClks,
  #[error("A sensor on the device detected a temperature approaching the device's maximum recommended operating temperature. The device will shut down if its temperature exceeds the maximum recommended operating temperature. To avoid the shutdown, ensure the device temperature does not get above the maximum recommended operating temperature. You can often prevent the shutdown by periodically cleaning fan filters. Refer to user documentation for more information.")]
  WarningDeviceMayShutDownDueToHighTemp =
    DAQmxWarningDeviceMayShutDownDueToHighTemp,
  #[error("Clock rate specified is less than the minimum conversion rate of the ADC. Your data may be invalid.")]
  WarningRateViolatesMinADCRate = DAQmxWarningRateViolatesMinADCRate,
  #[error("Requested Sample Clock Rate is higher than the maximum supported per device specifications. Reduce the Sample Clock Rate or use a device which supports the requested Sample Clock Rate.")]
  WarningSampClkRateAboveDevSpecs = DAQmxWarningSampClkRateAboveDevSpecs,
  #[error("Settings requested through a previous DAQmx Write were overwritten before they could be applied. This occurred either because DAQmx Write was invoked more than once between two consecutive sample clocks or because the frequency of the generated pulse train is lower than the sample clock rate (it takes more than one sample clock period to generate one period of the pulse train). The settings requested by the most recent DAQmx Write will be applied on the next sample clock. To avoid this warning, make sure that DAQmx Write is invoked exactly once between two consecutive sample clocks and that the frequency of the generated pulse train is higher than the sample clock rate.")]
  WarningCOPrevDAQmxWriteSettingsOverwrittenForHWTimedSinglePoint =
    DAQmxWarningCOPrevDAQmxWriteSettingsOverwrittenForHWTimedSinglePoint,
  #[error("Data may be invalid because the settling time of the enabled filter exceeds the period between two conversions on the analog-to-digital converter (ADC) for a task with more than one channel. Disable the filter by setting AI Lowpass Enable to false, increase the time between two ADC conversions by reducing the AI Convert Rate, or acquire data from only one channel in the task.")]
  WarningLowpassFilterSettlingTimeExceedsUserTimeBetween2ADCConversions =
    DAQmxWarningLowpassFilterSettlingTimeExceedsUserTimeBetween2ADCConversions,
  #[error("Data may be invalid because the settling time of the enabled filter exceeds the period between two conversions on the analog-to-digital converter (ADC) for a task with more than one channel. Disable the filter by setting AI Lowpass Enable to false, acquire data from only one channel in the task, or increase the time between two ADC conversions by reducing the Sample Clock Rate, the Sample Clock Delay, and/or the number of channels in the task.")]
  WarningLowpassFilterSettlingTimeExceedsDriverTimeBetween2ADCConversions = DAQmxWarningLowpassFilterSettlingTimeExceedsDriverTimeBetween2ADCConversions,
  #[error("Sample clock rate specified is so high that it violates the settling time requirements for the generation. Reduce the sample clock rate, or the accuracy of the generated signal might be compromised.")]
  WarningSampClkRateViolatesSettlingTimeForGen =
    DAQmxWarningSampClkRateViolatesSettlingTimeForGen,
  #[error("Calibration constants stored in EEPROM produced an invalid value for one or more analog input channels. The device will continue to function, but the accuracy of the measurements may be compromised. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, perform a self-calibration, or contact National Instruments Technical Support.")]
  WarningInvalidCalConstValueForAI = DAQmxWarningInvalidCalConstValueForAI,
  #[error("Calibration constants stored in the EEPROM produced an invalid value for one or more analog output channels. The device will continue to function, but the accuracy of the generated signals may be compromised. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, perform a self-calibration, or contact National Instruments Technical Support.")]
  WarningInvalidCalConstValueForAO = DAQmxWarningInvalidCalConstValueForAO,
  #[error("Date specified by the Channel Calibration Expiration Date property has expired. The channel calibration is applied in spite of this because the Apply Calibration if Expired property was set to true. To eliminate this warning, update the channel calibration, including the Expiration Date.")]
  WarningChanCalExpired = DAQmxWarningChanCalExpired,
  #[error("Invalid enumeration value was encountered during export.  The exported file will require modification in order to successfully import.")]
  WarningUnrecognizedEnumValueEncounteredInStorage =
    DAQmxWarningUnrecognizedEnumValueEncounteredInStorage,
  #[error("EEPROM of the device appears to be corrupt. Contact National Instruments if the device appears to be functioning improperly.")]
  WarningTableCRCNotCorrect = DAQmxWarningTableCRCNotCorrect,
  #[error("External calibration section of the EEPROM on the device appears to be corrupt. Perform an external calibration on the device.")]
  WarningExternalCRCNotCorrect = DAQmxWarningExternalCRCNotCorrect,
  #[error("Self-calibration section of the EEPROM on the device appears to be corrupt. Perform a self-calibration on the device.")]
  WarningSelfCalCRCNotCorrect = DAQmxWarningSelfCalCRCNotCorrect,
  #[error("Requested property value exceeds device specification limits. Device performance is not guaranteed. Use values within device specifications, or set the Allow Out of Specification User Settings property to true.")]
  WarningDeviceSpecExceeded = DAQmxWarningDeviceSpecExceeded,
  #[error("Calibration changed the gain calibration constants only and not the offset calibration constants because the necessary offset calibration data was not available. This device needs a reference signal of 0.0 Volts at gains of 1, 15, 20, and 310 in order to perform an offset calibration.")]
  WarningOnlyGainCalibrated = DAQmxWarningOnlyGainCalibrated,
  #[error("Output generation aborted by the reverse power protection circuitry of the device. Either the output signal exceeded the output power limit, or power was driven back into the output of the device by an external source.    Error = DAQmxError state has been cleared.")]
  WarningReversePowerProtectionActivated =
    DAQmxWarningReversePowerProtectionActivated,
  #[error("Input voltage limits exceeded. Protection circuity disabled the inputs, however proper voltage levels are now present, and the error state has been cleared.")]
  WarningOverVoltageProtectionActivated =
    DAQmxWarningOverVoltageProtectionActivated,
  #[error("Buffer size specified is not evenly divisible by 8 times the sector size. For optimal performance, use a buffer size that is a multiple of 8 times the sector size. Refer to the NI-DAQmx Help for more information.")]
  WarningBufferSizeNotMultipleOfSectorSize =
    DAQmxWarningBufferSizeNotMultipleOfSectorSize,
  #[error("Sample Rate specified may exceed device capabilities for some devices in the task. Specify a slower sample rate, decrease the number of channels, or use a separate task for some of the devices in the task.")]
  WarningSampleRateMayCauseAcqToFail = DAQmxWarningSampleRateMayCauseAcqToFail,
  #[error("EEPROM of the device appears to be corrupt. Contact National Instruments if the device appears to be functioning improperly.")]
  WarningUserAreaCRCNotCorrect = DAQmxWarningUserAreaCRCNotCorrect,
  #[error("Power-up state section of the device EEPROM appears to be corrupt. Reconfigure the digital power-up states and perform a self-calibration.")]
  WarningPowerUpInfoCRCNotCorrect = DAQmxWarningPowerUpInfoCRCNotCorrect,
  #[error("The connection count of the attached accessory has exceeded the recommended limit. Contact National Instruments if the accessory appears to be functioning improperly.")]
  WarningConnectionCountHasExceededRecommendedLimit =
    DAQmxWarningConnectionCountHasExceededRecommendedLimit,
  #[error("The network device already exists in the system.")]
  WarningNetworkDeviceAlreadyAdded = DAQmxWarningNetworkDeviceAlreadyAdded,
  #[error("The connection count stored on the EEPROM is invalid. Write a value to Accessory Connection Count to fix the problem. Contact National Instruments if the problem persists.")]
  WarningAccessoryConnectionCountIsInvalid =
    DAQmxWarningAccessoryConnectionCountIsInvalid,
  #[error(
    "The selected ports are not connected, so there is nothing to disconnect."
  )]
  WarningUnableToDisconnectPorts = DAQmxWarningUnableToDisconnectPorts,
  #[error("The same data is being read repetitively.")]
  WarningReadRepeatedData = DAQmxWarningReadRepeatedData,
  #[error("The NI PXI-5600 is not associated with a digitizer. Right-click on the NI PXI-5600 and select 'Configure' to associate the digitizer.")]
  WarningPXI5600_NotConfigured = DAQmxWarningPXI5600_NotConfigured,
  #[error("The NI PXI-5661 is not configured properly. Right-click on the NI PXI-5600 and select 'Configure' to associate the digitizer.")]
  WarningPXI5661_IncorrectlyConfigured =
    DAQmxWarningPXI5661_IncorrectlyConfigured,
  #[error("The NI PXIe-5601 is not associated with a digitizer or an LO. Right-click on the NI PXIe-5601 and select 'Configure' to associate the digitizer and LO.")]
  WarningPXIe5601_NotConfigured = DAQmxWarningPXIe5601_NotConfigured,
  #[error("The NI PXIe-5663 is not configured properly. Right-click on the NI PXIe-5601 and select 'Configure' to associate the digitizer and LO.")]
  WarningPXIe5663_IncorrectlyConfigured =
    DAQmxWarningPXIe5663_IncorrectlyConfigured,
  #[error("The NI PXIe-5663E is not configured properly. Right-click on the NI PXIe-5601 and select 'Configure' to associate the digitizer and LO.")]
  WarningPXIe5663E_IncorrectlyConfigured =
    DAQmxWarningPXIe5663E_IncorrectlyConfigured,
  #[error("The NI PXIe-5603 is not associated with a digitizer or an LO. Right-click on the NI PXIe-5603 and select 'Configure' to associate the digitizer and LO. You can also associate the NI PXIe-5603 with additional IF conditioning and RF conditioning modules.")]
  WarningPXIe5603_NotConfigured = DAQmxWarningPXIe5603_NotConfigured,
  #[error("The NI PXIe-5665 (3.6 GHz) is not configured properly. Right-click on the NI PXIe-5603 and select 'Configure' to associate the digitizer and LO. You can also associate the NI PXIe-5603 with additional IF conditioning and RF conditioning modules.")]
  WarningPXIe5665_5603_IncorrectlyConfigured =
    DAQmxWarningPXIe5665_5603_IncorrectlyConfigured,
  #[error("The NI PXIe-5667 (3.6 GHz) is not configured properly. Right-click on the NI PXIe-5603 and select 'Configure' to associate the digitizer and LO. You can also associate the NI PXIe-5603 with additional IF conditioning and RF conditioning modules.")]
  WarningPXIe5667_5603_IncorrectlyConfigured =
    DAQmxWarningPXIe5667_5603_IncorrectlyConfigured,
  #[error("The NI PXIe-5605 is not associated with a digitizer or an LO. Right-click on the NI PXIe-5605 and select 'Configure' to associate the digitizer and LO. You can also associate the NI PXIe-5605 with additional IF conditioning and RF conditioning modules.")]
  WarningPXIe5605_NotConfigured = DAQmxWarningPXIe5605_NotConfigured,
  #[error("The NI PXI-5665 (14 GHz) is not configured properly. Right-click on the NI PXIe-5605 and select 'Configure' to associate the digitizer and LO. You can also associate the NI PXIe-5605 with additional IF conditioning and RF conditioning modules.")]
  WarningPXIe5665_5605_IncorrectlyConfigured =
    DAQmxWarningPXIe5665_5605_IncorrectlyConfigured,
  #[error("The NI PXIe-5667 (7 GHz) is not configured properly. Right-click on the NI PXIe-5605 and select 'Configure' to associate the digitizer and LO. You can also associate the NI PXIe-5605 with additional IF conditioning and RF conditioning modules.")]
  WarningPXIe5667_5605_IncorrectlyConfigured =
    DAQmxWarningPXIe5667_5605_IncorrectlyConfigured,
  #[error("The NI PXIe-5606 is not associated with a digitizer or an LO. Right-click on the NI PXIe-5606 and select 'Configure' to associate the digitizer and LO.")]
  WarningPXIe5606_NotConfigured = DAQmxWarningPXIe5606_NotConfigured,
  #[error("The NI PXI-5665 is not configured properly. Right-click on the NI PXIe-5606 and select 'Configure' to associate the digitizer and LO.")]
  WarningPXIe5665_5606_IncorrectlyConfigured =
    DAQmxWarningPXIe5665_5606_IncorrectlyConfigured,
  #[error("The NI PXI-5610 is not configured and needs to be associated with an AWG. Right-click on the NI PXI-5610 and select 'Configure' to associate the AWG. For more information, refer to NI-RFSG documentation, RF Signal Generators Getting Started Guide.")]
  WarningPXI5610_NotConfigured = DAQmxWarningPXI5610_NotConfigured,
  #[error("The NI PXI-5610 is not configured properly and needs to be associated with an AWG. Right-click on the NI PXI-5610 and select 'Configure' to associate the AWG. For more information, refer to NI-RFSG documentation, RF Signal Generators Getting Started Guide.")]
  WarningPXI5610_IncorrectlyConfigured =
    DAQmxWarningPXI5610_IncorrectlyConfigured,
  #[error("The NI PXIe-5611 is not configured and needs to be associated with an AWG and LO. Right-click on the NI PXIe-5611 and select 'Configure' to associate the AWG and LO. For more information, refer to NI-RFSG documentation, RF Signal Generators Getting Started Guide.")]
  WarningPXIe5611_NotConfigured = DAQmxWarningPXIe5611_NotConfigured,
  #[error("The NI PXIe-5611 is not configured properly and needs to be associated with an AWG and LO. Right-click on the NI PXIe-5611 and select 'Configure' to associate the AWG and LO. For more information, refer to NI-RFSG documentation, RF Signal Generators Getting Started Guide.")]
  WarningPXIe5611_IncorrectlyConfigured =
    DAQmxWarningPXIe5611_IncorrectlyConfigured,
  #[error("Using USB DAQ devices on Windows XP might result in corrupt data or other errors. Visit ni.com/info and enter info code \"WindowsXPUSBHotfix\" to obtain a patch.")]
  WarningUSBHotfixForDAQ = DAQmxWarningUSBHotfixForDAQ,
  #[error("When specifying \"No Change\" as a watchdog state on a DSA channel, the actual behavior is determined by the idle output behavior configured by the last task run on the module.")]
  WarningNoChangeSupersededByIdleBehavior =
    DAQmxWarningNoChangeSupersededByIdleBehavior,
  #[error("DAQmx Read did not complete before the arrival of  the next sample clock or change detection event  which indicates that your program is not keeping up with the hardware clock  or the external change event. For tasks using sample clock timing slow down the hardware clock or else change your application so that it can keep up with the hardware clock.  For tasks using change detection timing decrease the frequency of your event or else change your application so that it can keep up with the change event.")]
  WarningReadNotCompleteBeforeSampClk =
    DAQmxWarningReadNotCompleteBeforeSampClk,
  #[error("DAQmx Write did not complete before the arrival of the next sample clock which indicates that your program is not keeping up with the hardware clock. To remove this warning, slow down the hardware clock, or else change your application so that it can keep up with the hardware clock.")]
  WarningWriteNotCompleteBeforeSampClk =
    DAQmxWarningWriteNotCompleteBeforeSampClk,
  #[error("DAQmx Wait for Next Sample Clock detected one or more missed sample clocks since the last call to Wait for Next Sample Clock which indicates that your program is not keeping up with the sample clock. To remove this warning, slow down the sample clock, or else change your application so that it can keep up with the sample clock.")]
  WarningWaitForNextSampClkDetectedMissedSampClk =
    DAQmxWarningWaitForNextSampClkDetectedMissedSampClk,
  #[error("The Onboard Memory Empty value of the AO.DataXferReqCond attribute/property is not supported on this device. The value of this attribute/property is coerced to a different value after Output.BufSize is set. You can emulate the behavior of the Onboard Memory Empty value by setting Output.BufSize to a lower value. The minimum value for this attribute/property is 2.")]
  WarningOutputDataTransferConditionNotSupported =
    DAQmxWarningOutputDataTransferConditionNotSupported,
  #[error("Your device has lost synchronization lock to the grand master. Any timestamp values for the task may be invalid, but the task was not stopped because the SyncUnlockBehavior property is set to \"Ignore Lost Sync Lock.\"  To query the status of your device, read either the ReadSyncUnlockedChansExist or WriteSyncUnlockedChansExist property.")]
  WarningTimestampMayBeInvalid = DAQmxWarningTimestampMayBeInvalid,
  #[error("When using a First Sample Timestamp on a sample clocked task, the timestamp returned is the first sample clock detected after starting the task, not the first sample clock after arming the counter.")]
  WarningFirstSampleTimestampInaccurate =
    DAQmxWarningFirstSampleTimestampInaccurate,
  #[error("The value of one of the specified parameters conflicts with the state or the value of another parameter. The operation was completed by ignoring or overriding the conflicting parameters.")]
  WarningPALValueConflict = DAQmxWarningPALValueConflict,
  #[error("An attribute, whether explicit or implicit, is not relevant or is not relevant given the current system state. The operation was completed by ignoring or overriding the attribute in question.")]
  WarningPALIrrelevantAttribute = DAQmxWarningPALIrrelevantAttribute,
  #[error("The specified device is not a valid device. The operation was completed by ignoring or overriding the device parameter.")]
  WarningPALBadDevice = DAQmxWarningPALBadDevice,
  #[error("A selector, usually of an enumerated type, is inappropriate or out of range. The operation was completed by ignoring or overriding the selector.")]
  WarningPALBadSelector = DAQmxWarningPALBadSelector,
  #[error("A pointer-type parameter is invalid. This may mean that the pointer is NULL when it should not be, or not NULL when it should be, or is not NULL but does not appear to point to the appropriate type of object or data. The operation was completed by ignoring or overriding the pointer.")]
  WarningPALBadPointer = DAQmxWarningPALBadPointer,
  #[error("The specified data width is not supported or cannot be transferred on the bus. The operation was completed by ignoring or overriding the specified data width.")]
  WarningPALBadDataSize = DAQmxWarningPALBadDataSize,
  #[error("The specified mode is not supported. The operation was completed by ignoring or overriding the mode.")]
  WarningPALBadMode = DAQmxWarningPALBadMode,
  #[error("The specified offset is out of range. The operation was completed by ignoring or overriding the offset.")]
  WarningPALBadOffset = DAQmxWarningPALBadOffset,
  #[error("The specified count is too small, too large, or of the wrong granularity. In cases where an offset is also specified, the maximum count may be derived from the offset. The operation was completed by ignoring or overriding the count.")]
  WarningPALBadCount = DAQmxWarningPALBadCount,
  #[error("The specified mode is not supported for read operations. The operation was completed by ignoring or overriding the mode.")]
  WarningPALBadReadMode = DAQmxWarningPALBadReadMode,
  #[error("The specified offset is out of range for read operations. The operation was completed by ignoring or overriding the offset.")]
  WarningPALBadReadOffset = DAQmxWarningPALBadReadOffset,
  #[error("The specified count is too small, too large, or of the wrong granularity for read operations. In cases where an offset is also specified, the maximum count may be derived from the offset. The operation was completed by ignoring or overriding the count.")]
  WarningPALBadReadCount = DAQmxWarningPALBadReadCount,
  #[error("The specified mode is not supported for write operations. The operation was completed by ignoring or overriding the mode.,")]
  WarningPALBadWriteMode = DAQmxWarningPALBadWriteMode,
  #[error("The specified offset is out of range for write operations. The operation was completed by ignoring or overriding the offset.")]
  WarningPALBadWriteOffset = DAQmxWarningPALBadWriteOffset,
  #[error("The specified count is too small, too large, or of the wrong granularity for write operations. In cases where an offset is also specified, the maximum count may be derived from the offset. The operation was completed by ignoring or overriding the count.")]
  WarningPALBadWriteCount = DAQmxWarningPALBadWriteCount,
  #[error("The specified address class is not valid in the context of the current device configuration. The operation was completed by ignoring or overriding the address class.")]
  WarningPALBadAddressClass = DAQmxWarningPALBadAddressClass,
  #[error("The specified window type is not valid, or is inappropriate in the context of the current device configuration. The operation was completed by ignoring or overriding the window type.")]
  WarningPALBadWindowType = DAQmxWarningPALBadWindowType,
  #[error("The specified multitasking mode is invalid. The operation was completed by ignoring or overriding the multitasking mode.")]
  WarningPALBadThreadMultitask = DAQmxWarningPALBadThreadMultitask,
  #[error("The resource is controlled by the OS or a related system and should not be manipulated directly. The operation was completed but the state of the system may be unreliable.")]
  WarningPALResourceOwnedBySystem = DAQmxWarningPALResourceOwnedBySystem,
  #[error("The requested resource does not exist, or exists but is in a state which prevents its reservation. The operation was completed by reserving a functional subset of the requested resource.")]
  WarningPALResourceNotAvailable = DAQmxWarningPALResourceNotAvailable,
  #[error("The specified resource is not reserved. The operation was completed by reserving the resource or by ignoring or overriding the specified resource.")]
  WarningPALResourceNotReserved = DAQmxWarningPALResourceNotReserved,
  #[error("The specified resource is reserved. The operation was completed by ignoring or overriding the specified resource.")]
  WarningPALResourceReserved = DAQmxWarningPALResourceReserved,
  #[error("The specified resource has not been initialized. The operation was completed by initializing the resource on your behalf.")]
  WarningPALResourceNotInitialized = DAQmxWarningPALResourceNotInitialized,
  #[error("The resource was already initialized. The operation was completed by initializing the resource again which may have invalidated the state of your system as you understand it.")]
  WarningPALResourceInitialized = DAQmxWarningPALResourceInitialized,
  #[error("The resource is in use. The operation was partially completed which may have invalidated the state of your system as you understand it.")]
  WarningPALResourceBusy = DAQmxWarningPALResourceBusy,
  #[error("The specified resource is ambiguous or a unique resource could not be determined based on the qualifying attributes. The operation was completed but doing so may have invalidated the state of your system as you understand it.")]
  WarningPALResourceAmbiguous = DAQmxWarningPALResourceAmbiguous,
  #[error("The firmware, which is software resident on a device, has entered an unknown state, usually as a result of a cascade failure induced by an unexpected series of state inputs. The operation was completed by disabling the firmware thus restoring the intrinsic behavior of the device.")]
  WarningPALFirmwareFault = DAQmxWarningPALFirmwareFault,
  #[error("A hardware failure has occurred. The operation was completed by using alternative hardware.")]
  WarningPALHardwareFault = DAQmxWarningPALHardwareFault,
  #[error(
        "This function is not relevant for this operating system. No operation was performed."
    )]
  WarningPALOSUnsupported = DAQmxWarningPALOSUnsupported,
  #[error("A system call returned a warning. The operation was completed but the state of the system may be unreliable.")]
  WarningPALOSFault = DAQmxWarningPALOSFault,
  #[error("The specified function has been deprecated. The operation was completed but you should update your source code as soon as it is practical to do so.")]
  WarningPALFunctionObsolete = DAQmxWarningPALFunctionObsolete,
  #[error("A function imported from another library could not be found. The operation was completed through alternate means.")]
  WarningPALFunctionNotFound = DAQmxWarningPALFunctionNotFound,
  #[error("The specified feature may not operate as expected in this environment. The operation was completed but doing so may have invalidated the state of your system as you understand it.")]
  WarningPALFeatureNotSupported = DAQmxWarningPALFeatureNotSupported,
  #[error("The specified software component initialized but returned a warning. The operation was completed but the state of the system may be unreliable.")]
  WarningPALComponentInitializationFault =
    DAQmxWarningPALComponentInitializationFault,
  #[error("The specified software component has already been loaded.")]
  WarningPALComponentAlreadyLoaded = DAQmxWarningPALComponentAlreadyLoaded,
  #[error("The specified software component cannot be unloaded. The operation was completed through alternate means.")]
  WarningPALComponentNotUnloadable = DAQmxWarningPALComponentNotUnloadable,
  #[error("The specified buffer is aligned to an address boundary that prevents moving data to or from the specified device in an efficient manner. The operation was completed by using a smaller data width.")]
  WarningPALMemoryAlignmentFault = DAQmxWarningPALMemoryAlignmentFault,
  #[error("The Memory Manager detected that one or more allocated chunks were not freed. The operation was completed but the state of the system may be unreliable.")]
  WarningPALMemoryHeapNotEmpty = DAQmxWarningPALMemoryHeapNotEmpty,
  #[error("No transfer is in progress because no transfer was initiated. The operation was completed but doing so may have invalidated the state of your system as you understand it.")]
  WarningPALTransferNotInProgress = DAQmxWarningPALTransferNotInProgress,
  #[error("A transfer is already in progress. The operation was completed but doing so may have invalidated the state of your system as you understand it.")]
  WarningPALTransferInProgress = DAQmxWarningPALTransferInProgress,
  #[error("No transfer is in progress because the transfer was halted by the system. The operation was completed but doing so may have invalidated the state of your system as you understand it.")]
  WarningPALTransferStopped = DAQmxWarningPALTransferStopped,
  #[error("No transfer is in progress because the transfer was aborted by the client. The operation was completed but doing so may have invalidated the state of your system as you understand it.")]
  WarningPALTransferAborted = DAQmxWarningPALTransferAborted,
  #[error("There is no more data in the buffer.")]
  WarningPALLogicalBufferEmpty = DAQmxWarningPALLogicalBufferEmpty,
  #[error("There is no more space in the buffer.")]
  WarningPALLogicalBufferFull = DAQmxWarningPALLogicalBufferFull,
  #[error("A hardware buffer has underflowed.")]
  WarningPALPhysicalBufferEmpty = DAQmxWarningPALPhysicalBufferEmpty,
  #[error("A hardware buffer has overflowed.")]
  WarningPALPhysicalBufferFull = DAQmxWarningPALPhysicalBufferFull,
  #[error("There was no more space in the buffer when new data was written. The oldest unread data in the buffer was lost as a result.")]
  WarningPALTransferOverwritten = DAQmxWarningPALTransferOverwritten,
  #[error("There was no more data in the buffer when new data was being read. The data returned by the operation is stale as a result.")]
  WarningPALTransferOverread = DAQmxWarningPALTransferOverread,
  #[error("The dispatch function has already been exported.")]
  WarningPALDispatcherAlreadyExported =
    DAQmxWarningPALDispatcherAlreadyExported,
  #[error(
        "The thread that acquired the synchronization object has exited or has been terminated."
    )]
  WarningPALSyncAbandoned = DAQmxWarningPALSyncAbandoned,
  #[error("Setting the specified property does not allow multiple task configuration of the sample clock.")]
  ErrorMultiTaskCfgSampRateNotSupportedWithPropSet =
    DAQmxErrorMultiTaskCfgSampRateNotSupportedWithPropSet,
  #[error("A property is set to a value that is not supported by multiple task configuration of the sample clock.")]
  ErrorMultiTaskCfgSampRateConflictingProp =
    DAQmxErrorMultiTaskCfgSampRateConflictingProp,
  #[error("A common sample rate for all specified tasks could not be found. If your application contains both fast- and slow-sampled devices, consider enabling allow repeated samples.")]
  ErrorNoCommonSampRateFoundNoRepeatSamps =
    DAQmxErrorNoCommonSampRateFoundNoRepeatSamps,
  #[error("A common sample rate for all specified tasks could not be found.")]
  ErrorNoCommonSampRateFound = DAQmxErrorNoCommonSampRateFound,
  #[error("The task contains multiple devices or chassis.  Multiple task configuration only supports tasks with a single device or chassis.")]
  ErrorMultiTaskCfgDoesNotSupportMultiDevTask =
    DAQmxErrorMultiTaskCfgDoesNotSupportMultiDevTask,
  #[error("The task does not support configuring its sample clock rate via multiple task configuration.")]
  ErrorMultiTaskSampRateCfgNotSupported =
    DAQmxErrorMultiTaskSampRateCfgNotSupported,
  #[error("Specified operation cannot be performed because the debug session's target task may have a registered timing source. The debug session does not support debugging tasks with registered timing sources.")]
  ErrorDebugSessionNotAllowedTimingSourceRegistered =
    DAQmxErrorDebugSessionNotAllowedTimingSourceRegistered,
  #[error("Specified operation cannot be performed because the debug session's target task has logging enabled. The debug session does not support debugging tasks with logging enabled. Turn off logging for the target task and try again.")]
  ErrorDebugSessionNotAllowedWhenLogging =
    DAQmxErrorDebugSessionNotAllowedWhenLogging,
  #[error("Specified operation cannot be performed because the debug session's target task has registered events. The debug session does not support debugging tasks with registered events. Unregister the events for the target task and try again.")]
  ErrorDebugSessionNotAllowedEventRegistered =
    DAQmxErrorDebugSessionNotAllowedEventRegistered,
  #[error("Specified operation cannot be performed because the debug session's target task is invalid or does not exist.")]
  ErrorInvalidTargetTaskForDebugSession =
    DAQmxErrorInvalidTargetTaskForDebugSession,
  #[error("Requested function is not supported by the device.")]
  ErrorFunctionNotSupportedForDevice = DAQmxErrorFunctionNotSupportedForDevice,
  #[error("Multiple tasks were found that match the debug session target settings. The debug session does not support debugging more than one task at a time. Change the target settings to match a specific task and try again.")]
  ErrorMultipleTargetTasksFoundForDebugSession =
    DAQmxErrorMultipleTargetTasksFoundForDebugSession,
  #[error(
    "A task was not found that matches the debug session target settings."
  )]
  ErrorTargetTaskNotFoundForDebugSession =
    DAQmxErrorTargetTaskNotFoundForDebugSession,
  #[error(
        "Specified operation cannot be performed because it is not supported in a debug session."
    )]
  ErrorOperationNotSupportedInDebugSession =
    DAQmxErrorOperationNotSupportedInDebugSession,
  #[error("Specified operation cannot be performed because it is not permitted in monitor mode. Make sure the debug session is in control mode before requesting this operation.")]
  ErrorOperationNotPermittedInMonitorModeForDebugSession =
    DAQmxErrorOperationNotPermittedInMonitorModeForDebugSession,
  #[error("Attempt to get property failed because your device contains multiple banks, and the property has different values for different banks. Get this property one bank at a time by changing the active device name to specify each individual bank.")]
  ErrorGetActiveDevPrptyFailedDueToDifftVals =
    DAQmxErrorGetActiveDevPrptyFailedDueToDifftVals,
  #[error("Timing source creation has failed because another timing source has already been registered for this task.")]
  ErrorTaskAlreadyRegisteredATimingSource =
    DAQmxErrorTaskAlreadyRegisteredATimingSource,
  #[error("Requested value is not a supported value for this hardware revision. However, it may be supported on later revisions. Visit ni.com/info, and enter the info code \"fielddaqfilter\" for additional information.")]
  ErrorFilterNotSupportedOnHWRev = DAQmxErrorFilterNotSupportedOnHWRev,
  #[error("To set the sensor power supply voltage level, you must specify the sensor power configuration.")]
  ErrorSensorPowerSupplyVoltageLevel = DAQmxErrorSensorPowerSupplyVoltageLevel,
  #[error("To enable the sensor power supply, you must specify the sensor power voltage level.")]
  ErrorSensorPowerSupply = DAQmxErrorSensorPowerSupply,
  #[error("The hardware cannot acquire data from the configured scanlist. Please enter the info code that follows at ni.com/info for additional information.")]
  ErrorInvalidScanlist = DAQmxErrorInvalidScanlist,
  #[error("All internal routes have been reserved. If you are using time-based synchronization, please refer to info code \"TimeTriggerLimits\" at ni.com/info for additional information.")]
  ErrorTimeResourceCannotBeRouted = DAQmxErrorTimeResourceCannotBeRouted,
  #[error("The requested Reset Delay cannot be set based on your task configuration. If you are using C Series modules from different chassis in the same task, it is possible that the sample rate and module types cannot be synchronized effectively.")]
  ErrorInvalidResetDelayRequested = DAQmxErrorInvalidResetDelayRequested,
  #[error("Exceeded total number of time triggers available. Try disabling time triggers that are enabled on one or more DAQ tasks in your system. If no more time triggers can be disabled, try disabling other features that require internal routing resources. Please refer to info code \"TimeTriggerLimits\" at ni.com/info for more information.")]
  ErrorExceededTotalTimetriggersAvailable =
    DAQmxErrorExceededTotalTimetriggersAvailable,
  #[error("Exceeded total number of timestamps available. Try disabling timestamps that are user-defined or enabled by default on one or more DAQ tasks in your system. (For example, First Sample Timestamp is enabled by default on hardware timed input tasks like AI, DI and CI)")]
  ErrorExceededTotalTimestampsAvailable =
    DAQmxErrorExceededTotalTimestampsAvailable,
  #[error("One or more devices in your task are not running the IEEE 802.1AS or IEEE 1588 synchronization protocol. Use the NI-Sync LabVIEW API to enable 802.1AS or 1588. Refer to the NI-Sync help or LabVIEW examples for more information.")]
  ErrorNoSynchronizationProtocolRunning =
    DAQmxErrorNoSynchronizationProtocolRunning,
  #[error("One or more devices could not be added to your task because they have existing coherency requirements that conflict with the new requirements of your current task. Stop the task(s) that use devices in this task and try running it again.")]
  ErrorConflictingCoherencyRequirements =
    DAQmxErrorConflictingCoherencyRequirements,
  #[error("The devices in your task must be synchronized to one another using the IEEE 802.1AS or IEEE 1588 synchronization protocol. Connect the devices, either directly or via an 802.1AS or 1588-capable switch.")]
  ErrorNoSharedTimescale = DAQmxErrorNoSharedTimescale,
  #[error("FieldDAQ bank name specified is invalid. FieldDAQ bank names are of the form <FieldDAQ name>-Bank<Bank number>. For example, FieldDAQ1-Bank1.")]
  ErrorInvalidFieldDAQBankName = DAQmxErrorInvalidFieldDAQBankName,
  #[error("Task contains physical channels on one or more devices that do not support the DAQmx hardware-timed single-point sample mode.")]
  ErrorDeviceDoesNotSupportHWTSP = DAQmxErrorDeviceDoesNotSupportHWTSP,
  #[error("Bank type in the source storage does not match the bank type in the destination.")]
  ErrorBankTypeDoesNotMatchBankTypeInDestination =
    DAQmxErrorBankTypeDoesNotMatchBankTypeInDestination,
  #[error("FieldDAQ device does not support the specified number of banks. The bank number specified may be too large. Change the bank number to be a valid bank number.")]
  ErrorInvalidFieldDAQBankNumberSpecd =
    DAQmxErrorInvalidFieldDAQBankNumberSpecd,
  #[error(
    "The simulated FieldDAQ bank is not supported on this simulated FieldDAQ."
  )]
  ErrorUnsupportedSimulatedBankForSimulatedFieldDAQ =
    DAQmxErrorUnsupportedSimulatedBankForSimulatedFieldDAQ,
  #[error("The IsSimulated flags for FieldDAQ and banks must match. Change the IsSimulated flags in the import file so that they match.")]
  ErrorFieldDAQBankSimMustMatchFieldDAQSim =
    DAQmxErrorFieldDAQBankSimMustMatchFieldDAQSim,
  #[error(
    "The specified device is no longer supported within the NI-DAQmx API."
  )]
  ErrorDevNoLongerSupportedWithinDAQmxAPI =
    DAQmxErrorDevNoLongerSupportedWithinDAQmxAPI,
  #[error("The requested Sample Timing Engine does not support Use Only Onboard Memory.")]
  ErrorTimingEngineDoesNotSupportOnBoardMemory =
    DAQmxErrorTimingEngineDoesNotSupportOnBoardMemory,
  #[error("Task name specified conflicts with an existing task name in another project.")]
  ErrorDuplicateTaskCrossProject = DAQmxErrorDuplicateTaskCrossProject,
  #[error("A time Arm Start Trigger and time Start Trigger are configured. Ensure the Arm Start Trigger comes first.")]
  ErrorTimeStartTriggerBeforeArmStartTrigger =
    DAQmxErrorTimeStartTriggerBeforeArmStartTrigger,
  #[error("The time trigger cannot be configured because your device has lost synchronization lock to the grand master.")]
  ErrorTimeTriggerCannotBeSet = DAQmxErrorTimeTriggerCannotBeSet,
  #[error("Specified value is not supported for this property. Supported values are -1 (infinite window) or non-negative numbers up to 15.768000e9.")]
  ErrorInvalidTriggerWindowValue = DAQmxErrorInvalidTriggerWindowValue,
  #[error("The property cannot be queried before or during a data acquisition. You must explicitly commit, run, and stop your task before attempting to read this property.")]
  ErrorCannotQueryPropertyBeforeOrDuringAcquisition =
    DAQmxErrorCannotQueryPropertyBeforeOrDuringAcquisition,
  #[error("The Sample Clock Timebase property you have requested is not supported by this task because it contains a reference clock device. Use the Reference Clock properties instead.")]
  ErrorSampleClockTimebaseNotSupported =
    DAQmxErrorSampleClockTimebaseNotSupported,
  #[error("Attempted to query a timestamp before it is available. Use the DAQmx Wait for Valid Timestamp VI/function to wait until the desired timestamp is available.")]
  ErrorTimestampNotYetReceived = DAQmxErrorTimestampNotYetReceived,
  #[error("Time operations are not supported by this device or task type.")]
  ErrorTimeTriggerNotSupported = DAQmxErrorTimeTriggerNotSupported,
  #[error("Timestamp requested is not enabled for this task.")]
  ErrorTimestampNotEnabled = DAQmxErrorTimestampNotEnabled,
  #[error("A time sync pulse and time start trigger are configured. Ensure the sync pulse comes first and the difference between them is larger than the task sync time.")]
  ErrorTimeTriggersInconsistent = DAQmxErrorTimeTriggersInconsistent,
  #[error("The configured time trigger is in the past.")]
  ErrorTriggerConfiguredIsInThePast = DAQmxErrorTriggerConfiguredIsInThePast,
  #[error("A time trigger configured is too far in the future. Use I/O Device Timescale instead of Host Timescale, configure the trigger at a point in the future closer to the trigger time, or schedule the trigger closer in time to the current Host Timescale time.")]
  ErrorTriggerConfiguredIsTooFarFromCurrentTime =
    DAQmxErrorTriggerConfiguredIsTooFarFromCurrentTime,
  #[error("Synchronization lock was lost during operation. If the occasional loss of synchronization is acceptable, change the Synchronization Unlock Behavior property to ignore sync loss. Otherwise, go to ni.com for more information about sync loss management.")]
  ErrorSynchronizationLockLost = DAQmxErrorSynchronizationLockLost,
  #[error("The timescales for time start trigger and time sync pulse must be the same.")]
  ErrorInconsistentTimescales = DAQmxErrorInconsistentTimescales,
  #[error("The devices in your task cannot be synchronized. This may be because there are no available synchronization mechanisms between the devices. Some synchronization paths are not available in interactive tools like the DAQ Assistant. To determine whether synchronization between these devices is possible, try deploying and executing your task in your application environment.")]
  ErrorCannotSynchronizeDevices = DAQmxErrorCannotSynchronizeDevices,
  #[error("The specified property requires all associated channels to use the same range as specified by the Maximum and Minimum properties. Try setting the same Maximum and Minimum Values on each channel or change the conflicting property.")]
  ErrorAssociatedChansHaveAttributeConflictWithMultipleMaxMinRanges =
    DAQmxErrorAssociatedChansHaveAttributeConflictWithMultipleMaxMinRanges,
  #[error("Sample rate exceeds the maximum sample rate for the number of channels and property values specified. Reduce the sample rate or number of channels. Changing the specified property values may also increase the maximum possible sample rate.")]
  ErrorSampleRateNumChansOrAttributeValues =
    DAQmxErrorSampleRateNumChansOrAttributeValues,
  #[error(
        "DAQmx Wait for Valid Timestamp is not supported by one or more devices in this task."
    )]
  ErrorWaitForValidTimestampNotSupported =
    DAQmxErrorWaitForValidTimestampNotSupported,
  #[error(
    "Specified Trigger Window has elapsed without a trigger being detected."
  )]
  ErrorTrigWinTimeoutExpired = DAQmxErrorTrigWinTimeoutExpired,
  #[error(
    "Not all devices in the task support the specified Trigger configuration."
  )]
  ErrorInvalidTriggerCfgForDevice = DAQmxErrorInvalidTriggerCfgForDevice,
  #[error("Not all devices in the task support the specified Data Transfer Mechanism.")]
  ErrorInvalidDataTransferMechanismForDevice =
    DAQmxErrorInvalidDataTransferMechanismForDevice,
  #[error("Onboard device memory overflow. Because of system and/or bus-bandwidth limitations, the driver could not read data from the device fast enough to keep up with the data transfer throughput. Reduce the maximum data transfer rate of the device. You can also increase bandwidth by reducing the number of programs your computer is executing.")]
  ErrorInputFIFOOverflow3 = DAQmxErrorInputFIFOOverflow3,
  #[error("All channels used for the analog trigger source must be on the same device. Remove trigger configurations from the task until all of the sources are channels from the same device.")]
  ErrorTooManyDevicesForAnalogMultiEdgeTrigCDAQ =
    DAQmxErrorTooManyDevicesForAnalogMultiEdgeTrigCDAQ,
  #[error("The device does not support using more than one trigger type at a time with the specified task configuration. Configure the device to use only one trigger type, or use a configuration that supports using multiple trigger types.")]
  ErrorTooManyTriggersTypesSpecifiedInTask =
    DAQmxErrorTooManyTriggersTypesSpecifiedInTask,
  #[error("Multiple sourced triggering requires the same number of values for all user-specified trigger configurations.")]
  ErrorMismatchedMultiTriggerConfigValues =
    DAQmxErrorMismatchedMultiTriggerConfigValues,
  #[error("The specified AO DAC range must be consistent with all reserved tasks on this device. Ensure that all reserved tasks on this device use the same value for the AO.DAC.Ref.Val channel attribute/property.")]
  ErrorInconsistentAODACRangeAcrossTasks =
    DAQmxErrorInconsistentAODACRangeAcrossTasks,
  #[error("The waveforms passed to DAQmx Write claim to be sampled at inconsistent rates. Check to make sure the data is sampled at the correct rate and either re-sample the data or correct the dt element of the waveforms.")]
  ErrorInconsistentDTToWrite = DAQmxErrorInconsistentDTToWrite,
  #[error("The function called is no longer supported by DAQmx.")]
  ErrorFunctionObsolete = DAQmxErrorFunctionObsolete,
  #[error("Negative duration values other than -1 are not supported. -1 indicates to read all samples.")]
  ErrorNegativeDurationNotSupported = DAQmxErrorNegativeDurationNotSupported,
  #[error("No samples were acquired within the specified duration. Increase the duration or sample rate so that at least one sample is acquired.")]
  ErrorDurationTooSmall = DAQmxErrorDurationTooSmall,
  #[error("The specified duration is too long. Specify a shorter duration.")]
  ErrorDurationTooLong = DAQmxErrorDurationTooLong,
  #[error(
    "Duration-based reads are supported only in sample clock timing mode."
  )]
  ErrorDurationBasedNotSupportedForSpecifiedTimingMode =
    DAQmxErrorDurationBasedNotSupportedForSpecifiedTimingMode,
  #[error("The selected LED state is invalid.")]
  ErrorInvalidLEDState = DAQmxErrorInvalidLEDState,
  #[error("A device in your watchdog task does not support different output state types. Set all channels to the same output state type or remove the channels from the task.")]
  ErrorWatchdogStatesNotUniform = DAQmxErrorWatchdogStatesNotUniform,
  #[error("Self-test of the device has failed because the measured power supply voltage is outside of tolerance. Please contact National Instruments technical support.")]
  ErrorSelfTestFailedPowerSupplyOutOfTolerance =
    DAQmxErrorSelfTestFailedPowerSupplyOutOfTolerance,
  #[error("DAQmx Write does not support multiple samples in Hardware-Timed Single-Point tasks. Specify a single sample.")]
  ErrorHWTSPMultiSampleWrite = DAQmxErrorHWTSPMultiSampleWrite,
  #[error("You cannot use onboard regeneration for a task with this many channels. Reduce the number of channels in the task, use fewer modules with more the 16-bits of precision, or set the AO.UseOnlyOnBrdMem attribute/property to false.")]
  ErrorOnboardRegenExceedsChannelLimit =
    DAQmxErrorOnboardRegenExceedsChannelLimit,
  #[error("To create watchdog task on this device, you must specify expiration states for all lines. Specify states for the missing channels.")]
  ErrorWatchdogChannelExpirationStateNotSpecified =
    DAQmxErrorWatchdogChannelExpirationStateNotSpecified,
  #[error(
        "The selected shunt source option for the shunt calibration is not valid for this device."
    )]
  ErrorInvalidShuntSourceForCalibration =
    DAQmxErrorInvalidShuntSourceForCalibration,
  #[error(
        "The selected shunt select option for the shunt calibration is not valid for this device."
    )]
  ErrorInvalidShuntSelectForCalibration =
    DAQmxErrorInvalidShuntSelectForCalibration,
  #[error("This device does not support shunt calibration for the requested configuration of shunt select and shunt source. Refer to the shunt calibration documentation for your application development environment for more information.")]
  ErrorInvalidShuntCalibrationConfiguration =
    DAQmxErrorInvalidShuntCalibrationConfiguration,
  #[error("The selected channels do not support buffered operations when alone in a task. Add an additional channel which supports buffered operations to the task or reconfigure the task to use On-Demand timing.")]
  ErrorBufferedOperationsNotSupportedOnChannelStandalone =
    DAQmxErrorBufferedOperationsNotSupportedOnChannelStandalone,
  #[error("The specified feature is not supported on the attached accessory. Refer to your accessory documentation for accessories that support the requested feature.")]
  ErrorFeatureNotAvailableOnAccessory =
    DAQmxErrorFeatureNotAvailableOnAccessory,
  #[error("The threshold voltage value must be consistent when using different terminals on the same device with RSE terminal configuration as a source for multiple inputs. For example, do not set 2.0 V on PFI0 for one input, and 3.0 V on PFI1 for another input.")]
  ErrorInconsistentThreshVoltageAcrossTerminals =
    DAQmxErrorInconsistentThreshVoltageAcrossTerminals,
  #[error("NI-DAQmx is not installed on the target system, is incompatible, or the installation is corrupt. Install or repair the driver software.")]
  ErrorDAQmxIsNotInstalledOnTarget = DAQmxErrorDAQmxIsNotInstalledOnTarget,
  #[error("Task with counter output detected a sample clock before the corresponding DAQmx Write was completed. This may have occurred because the frequency of the counter output is too low for the given sample clock rate. A full output period must complete before new data can be written. To avoid this problem make sure that the counter output frequency is significantly higher than the sample clock rate.")]
  ErrorCOCannotKeepUpInHWTimedSinglePoint =
    DAQmxErrorCOCannotKeepUpInHWTimedSinglePoint,
  #[error("DAQmx Wait for Next Sample Clock detected 3 or more missed sample clocks since the last call to Wait for Next Sample Clock which indicates your program is not keeping up with the sample clock, and data was subsequently lost before it could be read by the application. To remove this error, slow down the sample clock.  Consider restructuring the application so you can call DAQmx Read more often. Setting the Convert     Error = DAQmxError to Warning property to True will not eliminate the error because the data is lost.")]
  ErrorWaitForNextSampClkDetected3OrMoreSampClks =
    DAQmxErrorWaitForNextSampClkDetected3OrMoreSampClks,
  #[error("DAQmx Wait for Next Sample Clock detected one or more missed sample clocks since the last call to Wait for Next Sample Clock which indicates that your program is not keeping up with the sample clock. To remove this error, slow down the sample clock, or else change your application so that it can keep up with the sample clock. Alternatively, consider setting the Convert  Errors = DAQmxErrors to Warnings property to true and then handling the warning case appropriately.")]
  ErrorWaitForNextSampClkDetectedMissedSampClk =
    DAQmxErrorWaitForNextSampClkDetectedMissedSampClk,
  #[error("DAQmx Write did not complete before the arrival of the next sample clock which indicates that your program is not keeping up with the hardware clock. Slow down the hardware clock or else change your application so that it can keep up with the hardware clock.")]
  ErrorWriteNotCompleteBeforeSampClk = DAQmxErrorWriteNotCompleteBeforeSampClk,
  #[error("DAQmx Read did not complete before the arrival of  the next sample clock or change detection event  which indicates that your program is not keeping up with the hardware clock or the external change event. For tasks using sample clock timing, slow down the hardware clock or else change your application so that it can keep up with the hardware clock.  For tasks using change detection timing, decrease the frequency of your event or else change your application so that it can keep up with the change event.")]
  ErrorReadNotCompleteBeforeSampClk = DAQmxErrorReadNotCompleteBeforeSampClk,
  #[error("The digital filtering properties must be consistent when using the same terminal as a source for multiple inputs. For example, digital filtering can either be enabled with the same minimum pulse width configuration whenever PFI0 is used, or disabled for all cases.")]
  ErrorInconsistentDigitalFilteringAcrossTerminals =
    DAQmxErrorInconsistentDigitalFilteringAcrossTerminals,
  #[error("The logic level behavior must be consistent when using the same terminal as a source for multiple inputs. For example, do not enable the pull-up on PFI0 for one input, and set it to none on PFI0 for another input.")]
  ErrorInconsistentPullUpCfgAcrossTerminals =
    DAQmxErrorInconsistentPullUpCfgAcrossTerminals,
  #[error("The terminal configuration must be consistent when using the same terminal as a source for multiple inputs. For example, do not set PFI0 to differential for one input, and PFI0 to RSE for another input.")]
  ErrorInconsistentTermCfgAcrossTerminals =
    DAQmxErrorInconsistentTermCfgAcrossTerminals,
  #[error("A fatal hardware clocking error has occurred. The device is unusable until you restart it. If the problem persists, please contact National Instruments Technical Support.")]
  ErrorVCXODCMBecameUnlocked = DAQmxErrorVCXODCMBecameUnlocked,
  #[error("A hardware clocking error has occurred. Try the operation again. If the problem persists, please contact National Instruments Technical Support.")]
  ErrorPLLDACUpdateFailed = DAQmxErrorPLLDACUpdateFailed,
  #[error("The chassis requires at least one DAQ device directly cabled to it. Edit the chassis and assign a device.")]
  ErrorNoCabledDevice = DAQmxErrorNoCabledDevice,
  #[error("A fatal hardware clocking error has occurred. The device is unusable until you restart it. If you are using an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorLostRefClk = DAQmxErrorLostRefClk,
  #[error("The AI timing engine cannot be used for counter tasks. You can select a different timing engine or let the driver automatically select one.")]
  ErrorCantUseAITimingEngineWithCounters =
    DAQmxErrorCantUseAITimingEngineWithCounters,
  #[error("DAC Offset Voltage Value is not set. When the DAC Offset Voltage Source property for a channel is set to External, the DAC Offset Voltage Value property must be set. Set the DAC Offset Voltage Value property so the value matches the offset voltage source connected to your device. Alternatively, use the internal DAC offset voltage source available on the device.")]
  ErrorDACOffsetValNotSet = DAQmxErrorDACOffsetValNotSet,
  #[error(
    "The reference value input of calibration adjustment is out of range."
  )]
  ErrorCalAdjustRefValOutOfRange = DAQmxErrorCalAdjustRefValOutOfRange,
  #[error("The channel in calibration adjustment did not call calibration setup. Make sure you call calibration setup before calibration adjustment for this channel.")]
  ErrorChansForCalAdjustMustPerformSetContext =
    DAQmxErrorChansForCalAdjustMustPerformSetContext,
  #[error("The selected calibration mode cannot query calibration data points. Make sure you follow the calibration procedure.")]
  ErrorGetCalDataInvalidForCalMode = DAQmxErrorGetCalDataInvalidForCalMode,
  #[error("AI channels on this device do not support using AC coupling while not using IEPE excitation. Enable IEPE excitation or set the coupling mode to DC.")]
  ErrorNoIEPEWithACNotAllowed = DAQmxErrorNoIEPEWithACNotAllowed,
  #[error("You must run setupCalibration before running getCalDataPoints.")]
  ErrorSetupCalNeededBeforeGetCalDataPoints =
    DAQmxErrorSetupCalNeededBeforeGetCalDataPoints,
  #[error("Voltage settings not calibrated. Ensure all voltage settings are calibrated before trying to calibrate bridge settings.")]
  ErrorVoltageNotCalibrated = DAQmxErrorVoltageNotCalibrated,
  #[error("Device calibration requires all ranges to be calibrated for a single channel. Calibrate the ranges specified in the procedure.")]
  ErrorMissingRangeForCalibration = DAQmxErrorMissingRangeForCalibration,
  #[error("Device does not support simultaneous calibration of multiple channels. Calibrate channels one channel at a time passing individual channels to different invocations of DAQmx Adjust Calibration.")]
  ErrorMultipleChansNotSupportedDuringCalAdjust =
    DAQmxErrorMultipleChansNotSupportedDuringCalAdjust,
  #[error("Shunt calibration failed. The calculated gain adjust is out of range. Ensure that the shunt calibration terminals are connected properly and that the shunt resistance and shunt element location settings match how the hardware is wired.")]
  ErrorShuntCalFailedOutOfRange = DAQmxErrorShuntCalFailedOutOfRange,
  #[error("Attempted operation is not supported on simulated devices.")]
  ErrorOperationNotSupportedOnSimulatedDevice =
    DAQmxErrorOperationNotSupportedOnSimulatedDevice,
  #[error(
    "Firmware version being installed matches the currently installed version."
  )]
  ErrorFirmwareVersionSameAsInstalledVersion =
    DAQmxErrorFirmwareVersionSameAsInstalledVersion,
  #[error("Firmware version being installed is older than the currently installed firmware.")]
  ErrorFirmwareVersionOlderThanInstalledVersion =
    DAQmxErrorFirmwareVersionOlderThanInstalledVersion,
  #[error("Invalid state for firmware update. You must call the firmware update action after calling beginFirmwareAction and before calling disposeFirmwareAction.")]
  ErrorFirmwareUpdateInvalidState = DAQmxErrorFirmwareUpdateInvalidState,
  #[error("Firmware update failed. System attempted to update a different device than what you specified. Try updating again.")]
  ErrorFirmwareUpdateInvalidID = DAQmxErrorFirmwareUpdateInvalidID,
  #[error("You must disable automatic firmware updates before attempting to manually update the firmware of your device.")]
  ErrorFirmwareUpdateAutomaticManagementEnabled =
    DAQmxErrorFirmwareUpdateAutomaticManagementEnabled,
  #[error("You must call DAQmx Setup Calibration first in order to calibrate this module.")]
  ErrorSetupCalibrationNotCalled = DAQmxErrorSetupCalibrationNotCalled,
  #[error("Measured data size does not match reference data size. The module acquires data and receives reference data in different functions. Please make sure these calibration functions are executed the same number of times during calibration.")]
  ErrorCalMeasuredDataSizeVsActualDataSizeMismatch =
    DAQmxErrorCalMeasuredDataSizeVsActualDataSizeMismatch,
  #[error("The current task contains channels from both delta-sigma and non delta-sigma devices. The first channel in the task must be from a delta-sigma device and all devices must be in chassis that are connected using NI 9469 or time-based network synchronization.")]
  ErrorCDAQMissingDSAMasterForChanExpansion =
    DAQmxErrorCDAQMissingDSAMasterForChanExpansion,
  #[error("The selected master for the configured multi-device task is not able to export signals. Make sure that the first channel in the task is from a device on a chassis that has an NI 9469 capable of exporting signals to the slave devices. If you have any delta-sigma modules in your task, at least one must be in the master chassis. For time-based synchronization, ensure all the chassis in the task are in the same synchronization network.")]
  ErrorCDAQMasterNotFoundForChanExpansion =
    DAQmxErrorCDAQMasterNotFoundForChanExpansion,
  #[error("The current number of channels chosen for calibration is incorrect. You must specify all channels for calibration.")]
  ErrorAllChansShouldBeProvidedForCalibration =
    DAQmxErrorAllChansShouldBeProvidedForCalibration,
  #[error("If you specify an expiration state for any line on an NI 9401 in the range of line 0:3 or line 4:7, you must specify an expiration state for every line in that range.")]
  ErrorMustSpecifyExpirationStateForAllLinesInRange =
    DAQmxErrorMustSpecifyExpirationStateForAllLinesInRange,
  #[error("There is already a session open to the device from another process, or a calibration session is open. You must close the open session, exit the application holding the device, or release the device in the Soft Front Panel.")]
  ErrorOpenSessionExists = DAQmxErrorOpenSessionExists,
  #[error("The attribute/property ArmStart.Term cannot be queried when a software arm start trigger is configured because it cannot be exported to any terminal. Configure an external arm start trigger and query the desired attribute/property.")]
  ErrorCannotQueryTerminalForSWArmStart =
    DAQmxErrorCannotQueryTerminalForSWArmStart,
  #[error("Watchdog timer task has expired. Reset the chassis to resume normal operation.")]
  ErrorChassisWatchdogTimerExpired = DAQmxErrorChassisWatchdogTimerExpired,
  #[error("Reservation of watchdog timer task failed because another task is reserved or running. Stop and unreserve all other tasks before reserving a watchdog timer task.")]
  ErrorCantReserveWatchdogTaskWhileOtherTasksReserved =
    DAQmxErrorCantReserveWatchdogTaskWhileOtherTasksReserved,
  #[error("Task reservation failed because a watchdog timer task is reserved. Unreserve or commit the watchdog timer task to reserve a new task.")]
  ErrorCantReserveTaskWhileWatchdogTaskReserving =
    DAQmxErrorCantReserveTaskWhileWatchdogTaskReserving,
  #[error("Auxiliary power not detected. Verify that the auxiliary power source is properly connected to the device and that the auxiliary input fuse has not blown. Refer to the NI-DCPower documentation for information about replacing the input fuse.")]
  ErrorAuxPowerSourceRequired = DAQmxErrorAuxPowerSourceRequired,
  #[error("The device is not supported on the local system, is incompatible, or the installation is corrupt. Install and/or repair the appropriate driver.")]
  ErrorDeviceNotSupportedOnLocalSystem =
    DAQmxErrorDeviceNotSupportedOnLocalSystem,
  #[error(
        "You must specify exactly one timestamp channel for a Navigation With Timestamp read call."
    )]
  ErrorOneTimestampChannelRequiredForCombinedNavigationRead =
    DAQmxErrorOneTimestampChannelRequiredForCombinedNavigationRead,
  #[error("The physical channel string contains multiple devices. You can include multiple physical channels on a single device (for example, \"Dev1/ao0:3\") but not multiple physical channels on multiple devices (for example, \"Dev1/ao0:3, Dev2/ao2\").")]
  ErrorMultDevsMultPhysChans = DAQmxErrorMultDevsMultPhysChans,
  #[error("Invalid calibration adjustment point(s) provided. Use DAQmx Get Calibration Adjustment Points to retrieve valid adjustment values.")]
  ErrorInvalidCalAdjustmentPointValues =
    DAQmxErrorInvalidCalAdjustmentPointValues,
  #[error("You selected a different digitizer and communicator for an SCXI module that is in multiplexed mode. Configure the SCXI module with the digitizer set to the same mode as the chassis communicator in MAX.")]
  ErrorDifferentDigitizerFromCommunicator =
    DAQmxErrorDifferentDigitizerFromCommunicator,
  #[error("The requested cDAQ Sync device cannot be configured because the master timebase is not present.  Tasks containing cDAQ Sync devices that export or import a timebase must be committed in cascading order from the source to the destination. Use DAQmx Control Task to commit the master timebase source task prior to committing or starting a slave task.")]
  ErrorCDAQSyncMasterClockNotPresent = DAQmxErrorCDAQSyncMasterClockNotPresent,
  #[error("Associated channels have conflicting properties. Make the conflicting property values consistent across channels to fix the error.")]
  ErrorAssociatedChansHaveConflictingProps =
    DAQmxErrorAssociatedChansHaveConflictingProps,
  #[error("Cannot auto-configure connections between devices in different states (present, offline, and simulated). Please check requested set.")]
  ErrorAutoConfigBetweenMultipleDeviceStatesInvalid =
    DAQmxErrorAutoConfigBetweenMultipleDeviceStatesInvalid,
  #[error("Cannot perform auto-configure on offline devices. Please remove offline devices from the requested set.")]
  ErrorAutoConfigOfOfflineDevicesInvalid =
    DAQmxErrorAutoConfigOfOfflineDevicesInvalid,
  #[error("A hardware fault has occurred. Please contact National Instruments technical support. To clear the fault, power cycle the device.")]
  ErrorExternalFIFOFault = DAQmxErrorExternalFIFOFault,
  #[error(
        "The requested ports are not reciprocal. Make sure that the ports point at each other."
    )]
  ErrorConnectionsNotReciprocal = DAQmxErrorConnectionsNotReciprocal,
  #[error("The attempted connection is not between an output port and an input port. Port 0 is input only. All other ports are output only. Please check your connection and try again.")]
  ErrorInvalidOutputToInputCDAQSyncConnection =
    DAQmxErrorInvalidOutputToInputCDAQSyncConnection,
  #[error("Reference clock is not present. This task requires a reference clock to be present before configuring hardware.")]
  ErrorReferenceClockNotPresent = DAQmxErrorReferenceClockNotPresent,
  #[error("No devices scanned support cDAQ Sync connections. Requesting an empty string only scans physical devices (present and offline) and ignores NI-DAQmx simulated devices.  Please check your hardware configuration.")]
  ErrorBlankStringExpansionFoundNoSupportedCDAQSyncConnectionDevices =
    DAQmxErrorBlankStringExpansionFoundNoSupportedCDAQSyncConnectionDevices,
  #[error("Specified devices do not support cDAQ Sync connections. Please select a different set of devices.")]
  ErrorNoDevicesSupportCDAQSyncConnections =
    DAQmxErrorNoDevicesSupportCDAQSyncConnections,
  #[error("Specified timeout value is not supported. Set the timeout to to a value > 0 or -1 (wait infinitely).")]
  ErrorInvalidCDAQSyncTimeoutValue = DAQmxErrorInvalidCDAQSyncTimeoutValue,
  #[error("cDAQ Sync cannot add a connection from a port to the same port. Add a connection to a different port instead.")]
  ErrorCDAQSyncConnectionToSamePort = DAQmxErrorCDAQSyncConnectionToSamePort,
  #[error("The devices attempting to be configured for cDAQ Sync do not have a common sync connection strategy.")]
  ErrorDevsWithoutCommonSyncConnectionStrategy =
    DAQmxErrorDevsWithoutCommonSyncConnectionStrategy,
  #[error("cDAQ Sync connections are not allowed between physical and NI-DAQmx simulated devices. Use cDAQ Sync for either only physical devices or only NI-DAQmx simulated devices.")]
  ErrorNoCDAQSyncBetweenPhysAndSimulatedDevs =
    DAQmxErrorNoCDAQSyncBetweenPhysAndSimulatedDevs,
  #[error("Carrier physically unable to contain the declared cards.")]
  ErrorUnableToContainCards = DAQmxErrorUnableToContainCards,
  #[error("Cannot find disconnected connections between devices in different states (present and simulated).  Please check requested set.")]
  ErrorFindDisconnectedBetweenPhysAndSimDeviceStatesInvalid =
    DAQmxErrorFindDisconnectedBetweenPhysAndSimDeviceStatesInvalid,
  #[error("The operation has been aborted.")]
  ErrorOperationAborted = DAQmxErrorOperationAborted,
  #[error("The operation cannot be completed on only one port. Two ports are required.")]
  ErrorTwoPortsRequired = DAQmxErrorTwoPortsRequired,
  #[error("The requested device does not support cDAQ Sync connections.")]
  ErrorDeviceDoesNotSupportCDAQSyncConnections =
    DAQmxErrorDeviceDoesNotSupportCDAQSyncConnections,
  #[error("The requested port connection string is not in a valid format. The valid format is <device name>/port<port number> (e.g. Dev1/port2).")]
  ErrorInvalidcDAQSyncPortConnectionFormat =
    DAQmxErrorInvalidcDAQSyncPortConnectionFormat,
  #[error("No rosette measurements specified. Please specify one or more rosette measurements.")]
  ErrorRosetteMeasurementsNotSpecified =
    DAQmxErrorRosetteMeasurementsNotSpecified,
  #[error("The selected rosette requires three physical channels. Make sure the physical channel list contains three physical channels for each rosette.")]
  ErrorInvalidNumOfPhysChansForDeltaRosette =
    DAQmxErrorInvalidNumOfPhysChansForDeltaRosette,
  #[error("Each tee rosette requires two physical channels. Make sure the physical channel list contains two physical channels for each tee rosette.")]
  ErrorInvalidNumOfPhysChansForTeeRosette =
    DAQmxErrorInvalidNumOfPhysChansForTeeRosette,
  #[error("You must configure strain gage channels for each arm of the rosette. Specify the strain gage channel names.")]
  ErrorRosetteStrainChanNamesNeeded = DAQmxErrorRosetteStrainChanNamesNeeded,
  #[error("Multidevice tasks cannot use the on-demand sample timing type. Configure timing to synchronize and acquire samples from multiple devices.")]
  ErrorMultideviceWithOnDemandTiming = DAQmxErrorMultideviceWithOnDemandTiming,
  #[error("FREQOUT counter cannot generate the desired frequency. The 4-bit FREQOUT counter can divide the 20 MHz, 10 MHz (20 MHz / 2), or 100 kHz (20 MHz / 200) timebase by a number between 1 and 16. Choose a frequency within this range.")]
  ErrorFREQOUTCannotProduceDesiredFrequency3 =
    DAQmxErrorFREQOUTCannotProduceDesiredFrequency3,
  #[error("Cannot measure two-edge separation with both the first and second terminal set to the same signal and both the first and second edge set to the same edge. To measure the period of a signal, use a counter input period task. Otherwise, change one of the terminals to a different signal, or change one of the edges to be different from the other.")]
  ErrorTwoEdgeSeparationSameTerminalSameEdge =
    DAQmxErrorTwoEdgeSeparationSameTerminalSameEdge,
  #[error("On an NI 449x, specify either the sync pulse source or the sample clock timebase source but not both.")]
  ErrorDontMixSyncPulseAndSampClkTimebaseOn449x =
    DAQmxErrorDontMixSyncPulseAndSampClkTimebaseOn449x,
  #[error("Neither an external reference clock nor a sample clock timebase has been specified. For multi-device synchronization, you must specify the sync pulse source and either an external reference clock or sample clock timebase. Refer to the device documentation for details on multi-device synchronization.")]
  ErrorNeitherRefClkNorSampClkTimebaseConfiguredForDSASync =
    DAQmxErrorNeitherRefClkNorSampClkTimebaseConfiguredForDSASync,
  #[error("Retriggering is not allowed for finite Sample Clock-timed counter output tasks. Reconfigure the task to use a different sample timing type or disable retriggering.")]
  ErrorRetriggeringFiniteCONotAllowed =
    DAQmxErrorRetriggeringFiniteCONotAllowed,
  #[error("The device was rebooted after a watchdog timer expired due to unresponsive firmware or hardware components. A watchdog timer that times out can cause digital output lines to change state. To clear this error, reset or power cycle the device. Please also contact National Instruments technical support.")]
  ErrorDeviceRebootedFromWDTTimeout = DAQmxErrorDeviceRebootedFromWDTTimeout,
  #[error(
        "The timeout value specified exceeds the maximum timeout value supported by this device."
    )]
  ErrorTimeoutValueExceedsMaximum = DAQmxErrorTimeoutValueExceedsMaximum,
  #[error("Unable to route signals through the analog bus that are composed of different wire modes. Please disconnect any devices of different wire modes from the analog bus before routing this device through the analog bus.")]
  ErrorSharingDifferentWireModes = DAQmxErrorSharingDifferentWireModes,
  #[error(
    "No samples provided to DAQmx Write to initialize buffered generation."
  )]
  ErrorCantPrimeWithEmptyBuffer = DAQmxErrorCantPrimeWithEmptyBuffer,
  #[error("Configuration failed because the task tried to change the direction of a line while the watchdog timer is expired. Clear the expiration of the watchdog timer task before trying to change the direction of any line, even if the line is not watched by the watchdog timer task.")]
  ErrorConfigFailedBecauseWatchdogExpired =
    DAQmxErrorConfigFailedBecauseWatchdogExpired,
  #[error("Write failed because a watchdog timer task expired and changed the direction of the lines to tristate after the digital output task was committed. To avoid this, unreserve and recommit the digital output task after the watchdog timer expiration has been cleared to reconfigure the lines to output.")]
  ErrorWriteFailedBecauseWatchdogChangedLineDirection =
    DAQmxErrorWriteFailedBecauseWatchdogChangedLineDirection,
  #[error("Calibration adjustment cannot be completed on a device performing different types of measurements (for instance, voltage and current measurements). Make sure only one measurement type is being calibrated in each calibration session.")]
  ErrorMultipleSubsytemCalibration = DAQmxErrorMultipleSubsytemCalibration,
  #[error("Calibration offset adjustment has failed because the wrong channel was selected. Check the module calibration procedure to decide which channel to use for the calibration offset adjustment.")]
  ErrorIncorrectChannelForOffsetAdjustment =
    DAQmxErrorIncorrectChannelForOffsetAdjustment,
  #[error("You have requested an invalid number of reference voltages to write. Ensure the number of values to write is the same as the number of entries in the array.")]
  ErrorInvalidNumRefVoltagesToWrite = DAQmxErrorInvalidNumRefVoltagesToWrite,
  #[error("Start trigger delay is not available when a C Series Delta-Sigma module or a Reference Clock module is in the task.")]
  ErrorStartTrigDelayWithDSAModule = DAQmxErrorStartTrigDelayWithDSAModule,
  #[error("More than one sync pulse was detected. For proper operation, only a single sync pulse signal can be provided to all DSA modules in a task.")]
  ErrorMoreThanOneSyncPulseDetected = DAQmxErrorMoreThanOneSyncPulseDetected,
  #[error("The specified device is not supported within the NI-DAQmx API.")]
  ErrorDevNotSupportedWithinDAQmxAPI = DAQmxErrorDevNotSupportedWithinDAQmxAPI,
  #[error("One or more devices do not support multidevice tasks.")]
  ErrorDevsWithoutSyncStrategies = DAQmxErrorDevsWithoutSyncStrategies,
  #[error("The multidevice task does not have a method for synchronizing timing that is compatible with all of the included devices.")]
  ErrorDevsWithoutCommonSyncStrategy = DAQmxErrorDevsWithoutCommonSyncStrategy,
  #[error(
    "The multidevice task cannot be synchronized in its current configuration."
  )]
  ErrorSyncStrategiesCannotSync = DAQmxErrorSyncStrategiesCannotSync,
  #[error("Communication with the chassis has been interrupted. Check the cabling and/or the wireless signal to the chassis. Then reset the chassis using DAQmx Reset Device or Device Reset in MAX to re-establish communication.")]
  ErrorChassisCommunicationInterrupted =
    DAQmxErrorChassisCommunicationInterrupted,
  #[error("Your SwitchBlock carrier contains one or more cards with power characteristics unknown to the driver. To protect the hardware from overheating, all devices within your carrier are disallowed from drawing power. Upgrade your software driver to the latest version or shutdown your system and remove any unknown or invalid card(s).")]
  ErrorUnknownCardPowerProfileInCarrier =
    DAQmxErrorUnknownCardPowerProfileInCarrier,
  #[error(
    "The accessory attached to the device does not support this property."
  )]
  ErrorAttrNotSupportedOnAccessory = DAQmxErrorAttrNotSupportedOnAccessory,
  #[error("The network device is currently reserved by another host. Specify whether you want  to override the other host's reservation.")]
  ErrorNetworkDeviceReservedByAnotherHost =
    DAQmxErrorNetworkDeviceReservedByAnotherHost,
  #[error("Device firmware has not been updated because the firmware file uploaded is for a different type of device or an older revision of this device. Please verify that the firmware file is correct for this device.")]
  ErrorIncorrectFirmwareFileUploaded = DAQmxErrorIncorrectFirmwareFileUploaded,
  #[error("Device firmware has not been updated because the firmware file uploaded is corrupt or is not a valid firmware image file. Please verify that the file specified is a valid National Instruments firmware image.")]
  ErrorInvalidFirmwareFileUploaded = DAQmxErrorInvalidFirmwareFileUploaded,
  #[error("Timing is configured without supplying a clock signal. Either supply an external clock or use an internal timebase.")]
  ErrorInTimerTimeoutOnArm = DAQmxErrorInTimerTimeoutOnArm,
  #[error("Switch device has been disabled to prevent it from exceeding its simultaneous relay drive limit. To recover, disconnect a relay or channel.")]
  ErrorCantExceedSlotRelayDriveLimit = DAQmxErrorCantExceedSlotRelayDriveLimit,
  #[error("The module is not supported by the NI 9163.")]
  ErrorModuleUnsupportedFor9163 = DAQmxErrorModuleUnsupportedFor9163,
  #[error("The specified connection is not supported on the attached accessory. Refer to your accessory documentation for supported connections.")]
  ErrorConnectionsNotSupported = DAQmxErrorConnectionsNotSupported,
  #[error("There is no accessory attached to the device.")]
  ErrorAccessoryNotPresent = DAQmxErrorAccessoryNotPresent,
  #[error("The specified accessory channels are not present on this device. The accessory channels should be specified for the device in the calibration session.")]
  ErrorSpecifiedAccessoryChannelsNotPresentOnDevice =
    DAQmxErrorSpecifiedAccessoryChannelsNotPresentOnDevice,
  #[error("The accessory attached to the device does not support connections. Attach an accessory that supports connections.")]
  ErrorConnectionsNotSupportedOnAccessory =
    DAQmxErrorConnectionsNotSupportedOnAccessory,
  #[error("The requested Sample Clock rate is too fast for hardware-timed single point. Consider decreasing the Sample Clock rate, increasing the convert rate, or decreasing the delay from the Sample Clock. If using an external Sample Clock source, you might also decrease the number of samples to average.")]
  ErrorRateTooFastForHWTSP = DAQmxErrorRateTooFastForHWTSP,
  #[error("The requested delay from Sample Clock is out of range for a hardware-timed single-point acquisition.")]
  ErrorDelayFromSampleClockOutOfRangeForHWTSP =
    DAQmxErrorDelayFromSampleClockOutOfRangeForHWTSP,
  #[error("Averaging of data is only supported when the Sample Mode is set to Hardware Timed Single Point and the Sample Clock source is external to the device.")]
  ErrorAveragingWhenNotInternalHWTSP = DAQmxErrorAveragingWhenNotInternalHWTSP,
  #[error("Specified property is not supported unless Sample Mode is set to Hardware Timed Single Point.")]
  ErrorAttributeNotSupportedUnlessHWTSP =
    DAQmxErrorAttributeNotSupportedUnlessHWTSP,
  #[error("The 5 V fuse on your NI SwitchBlock carrier is blown. Refer to your documentation for help with replacing the fuse.")]
  ErrorFiveVoltDetectFailed = DAQmxErrorFiveVoltDetectFailed,
  #[error("An expansion bridge has been inserted or removed while your system is powered on. This can lead to unexpected behavior. Restart your system.")]
  ErrorAnalogBusStateInconsistent = DAQmxErrorAnalogBusStateInconsistent,
  #[error("One or more cards for your NI SwitchBlock device have been inserted and/or removed while your system was powered on. This can lead to unexpected behavior. Restart your system.")]
  ErrorCardDetectedDoesNotMatchExpectedCard =
    DAQmxErrorCardDetectedDoesNotMatchExpectedCard,
  #[error("You have specified a new file path but did not call DAQmx Start New File. To change the file path while logging, configure Logging.SampsPerFile or call DAQmx Start New File.")]
  ErrorLoggingStartNewFileNotCalled = DAQmxErrorLoggingStartNewFileNotCalled,
  #[error("The samples per file specified is not evenly divisible by the file write size.  Either change the samples per file or modify the file write size. If not explicitly set, the file write size can be inferred from the buffer size, which is based on the sample rate.")]
  ErrorLoggingSampsPerFileNotDivisible =
    DAQmxErrorLoggingSampsPerFileNotDivisible,
  #[error(
        "Retrieving properties from the network device failed. Make sure the device is connected."
    )]
  ErrorRetrievingNetworkDeviceProperties =
    DAQmxErrorRetrievingNetworkDeviceProperties,
  #[error("Failed to reserve file size. File size pre-allocation might require you to run the application with administrator privileges. If the operating system uses User Account Control, configure this control properly.")]
  ErrorFilePreallocationFailed = DAQmxErrorFilePreallocationFailed,
  #[error("Multi-device timed DIO tasks require all modules to be the same type. You can select either all your serial digital modules or all your parallel digital modules in this task.")]
  ErrorModuleMismatchInSameTimedTask = DAQmxErrorModuleMismatchInSameTimedTask,
  #[error("Requested value is not supported for this property. If you did not directly set this property to the unsupported value, check other properties that you have set, as they can influence the scaled value of this property.")]
  ErrorInvalidAttributeValuePossiblyDueToOtherAttributeValues =
    DAQmxErrorInvalidAttributeValuePossiblyDueToOtherAttributeValues,
  #[error("Change detection has detected interrupts occurring at a higher rate than can be handled. The change detection task has been stopped to prevent the device from being reset because of this condition. If this is the result of unwanted noise on a digital signal, use a digital filter to eliminate unwanted digital transitions.")]
  ErrorChangeDetectionStoppedToPreventDeviceHang =
    DAQmxErrorChangeDetectionStoppedToPreventDeviceHang,
  #[error("Filter Delay Removal is not supported when an analog start trigger is in use. Change Filter Delay Removal to false when using an analog start trigger. Refer to Filter Delay Removal in your DSA documentation for more details.")]
  ErrorFilterDelayRemovalNotPosssibleWithAnalogTrigger =
    DAQmxErrorFilterDelayRemovalNotPosssibleWithAnalogTrigger,
  #[error("The task is not buffered or has no channels.  If the task is not buffered, use the scalar version of this function. If the task has no channels, add one to the task.")]
  ErrorNonbufferedOrNoChannels = DAQmxErrorNonbufferedOrNoChannels,
  #[error("Tristate logic level is only port configurable for this device.")]
  ErrorTristateLogicLevelNotSpecdForEntirePort =
    DAQmxErrorTristateLogicLevelNotSpecdForEntirePort,
  #[error("Tristate logic level is not supported on output only lines.")]
  ErrorTristateLogicLevelNotSupportedOnDigOutChan =
    DAQmxErrorTristateLogicLevelNotSupportedOnDigOutChan,
  #[error(
    "Device does not support configuring tristate logic level in software."
  )]
  ErrorTristateLogicLevelNotSupported =
    DAQmxErrorTristateLogicLevelNotSupported,
  #[error("Device could not complete calibration because calibration was not performed for all gain and coupling settings. Use DAQmx Adjust DSA AI Calibration function/VI to calibrate for the following gain and coupling settings:")]
  ErrorIncompleteGainAndCouplingCalAdjustment =
    DAQmxErrorIncompleteGainAndCouplingCalAdjustment,
  #[error("Connection to the network device was lost. This can indicate an unplugged network cable, a failing network component, or a network device that is reserved by another host.")]
  ErrorNetworkStatusConnectionLost = DAQmxErrorNetworkStatusConnectionLost,
  #[error("Modules were inserted or removed while the connection to the network device was lost. Reset the chassis using DAQmx Reset Device or Measurement & Automation Explorer and wait for the modules to be redetected before proceeding.")]
  ErrorModuleChangeDuringConnectionLoss =
    DAQmxErrorModuleChangeDuringConnectionLoss,
  #[error("Network device is not reserved for this host.")]
  ErrorNetworkDeviceNotReservedByHost =
    DAQmxErrorNetworkDeviceNotReservedByHost,
  #[error("The DAQmx Adjust DSA AI Calibration with Gain and Coupling function/VI was executed more than once for the same combination of gain and coupling settings. Call the DAQmx Adjust DSA AI Calibration with Gain and Coupling function/VI only once for the following combination of gain and coupling settings:")]
  ErrorDuplicateCalibrationAdjustmentInput =
    DAQmxErrorDuplicateCalibrationAdjustmentInput,
  #[error("Self-calibration failed. Contact National Instruments technical support at ni.com/support.")]
  ErrorSelfCalFailedContactTechSupport =
    DAQmxErrorSelfCalFailedContactTechSupport,
  #[error("Self-calibration failed to converge. Performing an external calibration may fix the problem.")]
  ErrorSelfCalFailedToConverge = DAQmxErrorSelfCalFailedToConverge,
  #[error(
    "The simulated C Series module is not supported on this simulated chassis."
  )]
  ErrorUnsupportedSimulatedModuleForSimulatedChassis =
    DAQmxErrorUnsupportedSimulatedModuleForSimulatedChassis,
  #[error("The file write size specified is too large. Performance can suffer if the file write size is larger than one-fourth the size of the buffer length.")]
  ErrorLoggingWriteSizeTooBig = DAQmxErrorLoggingWriteSizeTooBig,
  #[error("The file write size specified is not evenly divisible by the volume sector size.")]
  ErrorLoggingWriteSizeNotDivisible = DAQmxErrorLoggingWriteSizeNotDivisible,
  #[error("One or more connections to external power rails are drawing too much power. The operation has been aborted to prevent the device from using too much power. Remove the connection(s) to the external power rails and restart your task.")]
  ErrorMyDAQPowerRailFault = DAQmxErrorMyDAQPowerRailFault,
  #[error("The requested operation is not supported by this device.")]
  ErrorDeviceDoesNotSupportThisOperation =
    DAQmxErrorDeviceDoesNotSupportThisOperation,
  #[error("Network devices are not supported on this platform.")]
  ErrorNetworkDevicesNotSupportedOnThisPlatform =
    DAQmxErrorNetworkDevicesNotSupportedOnThisPlatform,
  #[error("Firmware version requested was not found on the system.")]
  ErrorUnknownFirmwareVersion = DAQmxErrorUnknownFirmwareVersion,
  #[error("Device is unusable while firmware update is in progress.")]
  ErrorFirmwareIsUpdating = DAQmxErrorFirmwareIsUpdating,
  #[error("Data read from the EEPROM on the accessory attached to the device is invalid. Verify that any accessories configured with this device are connected. If the problem continues, contact National Instruments technical support. The device might need to be recalibrated or repaired by NI.")]
  ErrorAccessoryEEPROMIsCorrupt = DAQmxErrorAccessoryEEPROMIsCorrupt,
  #[error("Thermocouple lead offset nulling calibration is not supported by the specified channels. Make sure the device supports thermocouple lead offset nulling calibration. Make sure all channels are thermocouple channels. Make sure open thermocouple detection is enabled. Set skip unsupported channels to true.")]
  ErrorThrmcplLeadOffsetNullingCalNotSupported =
    DAQmxErrorThrmcplLeadOffsetNullingCalNotSupported,
  #[error("Self-calibration failed. Performing an external calibration may fix the problem.")]
  ErrorSelfCalFailedTryExtCal = DAQmxErrorSelfCalFailedTryExtCal,
  #[error("Output peer-to-peer streaming with the Stream statement is not allowed in a script with If Else, Repeat Until, or Break statements.")]
  ErrorOutputP2PNotSupportedWithMultithreadedScripts =
    DAQmxErrorOutputP2PNotSupportedWithMultithreadedScripts,
  #[error("Open thermocouple condition detected on thermocouple channel(s) you are attempting to calibrate. Ensure thermocouples are properly connected and functioning before performing lead offset nulling calibration.")]
  ErrorThrmcplCalibrationChannelsOpen =
    DAQmxErrorThrmcplCalibrationChannelsOpen,
  #[error(
    "Multicast DNS service instance specified is in use by another device."
  )]
  ErrorMDNSServiceInstanceAlreadyInUse =
    DAQmxErrorMDNSServiceInstanceAlreadyInUse,
  #[error("IP address specified is in use by another device.")]
  ErrorIPAddressAlreadyInUse = DAQmxErrorIPAddressAlreadyInUse,
  #[error("Hostname specified is in use by another device.")]
  ErrorHostnameAlreadyInUse = DAQmxErrorHostnameAlreadyInUse,
  #[error("Invalid number of calibration adjustment points provided. Use DAQmx Adjust Calibration to provide calibration points for the device.")]
  ErrorInvalidNumberOfCalAdjustmentPoints =
    DAQmxErrorInvalidNumberOfCalAdjustmentPoints,
  #[error("Filtering or digital synchronization of an internal signal is not supported by the device.")]
  ErrorFilterOrDigitalSyncInternalSignal =
    DAQmxErrorFilterOrDigitalSyncInternalSignal,
  #[error("Reference Clock signal was not found at the specified source.")]
  ErrorBadDDSSource = DAQmxErrorBadDDSSource,
  #[error("Onboard regeneration cannot be used when there are more than 16 channels in the task. Reduce the number of  channels in the task or set Use Only Onboard Memory to false.")]
  ErrorOnboardRegenWithMoreThan16Channels =
    DAQmxErrorOnboardRegenWithMoreThan16Channels,
  #[error("Trigger detected that could not be acted upon by the device. Slow down your trigger source.")]
  ErrorTriggerTooFast = DAQmxErrorTriggerTooFast,
  #[error("Minimum and maximum values specified are outside the bounds of the specified physical values for the table.  Change the table or the minimum and maximum value appropriately.")]
  ErrorMinMaxOutsideTableRange = DAQmxErrorMinMaxOutsideTableRange,
  #[error(
    "Analog reference trigger and sync pulse must come from the same device."
  )]
  ErrorChannelExpansionWithInvalidAnalogTriggerDevice =
    DAQmxErrorChannelExpansionWithInvalidAnalogTriggerDevice,
  #[error(
        "Sync pulse cannot originate from the specified device for this combination of devices."
    )]
  ErrorSyncPulseSrcInvalidForTask = DAQmxErrorSyncPulseSrcInvalidForTask,
  #[error("Carrier slot number specified for the card is invalid.")]
  ErrorInvalidCarrierSlotNumberSpecd = DAQmxErrorInvalidCarrierSlotNumberSpecd,
  #[error(
    "Multicard devices must consist of cards contained in the same carrier."
  )]
  ErrorCardsMustBeInSameCarrier = DAQmxErrorCardsMustBeInSameCarrier,
  #[error(
    "Simulation flag must be the same for the cards, devices, and carrier."
  )]
  ErrorCardDevCarrierSimMustMatch = DAQmxErrorCardDevCarrierSimMustMatch,
  #[error("Device must consist of at least one card.")]
  ErrorDevMustHaveAtLeastOneCard = DAQmxErrorDevMustHaveAtLeastOneCard,
  #[error("Topology specified is not supported by this card.")]
  ErrorCardTopologyError = DAQmxErrorCardTopologyError,
  #[error("Switch device has been disabled to prevent it from exceeding the power limit for the carrier. To recover, call DAQmx Disconnect All, or reset the device. The device can be reset either programmatically or by using Measurement & Automation Explorer. Refer to your device documentation for more information.")]
  ErrorExceededCarrierPowerLimit = DAQmxErrorExceededCarrierPowerLimit,
  #[error("Cards specified cannot be used to create a SwitchBlock device. Use the current set of cards as single-card devices, or use a different set of cards to create a valid combination.")]
  ErrorCardsIncompatible = DAQmxErrorCardsIncompatible,
  #[error("Analog bus needed by the current operation is invalid. Verify that all carriers connected using expansion bridge(s) are functioning properly.")]
  ErrorAnalogBusNotValid = DAQmxErrorAnalogBusNotValid,
  #[error("Analog bus line(s) in use by another device. Verify other devices are not currently using the analog bus line(s). If you intend to share the line(s) between devices, ensure the Analog Bus Sharing Enabled property is True for all shared channels.")]
  ErrorReservationConflict = DAQmxErrorReservationConflict,
  #[error("Memory mapping is not supported by this device for on-demand acquisitions.")]
  ErrorMemMappedOnDemandNotSupported = DAQmxErrorMemMappedOnDemandNotSupported,
  #[error("Synchronization Type cannot be Slave unless you configure the device to use a trigger with an external source. Configure an external trigger or set Synchronization Type to None or Master.")]
  ErrorSlaveWithNoStartTriggerConfigured =
    DAQmxErrorSlaveWithNoStartTriggerConfigured,
  #[error("Multidevice tasks using the specified devices do not support a Start Trigger and Reference Trigger from different devices. Using triggers from different devices can cause unwanted latency or incorrect behavior. Use triggers from a single device or manually synchronize the devices.")]
  ErrorChannelExpansionWithDifferentTriggerDevices =
    DAQmxErrorChannelExpansionWithDifferentTriggerDevices,
  #[error("Retriggered counter tasks do not support trigger skew correction. Set the Synchronization Type to Default or disable retriggering.")]
  ErrorCounterSyncAndRetriggered = DAQmxErrorCounterSyncAndRetriggered,
  #[error("Sync Pulse was not detected before attempting to start the task. Ensure you connected the source of the Sync Pulse to the device.")]
  ErrorNoExternalSyncPulseDetected = DAQmxErrorNoExternalSyncPulseDetected,
  #[error("Synchronization Type cannot be Slave without configuring the device to use an external Sync Pulse. Set the Synchronization Type to Master or configure the device to use an external Sync Pulse.")]
  ErrorSlaveAndNoExternalSyncPulse = DAQmxErrorSlaveAndNoExternalSyncPulse,
  #[error("Custom Timing Mode property is not supported unless the ADC Timing Mode property is set to Custom. Set ADC Timing Mode to Custom before setting Custom Timing Mode.")]
  ErrorCustomTimingRequiredForAttribute =
    DAQmxErrorCustomTimingRequiredForAttribute,
  #[error("ADC Timing Mode property was set to Custom, but the Custom Timing Mode property was not set. Specify a value for Custom Timing Mode or change the ADC Timing Mode.")]
  ErrorCustomTimingModeNotSet = DAQmxErrorCustomTimingModeNotSet,
  #[error("Overcurrent detected in the power supply for the accessory connected to the module. Check the external wiring and ensure the accessory, if present, is properly seated. Then, reset the module using DAQmx Reset Device or Measurement & Automation Explorer.")]
  ErrorAccessoryPowerTripped = DAQmxErrorAccessoryPowerTripped,
  #[error("Unsupported accessory is connected to the module. Insert a supported accessory and restart the task.")]
  ErrorUnsupportedAccessory = DAQmxErrorUnsupportedAccessory,
  #[error("Accessory cannot be connected while the task runs. The accessory uses different scaling or is unsupported. Ensure the accessory is seated properly and restart the task.")]
  ErrorInvalidAccessoryChange = DAQmxErrorInvalidAccessoryChange,
  #[error("Firmware on the device is out of date. Use Measurement & Automation Explorer to update the device firmware.")]
  ErrorFirmwareRequiresUpgrade = DAQmxErrorFirmwareRequiresUpgrade,
  #[error("External timebase rate specified is too fast for this device. Reduce the timebase rate to less than 1/4 the device oscillator frequency.")]
  ErrorFastExternalTimebaseNotSupportedForDevice =
    DAQmxErrorFastExternalTimebaseNotSupportedForDevice,
  #[error("Shunt resistor location specified is not valid for this calibration procedure.")]
  ErrorInvalidShuntLocationForCalibration =
    DAQmxErrorInvalidShuntLocationForCalibration,
  #[error("Device names must be 254 characters or shorter.")]
  ErrorDeviceNameTooLong = DAQmxErrorDeviceNameTooLong,
  #[error("Bridge scales are not supported for this measurement type.  Use a custom scale for additional scaling.")]
  ErrorBridgeScalesUnsupported = DAQmxErrorBridgeScalesUnsupported,
  #[error("Table scaling requires the same number of electrical values as physical values.")]
  ErrorMismatchedElecPhysValues = DAQmxErrorMismatchedElecPhysValues,
  #[error("Linear scaling requires unique electrical and physical values.")]
  ErrorLinearRequiresUniquePoints = DAQmxErrorLinearRequiresUniquePoints,
  #[error("Required scaling parameter has not been specified.")]
  ErrorMissingRequiredScalingParameter =
    DAQmxErrorMissingRequiredScalingParameter,
  #[error("Logging is not supported for output tasks.")]
  ErrorLoggingNotSupportOnOutputTasks =
    DAQmxErrorLoggingNotSupportOnOutputTasks,
  #[error("Memory mapping is not supported on this device for non-buffered tasks using Sample Clock timing. Disable memory mapping or change the buffer size.")]
  ErrorMemoryMappedHardwareTimedNonBufferedUnsupported =
    DAQmxErrorMemoryMappedHardwareTimedNonBufferedUnsupported,
  #[error("Pulse train specifications cannot be modified while the task is running if Auto Increment Count is greater than 0")]
  ErrorCannotUpdatePulseTrainWithAutoIncrementEnabled =
    DAQmxErrorCannotUpdatePulseTrainWithAutoIncrementEnabled,
  #[error("Data Transfer Mechanism must be DMA when Sample Mode is Hardware Timed Single Point on this device.")]
  ErrorHWTimedSinglePointAndDataXferNotDMA =
    DAQmxErrorHWTimedSinglePointAndDataXferNotDMA,
  #[error(
    "Second stage of a dual-stage analog input channel cannot be empty."
  )]
  ErrorSCCSecondStageEmpty = DAQmxErrorSCCSecondStageEmpty,
  #[error(
    "Modules specified are not valid for dual-stage analog input channels."
  )]
  ErrorSCCInvalidDualStageCombo = DAQmxErrorSCCInvalidDualStageCombo,
  #[error(
        "Module specified cannot be used as the second stage of a dual-stage analog input channel."
    )]
  ErrorSCCInvalidSecondStage = DAQmxErrorSCCInvalidSecondStage,
  #[error(
        "Module specified cannot be used as the first stage of a dual -stage analog input channel."
    )]
  ErrorSCCInvalidFirstStage = DAQmxErrorSCCInvalidFirstStage,
  #[error("Multiple counters are not allowed in a single counter output task when using Sample Clock timing.")]
  ErrorCounterMultipleSampleClockedChannels =
    DAQmxErrorCounterMultipleSampleClockedChannels,
  #[error("Sample Clock timing is not supported when using a two-counter measurement method.  Use a one-counter measurement method, or use a different timing type.")]
  Error2CounterMeasurementModeAndSampleClocked =
    DAQmxError2CounterMeasurementModeAndSampleClocked,
  #[error("Memory mapping must be the same setting for all simultaneous tasks that use channels from a single subsystem.")]
  ErrorCantHaveBothMemMappedAndNonMemMappedTasks =
    DAQmxErrorCantHaveBothMemMappedAndNonMemMappedTasks,
  #[error("Memory-mapped task detected data corruption because the memory was accessed by another program, such as a debugger or virus scanner. Disable other programs that might access this memory, or disable memory mapping for the task.")]
  ErrorMemMappedDataReadByAnotherProcess =
    DAQmxErrorMemMappedDataReadByAnotherProcess,
  #[error("Retriggering can only be enabled for finite task with a  Start trigger configured.")]
  ErrorRetriggeringInvalidForGivenSettings =
    DAQmxErrorRetriggeringInvalidForGivenSettings,
  #[error("Sample Clock pulse occurred before the previous sample was acquired from all channels in the task. Use a Sample Clock rate that allows time for the device to acquire samples from all channels. If you are using an external Sample Clock, ensure that clock signal is within the jitter and voltage level specifications and without glitches.")]
  ErrorAIOverrun = DAQmxErrorAIOverrun,
  #[error("Sample Clock pulse occurred before a pulse could be generated using the previous pulse specification. Use a Sample Clock that is slower than the pulse train you want to generate.")]
  ErrorCOOverrun = DAQmxErrorCOOverrun,
  #[error(
    "Multiple counters are not allowed in a buffered counter output task."
  )]
  ErrorCounterMultipleBufferedChannels =
    DAQmxErrorCounterMultipleBufferedChannels,
  #[error(
        "Timebase specified is too fast for a hardware-timed single-point counter output task."
    )]
  ErrorInvalidTimebaseForCOHWTSP = DAQmxErrorInvalidTimebaseForCOHWTSP,
  #[error("Events cannot be configured after writing samples to the task.")]
  ErrorWriteBeforeEvent = DAQmxErrorWriteBeforeEvent,
  #[error("Multiple Sample Clock pulses were detected within one period of the input signal. Use a Sample Clock rate that is slower than the input signal. If you are using an external Sample Clock, ensure that clock signal is within the jitter and voltage level specifications and without glitches.")]
  ErrorCIOverrun = DAQmxErrorCIOverrun,
  #[error("Non-responsive counter detected. NI-DAQmx reset the counter. Counter timebases at the specified speed must remain periodic, otherwise the counter can become non-responsive. Use an internal timebase or an external timebase that remains periodic.")]
  ErrorCounterNonResponsiveAndReset = DAQmxErrorCounterNonResponsiveAndReset,
  #[error("Logging is not supported for this channel and/or measurement type. Use a different measurement type or channel.")]
  ErrorMeasTypeOrChannelNotSupportedForLogging =
    DAQmxErrorMeasTypeOrChannelNotSupportedForLogging,
  #[error(
        "File specified is already opened for output. NI-DAQmx requires exclusive write access."
    )]
  ErrorFileAlreadyOpenedForWrite = DAQmxErrorFileAlreadyOpenedForWrite,
  #[error("TDMS support is not installed or is too old. Use an  NI-DAQmx runtime that includes TDMS support, or install a supported version of TDMS from a stand-alone installer.")]
  ErrorTdmsNotFound = DAQmxErrorTdmsNotFound,
  #[error("Unable to write to disk. Ensure that the file is accessible. If the problem persists, try logging to a different file.")]
  ErrorGenericFileIO = DAQmxErrorGenericFileIO,
  #[error("Logging is not supported for finite counter tasks on this device. Change the Sample Mode of the task to continuous.")]
  ErrorFiniteSTCCounterNotSupportedForLogging =
    DAQmxErrorFiniteSTCCounterNotSupportedForLogging,
  #[error("Logging is not supported for this measurement type.  Change the measurement type in order to use logging.")]
  ErrorMeasurementTypeNotSupportedForLogging =
    DAQmxErrorMeasurementTypeNotSupportedForLogging,
  #[error("File specified is already opened for output by another task. Specify a different file path.")]
  ErrorFileAlreadyOpened = DAQmxErrorFileAlreadyOpened,
  #[error(
    "Disk is full. Free up disk space, or specify a different file path."
  )]
  ErrorDiskFull = DAQmxErrorDiskFull,
  #[error("File path specified is invalid, or the file is not a valid TDMS file. Specify the location of a valid TDMS file.")]
  ErrorFilePathInvalid = DAQmxErrorFilePathInvalid,
  #[error("Logging to this file format is not supported by NI-DAQmx. Convert the file to TDMS version 2.0 or later, or specify a different file.")]
  ErrorFileVersionMismatch = DAQmxErrorFileVersionMismatch,
  #[error("File permission error. You do not have the correct permissions for the file.")]
  ErrorFileWriteProtected = DAQmxErrorFileWriteProtected,
  #[error("Reading samples is not supported in the specified logging mode. To access the samples while logging, either open the file being written to or use a different logging mode.")]
  ErrorReadNotSupportedForLoggingMode =
    DAQmxErrorReadNotSupportedForLoggingMode,
  #[error("Property specified is not supported when logging data.")]
  ErrorAttributeNotSupportedWhenLogging =
    DAQmxErrorAttributeNotSupportedWhenLogging,
  #[error("Log Only mode is only supported for buffered tasks. Either use Log and Read mode or configure the task as Finite or Continuous.")]
  ErrorLoggingModeNotSupportedNonBuffered =
    DAQmxErrorLoggingModeNotSupportedNonBuffered,
  #[error("Property specified is not supported in conjunction with a conflicting property.")]
  ErrorPropertyNotSupportedWithConflictingProperty =
    DAQmxErrorPropertyNotSupportedWithConflictingProperty,
  #[error("Sample and hold is not supported for SCXI modules in parallel mode using digitizer channels other than 0 through 7. Disable sample and hold, use only digitizer channels 0 through 7, or use multiplexed mode.")]
  ErrorParallelSSHOnConnector1 = DAQmxErrorParallelSSHOnConnector1,
  #[error("Sample Timing Type specified is not supported for counter output tasks on this device. Change the Sample Timing Type to Implicit.")]
  ErrorCOOnlyImplicitSampleTimingTypeSupported =
    DAQmxErrorCOOnlyImplicitSampleTimingTypeSupported,
  #[error("Device could not complete the calibration operation. Calibration could fail for the following reasons: 1. The actual reference signal applied for calibration was different from the value you specified. Ensure that the reference signal applied is the same as the values that were input. 2. The reference signal was not stable over the period of time that the hardware was being calibrated. Ensure that the reference signal specified is free of noise and does not drift over the duration of the calibration. 3. The device is not functioning properly.")]
  ErrorCalibrationFailedAOOutOfRange = DAQmxErrorCalibrationFailedAOOutOfRange,
  #[error("Device could not complete the calibration operation. Calibration could fail for the following reasons: 1. The actual reference signal applied for calibration was different from the value you specified. Ensure that the reference signal applied is the same as the values that were input. 2. The reference signal was not stable over the period of time that the hardware was being calibrated. Ensure that the reference signal specified is free of noise and does not drift over the duration of the calibration. 3. The device is not functioning properly.")]
  ErrorCalibrationFailedAIOutOfRange = DAQmxErrorCalibrationFailedAIOutOfRange,
  #[error("Device could not complete the calibration operation. Calibration could fail for the following reasons: 1. The actual reference signal applied for calibration was different from the value you specified. Ensure that the reference signal applied is the same as the values that were input. 2. The reference signal was not stable over the period of time that the hardware was being calibrated. Ensure that the reference signal specified is free of noise and does not drift over the duration of the calibration. 3. The device is not functioning properly.")]
  ErrorCalPWMLinearityFailed = DAQmxErrorCalPWMLinearityFailed,
  #[error("Sample Clock Overrun and Underflow Behaviors must be set to consistent values. Either set Overrun Behavior to Stop Task and  Error = DAQmxError and Underflow Behavior to Halt Output and  Error = DAQmxError, or  set Overrun Behavior to Ignore Overruns and Underflow Behavior to Pause Until Data Available.")]
  ErrorOverrunUnderflowConfigurationCombo =
    DAQmxErrorOverrunUnderflowConfigurationCombo,
  #[error(
        "Pulse specifications cannot be written to a finite counter output task on this device."
    )]
  ErrorCannotWriteToFiniteCOTask = DAQmxErrorCannotWriteToFiniteCOTask,
  #[error("WEP key must be either 10 or 26 characters long.")]
  ErrorNetworkDAQInvalidWEPKeyLength = DAQmxErrorNetworkDAQInvalidWEPKeyLength,
  #[error("Calibration procedure for your device does not support shorted inputs. Refer to the calibration procedure for your device for more information.")]
  ErrorCalInputsShortedNotSupported = DAQmxErrorCalInputsShortedNotSupported,
  #[error("Property specified cannot be set while the task is reserved. Set the property prior to reserving the task or unreserve the task prior to setting the property.")]
  ErrorCannotSetPropertyWhenTaskIsReserved =
    DAQmxErrorCannotSetPropertyWhenTaskIsReserved,
  #[error("Improper chassis power levels detected. The -12 V fuse on the device might have blown, or there might be a problem with the -12 V rail on the chassis. Try resetting the device. If the error persists, contact National Instruments.")]
  ErrorMinus12VFuseBlown = DAQmxErrorMinus12VFuseBlown,
  #[error("Improper chassis power levels detected. The +12 V fuse on the device might have blown, or there might be a problem with the +12 V rail on the chassis. Try resetting the device. If the error persists, contact National Instruments.")]
  ErrorPlus12VFuseBlown = DAQmxErrorPlus12VFuseBlown,
  #[error("Improper chassis power levels detected. The +5 V fuse on the device might have blown, or there might be a problem with the +5 V rail on the chassis. Try resetting the device. If the error persists, contact National Instruments.")]
  ErrorPlus5VFuseBlown = DAQmxErrorPlus5VFuseBlown,
  #[error("Improper chassis power levels detected. The +3.3 V fuse on the device might have blown, or there might be a problem with the +3.3 V rail on the chassis. Try resetting the device. If the error persists, contact National Instruments.")]
  ErrorPlus3VFuseBlown = DAQmxErrorPlus3VFuseBlown,
  #[error("Internal serial communication bus failed. Try resetting the device.  If the error persists, contact National Instruments.")]
  ErrorDeviceSerialPortError = DAQmxErrorDeviceSerialPortError,
  #[error("Device power up failed. Try resetting the device. If the error persists contact National Instruments.")]
  ErrorPowerUpStateMachineNotDone = DAQmxErrorPowerUpStateMachineNotDone,
  #[error("Device does not support using more than one trigger at a time. Configure the device to use only one trigger, or use a device that supports using multiple triggers.")]
  ErrorTooManyTriggersSpecifiedInTask =
    DAQmxErrorTooManyTriggersSpecifiedInTask,
  #[error("Vertical Offset is not supported by this device.")]
  ErrorVerticalOffsetNotSupportedOnDevice =
    DAQmxErrorVerticalOffsetNotSupportedOnDevice,
  #[error("Coupling specified conflicts with the Measurement Type of the channel. Configure the channel to use a coupling appropriate for the measurement and sensor. For example, use DC coupling for DC sensors.")]
  ErrorInvalidCouplingForMeasurementType =
    DAQmxErrorInvalidCouplingForMeasurementType,
  #[error("Consecutive writes to a digital line occurred more frequently than the device can safely allow.")]
  ErrorDigitalLineUpdateTooFastForDevice =
    DAQmxErrorDigitalLineUpdateTooFastForDevice,
  #[error("Certificate file is too large to transfer to the device.")]
  ErrorCertificateIsTooBigToTransfer = DAQmxErrorCertificateIsTooBigToTransfer,
  #[error("Certificate provided is not in PEM (Privacy-enhanced Electronic Mail) or DER (Distinguished Encoding Rules) format.  Only PEM or DER certificates are accepted.")]
  ErrorOnlyPEMOrDERCertiticatesAccepted =
    DAQmxErrorOnlyPEMOrDERCertiticatesAccepted,
  #[error("Device cannot be calibrated using the coupling specified. Calibrate using a different coupling mode.")]
  ErrorCalCouplingNotSupported = DAQmxErrorCalCouplingNotSupported,
  #[error("Device specified is not supported in 64-bit applications. To use this device, configure your development environment to create a 32-bit application, or use a 32-bit development environment. Refer to the documentation for your development environment for more information.")]
  ErrorDeviceNotSupportedIn64Bit = DAQmxErrorDeviceNotSupportedIn64Bit,
  #[error("Network device is already in use by another host.")]
  ErrorNetworkDeviceInUse = DAQmxErrorNetworkDeviceInUse,
  #[error("IP address provided is invalid.  IP addresses must be of the form x.x.x.x where x is a number from 0 to 255.")]
  ErrorInvalidIPv4AddressFormat = DAQmxErrorInvalidIPv4AddressFormat,
  #[error("Product at the address provided was not the expected type. This may be due to a module being replaced or IP addresses on the network being reassigned.  Reconnect to the device in MAX or delete it from your system and rediscover it.")]
  ErrorNetworkProductTypeMismatch = DAQmxErrorNetworkProductTypeMismatch,
  #[error("Certificate provided is not in PEM (Privacy Enhanced Mail) format.  Only PEM certificates are supported.")]
  ErrorOnlyPEMCertificatesAccepted = DAQmxErrorOnlyPEMCertificatesAccepted,
  #[error(
        "Calibration operation cannot be completed unless the prototyping board is powered on."
    )]
  ErrorCalibrationRequiresPrototypingBoardEnabled =
    DAQmxErrorCalibrationRequiresPrototypingBoardEnabled,
  #[error("Current limit specified cannot be applied to the channel because all current limit resources on the device have been reserved. Use a current limit setting that is already in use for another channel, or free a current limit resource by disabling current limiting on all channels that use a common current limit.")]
  ErrorAllCurrentLimitingResourcesAlreadyTaken =
    DAQmxErrorAllCurrentLimitingResourcesAlreadyTaken,
  #[error("User defined information string entered is of an invalid length.")]
  ErrorUserDefInfoStringBadLength = DAQmxErrorUserDefInfoStringBadLength,
  #[error("Property requested was not found. The property is either not supported by the object or has not been set.")]
  ErrorPropertyNotFound = DAQmxErrorPropertyNotFound,
  #[error("Input voltage limits exceeded. Protection circuity disabled the inputs. Ensure proper voltage levels on device inputs.")]
  ErrorOverVoltageProtectionActivated =
    DAQmxErrorOverVoltageProtectionActivated,
  #[error("Scaled waveform is too large. After multiplication by the software scaling factor, the magnitude of each sample must be less than 1.0.")]
  ErrorScaledIQWaveformTooLarge = DAQmxErrorScaledIQWaveformTooLarge,
  #[error("Firmware for this device could not be downloaded. To retry downloading the firmware, unplug the device and plug it back in. If this problem continues, contact National Instruments for assistance.")]
  ErrorFirmwareFailedToDownload = DAQmxErrorFirmwareFailedToDownload,
  #[error(
    "Property specified is not supported for the bus type of the device."
  )]
  ErrorPropertyNotSupportedForBusType =
    DAQmxErrorPropertyNotSupportedForBusType,
  #[error("Sample clock rate cannot be changed at this time. When changing the Sample clock rate for a running task, one full period of the Sample clock must complete at the previous rate before NI-DAQmx can safely update the timing circuitry.")]
  ErrorChangeRateWhileRunningCouldNotBeCompleted =
    DAQmxErrorChangeRateWhileRunningCouldNotBeCompleted,
  #[error("Manual control attribute cannot be read when manual control is disabled. Enable manual control before reading this attribute.")]
  ErrorCannotQueryManualControlAttribute =
    DAQmxErrorCannotQueryManualControlAttribute,
  #[error("Network configuration has been rejected by the device.")]
  ErrorInvalidNetworkConfiguration = DAQmxErrorInvalidNetworkConfiguration,
  #[error("Wireless configuration has been rejected by the device.")]
  ErrorInvalidWirelessConfiguration = DAQmxErrorInvalidWirelessConfiguration,
  #[error("Country code is not configured. This setting is required to determine available wireless channels.")]
  ErrorInvalidWirelessCountryCode = DAQmxErrorInvalidWirelessCountryCode,
  #[error("Wireless channel specified is not available for this country code configuration.")]
  ErrorInvalidWirelessChannel = DAQmxErrorInvalidWirelessChannel,
  #[error("EEPROM of this device has changed since this task began. Restart the task or reset the device to refresh the EEPROM contents.")]
  ErrorNetworkEEPROMHasChanged = DAQmxErrorNetworkEEPROMHasChanged,
  #[error("Serial numbers of the network device do not match the serial numbers NI-DAQmx expected. Replace the original device or module and try again, or use the reconnect button in MAX to locate the original device.")]
  ErrorNetworkSerialNumberMismatch = DAQmxErrorNetworkSerialNumberMismatch,
  #[error("Network is currently unavailable. This usually indicates an unplugged network cable, a failing network component, or an improperly configured network.")]
  ErrorNetworkStatusDown = DAQmxErrorNetworkStatusDown,
  #[error(
        "Device cannot be reached because no known route to the device exists on the network."
    )]
  ErrorNetworkTargetUnreachable = DAQmxErrorNetworkTargetUnreachable,
  #[error("Device could not be found on the network. This usually indicates an incorrect hostname or a DNS failure.")]
  ErrorNetworkTargetNotFound = DAQmxErrorNetworkTargetNotFound,
  #[error("Connection to the network device has timed out. The network device did not respond properly for a period of time. If timeouts persist, contact your system administrator.")]
  ErrorNetworkStatusTimedOut = DAQmxErrorNetworkStatusTimedOut,
  #[error(
        "Wireless security setting is invalid. Ensure that all necessary settings are specified."
    )]
  ErrorInvalidWirelessSecuritySelection =
    DAQmxErrorInvalidWirelessSecuritySelection,
  #[error("Device configuration may not be changed at this time because the device is locked.")]
  ErrorNetworkDeviceConfigurationLocked =
    DAQmxErrorNetworkDeviceConfigurationLocked,
  #[error("Device cannot be created in MAX because a driver could not be found for the device.  You may need to upgrade NI-DAQmx.")]
  ErrorNetworkDAQDeviceNotSupported = DAQmxErrorNetworkDAQDeviceNotSupported,
  #[error("Device cannot be created in MAX because the carrier contains no cartridge.  Plug in a cartridge and attempt to create the device again.")]
  ErrorNetworkDAQCannotCreateEmptySleeve =
    DAQmxErrorNetworkDAQCannotCreateEmptySleeve,
  #[error(
        "User defined information string entered exceeds the maximum allowable string length."
    )]
  ErrorUserDefInfoStringTooLong = DAQmxErrorUserDefInfoStringTooLong,
  #[error(
        "Module type in the source storage does not match the module type in the destination."
    )]
  ErrorModuleTypeDoesNotMatchModuleTypeInDestination =
    DAQmxErrorModuleTypeDoesNotMatchModuleTypeInDestination,
  #[error("Address specified is invalid.")]
  ErrorInvalidTEDSInterfaceAddress = DAQmxErrorInvalidTEDSInterfaceAddress,
  #[error("SCXI chassis communication is not supported by the device.")]
  ErrorDevDoesNotSupportSCXIComm = DAQmxErrorDevDoesNotSupportSCXIComm,
  #[error(
        "Connector 0 on the SCXI chassis communication device must be  cabled to the SCXI module."
    )]
  ErrorSCXICommDevConnector0MustBeCabledToModule =
    DAQmxErrorSCXICommDevConnector0MustBeCabledToModule,
  #[error("Digitization mode specified is not supported by the SCXI module.")]
  ErrorSCXIModuleDoesNotSupportDigitizationMode =
    DAQmxErrorSCXIModuleDoesNotSupportDigitizationMode,
  #[error("SCXI multiplexed digitization is not supported by the device.")]
  ErrorDevDoesNotSupportMultiplexedSCXIDigitizationMode =
    DAQmxErrorDevDoesNotSupportMultiplexedSCXIDigitizationMode,
  #[error(
    "SCXI digitization is not supported by the device or physical channel."
  )]
  ErrorDevOrDevPhysChanDoesNotSupportSCXIDigitization =
    DAQmxErrorDevOrDevPhysChanDoesNotSupportSCXIDigitization,
  #[error("Physical channel name specified is invalid. Physical channel names are of the form <device name>/<physical channel name>, for example, dev1/ai0.")]
  ErrorInvalidPhysChanName = DAQmxErrorInvalidPhysChanName,
  #[error("Communication mode specified is not valid for the SCXI Chassis.")]
  ErrorSCXIChassisCommModeInvalid = DAQmxErrorSCXIChassisCommModeInvalid,
  #[error("Required object dependency was not found in storage.")]
  ErrorRequiredDependencyNotFound = DAQmxErrorRequiredDependencyNotFound,
  #[error("Storage specified is not valid or could not be found.")]
  ErrorInvalidStorage = DAQmxErrorInvalidStorage,
  #[error("Object specified could not be found in storage.")]
  ErrorInvalidObject = DAQmxErrorInvalidObject,
  #[error("Target storage was altered by another  process before the changes could be saved.")]
  ErrorStorageAlteredPriorToSave = DAQmxErrorStorageAlteredPriorToSave,
  #[error("Local channel is not referenced by the task specified in the local channel name.")]
  ErrorTaskDoesNotReferenceLocalChannel =
    DAQmxErrorTaskDoesNotReferenceLocalChannel,
  #[error("Simulation flag of the referenced device does not match the simulation flag of the target.")]
  ErrorReferencedDevSimMustMatchTarget =
    DAQmxErrorReferencedDevSimMustMatchTarget,
  #[error("Memory mapping for programmed I/O cannot be enabled for the specified lines because a watchdog timer task uses these lines. Set DO.MemMapEnable to false for all lines in the task, or do not use a watchdog timer task.")]
  ErrorProgrammedIOFailsBecauseOfWatchdogTimer =
    DAQmxErrorProgrammedIOFailsBecauseOfWatchdogTimer,
  #[error("Watchdog timer task could not be created because one of the digital output lines in the task uses memory mapping for programmed I/O. Set DO.MemMapEnable to false for all lines in the task, or do not use a watchdog timer task.")]
  ErrorWatchdogTimerFailsBecauseOfProgrammedIO =
    DAQmxErrorWatchdogTimerFailsBecauseOfProgrammedIO,
  #[error("Timing engine requested can only be used with lines that span two contiguous ports. Use the default timing engine for the specified physical channels, use some lines from two contiguous ports, or use all of the physical data channels on the device.")]
  ErrorCantUseThisTimingEngineWithAPort =
    DAQmxErrorCantUseThisTimingEngineWithAPort,
  #[error("Memory Mapping for Programmed IO Enable setting must be the same for all virtual channels in the task.")]
  ErrorProgrammedIOConflict = DAQmxErrorProgrammedIOConflict,
  #[error("Change detection timing cannot be used on this device while Memory Mapping for Programmed IO is enabled. Set MemMapEnable to false or use a different timing type.")]
  ErrorChangeDetectionIncompatibleWithProgrammedIO =
    DAQmxErrorChangeDetectionIncompatibleWithProgrammedIO,
  #[error("Tristate setting must be applied to all lines in the port. Include all lines in the port in the Active Channel list.")]
  ErrorTristateNotEnoughLines = DAQmxErrorTristateNotEnoughLines,
  #[error("Tristate setting must be identical for all lines in the port.")]
  ErrorTristateConflict = DAQmxErrorTristateConflict,
  #[error("\"Generate\" or finite \"wait\" instruction expected before a \"break\" block.")]
  ErrorGenerateOrFiniteWaitExpectedBeforeBreakBlock =
    DAQmxErrorGenerateOrFiniteWaitExpectedBeforeBreakBlock,
  #[error("\"Break\" blocks are not allowed in finite or conditional loops.")]
  ErrorBreakBlockNotAllowedInLoop = DAQmxErrorBreakBlockNotAllowedInLoop,
  #[error("\"Clear trigger\" instruction not allowed in a \"break\" block.")]
  ErrorClearTriggerNotAllowedInBreakBlock =
    DAQmxErrorClearTriggerNotAllowedInBreakBlock,
  #[error("\"Break\" block cannot be nested in other \"break\" blocks.")]
  ErrorNestingNotAllowedInBreakBlock = DAQmxErrorNestingNotAllowedInBreakBlock,
  #[error("\"If-Else\" block not allowed in a \"break\" block.")]
  ErrorIfElseBlockNotAllowedInBreakBlock =
    DAQmxErrorIfElseBlockNotAllowedInBreakBlock,
  #[error(
    "\"Repeat until trigger\" instruction not allowed in a \"break\" block."
  )]
  ErrorRepeatUntilTriggerLoopNotAllowedInBreakBlock =
    DAQmxErrorRepeatUntilTriggerLoopNotAllowedInBreakBlock,
  #[error(
    "\"Wait until trigger\" instruction not allowed in a \"break\" block."
  )]
  ErrorWaitUntilTriggerNotAllowedInBreakBlock =
    DAQmxErrorWaitUntilTriggerNotAllowedInBreakBlock,
  #[error("Marker position specified is either too close to the end or the beginning of the waveform, or too close to another marker in the \"generate\" statement in \"break\" block.")]
  ErrorMarkerPosInvalidInBreakBlock = DAQmxErrorMarkerPosInvalidInBreakBlock,
  #[error("Wait duration is too small for the \"wait\" instruction in \"break\" block.")]
  ErrorInvalidWaitDurationInBreakBlock =
    DAQmxErrorInvalidWaitDurationInBreakBlock,
  #[error(
        "Waveform subset length is too small for the \"generate\" instruction in \"break\" block."
    )]
  ErrorInvalidSubsetLengthInBreakBlock =
    DAQmxErrorInvalidSubsetLengthInBreakBlock,
  #[error("Waveform length is too small for the \"generate\" instruction in \"break\" block.")]
  ErrorInvalidWaveformLengthInBreakBlock =
    DAQmxErrorInvalidWaveformLengthInBreakBlock,
  #[error("Wait duration is too small for the \"wait\" instruction before \"break\" block.")]
  ErrorInvalidWaitDurationBeforeBreakBlock =
    DAQmxErrorInvalidWaitDurationBeforeBreakBlock,
  #[error("Waveform subset length is too small for the \"generate\" instruction before \"break\" block.")]
  ErrorInvalidSubsetLengthBeforeBreakBlock =
    DAQmxErrorInvalidSubsetLengthBeforeBreakBlock,
  #[error(
        "Waveform length is too small for the \"generate\" instruction before \"break\" block."
    )]
  ErrorInvalidWaveformLengthBeforeBreakBlock =
    DAQmxErrorInvalidWaveformLengthBeforeBreakBlock,
  #[error("Sample rate specified is too fast for the ADC Timing Mode selected for this device. Decrease the sample rate or use a faster ADC Timing Mode.")]
  ErrorSampleRateTooHighForADCTimingMode =
    DAQmxErrorSampleRateTooHighForADCTimingMode,
  #[error("Active Device cannot be specified when reading or writing timing properties in this multidevice task, due to synchronization requirements.")]
  ErrorActiveDevNotSupportedWithMultiDevTask =
    DAQmxErrorActiveDevNotSupportedWithMultiDevTask,
  #[error("Task cannot contain a mixture of simulated devices and physical devices. Ensure the physical channels added to the task refer to all physical devices or all simulated devices.")]
  ErrorRealDevAndSimDevNotSupportedInSameTask =
    DAQmxErrorRealDevAndSimDevNotSupportedInSameTask,
  #[error("Device simulation flag does not match the simulation flag of the RTSI cable.")]
  ErrorRTSISimMustMatchDevSim = DAQmxErrorRTSISimMustMatchDevSim,
  #[error("Virtual channel specified does not support the Wheatstone bridge shunt calibration procedure.")]
  ErrorBridgeShuntCaNotSupported = DAQmxErrorBridgeShuntCaNotSupported,
  #[error(
        "Virtual channel specified does not support the strain gage shunt calibration procedure."
    )]
  ErrorStrainShuntCaNotSupported = DAQmxErrorStrainShuntCaNotSupported,
  #[error("Gain error measured for this calibration task is out of  range for the device. Ensure that the reference voltage is accurate, specified correctly, and connected to the correct channel. Also ensure that the measured output voltage is specified correctly and that the device is functioning properly.")]
  ErrorGainTooLargeForGainCalConst = DAQmxErrorGainTooLargeForGainCalConst,
  #[error("Offset error measured for this calibration task is out of range for the device. Ensure that the reference voltage is accurate, specified correctly, and connected to the correct channel. Also ensure that the measured output voltage is specified correctly and that the device is functioning properly.")]
  ErrorOffsetTooLargeForOffsetCalConst =
    DAQmxErrorOffsetTooLargeForOffsetCalConst,
  #[error("Requested operation could not be completed because the prototyping board has been removed or disabled.  The prototyping board can be disabled by either switching the prototyping board switch to the off position, or by an incorrect connection on the prototyping board causing too much power to be drawn from the device. Ensure that all connections on the prototyping board are correct and that the prototyping board is properly inserted and powered on before attempting the operation.")]
  ErrorElvisPrototypingBoardRemoved = DAQmxErrorElvisPrototypingBoardRemoved,
  #[error("Circuit connected to the prototyping board causes too much power to be drawn from the specified source.  The output of the source has been suspended to prevent the device from using too much power. Turn the prototyping board power switch to the off position; correct the circuit connected to the prototyping board; then turn the prototyping board power switch back on.")]
  ErrorElvis2PowerRailFault = DAQmxErrorElvis2PowerRailFault,
  #[error("Circuit connected to the prototyping board causes a short between the specified physical channel and the voltage source. The output of the function generator has been suspended to prevent the device from using too much power. Turn the prototyping board power switch to the off position; correct the circuit connected to the prototyping board; then turn the prototyping board power switch back on.")]
  ErrorElvis2PhysicalChansFault = DAQmxErrorElvis2PhysicalChansFault,
  #[error("Circuit connected to the prototyping board causes channels to use too much power.  Those channels were disabled to prevent the device from using too much power. Turn the prototyping board power switch to the off position; correct the circuit connected to the prototyping board; then turn the prototyping board power switch back on.")]
  ErrorElvis2PhysicalChansThermalEvent =
    DAQmxErrorElvis2PhysicalChansThermalEvent,
  #[error(
        "PCI Express interface layer error detected. Contact National Instruments for support."
    )]
  ErrorRXBitErrorRateLimitExceeded = DAQmxErrorRXBitErrorRateLimitExceeded,
  #[error(
        "PCI Express interface layer error detected. Contact National Instruments for support."
    )]
  ErrorPHYBitErrorRateLimitExceeded = DAQmxErrorPHYBitErrorRateLimitExceeded,
  #[error("Property cannot be read before reading the corresponding Channels Exist property. NI-DAQmx retrieves the channel state from the hardware when the application reads the corresponding Channels Exist property. After reading the corresponding Channels Exist property, you can retrieve other information about these channels.")]
  Error2PartAttributeCalledOutOfOrder =
    DAQmxErrorTwoPartAttributeCalledOutOfOrder,
  #[error("SCXI chassis address specified is invalid. Specify an SCXI chassis address between 0 and 31.")]
  ErrorInvalidSCXIChassisAddress = DAQmxErrorInvalidSCXIChassisAddress,
  #[error("Connection to target failed for the requested configuration operation. Confirm that NI-DAQmx is installed on the target.")]
  ErrorCouldNotConnectToRemoteMXS = DAQmxErrorCouldNotConnectToRemoteMXS,
  #[error("Property specified is not supported unless excitation is enabled. Enable excitation before attempting to access the property.")]
  ErrorExcitationStateRequiredForAttributes =
    DAQmxErrorExcitationStateRequiredForAttributes,
  #[error("Device is currently not usable and must be placed into firmware loader mode. Unplug the device USB cable and plug it back in. If the device is plugged into a USB hub, ensure that you unplug the device from the hub.")]
  ErrorDeviceNotUsableUntilUSBReplug = DAQmxErrorDeviceNotUsableUntilUSBReplug,
  #[error("Onboard device memory overflow. Because of system and/or bus bandwidth limitations, the driver could not read data from the device fast enough to keep up with the device throughput. This device supports high-speed (480Mb/s) USB but it is connected to a full-speed (12 Mb/s) USB port. Connect this device to a high-speed (480 Mb/s) USB port, reduce the number of  programs your computer is executing concurrently, or use a different computer to calibrate your device. If you are using a USB hub, ensure that it supports high-speed operation.")]
  ErrorInputFIFOOverflowDuringCalibrationOnFullSpeedUSB =
    DAQmxErrorInputFIFOOverflowDuringCalibrationOnFullSpeedUSB,
  #[error("Onboard device memory overflow. Because of system and/or bus bandwidth limitations, the driver could not read data from the device fast enough to keep up with the device throughput. Reduce the number of programs your computer is executing concurrently or use a different computer to calibrate your device.")]
  ErrorInputFIFOOverflowDuringCalibration =
    DAQmxErrorInputFIFOOverflowDuringCalibration,
  #[error("Cold junction compensation channel cannot be used unless the corresponding analog input channel is configured to measure temperature using a thermocouple. Use the thermocouple version of the DAQmx Create Channel VI/function to configure the channel.")]
  ErrorCJCChanConflictsWithNonThermocoupleChan =
    DAQmxErrorCJCChanConflictsWithNonThermocoupleChan,
  #[error("Device specified for PXI backplane communication is not in the rightmost slot of the PXI chassis.")]
  ErrorCommDeviceForPXIBackplaneNotInRightmostSlot =
    DAQmxErrorCommDeviceForPXIBackplaneNotInRightmostSlot,
  #[error("Device specified for PXI backplane communication is not in the PXI chassis.")]
  ErrorCommDeviceForPXIBackplaneNotInSameChassis =
    DAQmxErrorCommDeviceForPXIBackplaneNotInSameChassis,
  #[error(
    "Device specified for PXI backplane communication is not a PXI device."
  )]
  ErrorCommDeviceForPXIBackplaneNotPXI =
    DAQmxErrorCommDeviceForPXIBackplaneNotPXI,
  #[error(
    "Internal excitation frequency selected for calibration is not valid."
  )]
  ErrorInvalidCalExcitFrequency = DAQmxErrorInvalidCalExcitFrequency,
  #[error(
    "Internal excitation voltage selected for calibration is not valid."
  )]
  ErrorInvalidCalExcitVoltage = DAQmxErrorInvalidCalExcitVoltage,
  #[error("Input source in not valid. Ensure that AI Input Source and AI Coupling are not both set for the same task.")]
  ErrorInvalidAIInputSrc = DAQmxErrorInvalidAIInputSrc,
  #[error("Input cal data point must be an AC Voltage for this module.")]
  ErrorInvalidCalInputRef = DAQmxErrorInvalidCalInputRef,
  #[error("db reference value must be greater than zero.")]
  ErrordBReferenceValueNotGreaterThanZero =
    DAQmxErrordBReferenceValueNotGreaterThanZero,
  #[error("Sample clock rate specified is too fast for the sample clock timing type. Change the sample clock rate to be equal to or less than the maximum value or consider using the pipelined sample clock timing type. If you use the pipelined sample clock timing type, refer to device documentation for the differences between the sample clock timing type and the pipelined sample clock timing type.")]
  ErrorSampleClockRateIsTooFastForSampleClockTiming =
    DAQmxErrorSampleClockRateIsTooFastForSampleClockTiming,
  #[error("Device is not usable. The firmware was recently upgraded, and the system was not powered down and restarted. Power down the computer and restart.")]
  ErrorDeviceNotUsableUntilColdStart = DAQmxErrorDeviceNotUsableUntilColdStart,
  #[error("Sample clock rate specified is too fast for the burst handshaking timing type. Change the sample clock rate to be equal to or less than the maximum value or consider using the pipelined sample clock timing type. If you use the pipelined sample clock timing type, refer to the device documentation for the differences between the burst handshake timing type and the pipelined sample clock timing type.")]
  ErrorSampleClockRateIsTooFastForBurstTiming =
    DAQmxErrorSampleClockRateIsTooFastForBurstTiming,
  #[error("Dev.AssociatedResourceIDs property is not supported by the device. Remove the Dev.AssociatedResourceIDs property from the import file.")]
  ErrorDevImportFailedAssociatedResourceIDsNotSupported =
    DAQmxErrorDevImportFailedAssociatedResourceIDsNotSupported,
  #[error("SCXI-1600 does not support import through MAX. Deselect the SCXI-1600 in the import dialog or remove it from the import file.")]
  ErrorSCXI1600ImportNotSupported = DAQmxErrorSCXI1600ImportNotSupported,
  #[error("Power supply configuration failed. Reboot or cycle the power on the device.")]
  ErrorPowerSupplyConfigurationFailed =
    DAQmxErrorPowerSupplyConfigurationFailed,
  #[error("AI channels on this device do not support using DC coupling while using IEPE excitation. Set excitation source to none or the coupling mode to AC.")]
  ErrorIEPEWithDCNotAllowed = DAQmxErrorIEPEWithDCNotAllowed,
  #[error("Minimum temperature specified for the thermocouple measurement falls outside of the accuracy limit when using polynomial scaling. Specify a value greater than the minumum temperature for  polynomial scaling with this thermocouple type, or set the Thermocouple Scale Type property to Table.")]
  ErrorMinTempForThermocoupleTypeOutsideAccuracyForPolyScaling =
    DAQmxErrorMinTempForThermocoupleTypeOutsideAccuracyForPolyScaling,
  #[error("Device import failed because the device does not support simulation and a device to overwrite could not be found. Change the device type or do not import this device.")]
  ErrorDevImportFailedNoDeviceToOverwriteAndSimulationNotSupported =
    DAQmxErrorDevImportFailedNoDeviceToOverwriteAndSimulationNotSupported,
  #[error("Device import failed because the device is not supported by the installed version and/or platform of NI-DAQmx. Change the device type or do not import this device.")]
  ErrorDevImportFailedDeviceNotSupportedOnDestination =
    DAQmxErrorDevImportFailedDeviceNotSupportedOnDestination,
  #[error("Firmware for this device is too old. Upgrade the firmware for this device.")]
  ErrorFirmwareIsTooOld = DAQmxErrorFirmwareIsTooOld,
  #[error("Firmware for this device could not be updated. Contact National Instruments for help with this device.")]
  ErrorFirmwareCouldntUpdate = DAQmxErrorFirmwareCouldntUpdate,
  #[error("Firmware for this device is corrupt. Contact National Instruments for help with this device.")]
  ErrorFirmwareIsCorrupt = DAQmxErrorFirmwareIsCorrupt,
  #[error("Firmware for this device is too new. Downgrade the firmware for this device. If you need help downgrading, visit ni.com/support.")]
  ErrorFirmwareTooNew = DAQmxErrorFirmwareTooNew,
  #[error("Sample clock cannot be exported in this mode when the sample clock comes from an external source or an external timebase source.")]
  ErrorSampClockCannotBeExportedFromExternalSampClockSrc =
    DAQmxErrorSampClockCannotBeExportedFromExternalSampClockSrc,
  #[error("Physical channel cannot be used by the task because an input task has reserved this line to be tristated.")]
  ErrorPhysChanReservedForInputWhenDesiredForOutput =
    DAQmxErrorPhysChanReservedForInputWhenDesiredForOutput,
  #[error("Physical channel cannot be used by the task because an output task has reserved this line and the Digital Input Tristate property is set to true. Set the Digital Input Tristate property to false.")]
  ErrorPhysChanReservedForOutputWhenDesiredForInput =
    DAQmxErrorPhysChanReservedForOutputWhenDesiredForInput,
  #[error("Specified cDAQ chassis slot is already occupied by a C Series module. Change the slot numbers of the C Series modules in your import file so that they are unique.")]
  ErrorSpecifiedCDAQSlotNotEmpty = DAQmxErrorSpecifiedCDAQSlotNotEmpty,
  #[error("Device type specified does not support simulation. Remove the IsSimulated flag from the import file for this device or change the device type to one that supports simulation.")]
  ErrorDeviceDoesNotSupportSimulation =
    DAQmxErrorDeviceDoesNotSupportSimulation,
  #[error("cDAQ chassis does not have a slot that matches the specified slot number. The slot number specified is probably too large. Change the slot number to be a valid slot number.")]
  ErrorInvalidCDAQSlotNumberSpecd = DAQmxErrorInvalidCDAQSlotNumberSpecd,
  #[error("IsSimulated flag for cDAQ chassis and C Series modules must match. Change the IsSimulated flags in the import file so that they match.")]
  ErrorCSeriesModSimMustMatchCDAQChassisSim =
    DAQmxErrorCSeriesModSimMustMatchCDAQChassisSim,
  #[error("Non-simulated SCC carriers cannot be connected to simulated devices. Change the cabled device of the SCC carrier or change the IsSimulated flags on the SCC carrier to true.")]
  ErrorSCCCabledDevMustNotBeSimWhenSCCCarrierIsNotSim =
    DAQmxErrorSCCCabledDevMustNotBeSimWhenSCCCarrierIsNotSim,
  #[error("IsSimulated flags for an SCC carrier and all of the contained modules must be set to the same value. Change the IsSimulated flags to match.")]
  ErrorSCCModSimMustMatchSCCCarrierSim =
    DAQmxErrorSCCModSimMustMatchSCCCarrierSim,
  #[error("SCXI module type specified does not support simulation. Remove the module from the import file or change the module type.")]
  ErrorSCXIModuleDoesNotSupportSimulation =
    DAQmxErrorSCXIModuleDoesNotSupportSimulation,
  #[error("Non-simulated SCXI module cannot be connected to a simulated cabled device. Change the SCXI module IsSimulated flag or change the SCXI module so that it connects to a cabled device with the same simulation flag setting.")]
  ErrorSCXICableDevMustNotBeSimWhenModIsNotSim =
    DAQmxErrorSCXICableDevMustNotBeSimWhenModIsNotSim,
  #[error("Non-simulated SCXI module cannot be connected to a simulated digitizer. Change the SCXI Module IsSimulated flag or change the SCXI Module so that it connects to a digitizer with the same simulation setting.")]
  ErrorSCXIDigitizerSimMustNotBeSimWhenModIsNotSim =
    DAQmxErrorSCXIDigitizerSimMustNotBeSimWhenModIsNotSim,
  #[error("IsSimulated flags for SCXI chassis and SCXI modules must have the same value. Either make both simulated or make both non-simulated in the import file.")]
  ErrorSCXIModSimMustMatchSCXIChassisSim =
    DAQmxErrorSCXIModSimMustMatchSCXIChassisSim,
  #[error("PXI slot and PXI chassis numbers are required when creating a new simulated PXI device. Add PXI slot and chassis number values for the device in the import file.")]
  ErrorSimPXIDevReqSlotAndChassisSpecd =
    DAQmxErrorSimPXIDevReqSlotAndChassisSpecd,
  #[error("Simulated device cannot be imported to replace a non-simulated device of the same name. Change the device name in the import file and try importing again.")]
  ErrorSimDevConflictWithRealDev = DAQmxErrorSimDevConflictWithRealDev,
  #[error("Number of input/output points entered for the specified channel is insufficient for calibration. At least two points are needed because this device has only gain error calibration constants and not offset calibration constants. Enter more points to eliminate this error.")]
  ErrorInsufficientDataForCalibration =
    DAQmxErrorInsufficientDataForCalibration,
  #[error("Channel you are triggering on is not enabled. Enable the trigger source channel.")]
  ErrorTriggerChannelMustBeEnabled = DAQmxErrorTriggerChannelMustBeEnabled,
  #[error("Calibration procedure has failed to resolve the calibration data format conflict. Perform a complete external calibration on your device. This may modify the calibration data.")]
  ErrorCalibrationDataConflictCouldNotBeResolved =
    DAQmxErrorCalibrationDataConflictCouldNotBeResolved,
  #[error("Currently installed driver no longer supports the hardware self-calibration data format. Either self-calibrate your device (this may modify the calibration data) or downgrade your driver to the version supplied with the device. Older driver versions can also be downloaded from ni.com. If uncertain, contact National Instruments technical support.")]
  ErrorSoftwareTooNewForSelfCalibrationData =
    DAQmxErrorSoftwareTooNewForSelfCalibrationData,
  #[error("Currently installed driver no longer supports the hardware external calibration data format. Either externally calibrate your device (this may modify the calibration data) or downgrade your driver to the version supplied with the device. Older driver versions can also be downloaded from ni.com. If uncertain, contact National Instruments technical support.")]
  ErrorSoftwareTooNewForExtCalibrationData =
    DAQmxErrorSoftwareTooNewForExtCalibrationData,
  #[error("Hardware self-calibration data format is newer than the latest revision supported by the currently installed driver. Either self-calibrate your device (this may modify the calibration data) or upgrade your driver to the version supplied with the device. Driver updates can also be downloaded form ni.com. If uncertain, contact National Instruments technical support.")]
  ErrorSelfCalibrationDataTooNewForSoftware =
    DAQmxErrorSelfCalibrationDataTooNewForSoftware,
  #[error("Hardware external calibration data format is newer than the latest revision supported by the currently installed driver. Either externally calibrate your device (this may modify the calibration data) or upgrade your driver to the version supplied with the device. Driver updates can also be downloaded from ni.com. If uncertain, contact National Instruments technical support.")]
  ErrorExtCalibrationDataTooNewForSoftware =
    DAQmxErrorExtCalibrationDataTooNewForSoftware,
  #[error("Currently installed driver no longer supports the EEPROM format. Either self-calibrate your device (this may modify the EEPROM format) or downgrade your driver to the version supplied with the device. Older driver versions can also be downloaded from ni.com. If uncertain, contact National Instruments technical support.")]
  ErrorSoftwareTooNewForEEPROM = DAQmxErrorSoftwareTooNewForEEPROM,
  #[error("EEPROM format is newer than the latest revision supported by the currently installed driver. Either self-calibrate your device (this may modify the EEPROM format) or upgrade your driver to the version supplied with the device. Driver updates can also be downloaded from ni.com. If uncertain, contact National Instruments technical support.")]
  ErrorEEPROMTooNewForSoftware = DAQmxErrorEEPROMTooNewForSoftware,
  #[error("Currently installed driver no longer supports this revision of the hardware. Please downgrade your driver to the version supplied with the device. Older driver versions can also be downloaded from ni.com.")]
  ErrorSoftwareTooNewForHardware = DAQmxErrorSoftwareTooNewForHardware,
  #[error("Hardware revision is newer that the latest revision supported by the currently installed driver. Please upgrade your driver to the version supplied with the device. Driver updates can also be downloaded from ni.com.")]
  ErrorHardwareTooNewForSoftware = DAQmxErrorHardwareTooNewForSoftware,
  #[error("Task cannot be restarted because the first sample is not available to generate.")]
  ErrorTaskCannotRestartFirstSampNotAvailToGenerate =
    DAQmxErrorTaskCannotRestartFirstSampNotAvailToGenerate,
  #[error("Start Trigger Digital Pattern Source property can only be used with the data lines of the devices. Do not specify a PFI or a RTSI line in the pattern match source.")]
  ErrorOnlyUseStartTrigSrcPrptyWithDevDataLines =
    DAQmxErrorOnlyUseStartTrigSrcPrptyWithDevDataLines,
  #[error("Pause Trigger Digital Pattern Source property can only be used with the data lines of the devices. Do not specify a PFI or a RTSI line in the pattern match source.")]
  ErrorOnlyUsePauseTrigSrcPrptyWithDevDataLines =
    DAQmxErrorOnlyUsePauseTrigSrcPrptyWithDevDataLines,
  #[error("Reference Trigger Digital Pattern Source property can be used only with the data lines of the devices. Do not specify a PFI or a RTSI line in the pattern match source.")]
  ErrorOnlyUseRefTrigSrcPrptyWithDevDataLines =
    DAQmxErrorOnlyUseRefTrigSrcPrptyWithDevDataLines,
  #[error("Number of values specified with Pause Trigger Digital Pattern property does not match the number of physical lines requested with the Pause Trigger Digital Pattern Source property. Change one or both of the properties so the two numbers are equal.")]
  ErrorPauseTrigDigPatternSizeDoesNotMatchSrcSize =
    DAQmxErrorPauseTrigDigPatternSizeDoesNotMatchSrcSize,
  #[error("Device cannot be configured for input or output because lines and/or terminals on this device are in use by another task or route. This operation requires temporarily reserving all lines and terminals for communication, which interferes with the other task or route. If possible, use DAQmx Control Task to reserve all tasks that use this device before committing any tasks that use this device. Otherwise, uncommit or unreserve the other task or disconnect the other route before attempting to configure the device for input or output.")]
  ErrorLineConflictCDAQ = DAQmxErrorLineConflictCDAQ,
  #[error("Attempted to write a sample beyond the final finite sample. The sample specified by the combination of position and offset will never be writable. Specify a position and offset which selects a sample up to but not beyond the final sample to generate.")]
  ErrorCannotWriteBeyondFinalFiniteSample =
    DAQmxErrorCannotWriteBeyondFinalFiniteSample,
  #[error("Reference and start trigger sources cannot be the same. Make the reference and start trigger sources different from one another.")]
  ErrorRefAndStartTriggerSrcCantBeSame =
    DAQmxErrorRefAndStartTriggerSrcCantBeSame,
  #[error("Memory Mapping for Programmed IO Enable setting is not compatible with some of the physical channels in the task. Change Memory Mapping for Programmed IO Enable to false or do not create the task with the incompatible physical channels.")]
  ErrorMemMappingIncompatibleWithPhysChansInTask =
    DAQmxErrorMemMappingIncompatibleWithPhysChansInTask,
  #[error("Memory Mapping for Programmed IO Enable cannot be set to true when Output Drive Type is Open Collector. Change Output Drive Type to Active Drive or change Memory Mapping for Programmed IO Enable to false.")]
  ErrorOutputDriveTypeMemMappingConflict =
    DAQmxErrorOutputDriveTypeMemMappingConflict,
  #[error("Device index requested is invalid. The value of the index must be between one and the number of devices in the task.")]
  ErrorCAPIDeviceIndexInvalid = DAQmxErrorCAPIDeviceIndexInvalid,
  #[error("Your ratiometric device must use excitation for scaling. The Use Excitation for Scaling property cannot be set to false on this device. Use excitation for scaling by setting the Use Excitation for Scaling property to true. This will cause NI-DAQmx to return ratiometric data instead of voltage data which is not supported by ratiometric devices.")]
  ErrorRatiometricDevicesMustUseExcitationForScaling =
    DAQmxErrorRatiometricDevicesMustUseExcitationForScaling,
  #[error("DAQmx Timing property specified requires per device configuration. Explicitly specify the device(s) to which this property should apply.")]
  ErrorPropertyRequiresPerDeviceCfg = DAQmxErrorPropertyRequiresPerDeviceCfg,
  #[error("Channel properties conflict. If Analog Input Source is _aignd_vs_aignd, then Analog Input Coupling must be set to GND. If Analog Input Source is _external_channel, then Analog Input Coupling must be DC.")]
  ErrorAICouplingAndAIInputSourceConflict =
    DAQmxErrorAICouplingAndAIInputSourceConflict,
  #[error("Only one task is permitted to have the Digital Output Memory Mapping for Programmed I/O Enable set to true at a time. If the value is unset, NI-DAQmx will choose a value that is compatible with the system while reserving the task. Do not set the property to true explicitly, set the property to false explicitly, or set the value to the default value to allow NI-DAQmx to choose a value that is compatible with the system.")]
  ErrorOnlyOneTaskCanPerformDOMemoryMappingAtATime =
    DAQmxErrorOnlyOneTaskCanPerformDOMemoryMappingAtATime,
  #[error("Device supports an analog channel as the source of an analog reference trigger only when it is the only channel from this device in the task. Remove all of the channels from this device in the task except the channel that you want to use as the analog trigger source, change the analog trigger source to a terminal, or select a channel from another device that only has one channel in the task.")]
  ErrorTooManyChansForAnalogRefTrigCDAQ =
    DAQmxErrorTooManyChansForAnalogRefTrigCDAQ,
  #[error("Property requested is incompatible with the given Timing Type. NI-DAQmx can automatically select a compatible property value for you. To use the requested Timing Type, do not set the specified property and allow NI-DAQmx to set it for you. To use the requested property value, chose a different value for the Timing Type.")]
  ErrorSpecdPropertyValueIsIncompatibleWithSampleTimingType =
    DAQmxErrorSpecdPropertyValueIsIncompatibleWithSampleTimingType,
  #[error("Task cannot be reserved because the CPU does not support the Streaming SIMD Extensions (SSE).")]
  ErrorCPUNotSupportedRequireSSE = DAQmxErrorCPUNotSupportedRequireSSE,
  #[error("Property requested is incompatible with the given Timing Response Mode. NI-DAQmx can automatically select a compatible property value for you. To use the requested Timing Response Mode, do not set the specified property and allow NI-DAQmx to set it for you. To use the requested property value, choose a different value for the Timing Response Mode.")]
  ErrorSpecdPropertyValueIsIncompatibleWithSampleTimingResponseMode =
    DAQmxErrorSpecdPropertyValueIsIncompatibleWithSampleTimingResponseMode,
  #[error("Next Write is Last property not settable if Regeneration Mode is set to Allow Regeneration.")]
  ErrorConflictingNextWriteIsLastAndRegenModeProperties =
    DAQmxErrorConflictingNextWriteIsLastAndRegenModeProperties,
  #[error("Operation must be performed on the entire task. It cannot be performed only on specific devices in the task. Do not use the indexer, Item property in Visual Basic, or index operator in C++ to specify device names when performing this operation.")]
  ErrorMStudioOperationDoesNotSupportDeviceContext =
    DAQmxErrorMStudioOperationDoesNotSupportDeviceContext,
  #[error("Property setting must be identical for all channels in the task.")]
  ErrorPropertyValueInChannelExpansionContextInvalid =
    DAQmxErrorPropertyValueInChannelExpansionContextInvalid,
  #[error("Hardware timed non-buffered analog output is not supported on this device.")]
  ErrorHWTimedNonBufferedAONotSupported =
    DAQmxErrorHWTimedNonBufferedAONotSupported,
  #[error("Waveform length must be a multiple of the waveform quantum.")]
  ErrorWaveformLengthNotMultOfQuantum =
    DAQmxErrorWaveformLengthNotMultOfQuantum,
  #[error("Multi-device tasks with channels from both 446x and 447x devices require a 446x device to be in PXI slot 2.")]
  ErrorDSAExpansionMixedBoardsWrongOrderInPXIChassis =
    DAQmxErrorDSAExpansionMixedBoardsWrongOrderInPXIChassis,
  #[error("Power level is too low. OOK modulation requires the bypass path to be used, and power levels this low must use the main path.")]
  ErrorPowerLevelTooLowForOOK = DAQmxErrorPowerLevelTooLowForOOK,
  #[error("Device component test failed. If problem persists, contact National Instruments technical support at www.ni.com/support.")]
  ErrorDeviceComponentTestFailure = DAQmxErrorDeviceComponentTestFailure,
  #[error(
    "Device does not support User Defined Waveform with OOK modulation."
  )]
  ErrorUserDefinedWfmWithOOKUnsupported =
    DAQmxErrorUserDefinedWfmWithOOKUnsupported,
  #[error("Digital Modulation User Defined Waveform is invalid.")]
  ErrorInvalidDigitalModulationUserDefinedWaveform =
    DAQmxErrorInvalidDigitalModulationUserDefinedWaveform,
  #[error(
        "You cannot use the Ref In/Out connector as both an input and an output at the same time."
    )]
  ErrorBothRefInAndRefOutEnabled = DAQmxErrorBothRefInAndRefOutEnabled,
  #[error(
        "Device does not support both analog modulation and digital modulation simultaneously."
    )]
  ErrorBothAnalogAndDigitalModulationEnabled =
    DAQmxErrorBothAnalogAndDigitalModulationEnabled,
  #[error("Selected lines do not support buffered operations if the C Series module is installed in the specified slot. Ensure only lines that support buffered operations are being used in the task. If using change detection timing, the task must be changed to nonbuffered to support these lines. Move your cDAQ module to a slot that supports buffered operations.")]
  ErrorBufferedOpsNotSupportedInSpecdSlotForCDAQ =
    DAQmxErrorBufferedOpsNotSupportedInSpecdSlotForCDAQ,
  #[error("Physical channel specified may only be used if the C Series module is installed in a slot that supports this physical channel. Move your cDAQ module to a slot that supports this physical channel.")]
  ErrorPhysChanNotSupportedInSpecdSlotForCDAQ =
    DAQmxErrorPhysChanNotSupportedInSpecdSlotForCDAQ,
  #[error("Resource requested by this task has already been reserved by a different task with conflicting settings. Unreserve any other tasks using this device, or change their settings to be compatible with this task.")]
  ErrorResourceReservedWithConflictingSettings =
    DAQmxErrorResourceReservedWithConflictingSettings,
  #[error("An attempt has been made to use an analog trigger in multiple situations with differing properties. Change the analog trigger properties so they are the same, select an analog trigger source from another device for one of the triggers, or do not use an analog trigger for all situations.")]
  ErrorInconsistentAnalogTrigSettingsCDAQ =
    DAQmxErrorInconsistentAnalogTrigSettingsCDAQ,
  #[error("Device support an analog channel as the source of an analog pause trigger only when it is the only channel from this device in the task. Remove all of this device's channels currently in the task except the channel that will be used as the analog trigger source, change the analog trigger source to a terminal, or select a channel from another device that only has one channel in the task.")]
  ErrorTooManyChansForAnalogPauseTrigCDAQ =
    DAQmxErrorTooManyChansForAnalogPauseTrigCDAQ,
  #[error("Analog trigger source must be the first channel of the device in the acquisition or a valid analog trigger terminal. Create your channels in a different order so that this channel is first, select a different channel from this device, or select the first channel from another device in the task. If you explicitly named the virtual channel in DAQmx Create Channel you must use the name assigned to that channel.")]
  ErrorAnalogTrigNotFirstInScanListCDAQ =
    DAQmxErrorAnalogTrigNotFirstInScanListCDAQ,
  #[error("Number of channels in task exceeds the device maximum given the requested Timing Type. Reduce the number of channels or select a different Timing Type.")]
  ErrorTooManyChansGivenTimingType = DAQmxErrorTooManyChansGivenTimingType,
  #[error("Sample clock timebase  divisor may not be specified when an external sample clock source is specified. Change the sample clock source to onboard clock or do not configure the sample clock timebase divisor.")]
  ErrorSampClkTimebaseDivWithExtSampClk =
    DAQmxErrorSampClkTimebaseDivWithExtSampClk,
  #[error("Task specified cannot be saved because the DAQmx Timing properties were specified on a per device basis using the More: AI Convert: ActiveDevs property.")]
  ErrorCantSaveTaskWithPerDeviceTimingProperties =
    DAQmxErrorCantSaveTaskWithPerDeviceTimingProperties,
  #[error("When enabling auto zero on this device, all channels that are using auto zero must have the same auto zero mode. Channels with auto zero disabled may be present in the same task as channels with auto zero enabled. Select the same auto zero mode for all channels that are using auto zero.")]
  ErrorConflictingAutoZeroMode = DAQmxErrorConflictingAutoZeroMode,
  #[error("Sample Clock Rate requested is supported only if Enhanced Alias Rejection Enable is False. Set Enhanced Alias Rejection Enable to False or decrease the Sample Clock Rate.")]
  ErrorSampClkRateNotSupportedWithEAREnabled =
    DAQmxErrorSampClkRateNotSupportedWithEAREnabled,
  #[error("Sample clock timebase rate must be specified when using an external sample clock timebase.")]
  ErrorSampClkTimebaseRateNotSpecd = DAQmxErrorSampClkTimebaseRateNotSpecd,
  #[error("Driver was unloaded and then reloaded at a different base address after the session was created. Session is unusable. Close and reopen the session.")]
  ErrorSessionCorruptedByDLLReload = DAQmxErrorSessionCorruptedByDLLReload,
  #[error("An active device was specified for the attribute but it is not supported for channel expansion tasks. Do not specify an active device when setting the attribute or do not use channel expansion.")]
  ErrorActiveDevNotSupportedWithChanExpansion =
    DAQmxErrorActiveDevNotSupportedWithChanExpansion,
  #[error("Task contains physical channels that have incompatible hardware restrictions for their Sample Clock Rates. Remove incompatible physical channels from the task.")]
  ErrorSampClkRateInvalid = DAQmxErrorSampClkRateInvalid,
  #[error("Sync pulse cannot be exported when using an external sync pulse source. Do not export the sync pulse, or use the internal chassis sync pulse source.")]
  ErrorExtSyncPulseSrcCannotBeExported =
    DAQmxErrorExtSyncPulseSrcCannotBeExported,
  #[error("Minimum delay time between the sync pulse and start must be specified when using an external Sync Pulse Source. Specify SyncPulse.MinDelayToStart.")]
  ErrorSyncPulseMinDelayToStartNeededForExtSyncPulseSrc =
    DAQmxErrorSyncPulseMinDelayToStartNeededForExtSyncPulseSrc,
  #[error("Task contains physical channels on one or more devices that require the driver to select the Sync Pulse Source. Do not specify the Sync Pulse Source. DAQmx will set the Sync Pulse Source appropriately.")]
  ErrorSyncPulseSrcInvalid = DAQmxErrorSyncPulseSrcInvalid,
  #[error("Task contains physical channels on one or more devices that require a different Sample Clock Timebase Rate than the one specified. Do not specify the Sample Clock Timebase Rate.  DAQmx will set the Sample Clock Timebase Rate appropriately.")]
  ErrorSampClkTimebaseRateInvalid = DAQmxErrorSampClkTimebaseRateInvalid,
  #[error("Task contains physical channels on one or more devices that require a different Sample Clock Timebase Source than the one specified. Do not specify the Sample Clock Timebase Source.  DAQmx will set the Sample Clock Timebase Source appropriately.")]
  ErrorSampClkTimebaseSrcInvalid = DAQmxErrorSampClkTimebaseSrcInvalid,
  #[error("Task contains physical channels on one or more devices that require you to specify the Sample Clock rate. Use the Sample Clock Timing function/VI to specify a Sample Clock rate. You cannot specify a Sample Clock rate if Mode is set to On Demand.")]
  ErrorSampClkRateMustBeSpecd = DAQmxErrorSampClkRateMustBeSpecd,
  #[error("Attribute name specified is invalid. Validate the attribute name using your ADE. The attribute may have been exported from a later version of NI-DAQmx and is not supported by the version of NI-DAQmx installed on your system. Check the version specified in the file against the installed version of NI-DAQmx. You can upgrade the version of NI-DAQmx installed on your system, or remove the attribute from the file.")]
  ErrorInvalidAttributeName = DAQmxErrorInvalidAttributeName,
  #[error("Thermocouple CJC channel name property must be set when the thermocouple CJC source property is Channel. Set the thermocouple CJC channel name property or set the thermocouple CJC source property to a value other than Channel.")]
  ErrorCJCChanNameMustBeSetWhenCJCSrcIsScannableChan =
    DAQmxErrorCJCChanNameMustBeSetWhenCJCSrcIsScannableChan,
  #[error("Hidden channels listed for the task were not in the channels list for the task. Add the missing hidden channels to the channel list in the configuration file.")]
  ErrorHiddenChanMissingInChansPropertyInCfgFile =
    DAQmxErrorHiddenChanMissingInChansPropertyInCfgFile,
  #[error("Task must contain at least one channel. Add a channel to the task in the configuration file.")]
  ErrorChanNamesNotSpecdInCfgFile = DAQmxErrorChanNamesNotSpecdInCfgFile,
  #[error("Hidden channel was listed more than once in the task. A task cannot contain a hidden channel with the same name twice. Remove the duplicate entries from the configuration file.")]
  ErrorDuplicateHiddenChanNamesInCfgFile =
    DAQmxErrorDuplicateHiddenChanNamesInCfgFile,
  #[error("Channel was listed more than once in the task. A task cannot contain a channel with the same name twice. Remove the duplicate entries from the configuration file.")]
  ErrorDuplicateChanNameInCfgFile = DAQmxErrorDuplicateChanNameInCfgFile,
  #[error("SCC specified cannot be placed in the slot specified. Specify a supported SCC for the given slot or specify a slot that supports the given SCC.")]
  ErrorInvalidSCCModuleForSlotSpecd = DAQmxErrorInvalidSCCModuleForSlotSpecd,
  #[error("SCC slot number specified is invalid. Specify a slot number that is valid for the specified carrier.")]
  ErrorInvalidSCCSlotNumberSpecd = DAQmxErrorInvalidSCCSlotNumberSpecd,
  #[error("Section identifier specified is invalid. Refer to the NI-DAQmx configuration file documentation for a list of valid section identifiers. The section identifier may have been exported from a later version of NI-DAQmx and is not supported by the version of NI-DAQmx installed on your system. Check the version specified in the file against the installed version of NI-DAQmx. You can upgrade the version of NI-DAQmx installed on your system, or remove the section from the file.")]
  ErrorInvalidSectionIdentifier = DAQmxErrorInvalidSectionIdentifier,
  #[error("Section name specified is invalid. The format of the section name is [<objectType> <objectName>]. Example: [DAQmxDevice Dev1]")]
  ErrorInvalidSectionName = DAQmxErrorInvalidSectionName,
  #[error("NI-DAQmx version specified in the input file is newer than the installed NI-DAQmx version. Change the version in the file to match the installed version.  The import might still fail if the file contains properties that are not supported by the installed version of NI-DAQmx.")]
  ErrorDAQmxVersionNotSupported = DAQmxErrorDAQmxVersionNotSupported,
  #[error("Import operation does not support tasks, channels, and scales, but such an object was found. Remove all tasks, channels, and scales from the input file.")]
  ErrorSWObjectsFoundInFile = DAQmxErrorSWObjectsFoundInFile,
  #[error("Import operation supports tasks, channels, and scales only, but a hardware object was found in the file. Remove the hardware objects from the input file.")]
  ErrorHWObjectsFoundInFile = DAQmxErrorHWObjectsFoundInFile,
  #[error("Local channel specified is from a task that does not exist. Specify the task in question, move the local channel to an existing task, or change the local channel to a global channel.")]
  ErrorLocalChannelSpecdWithNoParentTask =
    DAQmxErrorLocalChannelSpecdWithNoParentTask,
  #[error("Task references a local channel that does not exist in this task. Remove the reference to the missing local channel or create the local channel.")]
  ErrorTaskReferencesMissingLocalChannel =
    DAQmxErrorTaskReferencesMissingLocalChannel,
  #[error("Task references a local channel from another task. Reference only global channels and local channels that belong to this task.")]
  ErrorTaskReferencesLocalChannelFromOtherTask =
    DAQmxErrorTaskReferencesLocalChannelFromOtherTask,
  #[error(
        "Task does not include the Channels property. Specify the Channels property for this task."
    )]
  ErrorTaskMissingChannelProperty = DAQmxErrorTaskMissingChannelProperty,
  #[error("Local channel name specified is invalid. Local channel names are of the form <task name>/<channel name>. Example: task1/chan1")]
  ErrorInvalidLocalChanName = DAQmxErrorInvalidLocalChanName,
  #[error(
    "Configuration file string contains invalid character escape sequence."
  )]
  ErrorInvalidEscapeCharacterInString =
    DAQmxErrorInvalidEscapeCharacterInString,
  #[error("Configuration file contains an invalid start of table identifier. The table identifier may have been exported from a later version of NI-DAQmx and is not supported by the version of NI-DAQmx installed on your system. Check the version specified in the file against the installed version of NI-DAQmx. You can upgrade the version of NI-DAQmx installed on your system, or remove the table from the file.")]
  ErrorInvalidTableIdentifier = DAQmxErrorInvalidTableIdentifier,
  #[error("Property setting found in a column with no property name heading. Remove the property setting from the column or add the property name to the table definition.")]
  ErrorValueFoundInInvalidColumn = DAQmxErrorValueFoundInInvalidColumn,
  #[error("Configuration file contains property names or values that are not contained within a valid table. Add the appropriate start of table string prior to property names.")]
  ErrorMissingStartOfTable = DAQmxErrorMissingStartOfTable,
  #[error("Configuration file is missing the required header fields. Add required header information at the top of the text file prior to any data.")]
  ErrorFileMissingRequiredDAQmxHeader =
    DAQmxErrorFileMissingRequiredDAQmxHeader,
  #[error("Device ID value in the driver does not match the device ID value from the device. Ensure the correct driver is being used for this device.")]
  ErrorDeviceIDDoesNotMatch = DAQmxErrorDeviceIDDoesNotMatch,
  #[error("Selected lines do not support buffered operations. Ensure only lines that support buffered operations are being used in the task. If using change detection timing, the task must be changed to non-buffered to support these lines.")]
  ErrorBufferedOperationsNotSupportedOnSelectedLines =
    DAQmxErrorBufferedOperationsNotSupportedOnSelectedLines,
  #[error("Property specified cannot return its value because the custom scale for the channel does not include the value in the range or table of pre-scaled values. Ensure that the custom scale includes all potential values for this property in the range or table of pre-scaled values, or use a linear or polynomial scale.")]
  ErrorPropertyConflictsWithScale = DAQmxErrorPropertyConflictsWithScale,
  #[error("Syntax error encountered in INI file. Valid INI syntax allows for the following 3 types of lines: section headers, items, and comments. A section header begins with an open bracket and ends with a closed bracket. Example: [mySection] An item has an equals sign in between two strings. Example: myItem = 46 A comment begins with a semicolon. Example: ; This is my comment.")]
  ErrorInvalidINIFileSyntax = DAQmxErrorInvalidINIFileSyntax,
  #[error("Device information retrieval failed because PXI chassis is not identified. Use MAX or nipxiconfig to identify your chassis.")]
  ErrorDeviceInfoFailedPXIChassisNotIdentified =
    DAQmxErrorDeviceInfoFailedPXIChassisNotIdentified,
  #[error("Hardware product number specified is invalid. Enter a valid product number. If the product number you entered is an actual product number, ensure the product type is appropriate for the object you are configuring. For example, do not use the product number of a PXI device where an SCXI module is expected.")]
  ErrorInvalidHWProductNumber = DAQmxErrorInvalidHWProductNumber,
  #[error("Hardware product type specified is invalid. Enter a valid product type. If the product number you entered is an actual product type, ensure the product type is appropriate for the object you are configuring. For example, do not use the product type of a PXI device where an SCXI module is expected.")]
  ErrorInvalidHWProductType = DAQmxErrorInvalidHWProductType,
  #[error("Numeric value specified is in an invalid format. Remove any non-numeric characters from the specified numeric value.")]
  ErrorInvalidNumericFormatSpecd = DAQmxErrorInvalidNumericFormatSpecd,
  #[error("Object contains two references to the same property. Remove one of the duplicate properties.")]
  ErrorDuplicatePropertyInObject = DAQmxErrorDuplicatePropertyInObject,
  #[error("Enumerated value specified is not a valid value for that enumeration. The enumerated value may have been exported from a later version of NI-DAQmx and is not supported by the version of NI-DAQmx installed on your system. Check the version specified in the file against the installed version of NI-DAQmx. You can upgrade the version of NI-DAQmx installed on your system, or change the value to one supported by the version of NI-DAQmx you have installed.")]
  ErrorInvalidEnumValueSpecd = DAQmxErrorInvalidEnumValueSpecd,
  #[error("Physical channel specified for the TEDS interface is already connected to a TEDS interface. Enter a physical channel that is not currently occupied, or remove the existing physical channel configuration.")]
  ErrorTEDSSensorPhysicalChannelConflict =
    DAQmxErrorTEDSSensorPhysicalChannelConflict,
  #[error("Physical channels specified for the TEDS interface are too great in number for the specified type of TEDS interface. Reduce the number of physical channels specified.")]
  ErrorTooManyPhysicalChansForTEDSInterfaceSpecd =
    DAQmxErrorTooManyPhysicalChansForTEDSInterfaceSpecd,
  #[error("Controlling device specified for the TEDS interface is not capable of controlling a TEDS interface.")]
  ErrorIncapableTEDSInterfaceControllingDeviceSpecd =
    DAQmxErrorIncapableTEDSInterfaceControllingDeviceSpecd,
  #[error("Carrier specified for the SCC module could not be found. Ensure that the SCC carrier specified for the module is also defined in the configuration file.")]
  ErrorSCCCarrierSpecdIsMissing = DAQmxErrorSCCCarrierSpecdIsMissing,
  #[error("Digitizing device specified for the SCC carrier is not capable of digitizing for SCC carriers. Specify a device that is capable of digitizing for SCC carriers.")]
  ErrorIncapableSCCDigitizingDeviceSpecd =
    DAQmxErrorIncapableSCCDigitizingDeviceSpecd,
  #[error("Accessory setting specified does not apply to the accessory type. Remove the non-applicable accessory setting from the accessory specification.")]
  ErrorAccessorySettingNotApplicable = DAQmxErrorAccessorySettingNotApplicable,
  #[error("Device and connector specified by the accessory can not be configured because there is already an accessory configured for that connector and device. Enter a device and connector that is not currently occupied, or remove the configuration of the existing accessory.")]
  ErrorDeviceAndConnectorSpecdAlreadyOccupied =
    DAQmxErrorDeviceAndConnectorSpecdAlreadyOccupied,
  #[error("Accessory type specified cannot be connected to the specified device. Enter an accessory type that can be connected to the device specified.")]
  ErrorIllegalAccessoryTypeForDeviceSpecd =
    DAQmxErrorIllegalAccessoryTypeForDeviceSpecd,
  #[error("Device connector number specified is invalid. Enter a valid connector number.")]
  ErrorInvalidDeviceConnectorNumberSpecd =
    DAQmxErrorInvalidDeviceConnectorNumberSpecd,
  #[error("Specified accessory name is invalid. The name of the accessory should be in this format: accessoryProductType / connectedDeviceIdentifier / connectorNumber Connector numbers start at zero.  A device with only one connector only has a connector zero.")]
  ErrorInvalidAccessoryName = DAQmxErrorInvalidAccessoryName,
  #[error("Device specification provided matches more than one device in the system. Change the device specification to be more specific.")]
  ErrorMoreThanOneMatchForSpecdDevice =
    DAQmxErrorMoreThanOneMatchForSpecdDevice,
  #[error("Device specification provided does not match any hardware in the system. Change the device specification to match a device present in your system. You can also change your device specification to be less specific.")]
  ErrorNoMatchForSpecdDevice = DAQmxErrorNoMatchForSpecdDevice,
  #[error("Product type and product number specified do not refer to the same product. Remove either the product type or the product number from the object.")]
  ErrorProductTypeAndProductNumberConflict =
    DAQmxErrorProductTypeAndProductNumberConflict,
  #[error(
        "Object specified contains an extra property. Remove the extra property from the object."
    )]
  ErrorExtraPropertyDetectedInSpecdObject =
    DAQmxErrorExtraPropertyDetectedInSpecdObject,
  #[error("Object lacks a required property. Add the required property to the object.")]
  ErrorRequiredPropertyMissing = DAQmxErrorRequiredPropertyMissing,
  #[error("Author property cannot be set on a local channel. Remove the author property from the local channel.")]
  ErrorCantSetAuthorForLocalChan = DAQmxErrorCantSetAuthorForLocalChan,
  #[error("Time value specified is invalid. Ensure that the time entered has only valid values for each of the fields in the time format.  For example: the month section must be between 01 and 12.")]
  ErrorInvalidTimeValue = DAQmxErrorInvalidTimeValue,
  #[error("Format of the time value specified is invalid. Enter the time value in the format: YYYY-MM-DDTHH:mm:ssZ UTC Where YYYY is the four digit year, MM is the two digit month, DD is the two digit day of the month, HH is the two digit hour of the day (24 hour clock), mm is the two digit minutes into the hour, and ss is the two digit seconds into the minute.  T is a literal separator between date and time. For example, the string: 2004-10-19T16:30:45Z UTC Represents October 19th, 2004 at 4:30:45 PM GMT.")]
  ErrorInvalidTimeFormat = DAQmxErrorInvalidTimeFormat,
  #[error("Digitizing Device Channels property is specified, but the Digitization Mode property is not set to parallel. Either remove the Digitizing Device Channels property or set the Digitizing property to parallel.")]
  ErrorDigDevChansSpecdInModeOtherThanParallel =
    DAQmxErrorDigDevChansSpecdInModeOtherThanParallel,
  #[error("Cascade digitization mode is not supported for SCXI. Select a different digitization mode.")]
  ErrorCascadeDigitizationModeNotSupported =
    DAQmxErrorCascadeDigitizationModeNotSupported,
  #[error("Slot specified is already occupied. Either specify a slot that is unoccupied or remove the module occupying the desired slot.")]
  ErrorSpecdSlotAlreadyOccupied = DAQmxErrorSpecdSlotAlreadyOccupied,
  #[error("SCXI slot number specified is invalid. Specify a slot number that is valid for the specified chassis.")]
  ErrorInvalidSCXISlotNumberSpecd = DAQmxErrorInvalidSCXISlotNumberSpecd,
  #[error("Address specified is already in use. Specify an address that is not in use.")]
  ErrorAddressAlreadyInUse = DAQmxErrorAddressAlreadyInUse,
  #[error("Device specified cannot be connected to a RTSI cable. If the device does not have a RTSI connector, you cannot connect it to a RTSI cable.  If the device is a PXI device then it is automatically connected to the PXI backplane, and therefore does not need to be manually configured as connected to a RTSI cable.")]
  ErrorSpecdDeviceDoesNotSupportRTSI = DAQmxErrorSpecdDeviceDoesNotSupportRTSI,
  #[error("Device specified is already connected to a RTSI cable. To connect the device to another RTSI cable, remove it from the RTSI cable to which it is currently connected.")]
  ErrorSpecdDeviceIsAlreadyOnRTSIBus = DAQmxErrorSpecdDeviceIsAlreadyOnRTSIBus,
  #[error("Name specified is already in use. Specify a name that is not currently in use.")]
  ErrorIdentifierInUse = DAQmxErrorIdentifierInUse,
  #[error("Counter task detected three or more missed Sample Clock pulses. Samples were lost before the application could read them. Decrease the Sample Clock rate or restructure the application so that DAQmx Read runs more frequently.  Setting the Convert    Error = DAQmxError to Warning property to True does not eliminate the error, because samples were lost.")]
  ErrorWaitForNextSampleClockOrReadDetected3OrMoreMissedSampClks =
    DAQmxErrorWaitForNextSampleClockOrReadDetected3OrMoreMissedSampClks,
  #[error("Data Transfer Mechanism is set to Programmed I/O which is not supported for hardware-timed operations for this device and Channel Type. Change Data Transfer Mechanism, do not configure Sample Clock timing, or set Sample Timing Type to On Demand.")]
  ErrorHWTimedAndDataXferPIO = DAQmxErrorHWTimedAndDataXferPIO,
  #[error("Non-buffered hardware-timed operations are not supported for this device and Channel Type. Set the Buffer Size to greater than 0, do not configure Sample Clock timing, or set Sample Timing Type to On Demand.")]
  ErrorNonBufferedAndHWTimed = DAQmxErrorNonBufferedAndHWTimed,
  #[error("DAQmx Write failed because the previously written value has not been generated. This error can occur if the Sample Clock period is shorter than the period of the generated pulse train. Reduce the Sample Clock rate or increase the frequency of the generated pulse train.")]
  ErrorCTROutSampClkPeriodShorterThanGenPulseTrainPeriodPolled =
    DAQmxErrorCTROutSampClkPeriodShorterThanGenPulseTrainPeriodPolled,
  #[error("DAQmx Write failed because the previously written value has not been generated. This error can occur if the Sample Clock period is shorter than the period of the generated pulse train. Reduce the Sample Clock rate, increase the frequency of the generated pulse train, or set Write Recovery Mode to Poll.")]
  ErrorCTROutSampClkPeriodShorterThanGenPulseTrainPeriod2 =
    DAQmxErrorCTROutSampClkPeriodShorterThanGenPulseTrainPeriod2,
  #[error("Write recovery could not complete before detecting another Sample Clock pulse. Reduce the Sample Clock rate or increase the frequency of the generated pulse train.")]
  ErrorCOCannotKeepUpInHWTimedSinglePointPolled =
    DAQmxErrorCOCannotKeepUpInHWTimedSinglePointPolled,
  #[error("Write recovery could not complete before detecting another Sample Clock pulse. Reduce the Sample Clock rate, increase the frequency of the generated pulse train, or set Write Recovery Mode to Poll.")]
  ErrorWriteRecoveryCannotKeepUpInHWTimedSinglePoint =
    DAQmxErrorWriteRecoveryCannotKeepUpInHWTimedSinglePoint,
  #[error("Lines specified do not support change detection. Select lines that support change detection.")]
  ErrorNoChangeDetectionOnSelectedLineForDevice =
    DAQmxErrorNoChangeDetectionOnSelectedLineForDevice,
  #[error("Pause triggering is not supported in a multi-device task. To configure pause triggering in a multi-device configuration, you must use no more than one device per task and manually route the clock signals in the application.")]
  ErrorSMIOPauseTriggersNotSupportedWithChannelExpansion =
    DAQmxErrorSMIOPauseTriggersNotSupportedWithChannelExpansion,
  #[error("You have selected an external clock source for the task, but the device importing the clock does not have the longest pipeline of all the devices in the task. This leads to an incomplete acquisition on that device because the device will not receive enough Sample Clock pulses. Route the external clock signal into the device with the longest pipeline. Refer to device documentation for information on pipeline depth.")]
  ErrorClockMasterForExternalClockNotLongestPipeline =
    DAQmxErrorClockMasterForExternalClockNotLongestPipeline,
  #[error("Byte order marker of the specified file is not supported by NI-DAQmx. For tab-delimeted files, NI-DAQmx supports UTF-8, UTF-16  / UCS-2 little endian, and UTF-32 / UCS-4 little endian. For ini files, NI-DAQmx only supports UTF-8. Save the file in one of the supported formats with the appropriate byte order marker.")]
  ErrorUnsupportedUnicodeByteOrderMarker =
    DAQmxErrorUnsupportedUnicodeByteOrderMarker,
  #[error("Too many compiled instructions in loop.  \"Generate\" and \"Wait\" instructions each result in at least one compiled instruction.  Each marker adds an additional compiled instruction.  Clear instruction does not result in a compiled instruction. If possible, reduce the number of generate instructions by concatenating the waveforms on two or more consecutive generate instructions.")]
  ErrorTooManyInstructionsInLoopInScript =
    DAQmxErrorTooManyInstructionsInLoopInScript,
  #[error("PLL lock operation failed or timed out. Ensure the module is fully inserted into the carrier.")]
  ErrorPLLNotLocked = DAQmxErrorPLLNotLocked,
  #[error("\"If-Else\" blocks are not allowed in \"Finite Repeat\" loops. If possible, remove the \"Repeat\" and \"End Repeat\" instructions and explicitly duplicate the instructions originally in the loop the desired number of times.")]
  ErrorIfElseBlockNotAllowedInFiniteRepeatLoopInScript =
    DAQmxErrorIfElseBlockNotAllowedInFiniteRepeatLoopInScript,
  #[error("\"If-Else\" blocks are not allowed in \"Repeat Until\" loops.")]
  ErrorIfElseBlockNotAllowedInConditionalRepeatLoopInScript =
    DAQmxErrorIfElseBlockNotAllowedInConditionalRepeatLoopInScript,
  #[error(
        "\"Clear Trigger\" instruction cannot be the last instruction in an \"If-Else\" block."
    )]
  ErrorClearIsLastInstructionInIfElseBlockInScript =
    DAQmxErrorClearIsLastInstructionInIfElseBlockInScript,
  #[error(
        "Wait duration is too small for the \"Wait\" instruction before the \"If-Else\" block."
    )]
  ErrorInvalidWaitDurationBeforeIfElseBlockInScript =
    DAQmxErrorInvalidWaitDurationBeforeIfElseBlockInScript,
  #[error("Marker position specified is too close to the end of the waveform in the \"Generate\" statement before the \"If-Else\" block.")]
  ErrorMarkerPosInvalidBeforeIfElseBlockInScript =
    DAQmxErrorMarkerPosInvalidBeforeIfElseBlockInScript,
  #[error("Length of waveform subset is too small for the \"Generate\" instruction before \"If-Else\" block.")]
  ErrorInvalidSubsetLengthBeforeIfElseBlockInScript =
    DAQmxErrorInvalidSubsetLengthBeforeIfElseBlockInScript,
  #[error("Waveform length is too small for the \"Generate\" instruction before the \"If-Else\" block.")]
  ErrorInvalidWaveformLengthBeforeIfElseBlockInScript =
    DAQmxErrorInvalidWaveformLengthBeforeIfElseBlockInScript,
  #[error("\"Generate\" or finite \"Wait\" instruction expected before \"If-Else\" block. Insert a \"Generate\" or finite \"Wait\" instruction before the If-Else block.")]
  ErrorGenerateOrFiniteWaitInstructionExpectedBeforeIfElseBlockInScript =
    DAQmxErrorGenerateOrFiniteWaitInstructionExpectedBeforeIfElseBlockInScript,
  #[error("Device does not support an external calibration password.")]
  ErrorCalPasswordNotSupported = DAQmxErrorCalPasswordNotSupported,
  #[error("Invoke DAQmx Setup Calibration before invoking the corresponding DAQmx Adjust Calibration.")]
  ErrorSetupCalNeededBeforeAdjustCal = DAQmxErrorSetupCalNeededBeforeAdjustCal,
  #[error("Device does not support simultaneous calibration of multiple channels. Calibrate channels one channel at a time, passing individual channels to different invocations of DAQmx Setup Calibration.")]
  ErrorMultipleChansNotSupportedDuringCalSetup =
    DAQmxErrorMultipleChansNotSupportedDuringCalSetup,
  #[error("Device cannot be accessed.  Possible causes: Device is no longer present in the system. Device is not powered. Device is powered, but was temporarily without power. Device and/or chassis driver support may have been removed. Device is damaged. Ensure the device is properly connected and turned on. Ensure the device and/or chassis is supported in the current version of the driver. Check the device's status in NI MAX. Turn the computer off and on again. If you suspect that the device is damaged, contact National Instruments at ni.com/support.")]
  ErrorDevCannotBeAccessed = DAQmxErrorDevCannotBeAccessed,
  #[error("Sample Clock Rate must match the frequency of the internal timebase specified as the Sample Clock Source. To use the specified Sample Clock Rate, set the Sample Clock Source to OnboardClock. To use the specified timebase as the Sample Clock, set the Sample Clock Rate to the frequency of that timebase.")]
  ErrorSampClkRateDoesntMatchSampClkSrc =
    DAQmxErrorSampClkRateDoesntMatchSampClkSrc,
  #[error("Sample Clock Rate requested is supported only if Enhanced Alias Rejection Enable is True. Set Enhanced Alias Rejection Enable to True or increase the Sample Clock Rate.")]
  ErrorSampClkRateNotSupportedWithEARDisabled =
    DAQmxErrorSampClkRateNotSupportedWithEARDisabled,
  #[error("DAQmx Events are not supported in this version of LabVIEW. LabVIEW 8.0 or later is required to use DAQmx Events.")]
  ErrorLabVIEWVersionDoesntSupportDAQmxEvents =
    DAQmxErrorLabVIEWVersionDoesntSupportDAQmxEvents,
  #[error("Requested property, Ready For New Value, is not supported when the Sample Timing Type is On Demand. To use the Ready For New Value property, change the Sample Timing Type.")]
  ErrorCOReadyForNewValNotSupportedWithOnDemand =
    DAQmxErrorCOReadyForNewValNotSupportedWithOnDemand,
  #[error("Hardware Timed Single Point is not a supported Sample Mode for the specified Measurement Type. Change Sample Mode and/or Measurement Type.")]
  ErrorCIHWTimedSinglePointNotSupportedForMeasType =
    DAQmxErrorCIHWTimedSinglePointNotSupportedForMeasType,
  #[error("Requested Sample Timing Type value, On Demand, is not supported when Sample Mode is Hardware Timed Single Point. Change Sample Timing Type and/or Sample Mode.")]
  ErrorOnDemandNotSupportedWithHWTimedSinglePoint =
    DAQmxErrorOnDemandNotSupportedWithHWTimedSinglePoint,
  #[error("Data Transfer Mechanism is not set to Programmed I/O, which is the only value supported when the Sample Mode is Hardware Timed Single Point. Set your Data Transfer Mechanism to Programmed I/O or change the Sample Mode.")]
  ErrorHWTimedSinglePointAndDataXferNotProgIO =
    DAQmxErrorHWTimedSinglePointAndDataXferNotProgIO,
  #[error("Requested Memory Mapping for Programmed IO Enable value, True, is not supported when Sample Mode is set to Hardware Timed Single Point. Change one or both of the properties.")]
  ErrorMemMapAndHWTimedSinglePoint = DAQmxErrorMemMapAndHWTimedSinglePoint,
  #[error("Requested property cannot be set while the task is running and the Sample Mode is set to Hardware Timing Single Point. Use DAQmx Write instead of setting the property.")]
  ErrorCannotSetPropertyWhenHWTimedSinglePointTaskIsRunning =
    DAQmxErrorCannotSetPropertyWhenHWTimedSinglePointTaskIsRunning,
  #[error("DAQmx Write failed, because it was called before the previously written value was output. This is likely a result of the sample clock period being shorter than the period of the generated pulse train. To correct this issue, increase your sample clock period and/or reduce the period of the generated pulse train.")]
  ErrorCTROutSampClkPeriodShorterThanGenPulseTrainPeriod =
    DAQmxErrorCTROutSampClkPeriodShorterThanGenPulseTrainPeriod,
  #[error("DAQmx Software Events are generated too quickly for the driver to keep up, and some of them have been lost. Reduce the rate at which your application is generating the events.  Consider reconfiguring the events you are using, or using different events.")]
  ErrorTooManyEventsGenerated = DAQmxErrorTooManyEventsGenerated,
  #[error("Task cannot be stopped, because at least one installed event handler has not been removed. Remove all installed event handlers by calling CNiDAQmxEvent::RemoveEventHandler or CNiDAQmxEvent::RemoveAllEventHandlers.  See the documentation for more information.")]
  ErrorMStudioCppRemoveEventsBeforeStop =
    DAQmxErrorMStudioCppRemoveEventsBeforeStop,
  #[error("All synchronous events for the task must be registered from the same thread.")]
  ErrorCAPICannotRegisterSyncEventsFromMultipleThreads =
    DAQmxErrorCAPICannotRegisterSyncEventsFromMultipleThreads,
  #[error("Combination of requested values for Read Wait Mode and Wait for Next Sample Clock Wait Mode properties is not supported for the given task on this device. Set both properties to Wait for Interrupt or set Read Wait Mode to a value other than Wait for Interrupt.")]
  ErrorReadWaitNextSampClkWaitMismatchTwo =
    DAQmxErrorReadWaitNextSampClkWaitMismatchTwo,
  #[error("Combination of requested values for Read Wait Mode and Wait ffor Next Sample Clock Wait Mode properties is not supported for the given task on this device. Set both properties to Wait for Interrupt or do not set either of the properties to Wait for Interrupt.")]
  ErrorReadWaitNextSampClkWaitMismatchOne =
    DAQmxErrorReadWaitNextSampClkWaitMismatchOne,
  #[error("DAQmx Signal Event type requested is not supported by the channel types or the devices in your task. DAQmx Signal events include the Counter Output event, the Sample Complete Event, the Sample Clock Event, and the Digital Change Detection event. Refer to product documentation for more details on which DAQmx Signal Events are supported by the channel types and devices in your task.")]
  ErrorDAQmxSignalEventTypeNotSupportedByChanTypesOrDevicesInTask =
    DAQmxErrorDAQmxSignalEventTypeNotSupportedByChanTypesOrDevicesInTask,
  #[error("DAQmx software event cannot be unregistered because the task is running. Unregister all your DAQmx software events prior to starting the task.")]
  ErrorCannotUnregisterDAQmxSoftwareEventWhileTaskIsRunning =
    DAQmxErrorCannotUnregisterDAQmxSoftwareEventWhileTaskIsRunning,
  #[error("DAQmx Write parameter Auto Start cannot be set to True when one or more DAQmx events are registered for the task. Set Auto Start to false and start the task manually.")]
  ErrorAutoStartWriteNotAllowedEventRegistered =
    DAQmxErrorAutoStartWriteNotAllowedEventRegistered,
  #[error("Auto Start cannot be set to True when one or more DAQmx events are registered for the task. Set Auto Start to False and start the task manually.")]
  ErrorAutoStartReadNotAllowedEventRegistered =
    DAQmxErrorAutoStartReadNotAllowedEventRegistered,
  #[error("You only can get the specified property while the task is reserved, committed or while the task is running. Reserve, commit or start the task prior to getting the property.")]
  ErrorCannotGetPropertyWhenTaskNotReservedCommittedOrRunning =
    DAQmxErrorCannotGetPropertyWhenTaskNotReservedCommittedOrRunning,
  #[error("DAQmx Signal Events are not supported by your device. DAQmx Signal events include the Counter Output event, the Sample Complete Event, the Sample Clock Event, and the Digital Change Detection event.")]
  ErrorSignalEventsNotSupportedByDevice =
    DAQmxErrorSignalEventsNotSupportedByDevice,
  #[error("DAQmx Every N Samples Acquired into Buffer Event is not supported by the channel types or devices in your task.")]
  ErrorEveryNSamplesAcqIntoBufferEventNotSupportedByDevice =
    DAQmxErrorEveryNSamplesAcqIntoBufferEventNotSupportedByDevice,
  #[error("DAQmx Every N Samples Transferred from Buffer Event is not supported by the  channel types or  devices in your task.")]
  ErrorEveryNSampsTransferredFromBufferEventNotSupportedByDevice =
    DAQmxErrorEveryNSampsTransferredFromBufferEventNotSupportedByDevice,
  #[error("When you use synchronous events, you can clear, stop, abort, unreserve, or start a task only from the thread in which you registered synchronous events.")]
  ErrorCAPISyncEventsTaskStateChangeNotAllowedFromDifferentThread =
    DAQmxErrorCAPISyncEventsTaskStateChangeNotAllowedFromDifferentThread,
  #[error("DAQmx Software Events cannot be registered with different call mechanisms on the same task. The software events for a task must all be registered with synchronous callbacks or they must be all registered with asynchronous callbacks.")]
  ErrorDAQmxSWEventsWithDifferentCallMechanisms =
    DAQmxErrorDAQmxSWEventsWithDifferentCallMechanisms,
  #[error("Channel specified cannot be saved with Allow Interactive Editing set to True, because the DAQ Assistant does not support polynomial calibration scales. Save the channel  with Allow Interactive Editing set to False, or use a table calibration scale.")]
  ErrorCantSaveChanWithPolyCalScaleAndAllowInteractiveEdit =
    DAQmxErrorCantSaveChanWithPolyCalScaleAndAllowInteractiveEdit,
  #[error("Thermocouple CJC (cold junction compensation) Channel specified cannot be used for CJC because the corresponding physical channel does not support temperature measurement. Select a different CJC Channel, set CJC Source to Internal, or set CJC Source to Constant Value and use CJC Value to specify the temperature of the cold junction.")]
  ErrorChanDoesNotSupportCJC = DAQmxErrorChanDoesNotSupportCJC,
  #[error("Querying the Counter Output Ready for New Value property is not supported by this device when the Sample Mode is set to Hardware Timed Single Point. Use DAQmx Wait for Next Sample Clock before DAQmx Write to make sure the counter is ready to accept the new value.")]
  ErrorCOReadyForNewValNotSupportedWithHWTimedSinglePoint =
    DAQmxErrorCOReadyForNewValNotSupportedWithHWTimedSinglePoint,
  #[error("Allow Connecting DAC Reference to Ground at Runtime set to True is not supported by this device when DAC Reference Voltage Source is set to External.")]
  ErrorDACAllowConnToGndNotSupportedByDevWhenRefSrcExt =
    DAQmxErrorDACAllowConnToGndNotSupportedByDevWhenRefSrcExt,
  #[error("Property cannot be queried because the task is not running. Start the task prior to getting the specified property.")]
  ErrorCantGetPropertyTaskNotRunning = DAQmxErrorCantGetPropertyTaskNotRunning,
  #[error("Property cannot be set because the task is not running. Start the task prior to setting the specified property.")]
  ErrorCantSetPropertyTaskNotRunning = DAQmxErrorCantSetPropertyTaskNotRunning,
  #[error("Property cannot be set because the task is not running or committed. Start or commit the task prior to setting the specified property.")]
  ErrorCantSetPropertyTaskNotRunningCommitted =
    DAQmxErrorCantSetPropertyTaskNotRunningCommitted,
  #[error("Every N Samples Event Interval requested must be an integer multiple of two for analog input tasks on this device.")]
  ErrorAIEveryNSampsEventIntervalNotMultipleOf2 =
    DAQmxErrorAIEveryNSampsEventIntervalNotMultipleOf2,
  #[error("TEDS operation failed because the corresponding physical channel is not an analog input channel.")]
  ErrorInvalidTEDSPhysChanNotAI = DAQmxErrorInvalidTEDSPhysChanNotAI,
  #[error("Requested operation cannot be performed inside the aysnchronous DAQmx Event callback thread. Use synchronous callback mechanism or perform the operation in a different thread.")]
  ErrorCAPICannotPerformTaskOperationInAsyncCallback =
    DAQmxErrorCAPICannotPerformTaskOperationInAsyncCallback,
  #[error("Every N Samples Transferred From Buffer Event registration has failed because the event is already registered within the task. Unregister the event before registering it again.")]
  ErrorEveryNSampsTransferredFromBufferEventAlreadyRegistered =
    DAQmxErrorEveryNSampsTransferredFromBufferEventAlreadyRegistered,
  #[error("Every N Samples Acquired Into Buffer Event registration has failed because the event is already registered within the task. Unregister the event before registering it again.")]
  ErrorEveryNSampsAcqIntoBufferEventAlreadyRegistered =
    DAQmxErrorEveryNSampsAcqIntoBufferEventAlreadyRegistered,
  #[error("Every N Samples Transferred from Buffer Event cannot be registered, because it is not supported for input tasks. Use the Every N Samples Acquired Into Buffer Event.")]
  ErrorEveryNSampsTransferredFromBufferNotForInput =
    DAQmxErrorEveryNSampsTransferredFromBufferNotForInput,
  #[error("Every N Samples Acquired Into Buffer Event cannot be registered, because it is not supported for output tasks. Use the Every N Samples Transferred From Buffer Event.")]
  ErrorEveryNSampsAcqIntoBufferNotForOutput =
    DAQmxErrorEveryNSampsAcqIntoBufferNotForOutput,
  #[error("Requested Sample Timing Type is not allowed, because there is already another task with analog output channels from the same device configured for a different Sample Timing Type. This is not supported on this device. Change your application so that all the channels from this device are used in one task, set Sample Timing Type to On Demand for all tasks, or consider using two devices for the two tasks.")]
  ErrorAOSampTimingTypeDifferentIn2Tasks =
    DAQmxErrorAOSampTimingTypeDifferentIn2Tasks,
  #[error("Firmware for this device could not be downloaded, and the device cannot be used. The failure may be due to damaged hardware. Contact National Instruments support at ni.com/support")]
  ErrorCouldNotDownloadFirmwareHWDamaged =
    DAQmxErrorCouldNotDownloadFirmwareHWDamaged,
  #[error("Firmware for the device could not be downloaded, and the device cannot be used. This failure is due to a missing or damaged firmware image file. Reinstall the driver to eliminate this error.")]
  ErrorCouldNotDownloadFirmwareFileMissingOrDamaged =
    DAQmxErrorCouldNotDownloadFirmwareFileMissingOrDamaged,
  #[error("DAQmx software event cannot be registered because the task is running. Register all your DAQmx software events prior to starting the task.")]
  ErrorCannotRegisterDAQmxSoftwareEventWhileTaskIsRunning =
    DAQmxErrorCannotRegisterDAQmxSoftwareEventWhileTaskIsRunning,
  #[error("2 channels in the task have different raw data compression property values, which is not supported.  All channels in the task must have the same raw data compression property values.")]
  ErrorDifferentRawDataCompression = DAQmxErrorDifferentRawDataCompression,
  #[error("TEDS interface device configured in MAX was not detected. Make sure that the type of TEDS interface device configured in MAX is correct and that the device is properly connected.")]
  ErrorConfiguredTEDSInterfaceDevNotDetected =
    DAQmxErrorConfiguredTEDSInterfaceDevNotDetected,
  #[error("Compressed Sample Size exceeds the Resolution of the channel. Configure the Compressed Sample Size to be less than or equal to the channel Resolution.")]
  ErrorCompressedSampSizeExceedsResolution =
    DAQmxErrorCompressedSampSizeExceedsResolution,
  #[error("Raw data compression has been configured for a channel that does not support raw data compression. Remove the channel from the task or set the Raw Data Compression Type to None.")]
  ErrorChanDoesNotSupportCompression = DAQmxErrorChanDoesNotSupportCompression,
  #[error("2 channels in the task have different raw data format property values, which is not supported. All channels in the task must have identical raw data format property values when raw data compression is configured. Only include channels with identical raw data format property values in the task when compression is configured.")]
  ErrorDifferentRawDataFormats = DAQmxErrorDifferentRawDataFormats,
  #[error("Sample Clock Output Terminal cannot include the Start Trigger Source terminal in the same task for this device.")]
  ErrorSampClkOutputTermIncludesStartTrigSrc =
    DAQmxErrorSampClkOutputTermIncludesStartTrigSrc,
  #[error("Start Trigger Source cannot be the same as the Sample Clock Source in the same task for this device.")]
  ErrorStartTrigSrcEqualToSampClkSrc = DAQmxErrorStartTrigSrcEqualToSampClkSrc,
  #[error("Event Output Terminal cannot include the Trigger Source terminal in the same task for this device.")]
  ErrorEventOutputTermIncludesTrigSrc =
    DAQmxErrorEventOutputTermIncludesTrigSrc,
  #[error("DAQmx Write for counter output detected that no sample clock has occurred since the last call to write which means that the writes are happening at a rate that exceeds the sample clock rate. To avoid this problem use the Wait for Next Sample Clock in your application.")]
  ErrorCOMultipleWritesBetweenSampClks =
    DAQmxErrorCOMultipleWritesBetweenSampClks,
  #[error("Done Event registration has  failed because the event is already registered within the task. Unregister the event before registering it again.")]
  ErrorDoneEventAlreadyRegistered = DAQmxErrorDoneEventAlreadyRegistered,
  #[error("You can only register one DAQmx Signal Event at a time on a task.  DAQmx Signal events include the Counter Output event, the Sample Complete Event, the Sample Clock Event, and the Digital Change Detection event. Unregister the event before registering it again.")]
  ErrorSignalEventAlreadyRegistered = DAQmxErrorSignalEventAlreadyRegistered,
  #[error("DAQmx tasks cannot provide a source for a Timed Loop and contain a DAQmx Signal Event at the same time. DAQmx Signal events include the Counter Output event, the Sample Complete Event, the Sample Clock Event, and the Digital Change Detection event. Tasks that contain a Timed Loop can contain DAQmx Events as long as the events are not a type of DAQmx Signal Event. See documentation for more details.")]
  ErrorCannotHaveTimedLoopAndDAQmxSignalEventsInSameTask =
    DAQmxErrorCannotHaveTimedLoopAndDAQmxSignalEventsInSameTask,
  #[error("DAQmx Events are not supported in this version of LabVIEW. To use DAQmx Events, install LabVIEW 7.1 and the LabVIEW 7.1.1 patch. The patch is available at ni.com/downloads.")]
  ErrorNeedLabVIEW711PatchToUseDAQmxEvents =
    DAQmxErrorNeedLabVIEW711PatchToUseDAQmxEvents,
  #[error("Task could not be started, because the driver could not write enough data to the device.  This was due to system and/or bus-bandwidth limitations. Reduce the number of programs your computer is executing concurrently.  If possible, perform operations with heavy bus usage sequentially instead of in parallel.  If you can't eliminate the problem, contact National Instruments support at ni.com/support.")]
  ErrorStartFailedDueToWriteFailure = DAQmxErrorStartFailedDueToWriteFailure,
  #[error("Operation failed because the Data Transfer Request Condition is set to Onboard Memory Custom Threshold. This device supports this Data Transfer Request Condition only when the Data Transfer Mechanism is set to DMA. To use the specified Data Transfer Request Condition, set Data Transfer Mechanism to DMA.  Otherwise, specify a different Data Transfer Request Condition.")]
  ErrorDataXferCustomThresholdNotDMAXferMethodSpecifiedForDev =
    DAQmxErrorDataXferCustomThresholdNotDMAXferMethodSpecifiedForDev,
  #[error("Operation failed because the Data Transfer Custom Threshold property is set, and no value is specified for the Data Transfer Request Condition. To use the specified Data Transfer Custom Threshold, set Data Transfer Request Condition to Onboard Memory Custom Threshold.  If you set Data Transfer Request Condition to any value other than Onboard Memory Custom Threshold, the Data Transfer Custom Threshold property is ignored.")]
  ErrorDataXferRequestConditionNotSpecifiedForCustomThreshold =
    DAQmxErrorDataXferRequestConditionNotSpecifiedForCustomThreshold,
  #[error("Operation failed because the Data Transfer Custom Threshold property is not set, and the Data Transfer Request Condition is set to Onboard Memory Custom Threshold. Specify a value for the Data Transfer Custom Threshold, or change the Data Transfer Request Condition.")]
  ErrorDataXferCustomThresholdNotSpecified =
    DAQmxErrorDataXferCustomThresholdNotSpecified,
  #[error("Call mechanism set to synchronous event callbacks is not supported for DAQmx events on this platform. To use DAQmx events on this platform, set the call mechanism to asynchronous event callbacks.")]
  ErrorCAPISyncCallbackNotSupportedOnThisPlatform =
    DAQmxErrorCAPISyncCallbackNotSupportedOnThisPlatform,
  #[error("Operation cannot be performed, because the Channel Calibration Polynomial Reverse Coefficients property is not specified when the Channel Calibration Scale Type is Polynomial.")]
  ErrorCalChanReversePolyCoefNotSpecd =
    DAQmxErrorCalChanReversePolyCoefNotSpecd,
  #[error("Operation cannot be performed, because the Channel Calibration Polynomial Forward Coefficients property is not specified when the Channel Calibration Scale Type is Polynomial.")]
  ErrorCalChanForwardPolyCoefNotSpecd =
    DAQmxErrorCalChanForwardPolyCoefNotSpecd,
  #[error("The Channel Calibration Table Scale Pre-Scaled Values specified is not supported, because one of the numbers appears more than once in the specified array. Ensure unique numbers are specified in the array.")]
  ErrorChanCalRepeatedNumberInPreScaledVals =
    DAQmxErrorChanCalRepeatedNumberInPreScaledVals,
  #[error("Operation cannot be performed, because the number of elements in the array specified for the Channel Calibration Table Pre-Scaled Values property is not equal to the number of elements in the array specified for Channel Calibration Table Scaled Values.")]
  ErrorChanCalTableNumScaledNotEqualNumPrescaledVals =
    DAQmxErrorChanCalTableNumScaledNotEqualNumPrescaledVals,
  #[error("Operation cannot be performed, because the Channel Calibration Table Scaled Values property is not specified when the Channel Calibration Scale Type is Table.")]
  ErrorChanCalTableScaledValsNotSpecd =
    DAQmxErrorChanCalTableScaledValsNotSpecd,
  #[error("Operation cannot be performed, because the Channel Calibration Table Pre-Scaled Values property is not specified when the Channel Calibration Scale Type is Table.")]
  ErrorChanCalTablePreScaledValsNotSpecd =
    DAQmxErrorChanCalTablePreScaledValsNotSpecd,
  #[error("Operation cannot be performed, because the Channel Calibration Scale Type is not specified and Channel Calibration Enable property is set to True. To use channel calibration, specify the Scale Type; otherwise, set Channel Calibration Enable to false.")]
  ErrorChanCalScaleTypeNotSet = DAQmxErrorChanCalScaleTypeNotSet,
  #[error("Operation cannot be performed, because the Channel Calibration Expiration Date has passed, and the Channel Calibration Apply Calibration if Expired property is False. Update the channel calibration, including the Expiration Date, or set Apply Calibration If Expired to True.")]
  ErrorChanCalExpired = DAQmxErrorChanCalExpired,
  #[error("Operation cannot be performed, because the Channel Calibration Expiration Date is not specified, and Channel Calibration Enable property is set to True. To use channel calibration, specify the Expiration Date; otherwise, set Channel Calibration Enable to false.")]
  ErrorChanCalExpirationDateNotSet = DAQmxErrorChanCalExpirationDateNotSet,
  #[error("Three ports alone are not supported given the Sample Timing Type on this device. Specify four ports (0, 1, 2, and 3).  Ensure the lines from the unwanted port are unwired, tristated, or are connected so no equipment can be damaged.")]
  ErrorThreeOutputPortCombinationGivenSampTimingType653x =
    DAQmxError3OutputPortCombinationGivenSampTimingType653x,
  #[error("Three ports alone are not supported given the Sample Timing Type on this device. Specify four ports (0, 1, 2, and 3), and disregard data from the unwanted port.")]
  ErrorThreeInputPortCombinationGivenSampTimingType653x =
    DAQmxError3InputPortCombinationGivenSampTimingType653x,
  #[error("2 specified ports alone are not supported given the Sample Timing Type on this device. To use only two ports, specify ports 0 and 1, or ports 2 and 3. To use the two ports specified, use four ports (0, 1, 2, and 3) on the device.  Ensure the lines from the unwanted ports are unwired, tristated, or are connected so no equipment can be damaged.")]
  Error2OutputPortCombinationGivenSampTimingType653x =
    DAQmxError2OutputPortCombinationGivenSampTimingType653x,
  #[error("2 specified ports alone are not supported given the Sample Timing Type on this device. To use only two ports, specify ports 0 and 1, or ports 2 and 3. To use the two ports specified, use four ports (0, 1, 2, and 3) on the device, and disregard data from the unwanted ports.")]
  Error2InputPortCombinationGivenSampTimingType653x =
    DAQmxError2InputPortCombinationGivenSampTimingType653x,
  #[error(
    "Pattern match hardware for this device can only be used for one trigger."
  )]
  ErrorPatternMatcherMayBeUsedByOneTrigOnly =
    DAQmxErrorPatternMatcherMayBeUsedByOneTrigOnly,
  #[error("Trigger Type was set to Digital Pattern Match but no physical channels were specified as the Digital Pattern Source. Specify physical channels for property.")]
  ErrorNoChansSpecdForPatternSource = DAQmxErrorNoChansSpecdForPatternSource,
  #[error("Rising and Falling Edge Physical Channels for Change Detection requested are not supported because at least one of the corresponding channels is not in the task. Ensure all the corresponding channels are in the task or choose different channels.")]
  ErrorChangeDetectionChanNotInTask = DAQmxErrorChangeDetectionChanNotInTask,
  #[error("Rising and Falling Edge Physical Channels for Change Detection requested are not supported because at least one of the channels is not tristated. Ensure all the corresponding channels are tristated or choose different channels.")]
  ErrorChangeDetectionChanNotTristated =
    DAQmxErrorChangeDetectionChanNotTristated,
  #[error("Wait Mode specified is not supported for the given non-buffered task. Specify a different Wait Mode, or change the task to be buffered.")]
  ErrorWaitModeValueNotSupportedNonBuffered =
    DAQmxErrorWaitModeValueNotSupportedNonBuffered,
  #[error("Wait Mode property is not supported for the given non-buffered task. Do not use this property inside the task, or change the task to be buffered.")]
  ErrorWaitModePropertyNotSupportedNonBuffered =
    DAQmxErrorWaitModePropertyNotSupportedNonBuffered,
  #[error("Channel specified cannot be saved with interactive editing allowed, because the DAQ Assistant does not support digital channels with different settings for different lines. Save the channel with 'allow interactive editing' set to false.  Alternatively, set all the lines in the channel identically, or break the channel up into channels with individual lines.")]
  ErrorCantSavePerLineConfigDigChanSoInteractiveEditsAllowed =
    DAQmxErrorCantSavePerLineConfigDigChanSoInteractiveEditsAllowed,
  #[error("Channel specified cannot be saved with interactive editing allowed, because the only digital channels with multiple lines supported by the DAQ Assistant are entire ports. Save the channel with 'allow interactive editing' set to false.  Alternatively, change the channel so it contains an entire port, or break it up into channels with individual lines.")]
  ErrorCantSaveNonPortMultiLineDigChanSoInteractiveEditsAllowed =
    DAQmxErrorCantSaveNonPortMultiLineDigChanSoInteractiveEditsAllowed,
  #[error("Requested Every N Samples Event Interval is not supported for the given buffer size. Modify the buffer size and/or the Every N Samples Event Interval so the buffer size is an even multiple of the Every N Samples Event Interval.")]
  ErrorBufferSizeNotMultipleOfEveryNSampsEventIntervalNoIrqOnDev =
    DAQmxErrorBufferSizeNotMultipleOfEveryNSampsEventIntervalNoIrqOnDev,
  #[error("Task name specified is already used for a global channel in the Data Neighborhood. NI-DAQmx does not support overlapping task and global channel names. Select a different name.")]
  ErrorGlobalTaskNameAlreadyChanName = DAQmxErrorGlobalTaskNameAlreadyChanName,
  #[error("Global channel name specified is already used for a task in the Data Neighborhood. NI-DAQmx does not support overlapping task and global channel names. Select a different name.")]
  ErrorGlobalChanNameAlreadyTaskName = DAQmxErrorGlobalChanNameAlreadyTaskName,
  #[error("Every N Samples Event Interval requested must be an integer multiple of two for analog output tasks on this device.")]
  ErrorAOEveryNSampsEventIntervalNotMultipleOf2 =
    DAQmxErrorAOEveryNSampsEventIntervalNotMultipleOf2,
  #[error("Sample Timebase Divisor requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Sample Timebase Divisor property. NI-DAQmx automatically selects a compatible Sample Timebase Divisor. To use the requested Sample Timebase Divisor, select a different Timing Type.")]
  ErrorSampleTimebaseDivisorNotSupportedGivenTimingType =
    DAQmxErrorSampleTimebaseDivisorNotSupportedGivenTimingType,
  #[error("Handshake Event Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Handshake Event Output Terminal property. NI-DAQmx automatically selects a compatible Handshake Event Output Terminal. To use the requested Handshake Event Output Terminal, select a different Timing Type.")]
  ErrorHandshakeEventOutputTermNotSupportedGivenTimingType =
    DAQmxErrorHandshakeEventOutputTermNotSupportedGivenTimingType,
  #[error("Change Detection Event Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Change Detection Event Output Terminal property. NI-DAQmx automatically selects a compatible Change Detection Output Terminal. To use the requested Change Detection Event Output Terminal, select a different Timing Type.")]
  ErrorChangeDetectionOutputTermNotSupportedGivenTimingType =
    DAQmxErrorChangeDetectionOutputTermNotSupportedGivenTimingType,
  #[error("Ready For Transfer Event Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Ready For Transfer Event Output Terminal property. NI-DAQmx automatically selects a compatible Reference Trigger Event Output Terminal. To use the requested Ready For Transfer Trigger Event Output Terminal, select a different Timing Type.")]
  ErrorReadyForTransferOutputTermNotSupportedGivenTimingType =
    DAQmxErrorReadyForTransferOutputTermNotSupportedGivenTimingType,
  #[error("Reference Trigger Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Reference Trigger Output Terminal property. NI-DAQmx automatically selects a compatible Reference Trigger Output Terminal. To use the requested Reference Trigger Output Terminal, select a different Timing Type.")]
  ErrorRefTrigOutputTermNotSupportedGivenTimingType =
    DAQmxErrorRefTrigOutputTermNotSupportedGivenTimingType,
  #[error("Start Trigger Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Start Trigger Output Terminal property. NI-DAQmx automatically selects a compatible Start Trigger Output Terminal. To use the requested Start Trigger Output Terminal, select a different Timing Type.")]
  ErrorStartTrigOutputTermNotSupportedGivenTimingType =
    DAQmxErrorStartTrigOutputTermNotSupportedGivenTimingType,
  #[error("Sample Clock Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Sample Clock Output Terminal property. NI-DAQmx automatically selects a compatible Sample Clock Output Terminal. To use the requested Sample Clock Output Terminal, select a different Timing Type.")]
  ErrorSampClockOutputTermNotSupportedGivenTimingType =
    DAQmxErrorSampClockOutputTermNotSupportedGivenTimingType,
  #[error("20 Mhz Timebase Output Terminal requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the 20 Mhz Timebase Output Terminal property. NI-DAQmx automatically selects a compatible20 Mhz Timebase Output Terminal. To use the requested 20 Mhz Timebase Output Terminal, select a different Timing Type.")]
  ErrorMhz20TimebaseNotSupportedGivenTimingType =
    DAQmxError20MhzTimebaseNotSupportedGivenTimingType,
  #[error("Sample Clock Source requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Sample Clock Source property. NI-DAQmx automatically selects a compatible Sample Clock Source setting. To use the requested Sample Clock Source, select a different Timing Type.")]
  ErrorSampClockSourceNotSupportedGivenTimingType =
    DAQmxErrorSampClockSourceNotSupportedGivenTimingType,
  #[error("Reference Trigger Type requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Reference Trigger Type property. NI-DAQmx automatically selects a compatible Reference Trigger Type setting. To use the requested Reference Trigger Type, select a different Timing Type.")]
  ErrorRefTrigTypeNotSupportedGivenTimingType =
    DAQmxErrorRefTrigTypeNotSupportedGivenTimingType,
  #[error("Pause Trigger Type requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Pause Trigger Type property. NI-DAQmx automatically selects a compatible Pause Trigger Type setting. To use the requested Pause Trigger Type, select a different Timing Type.")]
  ErrorPauseTrigTypeNotSupportedGivenTimingType =
    DAQmxErrorPauseTrigTypeNotSupportedGivenTimingType,
  #[error("Handshake Trigger Type requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Handshake Trigger Type property. NI-DAQmx automatically selects a compatible Handshake Trigger Type setting. To use the requested Handshake Trigger Type, select a different Timing Type.")]
  ErrorHandshakeTrigTypeNotSupportedGivenTimingType =
    DAQmxErrorHandshakeTrigTypeNotSupportedGivenTimingType,
  #[error("Start Trigger Type requested is not supported given the requested Timing Type. To use the requested Timing Type, do not set the Start Trigger Type property. NI-DAQmx automatically selects a compatible Start Trigger Type setting. To use the requested Start Trigger Type, select a different Timing Type.")]
  ErrorStartTrigTypeNotSupportedGivenTimingType =
    DAQmxErrorStartTrigTypeNotSupportedGivenTimingType,
  #[error(
        "Reference clock is not supported by this device. Do not set the Reference Clock property."
    )]
  ErrorRefClkSrcNotSupported = DAQmxErrorRefClkSrcNotSupported,
  #[error(
        "Data Voltage Low Level and Data Voltage High Level must be within a common voltage range."
    )]
  ErrorDataVoltageLowAndHighIncompatible =
    DAQmxErrorDataVoltageLowAndHighIncompatible,
  #[error("Digital Pattern string specified contains an invalid character.")]
  ErrorInvalidCharInDigPatternString = DAQmxErrorInvalidCharInDigPatternString,
  #[error("Port 3 cannot be used without port 2 on this device given the Sample Timing Type. You can use ports 0 and 2 by themselves.  To use port 3, you also need to use port 2.")]
  ErrorCantUsePort3AloneGivenSampTimingTypeOn653x =
    DAQmxErrorCantUsePort3AloneGivenSampTimingTypeOn653x,
  #[error("Port 1 cannot be used without port 0 on this device given the Sample Timing Type. You can use ports 0 and 2 by themselves.  To use port 1, you also need to use port 0.")]
  ErrorCantUsePort1AloneGivenSampTimingTypeOn653x =
    DAQmxErrorCantUsePort1AloneGivenSampTimingTypeOn653x,
  #[error("Partial use of physical lines within a physical port is not supported by this device, given the requested Sample Timing Type. Consider specifying the entire port and tristating the lines you do not want driven.")]
  ErrorPartialUseOfPhysicalLinesWithinPortNotSupported653x =
    DAQmxErrorPartialUseOfPhysicalLinesWithinPortNotSupported653x,
  #[error("Task contains a physical channel not supported by this device, given the requested Sample Timing Type. To keep the Sample Timing Type, use physical lines from port0/line0 through port3/line7.  To access the requested channel, change the Sample Timing Type.")]
  ErrorPhysicalChanNotSupportedGivenSampTimingType653x =
    DAQmxErrorPhysicalChanNotSupportedGivenSampTimingType653x,
  #[error("Export of the requested trigger is supported on this device only when the corresponding Trigger Type is Digital Edge.")]
  ErrorCanExportOnlyDigEdgeTrigs = DAQmxErrorCanExportOnlyDigEdgeTrigs,
  #[error("Number of values specified in Reference Trigger Digital Pattern does not match the number of physical lines requested in Reference Trigger Digital Pattern Source. Change one or both of the properties so these two numbers are equal.")]
  ErrorRefTrigDigPatternSizeDoesNotMatchSourceSize =
    DAQmxErrorRefTrigDigPatternSizeDoesNotMatchSourceSize,
  #[error("Number of values specified via the Start Trigger Digital Pattern does not match the number of physical lines requested via the Start Trigger Digital Pattern Source. Change one or both of the properties so these two numbers are equal.")]
  ErrorStartTrigDigPatternSizeDoesNotMatchSourceSize =
    DAQmxErrorStartTrigDigPatternSizeDoesNotMatchSourceSize,
  #[error("Change Detection Rising and Falling Edge Physical Channels must be set identically on this device.")]
  ErrorChangeDetectionRisingAndFallingEdgeChanDontMatch =
    DAQmxErrorChangeDetectionRisingAndFallingEdgeChanDontMatch,
  #[error("When the Sample Timing Type is Change Detection and the Trigger Type is Digital Pattern Match, the relevant physical channels must be consistent for this device.  Specifically, the Change Detection Rising Edge Physical Channels must match the physical channels from the Trigger Digital Pattern Source for which the Trigger Digital Pattern string is 0 or 1.")]
  ErrorPhysicalChansForChangeDetectionAndPatternMatch653x =
    DAQmxErrorPhysicalChansForChangeDetectionAndPatternMatch653x,
  #[error("Export of sample clock is supported by this device only when the Sample Clock Source is OnboardClock. Consider alternative methods for gaining access to the clock signal.")]
  ErrorCanExportOnlyOnboardSampClk = DAQmxErrorCanExportOnlyOnboardSampClk,
  #[error("Active Edge requested is not supported because the Sample Clock Source is OnboardClock. To use the selected Sample Clock Source, set Sample Clock Active Edge to Rising Edge.")]
  ErrorInternalSampClkNotRisingEdge = DAQmxErrorInternalSampClkNotRisingEdge,
  #[error("Reference Trigger Digital Pattern Source requested is not supported because at least one of the corresponding channels is not in the task. Ensure all the corresponding channels are in the task or choose different channels.")]
  ErrorRefTrigDigPatternChanNotInTask =
    DAQmxErrorRefTrigDigPatternChanNotInTask,
  #[error("Reference Trigger Digital Pattern Source requested is not supported because at least one of the corresponding channels is not tristated. Ensure all the corresponding channels are tristated or choose different channels.")]
  ErrorRefTrigDigPatternChanNotTristated =
    DAQmxErrorRefTrigDigPatternChanNotTristated,
  #[error("Start Trigger Digital Pattern Source requested is not supported because at least one of the corresponding channels is not in the task. Ensure all the corresponding channels are in the task or choose different channels.")]
  ErrorStartTrigDigPatternChanNotInTask =
    DAQmxErrorStartTrigDigPatternChanNotInTask,
  #[error("Start Trigger Digital Pattern Source requested is not supported because at least one of the corresponding channels is not tristated. Ensure all the corresponding channels are tristated or choose different channels.")]
  ErrorStartTrigDigPatternChanNotTristated =
    DAQmxErrorStartTrigDigPatternChanNotTristated,
  #[error("Combination of Reference Clock Source and Sample Clock Timebase Source specified is not supported by this device. To use the Reference Clock Source specified, do not set the Sample Clock Timebase Source. NI-DAQmx will set it to its default value: OnboardClock. To use the Sample Clock Timebase Source specified, do not set the Reference Clock Source. NI-DAQmx will set it to its default value: none.")]
  ErrorPXIStarAndClock10Sync = DAQmxErrorPXIStarAndClock10Sync,
  #[error("Channel specified cannot be saved with interactive editing allowed, because the DAQ Assistant does not support at least one of the specified properties. Save the channel with 'allow interactive editing' set to false, or specify only properties supported by the DAQ Assistant.")]
  ErrorGlobalChanCannotBeSavedSoInteractiveEditsAllowed =
    DAQmxErrorGlobalChanCannotBeSavedSoInteractiveEditsAllowed,
  #[error("Task specified cannot be saved with interactive editing allowed, because the DAQ Assistant does not support at least one of the specified properties. Save the task with 'allow interactive editing' set to false, or specify only properties supported by the DAQ Assistant.")]
  ErrorTaskCannotBeSavedSoInteractiveEditsAllowed =
    DAQmxErrorTaskCannotBeSavedSoInteractiveEditsAllowed,
  #[error("Specified channel is not a valid global channel. Ensure that the Channel Name matches a channel in the Data Neighborhood in MAX. Check for typing errors.")]
  ErrorInvalidGlobalChan = DAQmxErrorInvalidGlobalChan,
  #[error("Every N Samples Event registration has failed because the event is already registered within the task. Unregister the event before registering it again.")]
  ErrorEveryNSampsEventAlreadyRegistered =
    DAQmxErrorEveryNSampsEventAlreadyRegistered,
  #[error("Zero is not a supported value for the Every N Samples Event Interval. Specify an event interval greater than zero.")]
  ErrorEveryNSampsEventIntervalZeroNotSupported =
    DAQmxErrorEveryNSampsEventIntervalZeroNotSupported,
  #[error("Specified digital channel contains more bits than supported by the 16-bit version of DAQmx Port Write. Use the version of DAQmx Port Write that supports wider digital ports.")]
  ErrorChanSizeTooBigForU16PortWrite = DAQmxErrorChanSizeTooBigForU16PortWrite,
  #[error("Specified digital channel contains more bits than supported by the 16-bit version of DAQmx Port Read. Use a version of DAQmx Port Read that supports wider digital ports.")]
  ErrorChanSizeTooBigForU16PortRead = DAQmxErrorChanSizeTooBigForU16PortRead,
  #[error("Requested Every N Samples Event Interval is not supported for the given Data Transfer Mechanism and buffer size. To keep DMA or USB Bulk as the Data Transfer Mechanism, modify the buffer size and/or the Every N Samples Event Interval so the buffer size is an even multiple of the Every N Samples Event Interval. To keep the same Every N Samples Event Interval and buffer size, change the Data Transfer Mechanism to Interrupts if supported.")]
  ErrorBufferSizeNotMultipleOfEveryNSampsEventIntervalWhenDMA =
    DAQmxErrorBufferSizeNotMultipleOfEveryNSampsEventIntervalWhenDMA,
  #[error("DAQmx Write is supported for counter output channels only while the task is running. To use DAQmx Write with the given task, invoke DAQmx Start prior to DAQmx Write.  To specify the low and/or high ticks while the task is not running, set DAQmx properties instead of invoking DAQmx Write.")]
  ErrorWriteWhenTaskNotRunningCOTicks =
    DAQmxErrorWriteWhenTaskNotRunningCOTicks,
  #[error("DAQmx Write is supported for counter output channels only while the task is running. To use DAQmx Write with the given task, invoke DAQmx Start prior to DAQmx Write.  To specify the frequency and/or the duty cycle while the task is not running, set DAQmx properties instead of invoking DAQmx Write.")]
  ErrorWriteWhenTaskNotRunningCOFreq = DAQmxErrorWriteWhenTaskNotRunningCOFreq,
  #[error("DAQmx Write is supported for counter output channels only while the task is running. To use DAQmx Write with the given task, invoke DAQmx Start prior to DAQmx Write.  To specify the low and/or high time while the task is not running, set DAQmx properties instead of invoking DAQmx Write.")]
  ErrorWriteWhenTaskNotRunningCOTime = DAQmxErrorWriteWhenTaskNotRunningCOTime,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Range Low and High. To keep the specified AO Minimum and Maximum Values, supply higher reference voltage and specify the corresponding AO DAC Range Low and High. To keep the specified AO DAC Range Low and High, change the AO Minimum and Maximum Values.")]
  ErrorAOMinMaxNotSupportedDACRangeTooSmall =
    DAQmxErrorAOMinMaxNotSupportedDACRangeTooSmall,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Range Low and High. To keep the specified AO Minimum and Maximum Values, supply higher reference voltage and specify the corresponding AO DAC Range Low and High. To keep the specified AO DAC Range Low and High, change the AO Minimum and Maximum Values. Alternatively, supply an appropriate external DAC offset and specify the corresponding AO DAC Offset Value.  When supplying an external DAC offset, to get the optimum accuracy, you should manually calibrate the offset.  Refer to user documentation for details.")]
  ErrorAOMinMaxNotSupportedGivenDACRange =
    DAQmxErrorAOMinMaxNotSupportedGivenDACRange,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Range Low and High, and AO DAC Offset Value. To keep the specified AO DAC Range and Offset Values, change the AO Minimum and Maximum Values. To keep the specified AO Minimum and Maximum Values, supply higher reference voltage and specify the corresponding AO DAC Range. Alternatively, supply an appropriate external DAC offset and specify the corresponding AO DAC Offset Value.  When supplying an external DAC offset, to get the optimum accuracy, you should manually calibrate the offset.  Refer to user documentation for details.")]
  ErrorAOMinMaxNotSupportedGivenDACRangeAndOffsetVal =
    DAQmxErrorAOMinMaxNotSupportedGivenDACRangeAndOffsetVal,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Offset Value. To keep the specified AO Minimum and Maximum Values, supply an appropriate offset and specify the corresponding AO DAC Offset Value. To keep the specified AO DAC Offset Value, change the AO Minimum and Maximum Values.")]
  ErrorAOMinMaxNotSupportedDACOffsetValInappropriate =
    DAQmxErrorAOMinMaxNotSupportedDACOffsetValInappropriate,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Offset Value. To keep the specified AO Minimum and Maximum Values, supply an appropriate offset and specify the corresponding AO DAC Offset Value. To keep the specified AO DAC Offset Value, change the AO Minimum and Maximum Values. Alternatively, supply an appropriate external DAC reference and specify the corresponding AO DAC Reference Value.")]
  ErrorAOMinMaxNotSupportedGivenDACOffsetVal =
    DAQmxErrorAOMinMaxNotSupportedGivenDACOffsetVal,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Reference Value. To keep the specified AO Minimum and Maximum Values, supply higher reference voltage and specify the corresponding AO DAC Reference Value. To keep the specified AO DAC Reference Value, change the AO Minimum and Maximum Values.")]
  ErrorAOMinMaxNotSupportedDACRefValTooSmall =
    DAQmxErrorAOMinMaxNotSupportedDACRefValTooSmall,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Reference Value. To keep the specified AO Minimum and Maximum Values, supply higher reference voltage and specify the corresponding AO DAC Reference Value. To keep the specified AO DAC Reference Value, change the AO Minimum and Maximum Values. Alternatively, supply an appropriate external DAC offset and specify the corresponding AO DAC Offset Value.  When supplying an external DAC offset, to get the optimum accuracy, you should manually calibrate the offset.  Refer to user documentation for details.")]
  ErrorAOMinMaxNotSupportedGivenDACRefVal =
    DAQmxErrorAOMinMaxNotSupportedGivenDACRefVal,
  #[error("Specified AO Maximum and Minimum Values are not supported given the specified AO DAC Reference and Offset Values. To keep the specified AO DAC Reference and Offset Values, change the AO Minimum and Maximum Values. To keep the specified AO Minimum and Maximum Values, supply higher reference voltage and specify the corresponding AO DAC Reference Value. Alternatively, supply an appropriate external DAC offset and specify the corresponding AO DAC Offset Value.  When supplying an external DAC offset, to get the optimum accuracy, you should manually calibrate the offset.  Refer to user documentation for details.")]
  ErrorAOMinMaxNotSupportedGivenDACRefAndOffsetVal =
    DAQmxErrorAOMinMaxNotSupportedGivenDACRefAndOffsetVal,
  #[error("Data Transfer Request Condition is set to When Acquisition Complete, but the Number of Samples per Channel is greater than the On Board Buffer Size. Decrease the Number of Samples per Channel, remove some channels from the task, or change the Data Transfer Request Condition.")]
  ErrorWhenAcqCompAndNumSampsPerChanExceedsOnBrdBufSize =
    DAQmxErrorWhenAcqCompAndNumSampsPerChanExceedsOnBrdBufSize,
  #[error("Data Transfer Request Condition being set to When Acquisition Complete is only supported when the Reference Trigger Type is other than None. Change the Data Transfer Request Condition or configure a reference trigger for the task.")]
  ErrorWhenAcqCompAndNoRefTrig = DAQmxErrorWhenAcqCompAndNoRefTrig,
  #[error("DAQmx Wait for Next Sample Clock is not supported by the given device for tasks containing channels of the given type or timing type. DAQmx Wait for Next Sample Clock is only supported for the hardware-timed single-point timing type.")]
  ErrorWaitForNextSampClkNotSupported =
    DAQmxErrorWaitForNextSampClkNotSupported,
  #[error("One or more devices from a multiple-device task are in an unidentified PXI chassis, which is not supported. Identify the PXI chassis in MAX.")]
  ErrorDevInUnidentifiedPXIChassis = DAQmxErrorDevInUnidentifiedPXIChassis,
  #[error("Combination of specified AI Maximum Sound Pressure Level, AI Microphone Sensitivity, and other related AI property settings is not supported by the device. Change the values of the related AI properties or do not set them at all. If you do not set the related AI properties, NI-DAQmx sets them for you.  Alternatively, consider using a microphone with lower sensitivity.")]
  ErrorMaxSoundPressureMicSensitivitRelatedAIPropertiesNotSupportedByDev =
    DAQmxErrorMaxSoundPressureMicSensitivitRelatedAIPropertiesNotSupportedByDev,
  #[error("Combination of specified AI Maximum Sound Pressure Level and AI Microphone Sensitivity settings is not supported by the device. Consider using a microphone with lower sensitivity.  If clipping signals at high levels is acceptable, you can use the microphone with specified sensitivity as long as you reduce the AI Maximum Sound Pressure.")]
  ErrorMaxSoundPressureAndMicSensitivityNotSupportedByDev =
    DAQmxErrorMaxSoundPressureAndMicSensitivityNotSupportedByDev,
  #[error("To use Sample Clock as the Sample Timing Type for analog output on this device, specify buffer size greater than 0 in DAQmx Configure Output Buffer.")]
  ErrorAOBufferSizeZeroForSampClkTimingType =
    DAQmxErrorAOBufferSizeZeroForSampClkTimingType,
  #[error("To use Sample Clock as the Sample Timing Type for analog output on this device, call DAQmx Write before DAQmx Start.")]
  ErrorAOCallWriteBeforeStartForSampClkTimingType =
    DAQmxErrorAOCallWriteBeforeStartForSampClkTimingType,
  #[error("Specified low pass cutoff frequency is not supported.")]
  ErrorInvalidCalLowPassCutoffFreq = DAQmxErrorInvalidCalLowPassCutoffFreq,
  #[error("Simulation disabling is not supported for this device, because it was created as a simulated device.")]
  ErrorSimulationCannotBeDisabledForDevCreatedAsSimulatedDev =
    DAQmxErrorSimulationCannotBeDisabledForDevCreatedAsSimulatedDev,
  #[error("Devices cannot be added to a task after configuring timing, triggering, buffers, and/or exported signals. Add all devices to the task before configuring other aspects of the task.")]
  ErrorCannotAddNewDevsAfterTaskConfiguration =
    DAQmxErrorCannotAddNewDevsAfterTaskConfiguration,
  #[error("Given devices cannot be synchronized in a multiple-device task because the Sample Clock Timebase Source specifies a different device from the Synchronization Pulse Source. Modify the Synchronization Pulse Source and/or the Sample Clock Timebase Source to be from the same device or leave one or both unspecified.")]
  ErrorDifftSyncPulseSrcAndSampClkTimebaseSrcDevMultiDevTask =
    DAQmxErrorDifftSyncPulseSrcAndSampClkTimebaseSrcDevMultiDevTask,
  #[error("Terminal specified must include the device name for the given multiple-device task. Include the device name in the terminal name. Example syntax is myDevice3/PFI4.")]
  ErrorTermWithoutDevInMultiDevTask = DAQmxErrorTermWithoutDevInMultiDevTask,
  #[error("Given devices cannot be synchronized in a multiple-device task. Ensure that one of the devices in the task is in PXI slot 2, or specify the Synchronization Pulse Source and the Sample Clock Timebase Source to be from a device in PXI slot 2, even if that device is not in the task.")]
  ErrorSyncNoDevSampClkTimebaseOrSyncPulseInPXISlot2 =
    DAQmxErrorSyncNoDevSampClkTimebaseOrSyncPulseInPXISlot2,
  #[error("Physical channel specified is not available through the cabled device connector used for the SCC carrier. To use the specified channel with an SCC, connect the SCC carrier to the appropriate connector on the cabled device and specify the new configuration through MAX.")]
  ErrorPhysicalChanNotOnThisConnector =
    DAQmxErrorPhysicalChanNotOnThisConnector,
  #[error("Number of samples to wait in a finite wait instruction must be greater than 0.")]
  ErrorNumSampsToWaitNotGreaterThanZeroInScript =
    DAQmxErrorNumSampsToWaitNotGreaterThanZeroInScript,
  #[error("Number of samples to wait in a finite wait instruction must be a multiple of the alignment quantum.")]
  ErrorNumSampsToWaitNotMultipleOfAlignmentQuantumInScript =
    DAQmxErrorNumSampsToWaitNotMultipleOfAlignmentQuantumInScript,
  #[error("DAQmx Every N Samples Event is not supported within non-buffered tasks. To receive Every N Samples Event notifications, configure your task to use buffering.")]
  ErrorEveryNSamplesEventNotSupportedForNonBufferedTasks =
    DAQmxErrorEveryNSamplesEventNotSupportedForNonBufferedTasks,
  #[error("Data Transfer Mechanism is set to Programmed I/O, which is not supported for buffered operations for this device and Channel Type. Change Data Transfer Mechanism or do not use buffering.")]
  ErrorBufferedAndDataXferPIO = DAQmxErrorBufferedAndDataXferPIO,
  #[error("Write cannot be performed when the auto start input to DAQmx Write is false, task is not running, and timing for the task is not configured or Timing Type is set to On Demand. Set auto start to true, start the task, or configure timing and specify Timing Type other than On Demand.")]
  ErrorCannotWriteWhenAutoStartFalseAndTaskNotRunning =
    DAQmxErrorCannotWriteWhenAutoStartFalseAndTaskNotRunning,
  #[error("Data Transfer Mechanism is not set to Programmed I/O or DMA, the only values supported for non-buffered operations for this device and Channel Type. Set your Data Transfer Mechanism to Programmed I/O or DMA, or use buffering.")]
  ErrorNonBufferedAndDataXferInterrupts =
    DAQmxErrorNonBufferedAndDataXferInterrupts,
  #[error("Task contains a 'freqout' counter channel, which cannot be updated while the task is running. Create separate tasks for the 'freqout' channel and the other counter channels if you wish to write to the other counter channels. Alternatively, stop the task, reprogram the counters, and restart the task.")]
  ErrorWriteFailedMultipleCtrsWithFREQOUT =
    DAQmxErrorWriteFailedMultipleCtrsWithFREQOUT,
  #[error("DAQmx Read did not complete before the arrival of three sample clocks which indicates that your program is not keeping up with the hardware clock. Slow down the hardware clock or else change your application so that it can keep up with the hardware clock.")]
  ErrorReadNotCompleteBefore3SampClkEdges =
    DAQmxErrorReadNotCompleteBefore3SampClkEdges,
  #[error("Data Transfer Mechanism is not set to Programmed I/O, the only value supported when the Sample Mode is Hardware Timed Single Point. Set your Data Transfer Mechanism to Programmed I/O or change the Sample Mode.")]
  ErrorCtrHWTimedSinglePointAndDataXferNotProgIO =
    DAQmxErrorCtrHWTimedSinglePointAndDataXferNotProgIO,
  #[error("Prescaler value requested is not supported by this device, given the requested Input Terminal. Set Prescaler to 1, or change the Input Terminal.")]
  ErrorPrescalerNot1ForInputTerminal = DAQmxErrorPrescalerNot1ForInputTerminal,
  #[error("Prescaler value requested is not supported by this device, given the requested Timebase Source. Set Prescaler to 1, or change the Timebase Source.")]
  ErrorPrescalerNot1ForTimebaseSrc = DAQmxErrorPrescalerNot1ForTimebaseSrc,
  #[error("Tristate property cannot be set to False for any channel in the task when Sample Timing Type is Sample Clock on this device. Set the Tristate property to True for all channels or change the Sample Timing Type.")]
  ErrorSampClkTimingTypeWhenTristateIsFalse =
    DAQmxErrorSampClkTimingTypeWhenTristateIsFalse,
  #[error("Output buffer size (in samples per channel) must be an integer multiple of the transfer size for this device with the current Data Transfer Mechanism. Change the output buffer size or the Data Transfer Mechanism.")]
  ErrorOutputBufferSizeNotMultOfXferSize =
    DAQmxErrorOutputBufferSizeNotMultOfXferSize,
  #[error("Samples per Channel must be an integer multiple of the transfer size for this device with the current Data Transfer Mechanism. Change Samples per Channel or the Data Transfer Mechanism.")]
  ErrorSampPerChanNotMultOfXferSize = DAQmxErrorSampPerChanNotMultOfXferSize,
  #[error("Attempt to write to the TEDS sensor failed, possibly because the sensor is not connected properly or because the sensor is defective. Make sure the TEDS sensor is properly connected. Write to the TEDS sensor again. If the write fails again, try using another TEDS sensor. You may need to have the original TEDS sensor repaired.")]
  ErrorWriteToTEDSFailed = DAQmxErrorWriteToTEDSFailed,
  #[error("SCXI device cannot be used in this task because the power to the device was turned off after the task had been created. Call DAQmx Clear Task and then create a new task to use this SCXI device.")]
  ErrorSCXIDevNotUsablePowerTurnedOff =
    DAQmxErrorSCXIDevNotUsablePowerTurnedOff,
  #[error("DAQmx Read is not supported for non-buffered acquisitions if the Auto Start property is false and the task is not running. Start the task before reading samples by calling DAQmx Start Task, set the Read.Auto Start property to true, or call DAQmx Configure Input Buffer with a buffer size greater than zero.")]
  ErrorCannotReadWhenAutoStartFalseBufSizeZeroAndTaskNotRunning =
    DAQmxErrorCannotReadWhenAutoStartFalseBufSizeZeroAndTaskNotRunning,
  #[error("DAQmx Read is not supported if the Sample Mode is Hardware Timed Single Point, the Auto Start property is false, and the task is not running. Start the task before reading samples by calling DAQmx Start Task, set the Read.Auto Start property to true, or change the Sample Mode.")]
  ErrorCannotReadWhenAutoStartFalseHWTimedSinglePtAndTaskNotRunning =
    DAQmxErrorCannotReadWhenAutoStartFalseHWTimedSinglePtAndTaskNotRunning,
  #[error("DAQmx Read is not supported if the Sample Timing Type is On Demand, the Auto Start property is false, and the task is not running. Start the task before reading samples by calling DAQmx Start Task, set the Read.Auto Start property to true, or change the Sample Timing Type.")]
  ErrorCannotReadWhenAutoStartFalseOnDemandAndTaskNotRunning =
    DAQmxErrorCannotReadWhenAutoStartFalseOnDemandAndTaskNotRunning,
  #[error("On Demand Simultaneous Analog Output Enable cannot be set to true unless Sample Timing Type is On Demand.")]
  ErrorSimultaneousAOWhenNotOnDemandTiming =
    DAQmxErrorSimultaneousAOWhenNotOnDemandTiming,
  #[error("On Demand Simultaneous Analog Output Enable and Memory Mapping for Programmed IO Enable cannot both be set to true for this device.")]
  ErrorMemMapAndSimultaneousAO = DAQmxErrorMemMapAndSimultaneousAO,
  #[error("DAQmx Write failed because the counter channels have different Output Types. Writes to multiple counter output channels are supported only when all of the counters have identical Output Types. Use identical Output Types for all channels. Alternatively, create multiple tasks (one for each Output Type).")]
  ErrorWriteFailedMultipleCOOutputTypes =
    DAQmxErrorWriteFailedMultipleCOOutputTypes,
  #[error(
    "Writing to TEDS sensors is not supported on real-time (RT) systems."
  )]
  ErrorWriteToTEDSNotSupportedOnRT = DAQmxErrorWriteToTEDSNotSupportedOnRT,
  #[error("Virtual TEDS data file being written to the TEDS sensor contains an error. Ensure your Virtual TEDS data file conforms to the specification.")]
  ErrorVirtualTEDSDataFileError = DAQmxErrorVirtualTEDSDataFileError,
  #[error("TEDS sensor data being written to the TEDS sensor contains an error. Ensure your TEDS sensor data conforms to the specification.")]
  ErrorTEDSSensorDataError = DAQmxErrorTEDSSensorDataError,
  #[error("Write failed because the data size is greater than the size of the EEPROM on the TEDS sensor. Make sure the data size does not exceed the EEPROM size.")]
  ErrorDataSizeMoreThanSizeOfEEPROMOnTEDS =
    DAQmxErrorDataSizeMoreThanSizeOfEEPROMOnTEDS,
  #[error("Attempt to write the Basic TEDS data to the EEPROM failed because the PROM on the TEDS sensor already contains Basic TEDS data. A TEDS sensor can contain the Basic TEDS data in either the PROM or the EEPROM, but not in both. Do not write the Basic TEDS data to the TEDS sensor or replace the sensor.")]
  ErrorPROMOnTEDSContainsBasicTEDSData =
    DAQmxErrorPROMOnTEDSContainsBasicTEDSData,
  #[error("Attempt to write to the PROM on the TEDS sensor failed because the PROM has already been written and it cannot be rewritten. Do not write the Basic TEDS data to the TEDS sensor or replace the sensor.")]
  ErrorPROMOnTEDSAlreadyWritten = DAQmxErrorPROMOnTEDSAlreadyWritten,
  #[error("Attempt to write to the PROM on the TEDS failed because the TEDS sensor does not contain a PROM. Write the Basic TEDS data to the EEPROM of the sensor or replace the sensor.")]
  ErrorTEDSDoesNotContainPROM = DAQmxErrorTEDSDoesNotContainPROM,
  #[error("Sample Mode of Hardware Timed Single Point is not supported for analog input channels on this type of device. Use a different Sample Mode, or select a device which supports Hardware Timed Single Point.")]
  ErrorHWTimedSinglePointNotSupportedAI =
    DAQmxErrorHWTimedSinglePointNotSupportedAI,
  #[error("Sample Mode of Hardware Timed Single Point is not supported for analog input channels on this type of device when the number of channels in the task is odd (not divisible by 2). Add a channel to the task, remove a channel from the task, or use a different Sample Mode.")]
  ErrorHWTimedSinglePointOddNumChansInAITask =
    DAQmxErrorHWTimedSinglePointOddNumChansInAITask,
  #[error("Programmed I/O is not supported as the Data Transfer Mechanism when the Use Only On Board Memory property is set to true. Change the Data Transfer Mechanism or set Use Only On Board Memory to false.")]
  ErrorCantUseOnlyOnBoardMemWithProgrammedIO =
    DAQmxErrorCantUseOnlyOnBoardMemWithProgrammedIO,
  #[error("Device has shut down because a sensor on the device detected a temperature in excess of the maximum recommended operating temperature. Possible causes incude excessive current on the device channels and inadequate chassis cooling. To use the device again, reduce the current and/or improve the chassis cooling. Ensure that the device has cooled and reset the device (either programmatically or through Measurements & Automation Explorer).")]
  ErrorSwitchDevShutDownDueToHighTemp =
    DAQmxErrorSwitchDevShutDownDueToHighTemp,
  #[error("Excitation Value can only be zero when the Input Terminal Configuration is set to Differential on this device. Change the Input Terminal Configuration or set the Excitation Value to zero.")]
  ErrorExcitationNotSupportedWhenTermCfgDiff =
    DAQmxErrorExcitationNotSupportedWhenTermCfgDiff,
  #[error("TEDS sensor specifies a value for the Minimum Electrical Value that is greater than or equal to the Maximum Electrical Value. Replace the sensor or have the sensor repaired. If the memory is the only defective part of the sensor, consider using MAX to create a Task, a Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorTEDSMinElecValGEMaxElecVal = DAQmxErrorTEDSMinElecValGEMaxElecVal,
  #[error("TEDS sensor specifies a value for the Minimum Physical Value that is greater than or equal to the Maximum Physical Value. Replace the sensor or have the sensor repaired. If the memory is the only defective part of the sensor, consider using MAX to create a Task, a Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorTEDSMinPhysValGEMaxPhysVal = DAQmxErrorTEDSMinPhysValGEMaxPhysVal,
  #[error("Onboard Clock is not supported as an Input Terminal for counter measurements. Refer to user documentation for a list of supported input terminals.")]
  ErrorCIOnboardClockNotSupportedAsInputTerm =
    DAQmxErrorCIOnboardClockNotSupportedAsInputTerm,
  #[error("Selected Sample Mode is not supported with counter input position measurements.")]
  ErrorInvalidSampModeForPositionMeas =
    DAQmxErrorInvalidSampModeForPositionMeas,
  #[error("An attempt has been made to configure a trigger when analog output Sample Mode was set to Hardware Timed Single Point. Configure the analog output sample mode to something other than Hardware Timed Single Point to use a trigger.")]
  ErrorTrigWhenAOHWTimedSinglePtSampMode =
    DAQmxErrorTrigWhenAOHWTimedSinglePtSampMode,
  #[error("Specified string contains characters that cannot be interpreted by DAQmx due to installed language support and system locale settings. If possible, do not use this character. Otherwise, ensure that the appropriate language support is installed on the system, and that the system locale is set correctly. For most Windows operating systems, this is done through the Regional Settings option in the Control Panel. For a LabVIEW RT target not running LabVIEW NXG RT, you should install \"Language Support for LabVIEW RT\" and change the locale setting for the remote system under the \"System Setting\" tab in MAX.")]
  ErrorDAQmxCantUseStringDueToUnknownChar =
    DAQmxErrorDAQmxCantUseStringDueToUnknownChar,
  #[error("Requested string contains characters that cannot be interpreted by DAQmx due to installed language support and system locale settings. Ensure that the appropriate language support is installed on the system, and that the system locale is set correctly. For most Windows operating systems, this is done through the Regional Settings option in Control Panel. For a LabVIEW RT target, you should install \"Language Support for LabVIEW RT\" and change the locale setting for the remote system under the \"System Settings\" tab in MAX.")]
  ErrorDAQmxCantRetrieveStringDueToUnknownChar =
    DAQmxErrorDAQmxCantRetrieveStringDueToUnknownChar,
  #[error("TEDS sensors cannot be cleared on real-time (RT) systems.")]
  ErrorClearTEDSNotSupportedOnRT = DAQmxErrorClearTEDSNotSupportedOnRT,
  #[error("TEDS sensors cannot be configured on real-time (RT) systems. Use MAX to configure the TEDS sensor instead.")]
  ErrorCfgTEDSNotSupportedOnRT = DAQmxErrorCfgTEDSNotSupportedOnRT,
  #[error("Requested Minimum Pulse Width cannot be applied because the programmable filter clock has already been configured with a different Minimum Pulse Width when a different terminal was configured by the same task. For this type of device, there is only one programmable filter clock per device, and the device can use only one external timebase filter at a time.")]
  ErrorProgFilterClkCfgdToDifferentMinPulseWidthBySameTask1PerDev =
    DAQmxErrorProgFilterClkCfgdToDifferentMinPulseWidthBySameTask1PerDev,
  #[error("Requested Minimum Pulse Width cannot be applied because the programmable filter clock has already been configured with a different Minimum Pulse Width by another task. For this type of device, there is only one programmable filter clock per device.")]
  ErrorProgFilterClkCfgdToDifferentMinPulseWidthByAnotherTask1PerDev =
    DAQmxErrorProgFilterClkCfgdToDifferentMinPulseWidthByAnotherTask1PerDev,
  #[error("Last External Calibration Date/Time is not available, because the last external calibration if the device was not performed using the NI-DAQmx API. Last External Calibration Date/Time will become available after you perform external calibration of the device using the NI-DAQmx API.")]
  ErrorNoLastExtCalDateTimeLastExtCalNotDAQmx =
    DAQmxErrorNoLastExtCalDateTimeLastExtCalNotDAQmx,
  #[error("Write cannot be performed before you start the task for on demand or hardware-timed single-point operations. Start the task before you write samples, set the autostart input on DAQmx Write to true, or use hardware timing with a sample mode of finite or continuous.")]
  ErrorCannotWriteNotStartedAutoStartFalseNotOnDemandHWTimedSglPt =
    DAQmxErrorCannotWriteNotStartedAutoStartFalseNotOnDemandHWTimedSglPt,
  #[error("Write cannot be performed when the task is not started, the sample timing type is something other than On Demand, and the output buffer size is zero. Call DAQmx Start before DAQmx Write, set auto start to true on DAQmx Write, modify the sample timing type, or change the output buffer size.")]
  ErrorCannotWriteNotStartedAutoStartFalseNotOnDemandBufSizeZero =
    DAQmxErrorCannotWriteNotStartedAutoStartFalseNotOnDemandBufSizeZero,
  #[error("DAQmx Create Timing Source created an invalid source because the requested signal is not supported for counter output. To use this task as the timing source with a Timed Loop, specify the Counter Output Event as the signal.")]
  ErrorCOInvalidTimingSrcDueToSignal = DAQmxErrorCOInvalidTimingSrcDueToSignal,
  #[error("Event source signal specified is not supported with the Measurement Type and/or Sample Timing Type of the task.")]
  ErrorCIInvalidTimingSrcForSampClkDueToSampTimingType =
    DAQmxErrorCIInvalidTimingSrcForSampClkDueToSampTimingType,
  #[error("DAQmx Create Timing Source created an invalid source because the specified Sample Mode is not supported when the signal is Event Counting. To use this timing source with a Timed Loop, change the Sample Mode.")]
  ErrorCIInvalidTimingSrcForEventCntDueToSampMode =
    DAQmxErrorCIInvalidTimingSrcForEventCntDueToSampMode,
  #[error("Device does not support change detection for lines that do not allow digital input. Use lines that allow digital input for change detection.")]
  ErrorNoChangeDetectOnNonInputDigLineForDev =
    DAQmxErrorNoChangeDetectOnNonInputDigLineForDev,
  #[error("An empty string was specified as a terminal name which is not supported. Specify a valid terminal name.")]
  ErrorEmptyStringTermNameNotSupported =
    DAQmxErrorEmptyStringTermNameNotSupported,
  #[error("Hardware timed non-buffered analog output could not be performed because Memory Mapping for Programmed I/O Enable was set to true. Disable memory mapping for hardware timed non-buffered analog output.")]
  ErrorMemMapEnabledForHWTimedNonBufferedAO =
    DAQmxErrorMemMapEnabledForHWTimedNonBufferedAO,
  #[error("There was an overflow of the device onboard memory while performing a hardware timed non-buffered generation. Write only one sample per channel between two consecutive sample clocks to avoid this condition.")]
  ErrorDevOnboardMemOverflowDuringHWTimedNonBufferedGen =
    DAQmxErrorDevOnboardMemOverflowDuringHWTimedNonBufferedGen,
  #[error("You cannot use DAQmx Write for multiple counter channels within one task. If appropriate, create one task per counter output channel. To update multiple counter channels within one task use counter output properties.")]
  ErrorCODAQmxWriteMultipleChans = DAQmxErrorCODAQmxWriteMultipleChans,
  #[error("For a device of this type, setting the AO Idle Output Behavior to Maintain Existing Value is not supported when analog output is synchronized.")]
  ErrorCantMaintainExistingValueAOSync =
    DAQmxErrorCantMaintainExistingValueAOSync,
  #[error("You have specified more than one physical channel which is not supported. Specify a single physical channel.")]
  ErrorMStudioMultiplePhysChansNotSupported =
    DAQmxErrorMStudioMultiplePhysChansNotSupported,
  #[error("TEDS cannot be configured for the specified channel. Ensure that your TEDS sensor is connected to the channel through a TEDS interface (for example BNC-2096, SC-2350, or SCXI-1314T), and that this interface is configured in MAX. Alternatively a virtual TEDS file can be used.")]
  ErrorCantConfigureTEDSForChan = DAQmxErrorCantConfigureTEDSForChan,
  #[error("Write cannot be performed because this version of DAQmx Write uses a data type that is too small for the channels in this task. Use a different version of DAQmx Write.")]
  ErrorWriteDataTypeTooSmall = DAQmxErrorWriteDataTypeTooSmall,
  #[error("Read cannot be performed because this version of DAQmx Read uses a data type that is too small for the channels in this task. Use a different version of DAQmx Read.")]
  ErrorReadDataTypeTooSmall = DAQmxErrorReadDataTypeTooSmall,
  #[error("Measured bridge offset is outside the limits allowed for offset nulling calibration for this device. Ensure your sensor is wired and functioning properly, and that its output offset is within device limits.")]
  ErrorMeasuredBridgeOffsetTooHigh = DAQmxErrorMeasuredBridgeOffsetTooHigh,
  #[error("Specified Start Trigger Type is not supported for counter output tasks when the Sample Mode is Hardware Timed Single Point on this type of device. Set the Start Trigger Type to None to use this Sample Mode.")]
  ErrorStartTrigConflictWithCOHWTimedSinglePt =
    DAQmxErrorStartTrigConflictWithCOHWTimedSinglePt,
  #[error("Requested Sample Clock Rate cannot be generated given the specified external Sample Clock Timebase Rate. To keep the specified Sample Clock Timebase Rate, use one of the Sample Clock Rates that can be generated.")]
  ErrorSampClkRateExtSampClkTimebaseRateMismatch =
    DAQmxErrorSampClkRateExtSampClkTimebaseRateMismatch,
  #[error("Timing source created is invalid because of the Sample Timing Type settings. To use this timing source with a Timed Loop, set the Sample Timing Type to Change Detection. You can configure the Sample Timing Type to Change Detection while setting related properties through the DAQmx Timing (Change Detection) VI or function.")]
  ErrorInvalidTimingSrcDueToSampTimingType =
    DAQmxErrorInvalidTimingSrcDueToSampTimingType,
  #[error("Virtual TEDS file could not be found at the specified location. Specify correct location for the Virtual TEDS file.")]
  ErrorVirtualTEDSFileNotFound = DAQmxErrorVirtualTEDSFileNotFound,
  #[error("Forward Coefficients for a polynomial scale are not specified. This set of coefficients must contain at least one term.")]
  ErrorMStudioNoForwardPolyScaleCoeffs =
    DAQmxErrorMStudioNoForwardPolyScaleCoeffs,
  #[error("Reverse Coefficients for a polynomial scale are not specified. This set of coefficients must contain at least one term.")]
  ErrorMStudioNoReversePolyScaleCoeffs =
    DAQmxErrorMStudioNoReversePolyScaleCoeffs,
  #[error("Forward and Reverse Coefficients for a polynomial scale are not specified. Each of these two sets of coefficients must contain at least one term. The polynomial scale class constructor has overloads that can calculate one set of coefficients from the other set if only one set is available.")]
  ErrorMStudioNoPolyScaleCoeffsUseCalc =
    DAQmxErrorMStudioNoPolyScaleCoeffsUseCalc,
  #[error("Forward Coefficients for a polynomial scale are not specified. This set of coefficients must contain at least one term. The polynomial scale class constructor has overloads that can calculate the Reverse Coefficients from the Forward Coefficients if only one set of coefficients is available.")]
  ErrorMStudioNoForwardPolyScaleCoeffsUseCalc =
    DAQmxErrorMStudioNoForwardPolyScaleCoeffsUseCalc,
  #[error("Reverse Coefficients for a polynomial scale are not specified. This set of coefficients must contain at least one term. The polynomial scale class constructor has overloads that can calculate the Reverse Coefficients from the Forward Coefficients if only one set of coefficients is available.")]
  ErrorMStudioNoReversePolyScaleCoeffsUseCalc =
    DAQmxErrorMStudioNoReversePolyScaleCoeffsUseCalc,
  #[error("Sample Mode is set to a value other than Hardware Timed Single Point. This is the only value supported for counter generations when Sample Timing Type is set to Sample Clock. Change the Sample Mode or the Sample Timing Type.")]
  ErrorCOSampModeSampTimingTypeSampClkConflict =
    DAQmxErrorCOSampModeSampTimingTypeSampClkConflict,
  #[error("Desired Minimum Pulse Width could not be produced by the device.")]
  ErrorDevCannotProduceMinPulseWidth = DAQmxErrorDevCannotProduceMinPulseWidth,
  #[error("Desired Minimum Pulse Width could not be produced. Minimum Pulse Width is affected by the Digital Filter Timebase Source and the Digital Filter Timebase Rate. To see how these two property settings can affect the Minimum Pulse Width, refer to product documentation for more details.")]
  ErrorCannotProduceMinPulseWidthGivenPropertyValues =
    DAQmxErrorCannotProduceMinPulseWidthGivenPropertyValues,
  #[error("Terminal has already been configured with a different Minimum Pulse Width by another task.")]
  ErrorTermCfgdToDifferentMinPulseWidthByAnotherTask =
    DAQmxErrorTermCfgdToDifferentMinPulseWidthByAnotherTask,
  #[error("Terminal has already been configured with a different Minimum Pulse Width by another property in this task.")]
  ErrorTermCfgdToDifferentMinPulseWidthByAnotherProperty =
    DAQmxErrorTermCfgdToDifferentMinPulseWidthByAnotherProperty,
  #[error("Digital synchronization is not available for the given terminal.")]
  ErrorDigSyncNotAvailableOnTerm = DAQmxErrorDigSyncNotAvailableOnTerm,
  #[error("Digital filtering is not available for the given terminal.")]
  ErrorDigFilterNotAvailableOnTerm = DAQmxErrorDigFilterNotAvailableOnTerm,
  #[error("Digital Filter Enable is set to true but the Minimum Pulse Width property is not configured. Configure the Minimum Pulse Width property or set Digital Filter Enable to false.")]
  ErrorDigFilterEnabledMinPulseWidthNotCfg =
    DAQmxErrorDigFilterEnabledMinPulseWidthNotCfg,
  #[error("Digital Filter Enable and Digital Synchronization Enable properties cannot be true at the same time.")]
  ErrorDigFilterAndSyncBothEnabled = DAQmxErrorDigFilterAndSyncBothEnabled,
  #[error("Data Transfer Mechanism is not set to Programmed I/O, the only value supported when the Sample Mode is Hardware Timed Single Point. Set your Data Transfer Mechanism to Programmed I/O or change the Sample Mode.")]
  ErrorHWTimedSinglePointAOAndDataXferNotProgIO =
    DAQmxErrorHWTimedSinglePointAOAndDataXferNotProgIO,
  #[error("Data Transfer Mechanism is not set to Programmed I/O, the only value supported for non-buffered analog output. Set your Data Transfer Mechanism to Programmed I/O or use buffered analog output.")]
  ErrorNonBufferedAOAndDataXferNotProgIO =
    DAQmxErrorNonBufferedAOAndDataXferNotProgIO,
  #[error("Data Transfer Mechanism is set to Programmed I/O which is not supported for buffered analog output. Change Data Transfer Mechanism or use non-buffered analog output.")]
  ErrorProgIODataXferForBufferedAO = DAQmxErrorProgIODataXferForBufferedAO,
  #[error("Legacy template ID of the TEDS sensor connected to the specified physical channel is invalid or is not supported by the driver. If the sensor is defective, replace it or have it repaired. Otherwise, consider using MAX to create a Task, Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorTEDSLegacyTemplateIDInvalidOrUnsupported =
    DAQmxErrorTEDSLegacyTemplateIDInvalidOrUnsupported,
  #[error("Mapping method of the TEDS sensor connected to the specified physical channel is invalid or is not supported by the driver. If the sensor is defective, replace it or have it repaired. Otherwise, consider using MAX to create a Task, Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorTEDSMappingMethodInvalidOrUnsupported =
    DAQmxErrorTEDSMappingMethodInvalidOrUnsupported,
  #[error("TEDS sensor connected to the specified physical channel uses a linear mapping method and specifies the linear slope to be zero. Replace the sensor or have the sensor repaired. If the memory is the only defective part of the sensor, consider using MAX to create a Task, Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorTEDSLinearMappingSlopeZero = DAQmxErrorTEDSLinearMappingSlopeZero,
  #[error("For analog input with the current Data Transfer Mechanism on this type of device, the input buffer size (in samples per channel) must be an integer multiple of the transfer size. Change the Data Transfer Mechanism or the input buffer size.")]
  ErrorAIInputBufferSizeNotMultOfXferSize =
    DAQmxErrorAIInputBufferSizeNotMultOfXferSize,
  #[error("Task cannot issue sync pulse because the task has an external sample clock timebase. For this type of device, the task can issue a sync pulse if the Sample Clock Timebase Source is Onboard Clock.")]
  ErrorNoSyncPulseExtSampClkTimebase = DAQmxErrorNoSyncPulseExtSampClkTimebase,
  #[error("Task cannot issue sync pulse because another task is currently running on this device. For this type of device, the task can issue a sync pulse if it is the only task running on the device. If your task is not being used for synchronization, set the Sync Pulse Source property to \"\" or \"None\" to avoid receiving an error.")]
  ErrorNoSyncPulseAnotherTaskRunning = DAQmxErrorNoSyncPulseAnotherTaskRunning,
  #[error("Range specified by the AO Maximum and Minimum Value, and AO Voltage Units properties does not lie within the range specified by the AO Gain property. Change the values of these properties. If you do not specify AO Gain, the driver will set it based on other properties.")]
  ErrorAOMinMaxNotInGainRange = DAQmxErrorAOMinMaxNotInGainRange,
  #[error("Range specified by the AO Maximum and Minimum Value, and AO Voltage Units properties does not lie within the range specified by the AO DAC Range High and Low properties. Change the values of these properties. If you do not specify AO DAC Range High and Low, the driver will set them based on other properties.")]
  ErrorAOMinMaxNotInDACRange = DAQmxErrorAOMinMaxNotInDACRange,
  #[error("Sample Timing Type is set to On Demand which is not supported for analog output on this device. Set Sample Timing Type to Sample Clock. You can achieve this while setting related properties through DAQmx VIs or functions for configuring timing.")]
  ErrorDevOnlySupportsSampClkTimingAO =
    DAQmxErrorDevOnlySupportsSampClkTimingAO,
  #[error("Sample Timing Type is set to On Demand which is not supported for analog input on this device. Set Sample Timing Type to Sample Clock. You can achieve this whlie setting related properties through DAQmx VIs or functions for configuring timing.")]
  ErrorDevOnlySupportsSampClkTimingAI =
    DAQmxErrorDevOnlySupportsSampClkTimingAI,
  #[error("Type of TEDS sensor associated with the channel is incompatible with the Measurement Type. Use the TEDS sensor for measurements compatible with the sensor.")]
  ErrorTEDSIncompatibleSensorAndMeasType =
    DAQmxErrorTEDSIncompatibleSensorAndMeasType,
  #[error("TEDS sensor data or Virtual TEDS data file contains multiple calibration templates. Only one calibration template is supported by DAQmx. Use custom scales with this sensor.")]
  ErrorTEDSMultipleCalTemplatesNotSupported =
    DAQmxErrorTEDSMultipleCalTemplatesNotSupported,
  #[error("TEDS template specifies parameters that are not supported by DAQmx. Use custom scales with this sensor.")]
  ErrorTEDSTemplateParametersNotSupported =
    DAQmxErrorTEDSTemplateParametersNotSupported,
  #[error("TEDS sensor data or the Virtual TEDS data file contains an error which was detected during parsing. Ensure your TEDS sensor or Virtual TEDS data file conforms to the specification. If this is not possible, use custom scales with the sensor.")]
  ErrorParsingTEDSData = DAQmxErrorParsingTEDSData,
  #[error("You have specified more than one physical channel as the active channel which is not supported. Specify a single physical channel.")]
  ErrorMultipleActivePhysChansNotSupported =
    DAQmxErrorMultipleActivePhysChansNotSupported,
  #[error("Sample Timing Type was set to Change Detection but no physical channels on which to detect changes were specified. Specify the Change Detection Digital Input Rising and/or Falling Edge Physical Channels, or specify a different Sample Timing Type.")]
  ErrorNoChansSpecdForChangeDetect = DAQmxErrorNoChansSpecdForChangeDetect,
  #[error("Specified voltage is invalid for the given gain.")]
  ErrorInvalidCalVoltageForGivenGain = DAQmxErrorInvalidCalVoltageForGivenGain,
  #[error("Specified gain is not supported.")]
  ErrorInvalidCalGain = DAQmxErrorInvalidCalGain,
  #[error("DAQmx Write was invoked more than once between two consecutive sample clocks. When Sample Mode is Hardware Timed Single Point, invoke DAQmx Write only once between two consecutive sample clocks.")]
  ErrorMultipleWritesBetweenSampClks = DAQmxErrorMultipleWritesBetweenSampClks,
  #[error("Acquisition type specified is not supported by the FREQOUT channel. To use the FREQOUT channel, set the acquisition type to a value supported by FREQOUT. To use specified output type, use a different counter output channel.")]
  ErrorInvalidAcqTypeForFREQOUT = DAQmxErrorInvalidAcqTypeForFREQOUT,
  #[error("Initial Delay, High Time, and Low Time property values are inconsistent with one or more counter timebase properties. The conflicting properties must satisfy the following constraints: 2 / Counter Timebase Rate <= Initial Delay <= Counter Maximum Count / Counter Timebase Rate 2 / Counter Timebase Rate <= High Time <= Counter Maximum Count / Counter Timebase Rate 2 / Counter Timebase Rate <= Low Time <= Counter Maximum Count / Counter Timebase Rate If the Counter Timebase Rate is not specified, it is inferred from the Counter Timebase Source selection.")]
  ErrorSuitableTimebaseNotFoundTimeCombo2 =
    DAQmxErrorSuitableTimebaseNotFoundTimeCombo2,
  #[error("Frequency and Initial Delay property values are inconsistent with one or more counter timebase properties. The conflicting properties must satisfy the following constraints: Counter Timebase Rate / Counter Maximum Count <= Frequency <= Counter Timebase Rate / 4 Counter Timebase Rate / Counter Maximum Count <= 1 / Initial Delay <= Counter Timebase Rate / 2 If the Counter Timebase Rate is not specified, it is inferred from the Counter Timebase Source selection.")]
  ErrorSuitableTimebaseNotFoundFrequencyCombo2 =
    DAQmxErrorSuitableTimebaseNotFoundFrequencyCombo2,
  #[error("Specified reference clock rate does not match the specified reference clock source. Do not set the refererence clock rate when you are using an internal reference clock source. In this case, the driver sets the reference clock rate for you.")]
  ErrorRefClkRateRefClkSrcMismatch = DAQmxErrorRefClkRateRefClkSrcMismatch,
  #[error("For this device, a TEDS terminal block must be connected to the device and configured in MAX in order to perform a TEDS operation.")]
  ErrorNoTEDSTerminalBlock = DAQmxErrorNoTEDSTerminalBlock,
  #[error("Memory of the TEDS sensor connected to the specified physical channel is corrupted, as indicated by an invalid check-sum. Replace the sensor or have the sensor repaired. If the memory is the only defective part of the sensor, consider using MAX to create a Task, a Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorCorruptedTEDSMemory = DAQmxErrorCorruptedTEDSMemory,
  #[error("A TEDS sensor not supported by DAQmx is connected to the specified physical channel. Consider using MAX to create a Task, a Global Channel, or a Scale to acquire data using this sensor.")]
  ErrorTEDSNotSupported = DAQmxErrorTEDSNotSupported,
  #[error("Task used as the timing source for a Timed Loop was started before the Timed Loop was executed. Let the Timed Loop start the task, or use the task without the Timed Loop.")]
  ErrorTimingSrcTaskStartedBeforeTimedLoop =
    DAQmxErrorTimingSrcTaskStartedBeforeTimedLoop,
  #[error("Specified property is not supported for the given timing source.")]
  ErrorPropertyNotSupportedForTimingSrc =
    DAQmxErrorPropertyNotSupportedForTimingSrc,
  #[error("Specified timing source does not exist.")]
  ErrorTimingSrcDoesNotExist = DAQmxErrorTimingSrcDoesNotExist,
  #[error("For this type of device, the Input Buffer Size (in Samples per Channel) must be equal to the value of the Sample Quanity-Samples per Channel property when Sample Mode is Finite Samples.")]
  ErrorInputBufferSizeNotEqualSampsPerChanForFiniteSampMode =
    DAQmxErrorInputBufferSizeNotEqualSampsPerChanForFiniteSampMode,
  #[error("FREQOUT counter cannot generate the desired frequency. The FREQOUT counter is a 4-bit counter that can divide either the 10 MHz Timebase or the 100 kHz Timebase by a number between one and sixteen. Chose a frequency within this range.")]
  ErrorFREQOUTCannotProduceDesiredFrequency2 =
    DAQmxErrorFREQOUTCannotProduceDesiredFrequency2,
  #[error("Given the specified Reference Clock Source, you must set the Reference Clock Rate to a value equal to the frequency of the supplied signal.")]
  ErrorExtRefClkRateNotSpecified = DAQmxErrorExtRefClkRateNotSpecified,
  #[error("Device does not support DMA for the Data Transfer Mechanism when performing non-buffered acquisitions. Set Data Transfer Mechanism to Programmed I/O.")]
  ErrorDeviceDoesNotSupportDMADataXferForNonBufferedAcq =
    DAQmxErrorDeviceDoesNotSupportDMADataXferForNonBufferedAcq,
  #[error("Some or all of the lines in the task have ahd their Digital Filter Minimum Pulse Width property set, which is not supported when the value of the Tristate property is False. Change the value of the Tristate property, or do not set the Digital Filter Minimum Pulse Width property.")]
  ErrorDigFilterMinPulseWidthSetWhenTristateIsFalse =
    DAQmxErrorDigFilterMinPulseWidthSetWhenTristateIsFalse,
  #[error("Some of all of the lines in the task have had their Digital Filter Enable property set, which is not supported when the value of the Tristate property is False. Change the value of the Tristate property, or do not set the Digital Filter Enable property.")]
  ErrorDigFilterEnableSetWhenTristateIsFalse =
    DAQmxErrorDigFilterEnableSetWhenTristateIsFalse,
  #[error("Sample Mode is Hardware Timed, and Sample Timing Type is On Demand, which is not supported by this device. Change Sample Mode or Sample Timing Type.")]
  ErrorNoHWTimingWithOnDemand = DAQmxErrorNoHWTimingWithOnDemand,
  #[error("Value of Tristate property for some or all of the channels in the task is False, and Sample Timing Type is Change Detection, which is not supported by this device. Set the Tristate property to True for all channels or change the Sample Timing Type.")]
  ErrorCannotDetectChangesWhenTristateIsFalse =
    DAQmxErrorCannotDetectChangesWhenTristateIsFalse,
  #[error("Value of the Tristate property for some or all of the channels in the task is False, and Sample Timing Type is Handshake, which is not supported by this device. Set the Tristate property to True for all channels or change the Sample Timing Type.")]
  ErrorCannotHandshakeWhenTristateIsFalse =
    DAQmxErrorCannotHandshakeWhenTristateIsFalse,
  #[error("Some or all of the handshaking control lines for this task are used by another task for static input. These lines cannot be used for handshaking control. Use a port whose handshaking control lines are not already used by a static input task or stop using the lines for handshaking control.")]
  ErrorLinesUsedForStaticInputNotForHandshakingControl =
    DAQmxErrorLinesUsedForStaticInputNotForHandshakingControl,
  #[error("Some or all of the lines in the task are used by another task for handshaking control. These lines cannot be used in a static input task. Use a line that is not also a control line in a handshaking task, or stop using the line for handshaking control.")]
  ErrorLinesUsedForHandshakingControlNotForStaticInput =
    DAQmxErrorLinesUsedForHandshakingControlNotForStaticInput,
  #[error("Some or all of the lines in the task are used by another task for static input. These lines cannot be used in a handshaking input task. Use a port that is not already used by a static input task or stop using the line in the static input task.")]
  ErrorLinesUsedForStaticInputNotForHandshakingInput =
    DAQmxErrorLinesUsedForStaticInputNotForHandshakingInput,
  #[error("Some or all of the lines in the task are used by another task for handshaking input. These lines cannot be used in a static input task. Use a line that is not in a handshaking input task or stop using the line in the handshaking input task.")]
  ErrorLinesUsedForHandshakingInputNotForStaticInput =
    DAQmxErrorLinesUsedForHandshakingInputNotForStaticInput,
  #[error("Digital Input Tristate property has different values for different channels in the task, which is not supported for this type of device. Change the property to a single value for all channels in the task, or use more than one task.")]
  ErrorDifferentDITristateValsForChansInTask =
    DAQmxErrorDifferentDITristateValsForChansInTask,
  #[error("Variance of the measured external reference frequency is too large to perform timebase calibration. Ensure that the reference frequency is stable. Repeat the external timebase calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorTimebaseCalFreqVarianceTooLarge =
    DAQmxErrorTimebaseCalFreqVarianceTooLarge,
  #[error("Timebase calibration algorithm failed to converge within the required tolerance. Ensure that the reference frequency is stable and that the frequency passed to the calibration VI or function is correct. Repeat the external timebase calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorTimebaseCalFailedToConverge = DAQmxErrorTimebaseCalFailedToConverge,
  #[error("Computed frequency resolution of the VCXO CalDAC circuitry is not sufficient to perform timebase calibration. Ensure that the reference frequency is stable and that the frequency passed to the calibration VI or function is correct. Repeat the external timebase calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorInadequateResolutionForTimebaseCal =
    DAQmxErrorInadequateResolutionForTimebaseCal,
  #[error("Measurement taken during calibration produced an invalid AO gain calibration constant. If performing an external calibration, ensure that the reference voltage passed to the calibration VI or function is correct. Repeat the calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorInvalidAOGainCalConst = DAQmxErrorInvalidAOGainCalConst,
  #[error("Measurement taken during calibration produced an invalid AO offset calibration constant. If performing an external calibration, ensure that the reference voltage value passed to the calibration VI or function is correct. Repeat the calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorInvalidAOOffsetCalConst = DAQmxErrorInvalidAOOffsetCalConst,
  #[error("Measurement taken during calibration produced an invalid AI gain calibration constant. If performing an external calibration, ensure that the reference voltage value passed to the calibration VI or function is correct. Repeat the calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorInvalidAIGainCalConst = DAQmxErrorInvalidAIGainCalConst,
  #[error("Measurement taken during calibration produced an invalid AI offset calibration constant. If performing an external calibration, ensure that the reference voltage value passed to the calibration VI or function is correct. Repeat the calibration. If the error persists, contact National Instruments Technical Support.")]
  ErrorInvalidAIOffsetCalConst = DAQmxErrorInvalidAIOffsetCalConst,
  #[error("Digital output detected a new sample clock edge before the previous sample could be written from the onboard memory. If you are using an external sample clock, ensure that it is connected, within jitter and voltage level specifications, and without glitches. If applicable, reduce your sample clock rate or use a product capable of higher sample clock rates.")]
  ErrorDigOutputOverrun = DAQmxErrorDigOutputOverrun,
  #[error("Digital input detected a new sample clock before the previous sample was latched into onboard memory. If you are using an external sample clock, ensure that it is connected, within the jitter and voltage level specifications, and without glitches. If applicable, reduce your sample clock rate or use a product capable of higher sample clock rates.")]
  ErrorDigInputOverrun = DAQmxErrorDigInputOverrun,
  #[error("Acquisition has stopped because the driver could not transfer the data from the device to the computer memory fast enough. This was caused by computer system limitations. Reduce your sample clock rate, the number of channels in the task, or the number of programs your computer is executing concurrently.")]
  ErrorAcqStoppedDriverCantXferDataFastEnough =
    DAQmxErrorAcqStoppedDriverCantXferDataFastEnough,
  #[error("Channels specified cannot appear in the same task on this device. Create a separate task for each of the channels specified.")]
  ErrorChansCantAppearInSameTask = DAQmxErrorChansCantAppearInSameTask,
  #[error("Digital input configuration failed because at least one of the lines in the task was in an expired watchdog task, and the expiration state of the line was set to output. Clear the expiration of the watchdog task by using DAQmx Control Task, by restarting the watchdog task, or by resetting the device either programmatically or by using Measurement & Automation Explorer. To prevent this error in the future, reset the watchdog timer more frequently or increase the watchdog timer timeout.")]
  ErrorInputCfgFailedBecauseWatchdogExpired =
    DAQmxErrorInputCfgFailedBecauseWatchdogExpired,
  #[error("An attempt has been made to use an analog trigger with the Input Source Select property set to an internal source. Either change the Input Source Select property ot specify an external source, or use a different analog trigger source.")]
  ErrorAnalogTrigChanNotExternal = DAQmxErrorAnalogTrigChanNotExternal,
  #[error("Input Source Select property is set to an internal source with more than one channel in the task. On this device, an internal input source is supported only when there is one channel in the task. Remove all of the channels currently in the task except the channel that will be used to acquire the internal input source.")]
  ErrorTooManyChansForInternalAIInputSrc =
    DAQmxErrorTooManyChansForInternalAIInputSrc,
  #[error("No TEDS sensor was detected on the specified physical channel. Ensure that your sensor is properly connected. If the sensor is connected to a TEDS interface device with addresses, make sure the configured address matches the address set on the interface device.")]
  ErrorTEDSSensorNotDetected = DAQmxErrorTEDSSensorNotDetected,
  #[error("Getting a property that pertains to multiple items failed because the value was different for different items. Get the specified property for one item at a time. For example, if you are getting a property for two markers, such as \"marker0:1\" or \"marker0, marker1\", and the property values are different for the two markers, you must get them in two steps (one for marker0 and another for marker1).")]
  ErrorPrptyGetSpecdActiveItemFailedDueToDifftValues =
    DAQmxErrorPrptyGetSpecdActiveItemFailedDueToDifftValues,
  #[error("PXI_Clk10In is available as a destination terminal only for devices in the star trigger controller slot (slot 2). Move your device to PXI slot 2.")]
  ErrorRoutingDestTermPXIClk10InNotInSlot2 =
    DAQmxErrorRoutingDestTermPXIClk10InNotInSlot2,
  #[error("PXI_Star<n> is available as a destination terminal only for devices in the star trigger controller slot (slot 2). To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to slot 2.")]
  ErrorRoutingDestTermPXIStarXNotInSlot2 =
    DAQmxErrorRoutingDestTermPXIStarXNotInSlot2,
  #[error("PXI_Star<n> is available as a source terminal only for devices in the star trigger controller slot (slot 2). To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to slot 2.")]
  ErrorRoutingSrcTermPXIStarXNotInSlot2 =
    DAQmxErrorRoutingSrcTermPXIStarXNotInSlot2,
  #[error("PXI_Star is not available as a source terminal for devices in PXI slots 16 and above. Move your device to one of slots 3 through 15, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInSlot16AndAbove =
    DAQmxErrorRoutingSrcTermPXIStarInSlot16AndAbove,
  #[error("PXI_Star is not available as a destination terminal for devices in PXI slots 16 and above. Move your device to one of slots 3 through 15, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInSlot16AndAbove =
    DAQmxErrorRoutingDestTermPXIStarInSlot16AndAbove,
  #[error("PXI_Star is not available as a destination terminal for devices in PXI slot 2. PXI slot 2 has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of slots 3 through 15, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInSlot2 = DAQmxErrorRoutingDestTermPXIStarInSlot2,
  #[error("PXI_Star is not available as a source terminal for devices in PXI slot 2. PXI slot 2 has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of slots 3 through 15, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInSlot2 = DAQmxErrorRoutingSrcTermPXIStarInSlot2,
  #[error("Route failed because the PXI chassis is not identified. The existence of the destination terminal depends on the chassis being identified. Use the Measurements & Automation Explorer (MAX) to identify your chassis.")]
  ErrorRoutingDestTermPXIChassisNotIdentified =
    DAQmxErrorRoutingDestTermPXIChassisNotIdentified,
  #[error("Route failed because the PXI chassis is not identified. The existence of the source terminal depends on the chassis being identified. Use the Measurements & Automation Explorer (MAX) to identify your chassis.")]
  ErrorRoutingSrcTermPXIChassisNotIdentified =
    DAQmxErrorRoutingSrcTermPXIChassisNotIdentified,
  #[error("Calibration data could not be acquired. Ensure that the device(s) are configured and functioning properly.")]
  ErrorFailedToAcquireCalData = DAQmxErrorFailedToAcquireCalData,
  #[error("Bridge offset nulling calibration is not supported by the specified channels. Specify only the analog input channels that are configured to measure sensors in a bridge configuration.")]
  ErrorBridgeOffsetNullingCalNotSupported =
    DAQmxErrorBridgeOffsetNullingCalNotSupported,
  #[error("AI Maximum was not specified for an operation that requires it.")]
  ErrorAIMaxNotSpecified = DAQmxErrorAIMaxNotSpecified,
  #[error("AI Minimum was not specified for an operation that requires it.")]
  ErrorAIMinNotSpecified = DAQmxErrorAIMinNotSpecified,
  #[error("Buffer size (in samples per channel) multiplied by the number of channels in the task cannot be an odd number for this device. Adjust the buffer size or the number of channels in the task so that their product is an integer multiple of two.")]
  ErrorOddTotalBufferSizeToWrite = DAQmxErrorOddTotalBufferSizeToWrite,
  #[error("Number of samples per channel to write multiplied by the number of channels in the task cannot be an odd number for this device. Adjust the number of samples per channel to write or the number of channels in the task so that their product is an integer multiple of two.")]
  ErrorOddTotalNumSampsToWrite = DAQmxErrorOddTotalNumSampsToWrite,
  #[error("Buffered operations are not compatible with the requested Wait Mode. Do not configure a buffer, or set Wait Mode to Yield.")]
  ErrorBufferWithWaitMode = DAQmxErrorBufferWithWaitMode,
  #[error("Buffered operations are not supported if Sample Mode is Hardware Timed Single Point. Do not configure a buffer, or change the Sample Mode value.")]
  ErrorBufferWithHWTimedSinglePointSampMode =
    DAQmxErrorBufferWithHWTimedSinglePointSampMode,
  #[error("Pulse low tick count specified is not supported for this device.")]
  ErrorCOWritePulseLowTicksNotSupported =
    DAQmxErrorCOWritePulseLowTicksNotSupported,
  #[error("Pulse high tick count specified is not supported for this device.")]
  ErrorCOWritePulseHighTicksNotSupported =
    DAQmxErrorCOWritePulseHighTicksNotSupported,
  #[error("Pulse low time specified is not supported for this device given the Counter Timebase Rate.")]
  ErrorCOWritePulseLowTimeOutOfRange = DAQmxErrorCOWritePulseLowTimeOutOfRange,
  #[error("Pulse high time specified is not supported for this device given the Counter Timebase Rate.")]
  ErrorCOWritePulseHighTimeOutOfRange =
    DAQmxErrorCOWritePulseHighTimeOutOfRange,
  #[error("Pulse frequency specified is not supported for this device given the Counter Timebase Rate.")]
  ErrorCOWriteFreqOutOfRange = DAQmxErrorCOWriteFreqOutOfRange,
  #[error("Pulse duty cycle specified is not supported for this device given the pulse frequency and Counter Timebase Rate.")]
  ErrorCOWriteDutyCycleOutOfRange = DAQmxErrorCOWriteDutyCycleOutOfRange,
  #[error("NI-DAQmx has detected a corrupt installation.  Please re-install NI-DAQmx.  If you continue to receive this message, please contact National Instruments for assistance.")]
  ErrorInvalidInstallation = DAQmxErrorInvalidInstallation,
  #[error("Data could not be read because the reference trigger master session is unavailable. To avoid this error, read data before closing the reference trigger master session.")]
  ErrorRefTrigMasterSessionUnavailable =
    DAQmxErrorRefTrigMasterSessionUnavailable,
  #[error("Route failed because either the source or destination of the route is also a line in a watchdog timer task whose watchdog timer has expired. Clear the expiration of the watchdog timer before routing by using DAQmx Control Watchdog Task, by restarting the watchdog timer task, or by resetting the device programmatically or through Measurements & Automation Explorer. To prevent this error in the future reset the watchdog timer more frequently or increase the watchdog timer timeout.")]
  ErrorRouteFailedBecauseWatchdogExpired =
    DAQmxErrorRouteFailedBecauseWatchdogExpired,
  #[error("Device has shut down because a sensor on the device detected a temperature above the device's maximum recommended operating temperature. To use the device again, either turn the chassis/computer off until the device has cooled, or ensure the device has cooled, and reset the device (either programmatically or through Measurements & Automation Explorer).")]
  ErrorDeviceShutDownDueToHighTemp = DAQmxErrorDeviceShutDownDueToHighTemp,
  #[error("When Sample Mode is Hardware Timed Single Point, Memory Mapping for Programmed IO Enable cannot be true. Set Memory Mapping for Programmed IO Enable to false or change Sample Mode.")]
  ErrorNoMemMapWhenHWTimedSinglePoint =
    DAQmxErrorNoMemMapWhenHWTimedSinglePoint,
  #[error("Write failed because at least one of the lines in the task is also in a watchdog timer task whose watchdog timer has expired. Clear the expiration of the watchdog timer before writing by using DAQmx Control Watchdog Task, by restarting the watchdog timer task, or by resetting the device programmatically or through the Measurements & Automation Explorer. To prevent this error in the future reset the watchdog timer more frequently or increase the watchdog timer timeout.")]
  ErrorWriteFailedBecauseWatchdogExpired =
    DAQmxErrorWriteFailedBecauseWatchdogExpired,
  #[error("Input Source Select property is set to different internal sources on some channels. On this device, Input Source Select must be set the same way for all channels with internal sources. Refer to documentation for details.")]
  ErrorDifftInternalAIInputSrcs = DAQmxErrorDifftInternalAIInputSrcs,
  #[error("Input Source Select property is set differently for channels in one channel group on a device that supports only identical settings within a channel group. Refer to documentation for details.")]
  ErrorDifftAIInputSrcInOneChanGroup = DAQmxErrorDifftAIInputSrcInOneChanGroup,
  #[error("Input Source Select is set to an internal source for channels in different channel groups. On this device, only one channel group at a time can be configured to use an internal source. Refer to documentation for details.")]
  ErrorInternalAIInputSrcInMultChanGroups =
    DAQmxErrorInternalAIInputSrcInMultChanGroups,
  #[error("Switch operation failed due to a previous error. The device may have been powered off and back on. To use the device again, reset the device either programmatically or by using Measurement & Automation Explorer.")]
  ErrorSwitchOpFailedDueToPrevError = DAQmxErrorSwitchOpFailedDueToPrevError,
  #[error("Multiple samples cannot be written using a single sample write. Ensure the waveform contains only a single sample.")]
  ErrorWroteMultiSampsUsingSingleSampWrite =
    DAQmxErrorWroteMultiSampsUsingSingleSampWrite,
  #[error("Input arrays are of different sizes. These arrays must have the same size.")]
  ErrorMismatchedInputArraySizes = DAQmxErrorMismatchedInputArraySizes,
  #[error("Switch device has been disabled to prevent it from exceeding its simultaneous relay drive limit. To recover, call DAQmx Disconnect All, or reset the device. The device can be reset either programmatically or by using Measurement & Automation Explorer.")]
  ErrorCantExceedRelayDriveLimit = DAQmxErrorCantExceedRelayDriveLimit,
  #[error("DAC Range Low is not equal in magnitude and opposite in sign from DAC Reference Value. If you do not set the DAC Range Low property, the driver sets it for you. Otherwise, ensure DAC Range Low and DAC Reference Voltage Value are equal in magnitude and opposite in sign.")]
  ErrorDACRngLowNotEqualToMinusRefVal =
    DAQmxErrorDACRngLowNotEqualToMinusRefVal,
  #[error("Attempt to set the Connect DAC Reference to Ground property failed, because the Allow Connecting DAC Reference to Ground property was not True. To connect DAQ reference to ground, you must set two properties to True: Connect DAC Reference to Ground and Allow Connecting DAC Reference to Ground.")]
  ErrorCantAllowConnectDACToGnd = DAQmxErrorCantAllowConnectDACToGnd,
  #[error("Requested value is not a supported value for Watchdog Timer Timeout. Use special value -1.0 to indicate that the internal timer should be disabled, and the watchdog timer will expire based on the external expiration trigger, or specify another valid value.")]
  ErrorWatchdogTimeoutOutOfRangeAndNotSpecialVal =
    DAQmxErrorWatchdogTimeoutOutOfRangeAndNotSpecialVal,
  #[error("Attempt to configure a port or any of its lines to have an expiration state of Output failed, because the port or some of its lines are currently reserved for use by an input task. Set the expiration state of the port to Tristate or No Change, or choose a different port for digital input.")]
  ErrorNoWatchdogOutputOnPortReservedForInput =
    DAQmxErrorNoWatchdogOutputOnPortReservedForInput,
  #[error("Attempt to configure a port or any of its lines for input failed, because this port is currently configured for digital output by the watchdog timer. Choose another port, or modify the watchdog timer task to set the expiration state of the port to Tristate or No Change.")]
  ErrorNoInputOnPortCfgdForWatchdogOutput =
    DAQmxErrorNoInputOnPortCfgdForWatchdogOutput,
  #[error("Expiration states requested are not supported by the lines in the port. For this device, the Watchdog Timer Expiration States must be either No Change for all lines in a port or a combination of values other than No Change for all lines in a port. For example, the combination of No Change for one line and High for another line is not supported, while a value of Low for one line and High for another line is supported.")]
  ErrorWatchdogExpirationStateNotEqualForLinesInPort =
    DAQmxErrorWatchdogExpirationStateNotEqualForLinesInPort,
  #[error("Specified operation cannot be performed because the task has not been started, committed, or reserved. Call DAQmx Start or DAQmx Control with action set to Commit or Reserve prior to requesting this operation.")]
  ErrorCannotPerformOpWhenTaskNotReserved =
    DAQmxErrorCannotPerformOpWhenTaskNotReserved,
  #[error("Attempt to set programmable powerup states failed because the device does not support the feature.")]
  ErrorPowerupStateNotSupported = DAQmxErrorPowerupStateNotSupported,
  #[error("Attempt to create a watchdog timer task failed because the device does not support the feature, or the wrong device type was specified. For NI CompactDAQ devices supporting this feature, specify the chassis name on which to create a watchdog timer task.")]
  ErrorWatchdogTimerNotSupported = DAQmxErrorWatchdogTimerNotSupported,
  #[error(
        "Requested operation is not supported because the Reference Clock Source is \"None\"."
    )]
  ErrorOpNotSupportedWhenRefClkSrcNone =
    DAQmxErrorOpNotSupportedWhenRefClkSrcNone,
  #[error("Requested Sample Clock Rate is not available because this task shares the Sample Clock Source or the Sample Clock Timebase with another task. The other task has already programmed one of those properties in a manner inconsistent with the requested Sample Clock Rate. Specify a Sample Clock Rate consistent with the settings in the other task, or change the settings in the other task. Refer to documentation for more detailed information.")]
  ErrorSampClkRateUnavailable = DAQmxErrorSampClkRateUnavailable,
  #[error("Attempt to get property failed, because the single channel you specified corresponds to multiple physical channels, and the property has different values for those different physical channels. Get this property one physical channel at a time. For digital channels, you might have to specify a single digital line.")]
  ErrorPrptyGetSpecdSingleActiveChanFailedDueToDifftVals =
    DAQmxErrorPrptyGetSpecdSingleActiveChanFailedDueToDifftVals,
  #[error("Attempt to get property failed, because your task contains multiple channels, and the property has different values for different channels. Get this property one channel at a time using Active Channel to specify each individual channel.")]
  ErrorPrptyGetImpliedActiveChanFailedDueToDifftVals =
    DAQmxErrorPrptyGetImpliedActiveChanFailedDueToDifftVals,
  #[error("Attempt to get property failed, because you specified multiple channels, and the property has different values for different channels. Get this property one channel at a time.")]
  ErrorPrptyGetSpecdActiveChanFailedDueToDifftVals =
    DAQmxErrorPrptyGetSpecdActiveChanFailedDueToDifftVals,
  #[error("Operation failed, because an attempt was made to use only the onboard memory for generation when regeneration of data was not allowed. Set the Regeneration Mode property to Allow Regeneration or set the Use Only Onboard Memory property to false.")]
  ErrorNoRegenWhenUsingBrdMem = DAQmxErrorNoRegenWhenUsingBrdMem,
  #[error("Attempted to read more samples than what was configured in the acquisition. Restart the acquisition, increase the Samples Per Channel property, or set the Sample Mode property to Continuous Samples.")]
  ErrorNonbufferedReadMoreThanSampsPerChan =
    DAQmxErrorNonbufferedReadMoreThanSampsPerChan,
  #[error("Attempt to set watchdog timer expiration state failed, because some of the lines in a port were tristated, and others were not. For this type of device, watchdog timer expiration states of all lines in a port have to be either tristated or not tristated.")]
  ErrorWatchdogExpirationTristateNotSpecdForEntirePort =
    DAQmxErrorWatchdogExpirationTristateNotSpecdForEntirePort,
  #[error("Attempt to set programmable powerup state failed, because some of the lines in a port were tristated and others were not. For this type of device, programmable powerup states of all lines in a port have to be either tristated or not tristated.")]
  ErrorPowerupTristateNotSpecdForEntirePort =
    DAQmxErrorPowerupTristateNotSpecdForEntirePort,
  #[error("Attempt to set programmable powerup state failed, because only some of the channels from the port were specified. For this type of device, you must specify programmable powerup state for entire ports.")]
  ErrorPowerupStateNotSpecdForEntirePort =
    DAQmxErrorPowerupStateNotSpecdForEntirePort,
  #[error("Attempt to set watchdog timer expiration state failed, because the specified physical channel only supports digital input, and watchdog timer expiration state does not apply.")]
  ErrorCantSetWatchdogExpirationOnDigInChan =
    DAQmxErrorCantSetWatchdogExpirationOnDigInChan,
  #[error("Attempt to set programmable powerup state failed, because the specified physical channel only supports digital input, and the programmable powerup state does not apply.")]
  ErrorCantSetPowerupStateOnDigInChan =
    DAQmxErrorCantSetPowerupStateOnDigInChan,
  #[error("Specified a physical channel for change detection that is not contained by any channel in the task. Use only physical channels already contained by a channel, or create an additional channel containing the desired physical channel .")]
  ErrorPhysChanNotInTask = DAQmxErrorPhysChanNotInTask,
  #[error("Device identifier of the physical channel specified is not the same as the device used in the task. Use only the physical channels on the device used in the task.")]
  ErrorPhysChanDevNotInTask = DAQmxErrorPhysChanDevNotInTask,
  #[error("Specified physical channel does not support digital input. Change the direction of the task, use another terminal, or use another device. To read from digital output lines, create a digital output task and use DAQmx Read.")]
  ErrorDigInputNotSupported = DAQmxErrorDigInputNotSupported,
  #[error("Some of the physical channels in the task are configured for different filter intervals, which is not supported by this type of device. Configure all lines in the task to use the same digital filter interval.")]
  ErrorDigFilterIntervalNotEqualForLines =
    DAQmxErrorDigFilterIntervalNotEqualForLines,
  #[error("Attempt to set the digital filter interval failed, because another task has already configured a different digital filter interval. Use the same digital filter interval in the two tasks, or wait for the other task to finish before starting or committing this task.")]
  ErrorDigFilterIntervalAlreadyCfgd = DAQmxErrorDigFilterIntervalAlreadyCfgd,
  #[error("Attempt to reset watchdog timer failed, because the timer had already expired. Clear expiration of the watchdog timer, or configure a longer watchdog timer timeout.")]
  ErrorCantResetExpiredWatchdog = DAQmxErrorCantResetExpiredWatchdog,
  #[error("You have specified more than one line when getting a property. Specify a single line as the active channel.")]
  ErrorActiveChanTooManyLinesSpecdWhenGettingPrpty =
    DAQmxErrorActiveChanTooManyLinesSpecdWhenGettingPrpty,
  #[error("You have not specified an active channel when getting a property. Specify a single line as the active channel.")]
  ErrorActiveChanNotSpecdWhenGetting1LinePrpty =
    DAQmxErrorActiveChanNotSpecdWhenGetting1LinePrpty,
  #[error("Property cannot be set separately for each line. When setting this property, specify a virtual channel as the active channel.")]
  ErrorDigPrptyCannotBeSetPerLine = DAQmxErrorDigPrptyCannotBeSetPerLine,
  #[error("For this device, send advance complete (<sac>) is not supported after any wait for triggers (;) in the scan list. If your scan list contains an action sequence similar to \"ch0->com0; <sac>\", change the action sequence to \"ch0->com0;\".")]
  ErrorSendAdvCmpltAfterWaitForTrigInScanlist =
    DAQmxErrorSendAdvCmpltAfterWaitForTrigInScanlist,
  #[error("For this device, connections specified in the scan list must be disconnected before making new connections. If your scanlist contains an action sequence similar to \"ch0->com0;;\", change the action sequence to \"ch0->com0; ~ch0->com0\".")]
  ErrorDisconnectionRequiredInScanlist =
    DAQmxErrorDisconnectionRequiredInScanlist,
  #[error("For this device, two consecutive wait for triggers are not supported after any connection in the scan list. If your scan list contains an action sequence similar to \"ch0->com0;;\", change the action sequence to \"ch0->com0;\".")]
  ErrorTwoWaitForTrigsAfterConnectionInScanlist =
    DAQmxErrorTwoWaitForTrigsAfterConnectionInScanlist,
  #[error("For this device, an action separator (& or &&) is required after breaking a connection in the scan list. If your scan list contains an action sequence similar to \"~ch0->com0;\", change the action sequence to \"~ch0->com0 &\" or \"~ch0->com0 &&\".")]
  ErrorActionSeparatorRequiredAfterBreakingConnectionInScanlist =
    DAQmxErrorActionSeparatorRequiredAfterBreakingConnectionInScanlist,
  #[error("For this device, any connection specified in the scan list must wait for a trigger (;). If your scan list contains an action sequence similar to \"ch0->com0 &\" or \"ch0->com0 &&\", change the action sequence to \"ch0->com0;\".")]
  ErrorConnectionInScanlistMustWaitForTrig =
    DAQmxErrorConnectionInScanlistMustWaitForTrig,
  #[error("You have attempted to control a Watchdog Task, but the task supplied was not a Watchdog Task.")]
  ErrorActionNotSupportedTaskNotWatchdog =
    DAQmxErrorActionNotSupportedTaskNotWatchdog,
  #[error("Waveform name is the same as an existing script name. Make sure that the waveform name is different from the names of previously written scripts.")]
  ErrorWfmNameSameAsScriptName = DAQmxErrorWfmNameSameAsScriptName,
  #[error("Script name is the same as an existing waveform name. Make sure that the script name is different from the names of previously written or allocated waveforms.")]
  ErrorScriptNameSameAsWfmName = DAQmxErrorScriptNameSameAsWfmName,
  #[error("Hardware clocking error occurred. If you are using an external sample clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDSFStopClock = DAQmxErrorDSFStopClock,
  #[error("Hardware clocking error occurred. If you are using an external sample clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDSFReadyForStartClock = DAQmxErrorDSFReadyForStartClock,
  #[error("Requested write offset is invalid, because it is not an integer multiple of the write increment.")]
  ErrorWriteOffsetNotMultOfIncr = DAQmxErrorWriteOffsetNotMultOfIncr,
  #[error(
        "Requested different values for properties that must have equal values on this device."
    )]
  ErrorDifferentPrptyValsNotSupportedOnDev =
    DAQmxErrorDifferentPrptyValsNotSupportedOnDev,
  #[error(
        "Pause and reference triggers are both configured, which is not supported in this task."
    )]
  ErrorRefAndPauseTrigConfigured = DAQmxErrorRefAndPauseTrigConfigured,
  #[error("Hardware clocking error occurred. If you are using an external sample clock, make sure it is connected and within the jitter and voltage level specifications. Also,  verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorFailedToEnableHighSpeedInputClock =
    DAQmxErrorFailedToEnableHighSpeedInputClock,
  #[error("An empty physical channel has been specified within the power up states array. Specify a correct physical channel.")]
  ErrorEmptyPhysChanInPowerUpStatesArray =
    DAQmxErrorEmptyPhysChanInPowerUpStatesArray,
  #[error("You have specified more than one line as the active physical channel when getting a property. Specify a single line.")]
  ErrorActivePhysChanTooManyLinesSpecdWhenGettingPrpty =
    DAQmxErrorActivePhysChanTooManyLinesSpecdWhenGettingPrpty,
  #[error("You have not specified an active channel when getting a property. Specify a single line as the active physical channel.")]
  ErrorActivePhysChanNotSpecdWhenGetting1LinePrpty =
    DAQmxErrorActivePhysChanNotSpecdWhenGetting1LinePrpty,
  #[error("Device has shut down because a sensor on the device detected a temperature above the device's maximum recommended operating temperature. To use the device again, either turn the chassis/computer off until the device has cooled, or ensure the device has cooled, and reset the device (either programmatically or through Measurements & Automation Explorer).")]
  ErrorPXIDevTempCausedShutDown = DAQmxErrorPXIDevTempCausedShutDown,
  #[error("Requested number of samples to write is invalid. Change the number of samples to be written to a number equal to or greater than zero.")]
  ErrorInvalidNumSampsToWrite = DAQmxErrorInvalidNumSampsToWrite,
  #[error("Onboard device memory underflow. Because of system and/or bus-bandwidth limitations, the driver could not write data to the device fast enough to keep up with the device output rate. Reduce your sample rate. If your data transfer method is interrupts, try using DMA or USB Bulk. You can also reduce the number of programs your computer is executing concurrently.")]
  ErrorOutputFIFOUnderflow2 = DAQmxErrorOutputFIFOUnderflow2,
  #[error("Requested multiple virtual channels that correspond to the same analog input physical channel within a single task. A task cannot contain multiple analog input physical channels for this type of device. Use different physical channels for each virtual channel.")]
  ErrorRepeatedAIPhysicalChan = DAQmxErrorRepeatedAIPhysicalChan,
  #[error("Chassis cannot be used for more than one scanning operation at the same time. Do only one scanning operation, or combine multiple scanning operations into a single operation.")]
  ErrorMultScanOpsInOneChassis = DAQmxErrorMultScanOpsInOneChassis,
  #[error("Analog input virtual channels cannot be created out of order with respect to their physical channel numbers for the type of analog device you are using. For example, a virtual channel using physical channel ai0 must be created before a virtual channel with physical channel ai1.")]
  ErrorInvalidAIChanOrder = DAQmxErrorInvalidAIChanOrder,
  #[error("Output generation was aborted by the reverse power protection circuitry of the device. Either the output signal exceeded the output power limit, or power was being driven back into the output of the device by an external source. Correct the problem, then generate the signal again.")]
  ErrorReversePowerProtectionActivated =
    DAQmxErrorReversePowerProtectionActivated,
  #[error("Specified asynchronous operation handle is invalid.")]
  ErrorInvalidAsynOpHandle = DAQmxErrorInvalidAsynOpHandle,
  #[error("Hardware clocking error occurred. If you are using an external sample clock, make sure it is connected and within the jitter and voltage level specifications. Also,  verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorFailedToEnableHighSpeedOutput = DAQmxErrorFailedToEnableHighSpeedOutput,
  #[error("Combination of Samples to Read, Position, and Offset results in an attempt to read past the end of the record. You only  can read samples within the record.")]
  ErrorCannotReadPastEndOfRecord = DAQmxErrorCannotReadPastEndOfRecord,
  #[error("Acquisition has been stopped to prevent an input buffer overwrite. Your application was unable to read samples from the buffer fast enough to prevent new samples from overwriting unread data. To avoid this error, you can do any of the following: 1. Increase the size of the buffer. 2. Increase the number of samples you read each time you invoke a read operation. 3. Read samples more often. 4. Reduce the sample rate. 5. If your data transfer method is interrupts, try using DMA or USB Bulk. 6. Reduce the number of applications your computer is running concurrently. In addition, if you do not need to read every sample that is acquired, use the Relative To and Offset properties to read the desired samples.")]
  ErrorAcqStoppedToPreventInputBufferOverwriteOneDataXferMech =
    DAQmxErrorAcqStoppedToPreventInputBufferOverwriteOneDataXferMech,
  #[error("Requested channel index is invalid. The value of the index must be between 0 and the number of channels in the task minus one.")]
  ErrorZeroBasedChanIndexInvalid = DAQmxErrorZeroBasedChanIndexInvalid,
  #[error("Operation cannot be performed, because there are no channels of the requested type in the task.")]
  ErrorNoChansOfGivenTypeInTask = DAQmxErrorNoChansOfGivenTypeInTask,
  #[error("Requested sample clock source is invalid for output. The specified sample clock source terminal is only supported for input.")]
  ErrorSampClkSrcInvalidForOutputValidForInput =
    DAQmxErrorSampClkSrcInvalidForOutputValidForInput,
  #[error("Generation cannot be started, because the selected buffer size is too small. Increase the buffer size.")]
  ErrorOutputBufSizeTooSmallToStartGen =
    DAQmxErrorOutputBufSizeTooSmallToStartGen,
  #[error("Acquisition cannot be started, because the selected buffer size is too small. Increase the buffer size.")]
  ErrorInputBufSizeTooSmallToStartAcq =
    DAQmxErrorInputBufSizeTooSmallToStartAcq,
  #[error("2 signals cannot be simultaneously exported on the same terminal.")]
  ErrorExportTwoSignalsOnSameTerminal =
    DAQmxErrorExportTwoSignalsOnSameTerminal,
  #[error("Requested channel index is invalid. The value of the index must be between one and the number of channels in the task.")]
  ErrorChanIndexInvalid = DAQmxErrorChanIndexInvalid,
  #[error("Given range in the input string contains a number that is too large. Check the string. Use smaller numbers in the range, or replace the range with a comma-separated list.")]
  ErrorRangeSyntaxNumberTooBig = DAQmxErrorRangeSyntaxNumberTooBig,
  #[error("NULL pointer was passed for a required parameter.")]
  ErrorNULLPtr = DAQmxErrorNULLPtr,
  #[error(
    "Scaled Minimum cannot be equal to Scaled Maximum for output operations."
  )]
  ErrorScaledMinEqualMax = DAQmxErrorScaledMinEqualMax,
  #[error("Prescaled Minimum cannot be equal to Prescaled Maximum for input operations.")]
  ErrorPreScaledMinEqualMax = DAQmxErrorPreScaledMinEqualMax,
  #[error("Property not supported by this scale type.")]
  ErrorPropertyNotSupportedForScaleType =
    DAQmxErrorPropertyNotSupportedForScaleType,
  #[error("NI-DAQmx cannot generate virtual channel names for some of the physical channels specified, because the numeric suffix of the resulting channel names would be too large. Either explicitly specify a virtual channel name for each physical channel name, or decrease the numeric suffix of the last set of virtual channel names.")]
  ErrorChannelNameGenerationNumberTooBig =
    DAQmxErrorChannelNameGenerationNumberTooBig,
  #[error(
    "Repetition of a number in Scaled Values is invalid for output operations."
  )]
  ErrorRepeatedNumberInScaledValues = DAQmxErrorRepeatedNumberInScaledValues,
  #[error("Repetition of a number in the Prescaled Values is invalid for input operations.")]
  ErrorRepeatedNumberInPreScaledValues =
    DAQmxErrorRepeatedNumberInPreScaledValues,
  #[error("Specified output operation cannot be satisfied, because it requires lines that are currently in use by another output operation.")]
  ErrorLinesAlreadyReservedForOutput = DAQmxErrorLinesAlreadyReservedForOutput,
  #[error("Channels in the switch operation span different devices.")]
  ErrorSwitchOperationChansSpanMultipleDevsInList =
    DAQmxErrorSwitchOperationChansSpanMultipleDevsInList,
  #[error("Invalid identifier at the beginning of the switch operation in the list entry.")]
  ErrorInvalidIDInListAtBeginningOfSwitchOperation =
    DAQmxErrorInvalidIDInListAtBeginningOfSwitchOperation,
  #[error("Value passed for the direction parameter is invalid. Use one of the values of the corresponding enumeration.")]
  ErrorMStudioInvalidPolyDirection = DAQmxErrorMStudioInvalidPolyDirection,
  #[error("Value of this property cannot be determined until the containing task is verified. Before attempting to get the value of this property, you must make sure the task has been verified.  You can do this by starting the task, using the task control method to verify the task, reading from the task if the Read Auto Start property is true, or writing to the task and specifying true for the auto start parameter.")]
  ErrorMStudioPropertyGetWhileTaskNotVerified =
    DAQmxErrorMStudioPropertyGetWhileTaskNotVerified,
  #[error("Given range in the input string contains too many objects. Check the string. If necessary, split the input string into smaller ranges.")]
  ErrorRangeWithTooManyObjects = DAQmxErrorRangeWithTooManyObjects,
  #[error("Negative buffer size was supplied. The buffer size must be zero or greater.")]
  ErrorCppDotNetAPINegativeBufferSize =
    DAQmxErrorCppDotNetAPINegativeBufferSize,
  #[error("Specified event handler cannot be removed, because it is invalid. It has never been installed on this or any other NI-DAQmx object.")]
  ErrorCppCantRemoveInvalidEventHandler =
    DAQmxErrorCppCantRemoveInvalidEventHandler,
  #[error("Specified event handler cannot be removed, because it has already been removed.")]
  ErrorCppCantRemoveEventHandlerTwice =
    DAQmxErrorCppCantRemoveEventHandlerTwice,
  #[error("Specified event handler cannot be removed, because it is installed on a different NI-DAQmx object. Remove the event handler from the NI-DAQmx object on which it was installed.")]
  ErrorCppCantRemoveOtherObjectsEventHandler =
    DAQmxErrorCppCantRemoveOtherObjectsEventHandler,
  #[error("Requested operation could not be performed, because the specified digital lines are either reserved or the device is not present in NI-DAQmx. It is possible that these lines are reserved by another task or the device is being reset. If you are using these lines with another task, wait for the task to complete.  If you want to force the other task to relinquish the device, reset the device. If you are resetting the device, wait for the reset to finish.")]
  ErrorDigLinesReservedOrUnavailable = DAQmxErrorDigLinesReservedOrUnavailable,
  #[error("Hardware clocking error occurred. If you are using an external sample clock, make sure it is connected and within specifications.  If you are generating your sample clock internally, please contact National Instruments Technical Support.")]
  ErrorDSFFailedToResetStream = DAQmxErrorDSFFailedToResetStream,
  #[error("Hardware clocking error occurred. If you are using an external sample clock, make sure it is connected and within specifications.  If you are generating your sample clock internally, please contact National Instruments Technical Support.")]
  ErrorDSFReadyForOutputNotAsserted = DAQmxErrorDSFReadyForOutputNotAsserted,
  #[error("Write failed, because the number of samples to write per channel is invalid. The number of samples to write per channel must be an integer multiple of the samples to write per channel increment.")]
  ErrorSampToWritePerChanNotMultipleOfIncr =
    DAQmxErrorSampToWritePerChanNotMultipleOfIncr,
  #[error("Values of the AO channel properties lead to an output voltage that is less than the minimum for the device.")]
  ErrorAOPropertiesCauseVoltageBelowMin =
    DAQmxErrorAOPropertiesCauseVoltageBelowMin,
  #[error("Values for AO channel properties lead to an output voltage that exceeds the maximum for the device.")]
  ErrorAOPropertiesCauseVoltageOverMax =
    DAQmxErrorAOPropertiesCauseVoltageOverMax,
  #[error("Specified property is not supported, because Reference Clock Source is \"None\".")]
  ErrorPropertyNotSupportedWhenRefClkSrcNone =
    DAQmxErrorPropertyNotSupportedWhenRefClkSrcNone,
  #[error("Requested AI Maximum value is too small.")]
  ErrorAIMaxTooSmall = DAQmxErrorAIMaxTooSmall,
  #[error("Requested AI Maximum value is too large.")]
  ErrorAIMaxTooLarge = DAQmxErrorAIMaxTooLarge,
  #[error("Requested AI Minimum value is too small.")]
  ErrorAIMinTooSmall = DAQmxErrorAIMinTooSmall,
  #[error("Requested AI Minimum value is too large.")]
  ErrorAIMinTooLarge = DAQmxErrorAIMinTooLarge,
  #[error("CJC Source cannot be set to Built-In for the specified thermocouple channel. The physical channel does not support a built-in CJC temperature sensor. If your hardware contains a CJC temperature sensor on the physical channel corresponding to the built-in CJC source, make sure that the hardware configuration (including any accessories and/or terminal blocks) is correct. Alternatively, specify a different CJC Source, or use hardware with a built-in CJC temperature sensor.")]
  ErrorBuiltInCJCSrcNotSupported = DAQmxErrorBuiltInCJCSrcNotSupported,
  #[error("Requested value for Samples per Channel is too high when a reference trigger is used. In this case, Samples per Channel cannot exceed the sum of Pretrigger Samples per Channel and the maximum Post-trigger Samples per Channel. Reduce Samples per Channel. Alternatively, consider performing an acquisition with Continuous Sample Mode, or increase the Pretrigger Samples per Channel.")]
  ErrorTooManyPostTrigSampsPerChan = DAQmxErrorTooManyPostTrigSampsPerChan,
  #[error("Driver cannot complete the route, because the only way to make the route requires a trigger bus line, and no trigger bus has been configured in MAX for this device. If you have a PXI chassis, make sure it has been properly identified in MAX. If you are using a PCI device, create a RTSI cable in MAX that includes your PCI device even if you are not using any RTSI cables.")]
  ErrorTrigLineNotFoundSingleDevRoute =
    DAQmxErrorTrigLineNotFoundSingleDevRoute,
  #[error("Input Source Select property is set to different internal sources on some channels. On this device, Input Source Select must be set to the same value for all channels with internal sources. Refer to the documentation for more details.")]
  ErrorDifferentInternalAIInputSources =
    DAQmxErrorDifferentInternalAIInputSources,
  #[error("Input Source Select property is set differently for channels in one channel group on a device that supports only identical settings within a channel group. Refer to the documentation for more details.")]
  ErrorDifferentAIInputSrcInOneChanGroup =
    DAQmxErrorDifferentAIInputSrcInOneChanGroup,
  #[error("Input Source Select property is set to an internal source for channels in different channel groups. On this device, only one channel group at a time can be configured to use an internal source. Refer to the documentation for details.")]
  ErrorInternalAIInputSrcInMultipleChanGroups =
    DAQmxErrorInternalAIInputSrcInMultipleChanGroups,
  #[error("Requested channel index is invalid. The value of the index must be between one and the number of channels in the task.")]
  ErrorCAPIChanIndexInvalid = DAQmxErrorCAPIChanIndexInvalid,
  #[error("Type of channel collection used to access the specified channel does not match the channel type. Access the channel through the channel collection that matches the channel type.")]
  ErrorCollectionDoesNotMatchChanType =
    DAQmxErrorCollectionDoesNotMatchChanType,
  #[error("Generation cannot be started, because the Regeneration Mode property was changed since the last write, and this change caused data to be lost. Write data after changing the Regeneration Mode property.")]
  ErrorOutputCantStartChangedRegenerationMode =
    DAQmxErrorOutputCantStartChangedRegenerationMode,
  #[error("Generation cannot be started, because the buffer size was changed since the last write, and this change caused data to be lost. Write data after changing the buffer size.")]
  ErrorOutputCantStartChangedBufferSize =
    DAQmxErrorOutputCantStartChangedBufferSize,
  #[error("Specified digital channel contains more bits than supported by the 32-bit version of DAQmx Port Write.")]
  ErrorChanSizeTooBigForU32PortWrite = DAQmxErrorChanSizeTooBigForU32PortWrite,
  #[error("Specified digital channel contains more bits than supported by the 8-bit version of DAQmx Port Write. Use the version of DAQmx Port Write that supports wider digital ports.")]
  ErrorChanSizeTooBigForU8PortWrite = DAQmxErrorChanSizeTooBigForU8PortWrite,
  #[error("Specified digital channel contains more bits than supported by the 32-bit version of DAQmx Read. Use a version of DAQmx Read that returns an array of Boolean values or digital waveforms.")]
  ErrorChanSizeTooBigForU32PortRead = DAQmxErrorChanSizeTooBigForU32PortRead,
  #[error("Specified digital channel contains more bits than supported by the 8-bit version of DAQmx Port Read. Use a version of DAQmx Port Read that supports wider digital ports.")]
  ErrorChanSizeTooBigForU8PortRead = DAQmxErrorChanSizeTooBigForU8PortRead,
  #[error("Attempted writing digital data that is not supported.")]
  ErrorInvalidDigDataWrite = DAQmxErrorInvalidDigDataWrite,
  #[error("Attempted writing analog data that is too large or too small. Change Minimum Value and Maximum Value to reflect the range of the channel.")]
  ErrorInvalidAODataWrite = DAQmxErrorInvalidAODataWrite,
  #[error("Wait Until Done did not indicate that the task was done within the specified timeout. Increase the timeout, check the program, and make sure connections for external timing and triggering are in place.")]
  ErrorWaitUntilDoneDoesNotIndicateDone =
    DAQmxErrorWaitUntilDoneDoesNotIndicateDone,
  #[error("Task cannot contain a channel with the specified channel type, because the task already contains channels with a different channel type. Create one task for each channel type.")]
  ErrorMultiChanTypesInTask = DAQmxErrorMultiChanTypesInTask,
  #[error("One task cannot contain multiple independent devices. Create one task for each independent device.")]
  ErrorMultiDevsInTask = DAQmxErrorMultiDevsInTask,
  #[error("Specified property cannot be set while the task is running. Set the property prior to starting the task, or stop the task prior to setting the property.")]
  ErrorCannotSetPropertyWhenTaskRunning =
    DAQmxErrorCannotSetPropertyWhenTaskRunning,
  #[error("You only can get the specified property while the task is committed or while the task is running. Commit or start the task prior to getting the property.")]
  ErrorCannotGetPropertyWhenTaskNotCommittedOrRunning =
    DAQmxErrorCannotGetPropertyWhenTaskNotCommittedOrRunning,
  #[error("Specified string is not valid, because its first character is an underscore.")]
  ErrorLeadingUnderscoreInString = DAQmxErrorLeadingUnderscoreInString,
  #[error("Specified string is not valid, because its last character is a space character.")]
  ErrorTrailingSpaceInString = DAQmxErrorTrailingSpaceInString,
  #[error("Specified string is not valid, because its first character is a space character.")]
  ErrorLeadingSpaceInString = DAQmxErrorLeadingSpaceInString,
  #[error(
    "Specified string is not valid, because it contains an invalid character."
  )]
  ErrorInvalidCharInString = DAQmxErrorInvalidCharInString,
  #[error("Hardware clocking error occurred. If you are using an external reference or sample clock, make sure it is connected and within the jitter and voltage level specifications at all times. Also, verify that its rate matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDLLBecameUnlocked = DAQmxErrorDLLBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external reference or sample clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that its rate matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDLLLock = DAQmxErrorDLLLock,
  #[error("Self-calibration constants are invalid. Perform a self-calibration. Contact National Instruments Technical Support if you need additional information.")]
  ErrorSelfCalConstsInvalid = DAQmxErrorSelfCalConstsInvalid,
  #[error("Requested coupling type is only valid when the trigger source is an external trigger channel.")]
  ErrorInvalidTrigCouplingExceptForExtTrigChan =
    DAQmxErrorInvalidTrigCouplingExceptForExtTrigChan,
  #[error("DAQmx Write failed, because a previous DAQmx Write automatically configured the output buffer size. The buffer size is equal to the original number of samples written per channel, so no more data can be written prior to starting the task. Start the generation before the second DAQmx Write, or set Auto Start to true in all occurences of DAQmx Write. To incrementally write into the buffer prior to starting the task, call DAQmx Configure Output Buffer before the first DAQmx Write.")]
  ErrorWriteFailsBufferSizeAutoConfigured =
    DAQmxErrorWriteFailsBufferSizeAutoConfigured,
  #[error("External calibration failed, and the external calibration data has not been changed. Make sure that the reference signal used for the calibration is stable and that the voltage matches the value specified to the calibration software. Disconnect the device from any external signals that might be introducing noise.")]
  ErrorExtCalAdjustExtRefVoltageFailed =
    DAQmxErrorExtCalAdjustExtRefVoltageFailed,
  #[error("Self-calibration failed.  The self-calibration date has not changed. Disconnect the device from external signals, as they might introduce noise.  Externally calibrate the device to recalibrate the onboard voltage reference that is used for self-calibration.")]
  ErrorSelfCalFailedExtNoiseOrRefVoltageOutOfCal =
    DAQmxErrorSelfCalFailedExtNoiseOrRefVoltageOutOfCal,
  #[error("Last External Calibration Temperature has not been stored on the device by NI-DAQmx. Externally calibrate the board using NI-DAQmx.")]
  ErrorExtCalTemperatureNotDAQmx = DAQmxErrorExtCalTemperatureNotDAQmx,
  #[error("Last External Calibration Date/Time has not been stored on the device by NI-DAQmx. Externally calibrate the board using NI-DAQmx.")]
  ErrorExtCalDateTimeNotDAQmx = DAQmxErrorExtCalDateTimeNotDAQmx,
  #[error("Last Self Calibration Temperature has not been stored on the device by NI-DAQmx. Self-calibrate the board using NI-DAQmx. Alternatively, externally calibrate the board using NI-DAQmx, and then call DAQmx Restore Last External Calibration Constants.")]
  ErrorSelfCalTemperatureNotDAQmx = DAQmxErrorSelfCalTemperatureNotDAQmx,
  #[error("Last Self Calibration Date/Time has not been stored on the device by NI-DAQmx. Self-calibrate the board using NI-DAQmx. Alternatively, externally calibrate the board using NI-DAQmx, and then call DAQmx Restore Last External Calibration Constants.")]
  ErrorSelfCalDateTimeNotDAQmx = DAQmxErrorSelfCalDateTimeNotDAQmx,
  #[error("DAC Reference Voltage Value is not set. When the DAC Reference Voltage Source property for a channel is set to External, the DAC Reference Voltage Value property must be set. Set the DAC Reference Voltage Value property so the value matches the reference voltage source connected to your device. Alternatively, consider using the internal DAC reference voltage source available on the device.")]
  ErrorDACRefValNotSet = DAQmxErrorDACRefValNotSet,
  #[error("Device does not support analog writes with multiple samples per channel. To output multiple samples, call DAQmx Analog Single Sample Write multiple times.")]
  ErrorAnalogMultiSampWriteNotSupported =
    DAQmxErrorAnalogMultiSampWriteNotSupported,
  #[error("Action requested is invalid.")]
  ErrorInvalidActionInControlTask = DAQmxErrorInvalidActionInControlTask,
  #[error("Supplied forward and reverse coefficients yield inconsistent results when they are used for computations related to this property. In other words, using the result of the forward scale as input to the reverse scale does not yield the original data. Based on the forward coefficients, the value for the property is invalid. Supply forward and reverse coefficients that yield consistent results.")]
  ErrorPolyCoeffsInconsistent = DAQmxErrorPolyCoeffsInconsistent,
  #[error("Specified value is smaller than the minimum value supported for this property.")]
  ErrorSensorValTooLow = DAQmxErrorSensorValTooLow,
  #[error("Specified value is larger than the maximum value supported for this property.")]
  ErrorSensorValTooHigh = DAQmxErrorSensorValTooHigh,
  #[error("Waveform name is too long. Use waveform names with no more than 511 characters.")]
  ErrorWaveformNameTooLong = DAQmxErrorWaveformNameTooLong,
  #[error("Identifier found in the script is too long. Use identifiers with no more than 511 characters.")]
  ErrorIdentifierTooLongInScript = DAQmxErrorIdentifierTooLongInScript,
  #[error("Unexpected identifier following switch channel name.")]
  ErrorUnexpectedIDFollowingSwitchChanName =
    DAQmxErrorUnexpectedIDFollowingSwitchChanName,
  #[error("Relay name is not specified in the list entry.")]
  ErrorRelayNameNotSpecifiedInList = DAQmxErrorRelayNameNotSpecifiedInList,
  #[error("Unexpected identifier following the relay name in the relay list. Relay names must be separated by a comma inside the relay list.")]
  ErrorUnexpectedIDFollowingRelayNameInList =
    DAQmxErrorUnexpectedIDFollowingRelayNameInList,
  #[error("Unexpected identifier following the switch operation in the connection list. Switch operations must be separated by a comma inside the connection list.")]
  ErrorUnexpectedIDFollowingSwitchOpInList =
    DAQmxErrorUnexpectedIDFollowingSwitchOpInList,
  #[error("Requested line grouping is invalid. Either choose one channel for all lines or one channel for each line as the line grouping.")]
  ErrorInvalidLineGrouping = DAQmxErrorInvalidLineGrouping,
  #[error("Requested values of the Minimum and Maximum properties for the counter channel are not supported for the given type of device. The values that can be specified for Minimum and Maximum depend on the counter timebase rate.")]
  ErrorCtrMinMax = DAQmxErrorCtrMinMax,
  #[error("Write cannot be performed because this version of DAQmx Write does not match the type of channels in the task. Use the version of DAQmx Write that corresponds to the channel type.")]
  ErrorWriteChanTypeMismatch = DAQmxErrorWriteChanTypeMismatch,
  #[error("Read cannot be performed because this version of DAQmx Read does not match the type of channels in the task. Use the version of DAQmx Read that corresponds to the channel type.")]
  ErrorReadChanTypeMismatch = DAQmxErrorReadChanTypeMismatch,
  #[error("Write cannot be performed, because the number of channels in the data does not match the number of channels in the task. When writing, supply data for all channels in the task. Alternatively, modify the task to contain the same number of channels as the data written.")]
  ErrorWriteNumChansMismatch = DAQmxErrorWriteNumChansMismatch,
  #[error("Read cannot be performed because this version of DAQmx Read only returns data from a single channel, and there are multiple channels in the task. Use the multichannel version of DAQmx Read.")]
  ErrorOneChanReadForMultiChanTask = DAQmxErrorOneChanReadForMultiChanTask,
  #[error(
    "A self-calibration cannot be performed during external calibration."
  )]
  ErrorCannotSelfCalDuringExtCal = DAQmxErrorCannotSelfCalDuringExtCal,
  #[error("A measurement taken during adjustment of the oscillator phase DAC produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustOscillatorPhaseDAC =
    DAQmxErrorMeasCalAdjustOscillatorPhaseDAC,
  #[error("Calibration constants stored in the EEPROM and used to adjust reads from the cal ADC are invalid. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, or contact National Instruments Technical Support.")]
  ErrorInvalidCalConstCalADCAdjustment =
    DAQmxErrorInvalidCalConstCalADCAdjustment,
  #[error("Calibration constants stored in the EEPROM produced an invalid value for the oscillator frequency DAC. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, or contact National Instruments Technical Support.")]
  ErrorInvalidCalConstOscillatorFreqDACValue =
    DAQmxErrorInvalidCalConstOscillatorFreqDACValue,
  #[error("Calibration constants stored in the EEPROM produced an invalid value for the oscillator phase DAC. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, perform a self-calibration, or contact National Instruments Technical Support.")]
  ErrorInvalidCalConstOscillatorPhaseDACValue =
    DAQmxErrorInvalidCalConstOscillatorPhaseDACValue,
  #[error("Calibration constants stored in the EEPROM produced an invalid value for the offset DAC. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, or a self-calibration, or contact National Instruments Technical Support.")]
  ErrorInvalidCalConstOffsetDACValue = DAQmxErrorInvalidCalConstOffsetDACValue,
  #[error("Calibration constants stored in the EEPROM produced an invalid value for the gain DAC. An incorrect calibration might have been performed, or the calibration data in the EEPROM might have been corrupted. Perform an external calibration, perform a self-calibration, or contact National Instruments Technical Support.")]
  ErrorInvalidCalConstGainDACValue = DAQmxErrorInvalidCalConstGainDACValue,
  #[error("Specified number of reads to average from the calibration ADC is invalid. The number of reads to average must be greater than 0.")]
  ErrorInvalidNumCalADCReadsToAverage =
    DAQmxErrorInvalidNumCalADCReadsToAverage,
  #[error("Requested an invalid configuration value for adjusting the direct path output impedance. Refer to the documentation for a list of valid configuration values.")]
  ErrorInvalidCfgCalAdjustDirectPathOutputImpedance =
    DAQmxErrorInvalidCfgCalAdjustDirectPathOutputImpedance,
  #[error("Requested an invalid configuration value for adjusting the main path output impedance. Refer to the documentation for a list of valid configuration values.")]
  ErrorInvalidCfgCalAdjustMainPathOutputImpedance =
    DAQmxErrorInvalidCfgCalAdjustMainPathOutputImpedance,
  #[error("Requested an invalid configuration value for adjusting the main path post-amplifier gain and offset. Refer to the documentation for a list of valid configuration values.")]
  ErrorInvalidCfgCalAdjustMainPathPostAmpGainAndOffset =
    DAQmxErrorInvalidCfgCalAdjustMainPathPostAmpGainAndOffset,
  #[error("Requested an invalid configuration value for adjusting the main path pre-amplifier gain. Refer to the documentation for a list of valid configuration values.")]
  ErrorInvalidCfgCalAdjustMainPathPreAmpGain =
    DAQmxErrorInvalidCfgCalAdjustMainPathPreAmpGain,
  #[error("Requested an invalid configuration value for adjusting the main path pre-amplifier offset. Refer to the documentation for a list of valid configuration values.")]
  ErrorInvalidCfgCalAdjustMainPreAmpOffset =
    DAQmxErrorInvalidCfgCalAdjustMainPreAmpOffset,
  #[error("A measurement taken during adjustment of calibration ADC produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustCalADC = DAQmxErrorMeasCalAdjustCalADC,
  #[error("A measurement taken during adjustment of the oscillator frequency produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustOscillatorFrequency =
    DAQmxErrorMeasCalAdjustOscillatorFrequency,
  #[error("A measurement taken during adjustment of direct path output impedance produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustDirectPathOutputImpedance =
    DAQmxErrorMeasCalAdjustDirectPathOutputImpedance,
  #[error("A measurement taken during adjustment of main path output impedance produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustMainPathOutputImpedance =
    DAQmxErrorMeasCalAdjustMainPathOutputImpedance,
  #[error("A measurement taken during adjustment of direct path gain produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustDirectPathGain = DAQmxErrorMeasCalAdjustDirectPathGain,
  #[error("A measurement taken during adjustment of main path post-amplifier gain and offset produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustMainPathPostAmpGainAndOffset =
    DAQmxErrorMeasCalAdjustMainPathPostAmpGainAndOffset,
  #[error("A measurement taken during adjustment of main path pre-amplifier gain produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustMainPathPreAmpGain =
    DAQmxErrorMeasCalAdjustMainPathPreAmpGain,
  #[error("A measurement taken during adjustment of main path pre-amplifier offset produced an invalid calibration constant. Make sure that the measured values passed to the calibration VI or function are correct. Verify the calibration procedure, and repeat the calibration. If the problem persists, there might be a hardware malfunction with your device. Contact National Instruments Technical Support.")]
  ErrorMeasCalAdjustMainPathPreAmpOffset =
    DAQmxErrorMeasCalAdjustMainPathPreAmpOffset,
  #[error("EEPROM contains an invalid calibration date and/or time. The calibration data in the EEPROM might have been corrupted. Perform an external calibration, perform a self-calibration, or contact National Instruments Technical Support.")]
  ErrorInvalidDateTimeInEEPROM = DAQmxErrorInvalidDateTimeInEEPROM,
  #[error(" Error = DAQmxError code could not be found. Reinstalling the driver might fix the issue. Otherwise, contact National Instruments technical support.")]
  ErrorUnableToLocateErrorResources = DAQmxErrorUnableToLocateErrorResources,
  #[error("Value passed is not between 0 and 4,294,967,295 (unsigned 32-bit integer).")]
  ErrorDotNetAPINotUnsigned32BitNumber =
    DAQmxErrorDotNetAPINotUnsigned32BitNumber,
  #[error("Syntax for a range of objects in the input string is invalid. For ranges of objects, specify a number immediately before and after every colon (\":\") in the input string. Or, if a name is specified after the colon, it must be identical to the name specified immediately before the colon. Colons are not allowed within the names of the individual objects.")]
  ErrorInvalidRangeOfObjectsSyntaxInString =
    DAQmxErrorInvalidRangeOfObjectsSyntaxInString,
  #[error("Attempted to enable output data lines that were not previously disabled. Make sure that you are enabling data lines for output only after explicitly disabling them.")]
  ErrorAttemptToEnableLineNotPreviouslyDisabled =
    DAQmxErrorAttemptToEnableLineNotPreviouslyDisabled,
  #[error("Pattern contains an invalid character.")]
  ErrorInvalidCharInPattern = DAQmxErrorInvalidCharInPattern,
  #[error("An intermediate acquisition buffer has overflowed.  The driver was unable to read samples from the intermediate buffer fast enough to prevent the buffer from overflowing.")]
  ErrorIntermediateBufferFull = DAQmxErrorIntermediateBufferFull,
  #[error("Specified task cannot be loaded, because it requires a device that supports timing, and the associated device does not support timing. Create a new task without timing, or associate this task with a device that supports timing.")]
  ErrorLoadTaskFailsBecauseNoTimingOnDev =
    DAQmxErrorLoadTaskFailsBecauseNoTimingOnDev,
  #[error(
    "Reserved character string parameter must be NULL or an empty string."
  )]
  ErrorCAPIReservedParamNotNULLNorEmpty =
    DAQmxErrorCAPIReservedParamNotNULLNorEmpty,
  #[error("Reserved parameter must be NULL.")]
  ErrorCAPIReservedParamNotNULL = DAQmxErrorCAPIReservedParamNotNULL,
  #[error("Reserved parameter must be zero.")]
  ErrorCAPIReservedParamNotZero = DAQmxErrorCAPIReservedParamNotZero,
  #[error("Sample value detected outside of the specified range.")]
  ErrorSampleValueOutOfRange = DAQmxErrorSampleValueOutOfRange,
  #[error("Specified channel cannot be added to the task, because a channel with the same name is already in the task.")]
  ErrorChanAlreadyInTask = DAQmxErrorChanAlreadyInTask,
  #[error("Specified virtual channel or task is invalid. If you are attempting to add channels to a task, create a virtual channel using the DAQ Assistant or DAQmx Create Virtual Channel, and then add the virtual channel to the task. If you are attempting to reference a LabVIEW NXG project task from a LabVIEW NXG VI, any of the following may resolve the error: 1. Ensure the task name is correct. 2. Use a DAQmx Task constant to reference the task from the VI's diagram. 3. Configure the task's owning Application or Library to export or always include the task.")]
  ErrorVirtualChanDoesNotExist = DAQmxErrorVirtualChanDoesNotExist,
  #[error("Specified channel is not in the task.")]
  ErrorChanNotInTask = DAQmxErrorChanNotInTask,
  #[error("The specified task cannot be loaded, because it is not in Data Neighborhood. Check Data Neighborhood in MAX.  Look for similar characters, such as the capital letter \"O\" and the number zero.")]
  ErrorTaskNotInDataNeighborhood = DAQmxErrorTaskNotInDataNeighborhood,
  #[error("Specified task cannot be saved, because a task with that name already exists in Data Neighborhood in MAX. Save the task under a different name, or specify that the existing task should be replaced.")]
  ErrorCantSaveTaskWithoutReplace = DAQmxErrorCantSaveTaskWithoutReplace,
  #[error("Specified virtual channel cannot be saved, because a virtual channel with that name already exists in Data Neighborhood in MAX. Save the virtual channel under a different name, or specify that the existing virtual channel should be replaced.")]
  ErrorCantSaveChanWithoutReplace = DAQmxErrorCantSaveChanWithoutReplace,
  #[error("Specified device is not in the task.")]
  ErrorDevNotInTask = DAQmxErrorDevNotInTask,
  #[error("Specified device cannot be added to the task, because it is already in the task.")]
  ErrorDevAlreadyInTask = DAQmxErrorDevAlreadyInTask,
  #[error(
    "Specified operation cannot be performed while the task is running."
  )]
  ErrorCanNotPerformOpWhileTaskRunning =
    DAQmxErrorCanNotPerformOpWhileTaskRunning,
  #[error("Specified operation cannot be performed when there are no channels in the task.")]
  ErrorCanNotPerformOpWhenNoChansInTask =
    DAQmxErrorCanNotPerformOpWhenNoChansInTask,
  #[error("Specified operation cannot be performed when there are no devices in the task.")]
  ErrorCanNotPerformOpWhenNoDevInTask =
    DAQmxErrorCanNotPerformOpWhenNoDevInTask,
  #[error("Specified operation can be performed only when the task is running. Start the task before requesting this operation.")]
  ErrorCannotPerformOpWhenTaskNotRunning =
    DAQmxErrorCannotPerformOpWhenTaskNotRunning,
  #[error("Specified operation did not complete, because the specified timeout expired.")]
  ErrorOperationTimedOut = DAQmxErrorOperationTimedOut,
  #[error("Read cannot be performed when the Auto Start property is false and the task is not running or committed. Start the task before reading, or set Auto Start to true.")]
  ErrorCannotReadWhenAutoStartFalseAndTaskNotRunningOrCommitted =
    DAQmxErrorCannotReadWhenAutoStartFalseAndTaskNotRunningOrCommitted,
  #[error("Write cannot be performed when the auto start input to DAQmx Write is false, and timing for the task is not configured or Timing Type is set to On Demand. Set auto start to true, or configure timing and specify Timing Type other than On Demand.")]
  ErrorCannotWriteWhenAutoStartFalseAndTaskNotRunningOrCommitted =
    DAQmxErrorCannotWriteWhenAutoStartFalseAndTaskNotRunningOrCommitted,
  #[error("Specified task cannot be loaded, because it was saved with an incompatible, more recent version of NI-DAQ. Create the task again, or upgrade NI-DAQ to a version compatible with the version used to save the task. Consult the documentation for the version of NI-DAQ used to create the task for more details.")]
  ErrorTaskVersionNew = DAQmxErrorTaskVersionNew,
  #[error("Specified channel cannot be loaded, because it was saved with an incompatible, more recent version of NI-DAQ. Create the channel again, or upgrade NI-DAQ to a version compatible with the version used to save the channel. Consult the documentation for the version of NI-DAQ used to create the channel for more details.")]
  ErrorChanVersionNew = DAQmxErrorChanVersionNew,
  #[error("Specified string is not valid, because it is empty.")]
  ErrorEmptyString = DAQmxErrorEmptyString,
  #[error("Specified digital channel contains more bits than what is supported by DAQmx Port Read.  With the U8 version, channels must contain 8 bits or less; while for the U32 version, channels must contain 32 bits or less.")]
  ErrorChannelSizeTooBigForPortReadType =
    DAQmxErrorChannelSizeTooBigForPortReadType,
  #[error("Specified digital channel contains more bits than what is supported by DAQmx Port Write.  With the U8 version, channels must contain 8 bits or less; while for the U32 version, channels must contain 32 bits or less.")]
  ErrorChannelSizeTooBigForPortWriteType =
    DAQmxErrorChannelSizeTooBigForPortWriteType,
  #[error("If performing a Write operation, the operation cannot be performed, because the number of channels in the specified data does not match the number of channels in this task.  Adjust the data to match the number of channels in this task.  If you are performing a read operation, the operation cannot be performed because this DAQmx Read only returns data from a single channel, and there are multiple channels in this task.  Use the multichannel DAQmx Read.")]
  ErrorExpectedNumberOfChannelsVerificationFailed =
    DAQmxErrorExpectedNumberOfChannelsVerificationFailed,
  #[error("Specified read or write operation failed, because the number of lines in the data for a channel does not match the number of lines in the channel. If you are using the Digital Waveform datatype, make sure the number of lines in the digital waveform matches the number of lines in the channel. If you are using boolean data, make sure the array dimension for lines in the data matches the number of lines in the channel.")]
  ErrorNumLinesMismatchInReadOrWrite = DAQmxErrorNumLinesMismatchInReadOrWrite,
  #[error("Generation cannot be started because the output buffer is empty. Write data before starting a buffered generation. The following actions can empty the buffer: changing the size of the buffer, unreserving a task, setting the Regeneration Mode property, changing the Sample Mode, or configuring retriggering.")]
  ErrorOutputBufferEmpty = DAQmxErrorOutputBufferEmpty,
  #[error("Specified channel name is invalid.")]
  ErrorInvalidChanName = DAQmxErrorInvalidChanName,
  #[error(
        "Read failed, because there are no channels in this task from which data can be read."
    )]
  ErrorReadNoInputChansInTask = DAQmxErrorReadNoInputChansInTask,
  #[error("Write failed, because there are no output channels in this task to which data can be written.")]
  ErrorWriteNoOutputChansInTask = DAQmxErrorWriteNoOutputChansInTask,
  #[error("Specified property is not supported, because the task is not an input task.")]
  ErrorPropertyNotSupportedNotInputTask =
    DAQmxErrorPropertyNotSupportedNotInputTask,
  #[error("Specified property is not supported, because the task is not an output task.")]
  ErrorPropertyNotSupportedNotOutputTask =
    DAQmxErrorPropertyNotSupportedNotOutputTask,
  #[error(
        "You cannot get the specified property, because the task is not a buffered input task."
    )]
  ErrorGetPropertyNotInputBufferedTask =
    DAQmxErrorGetPropertyNotInputBufferedTask,
  #[error(
        "You cannot get the specified property, because the task is not a buffered output task."
    )]
  ErrorGetPropertyNotOutputBufferedTask =
    DAQmxErrorGetPropertyNotOutputBufferedTask,
  #[error("Specified timeout value is not supported.  Supported timeout values are 0 (try or check once and return), positive numbers up to 4294967, and -1 (try or check until success or error).")]
  ErrorInvalidTimeoutVal = DAQmxErrorInvalidTimeoutVal,
  #[error("Specified property is not supported by the device or is not applicable to the task.")]
  ErrorAttributeNotSupportedInTaskContext =
    DAQmxErrorAttributeNotSupportedInTaskContext,
  #[error("You can get the specified property only while the task is committed or running.")]
  ErrorAttributeNotQueryableUnlessTaskIsCommitted =
    DAQmxErrorAttributeNotQueryableUnlessTaskIsCommitted,
  #[error("Specified property cannot be set while the task is running.")]
  ErrorAttributeNotSettableWhenTaskIsRunning =
    DAQmxErrorAttributeNotSettableWhenTaskIsRunning,
  #[error("DAC Range Low must be equal to either the negative DAC Reference Voltage Value or to zero. If you do not set the DAC Range Low property, the driver sets it for you. Otherwise, make sure DAC Range Low is equal to either the negative DAC Reference Voltage Value or to zero.")]
  ErrorDACRngLowNotMinusRefValNorZero =
    DAQmxErrorDACRngLowNotMinusRefValNorZero,
  #[error("DAC Range High is not equal to the DAC Reference Voltage Value. When you do not set the DAC Range High property, the driver always makes sure it is equal to the DAC Reference Voltage value. If you do set the DAC Range High property make sure DAC Range High and DAC Reference Voltage Value are equal.")]
  ErrorDACRngHighNotEqualRefVal = DAQmxErrorDACRngHighNotEqualRefVal,
  #[error("Units for the channel must be set to From Custom Scale when a custom scale is used with a channel.")]
  ErrorUnitsNotFromCustomScale = DAQmxErrorUnitsNotFromCustomScale,
  #[error("External stimulus voltage read was outside the expected range. Verify that the external stimulus voltage is properly connected and has the correct amplitude.")]
  ErrorInvalidVoltageReadingDuringExtCal =
    DAQmxErrorInvalidVoltageReadingDuringExtCal,
  #[error("Requested calibration function is not supported by the device.")]
  ErrorCalFunctionNotSupported = DAQmxErrorCalFunctionNotSupported,
  #[error("Invalid physical channel selected for calibration.")]
  ErrorInvalidPhysicalChanForCal = DAQmxErrorInvalidPhysicalChanForCal,
  #[error("Attempt to store calibration constants without completing all the necessary external calibration steps. Refer to the documentation for the calibration procedure. Verify that all necessary steps are performed before closing the external calibration session.")]
  ErrorExtCalNotComplete = DAQmxErrorExtCalNotComplete,
  #[error("Unable to synchronize to the external stimulus frequency. Verify that the external stimulus has the correct frequency, amplitude, and stability. Consult the documentation for the calibration procedure for valid ranges.")]
  ErrorCantSyncToExtStimulusFreqDuringCal =
    DAQmxErrorCantSyncToExtStimulusFreqDuringCal,
  #[error("Unable to detect the external stimulus frequency. Verify that the external stimulus is properly connected and has the correct frequency and amplitude.")]
  ErrorUnableToDetectExtStimulusFreqDuringCal =
    DAQmxErrorUnableToDetectExtStimulusFreqDuringCal,
  #[error(
    "Requested calibration close action is invalid. Select Store or Abort."
  )]
  ErrorInvalidCloseAction = DAQmxErrorInvalidCloseAction,
  #[error("Requested operation only can be used during an external calibration session.")]
  ErrorExtCalFunctionOutsideExtCalSession =
    DAQmxErrorExtCalFunctionOutsideExtCalSession,
  #[error("Invalid calibration area selected. Select self-calibration or external calibration.")]
  ErrorInvalidCalArea = DAQmxErrorInvalidCalArea,
  #[error("External calibration constants are invalid. Perform an external calibration. Contact National Instruments Technical Support if you need additional information.")]
  ErrorExtCalConstsInvalid = DAQmxErrorExtCalConstsInvalid,
  #[error("Start trigger delay is not available when an external sample clock source is specified. Change the sample clock to onboard clock, or do not configure the start trigger delay.")]
  ErrorStartTrigDelayWithExtSampClk = DAQmxErrorStartTrigDelayWithExtSampClk,
  #[error("Delay from sample clock is not available when an external convert source is specified. Change the convert source to onboard clock, or do not configure the delay from sample clock.")]
  ErrorDelayFromSampClkWithExtConv = DAQmxErrorDelayFromSampClkWithExtConv,
  #[error(
    "Prescaled Values for a table scale must contain at least two values."
  )]
  ErrorFewerThan2PreScaledVals = DAQmxErrorFewerThan2PreScaledVals,
  #[error("Scaled Values for a table scale must contain at least two values.")]
  ErrorFewerThan2ScaledValues = DAQmxErrorFewerThan2ScaledValues,
  #[error("Selected physical channel does not support the output type required by the virtual channel you are creating. Create a channel of an output type that is supported by the physical channel, or select a physical channel that supports the output type.")]
  ErrorPhysChanOutputType = DAQmxErrorPhysChanOutputType,
  #[error("Selected physical channel does not support the measurement type required by the virtual channel you are creating. Create a channel of a measurement type that is supported by the physical channel, or select a physical channel that supports the measurement type.")]
  ErrorPhysChanMeasType = DAQmxErrorPhysChanMeasType,
  #[error("I/O type of the physical channel does not match the I/O type required for the virtual channel you are creating.")]
  ErrorInvalidPhysChanType = DAQmxErrorInvalidPhysChanType,
  #[error("Value passed to the Task/Channels In control is an empty string (or I/O control). The value must refer to a valid task or valid channels.")]
  ErrorLabVIEWEmptyTaskOrChans = DAQmxErrorLabVIEWEmptyTaskOrChans,
  #[error("Value passed to the Task/Channels In control is invalid. The value must refer to a valid task or valid virtual channels.")]
  ErrorLabVIEWInvalidTaskOrChans = DAQmxErrorLabVIEWInvalidTaskOrChans,
  #[error("Configured reference clock rate is invalid. The reference clock rate must be within the valid range and a multiple of the increment value.")]
  ErrorInvalidRefClkRate = DAQmxErrorInvalidRefClkRate,
  #[error("Requested impedance for the external trigger is invalid. Specify an impedance that is appropriate for the external trigger, or choose a different trigger source.")]
  ErrorInvalidExtTrigImpedance = DAQmxErrorInvalidExtTrigImpedance,
  #[error("Requested hysteresis is not valid with the configured trigger level and AI Maximum. Configure the task so the following formula is satisfied: Trigger Level + Hysteresis < AI Maximum")]
  ErrorHystTrigLevelAIMax = DAQmxErrorHystTrigLevelAIMax,
  #[error(
        "Requested video trigger line number is incompatible with the chosen video signal format."
    )]
  ErrorLineNumIncompatibleWithVideoSignalFormat =
    DAQmxErrorLineNumIncompatibleWithVideoSignalFormat,
  #[error("Requested window trigger level is not valid with the configured AI Minimum and AI Maximum. Configure the instrument so the following formula is satisfied: AI Minimum < Window Trigger Level < AI Maximum")]
  ErrorTrigWindowAIMinAIMaxCombo = DAQmxErrorTrigWindowAIMinAIMaxCombo,
  #[error("Requested Trigger Level is not valid with the configured AI Minimum and AI Maximum. Configure the instrument so the following formula is satisfied: AI Minimum < Trigger Level < AI Maximum")]
  ErrorTrigAIMinAIMax = DAQmxErrorTrigAIMinAIMax,
  #[error("Requested Hysteresis is not valid with the configured Trigger Level and AI Minimum. Configure the task so the following formula is satisfied: Trigger Level - Hysteresis > AI Minimum")]
  ErrorHystTrigLevelAIMin = DAQmxErrorHystTrigLevelAIMin,
  #[error("Requested sample rate exceeds maximum real-time sample rate. If you want a higher sampling rate and have a repetitive signal, enable RIS.")]
  ErrorInvalidSampRateConsiderRIS = DAQmxErrorInvalidSampRateConsiderRIS,
  #[error("Requested Read Position is invalid in RIS mode.")]
  ErrorInvalidReadPosDuringRIS = DAQmxErrorInvalidReadPosDuringRIS,
  #[error("Requested immediate trigger type while in RIS mode. Immediate triggering is not compatible with the RIS mode. Select a different trigger type, or do not use RIS.")]
  ErrorImmedTrigDuringRISMode = DAQmxErrorImmedTrigDuringRISMode,
  #[error("TDC is not enabled during RIS mode. TDC must be enabled when the digitizer is in the RIS mode. Enable TDC, or do not use RIS.")]
  ErrorTDCNotEnabledDuringRISMode = DAQmxErrorTDCNotEnabledDuringRISMode,
  #[error("Multiple records are not available with RIS.")]
  ErrorMultiRecWithRIS = DAQmxErrorMultiRecWithRIS,
  #[error("Requested reference clock source is invalid.")]
  ErrorInvalidRefClkSrc = DAQmxErrorInvalidRefClkSrc,
  #[error("Requested sample clock source is invalid.")]
  ErrorInvalidSampClkSrc = DAQmxErrorInvalidSampClkSrc,
  #[error("Insufficient onboard memory for requested Number of Records and Samples per Channel combination. Reduce the Number of Records and/or Samples per Channel.")]
  ErrorInsufficientOnBoardMemForNumRecsAndSamps =
    DAQmxErrorInsufficientOnBoardMemForNumRecsAndSamps,
  #[error("Requested analog input attenuation is invalid.")]
  ErrorInvalidAIAttenuation = DAQmxErrorInvalidAIAttenuation,
  #[error("AC coupling is not allowed with 50 Ohm impedance for this device. Use DC coupling, or configure a different impedance setting.")]
  ErrorACCouplingNotAllowedWith50OhmImpedance =
    DAQmxErrorACCouplingNotAllowedWith50OhmImpedance,
  #[error("Requested record number is invalid. Use the Records Done property to find out how many records are available. Record numbers start at 0. Use -1 for all available records.")]
  ErrorInvalidRecordNum = DAQmxErrorInvalidRecordNum,
  #[error(
    "Slope for a linear scale is set to zero. The slope must be non-zero."
  )]
  ErrorZeroSlopeLinearScale = DAQmxErrorZeroSlopeLinearScale,
  #[error("Reverse Coefficients for a polynomial scale are all set to zero. At least one of these coefficients must be non-zero.")]
  ErrorZeroReversePolyScaleCoeffs = DAQmxErrorZeroReversePolyScaleCoeffs,
  #[error("Forward Coefficients for a polynomial scale are all set to zero. At least one of these coefficients must be non-zero.")]
  ErrorZeroForwardPolyScaleCoeffs = DAQmxErrorZeroForwardPolyScaleCoeffs,
  #[error("Reverse Coefficients for a polynomial scale are not specified. This set of coefficients must contain at least one term. If the coefficients are not available, use the Compute Reverse Polynomial Coefficient utility to calculate the required coefficients from the supplied Forward Coefficients.")]
  ErrorNoReversePolyScaleCoeffs = DAQmxErrorNoReversePolyScaleCoeffs,
  #[error("Forward Coefficients for a polynomial scale are not specified. This set of coefficients must contain at least one term. If the coefficients are not available, you can pass the supplied Reverse Coefficients to the Compute Reverse Polynomial Coefficients utility to calculate the required Forward Coefficients.")]
  ErrorNoForwardPolyScaleCoeffs = DAQmxErrorNoForwardPolyScaleCoeffs,
  #[error("Forward and Reverse Coefficients for a polynomial scale are not specified. Each of these two sets of coefficients must contain at least one term. If only one set of coefficients is available, use the Compute Reverse Polynomial Coefficient utility to calculate the other set of coefficients.")]
  ErrorNoPolyScaleCoeffs = DAQmxErrorNoPolyScaleCoeffs,
  #[error("Order of the reverse polynomial to compute is less than or equal to the number of points to compute over the range of x values. Reduce the order of the reverse polynomial or increase the number of points to compute over the range of x values.")]
  ErrorReversePolyOrderLessThanNumPtsToCompute =
    DAQmxErrorReversePolyOrderLessThanNumPtsToCompute,
  #[error("Order of the reverse polynomial to compute is not positive. Specify a value greater than 0 for this input.")]
  ErrorReversePolyOrderNotPositive = DAQmxErrorReversePolyOrderNotPositive,
  #[error("Number of points to compute over the range of x values is not positive. Specify a value greater than 0 for this input.")]
  ErrorNumPtsToComputeNotPositive = DAQmxErrorNumPtsToComputeNotPositive,
  #[error("Requested waveform length is invalid, because the number of samples is not an integer multiple of the waveform length increment.")]
  ErrorWaveformLengthNotMultipleOfIncr =
    DAQmxErrorWaveformLengthNotMultipleOfIncr,
  #[error("No extended error information is available for the last error code. It is possible that there was a problem initializing the internal errors database. Please contact National Instruments Technical Support.")]
  ErrorCAPINoExtendedErrorInfoAvailable =
    DAQmxErrorCAPINoExtendedErrorInfoAvailable,
  #[error("Unable to find function in NI-DAQmx dynamic link library NICAIU.DLL. The DLL exists on your computer, but the version is incorrect. Install the correct version of the DLL on your computer.")]
  ErrorCVIFunctionNotFoundInDAQmxDLL = DAQmxErrorCVIFunctionNotFoundInDAQmxDLL,
  #[error("Unable to load NI-DAQmx dynamic link library NICAIU.DLL. Make sure that NI-DAQmx is installed on your computer.")]
  ErrorCVIFailedToLoadDAQmxDLL = DAQmxErrorCVIFailedToLoadDAQmxDLL,
  #[error("There are no shared trigger lines between the two devices that are acceptable to both devices. While each of the two devices support some shared trigger lines, none of the shared trigger lines work for both devices for the specified source and destination terminals. Consider routing the signal through the I/O connectors of the two devices, if applicable.")]
  ErrorNoCommonTrigLineForImmedRoute = DAQmxErrorNoCommonTrigLineForImmedRoute,
  #[error("There are no shared trigger lines between the two devices which are acceptable to both devices. While each of these two devices support some shared trigger lines, none of these shared trigger lines work for both devices for the specified property and corresponding value. Consider routing the signal through the I/O connectors of the two devices, if applicable.")]
  ErrorNoCommonTrigLineForTaskRoute = DAQmxErrorNoCommonTrigLineForTaskRoute,
  #[error("Requested value for the property is invalid, because it is not an unsigned integer. Even though the datatype of the property is a floating point number, the value must be an unsigned integer less than or equal to 9,007,199,254,740,992 (2^53).")]
  ErrorF64PrptyValNotUnsignedInt = DAQmxErrorF64PrptyValNotUnsignedInt,
  #[error("You are attempting to write to a read-only register.")]
  ErrorRegisterNotWritable = DAQmxErrorRegisterNotWritable,
  #[error("Specified output voltage is not valid with the given sample clock rate. Make sure your output voltage level is compatible with your sample clock rate by altering the output voltage level or the sample clock rate. Consult your documentation for more information.")]
  ErrorInvalidOutputVoltageAtSampClkRate =
    DAQmxErrorInvalidOutputVoltageAtSampClkRate,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorStrobePhaseShiftDCMBecameUnlocked =
    DAQmxErrorStrobePhaseShiftDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDrivePhaseShiftDCMBecameUnlocked =
    DAQmxErrorDrivePhaseShiftDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorClkOutPhaseShiftDCMBecameUnlocked =
    DAQmxErrorClkOutPhaseShiftDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorOutputBoardClkDCMBecameUnlocked =
    DAQmxErrorOutputBoardClkDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorInputBoardClkDCMBecameUnlocked =
    DAQmxErrorInputBoardClkDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorInternalClkDCMBecameUnlocked = DAQmxErrorInternalClkDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDCMLock = DAQmxErrorDCMLock,
  #[error("Static output cannot be performed, because some data lines have already been reserved for a dynamic output.")]
  ErrorDataLineReservedForDynamicOutput =
    DAQmxErrorDataLineReservedForDynamicOutput,
  #[error("Reference Clock Source specified is invalid, given the Sample Clock Source. When Sample Clock Source is anything other than \"OnboardClock\", you must set Reference Clock Source to \"None\", and you cannot export the Reference Clock.")]
  ErrorInvalidRefClkSrcGivenSampClkSrc =
    DAQmxErrorInvalidRefClkSrcGivenSampClkSrc,
  #[error("Specified trigger type for pattern match mode could not be configured, because all pattern matchers are already in use.")]
  ErrorNoPatternMatcherAvailable = DAQmxErrorNoPatternMatcherAvailable,
  #[error("Unable to configure requested delay property given the current clock rate. Make sure the sample clock rate is greater than or equal to the phase shift DMC threshold of your device, or do not configure the delay. Consult your documentation for more information.")]
  ErrorInvalidDelaySampRateBelowPhaseShiftDCMThresh =
    DAQmxErrorInvalidDelaySampRateBelowPhaseShiftDCMThresh,
  #[error("Strain gage calibration procedure has failed. Make sure the strain gages are connected to all the specified strain channels, the strain gage connections are appropriate for their bridge type configurations, the shunt resistor location is specified correctly, and your hardware jumpers (if any) are set up correctly.")]
  ErrorStrainGageCalibration = DAQmxErrorStrainGageCalibration,
  #[error("External clock frequency and external clock divisor values result in an invalid cutoff frequency for this device. The relationship between cutoff frequency, external clock frequency, and external clock divisor is: cutoffFreq = extClkFreq / (100 * extClkDiv) Change your external clock frequency or external clock divisor.")]
  ErrorInvalidExtClockFreqAndDivCombo =
    DAQmxErrorInvalidExtClockFreqAndDivCombo,
  #[error("Custom scale specified does not exist.")]
  ErrorCustomScaleDoesNotExist = DAQmxErrorCustomScaleDoesNotExist,
  #[error("Requested operation is not supported by the device during a scan. The device only supports operations on front-end channels (for example, ch0, ch1, ... or cjtemp) while scanning. Other operations, such as operations on analog bus channels (such as ab0 and ab1), are not supported by the device during a scan. Make sure your scan list contains only supported operations.")]
  ErrorOnlyFrontEndChanOpsDuringScan = DAQmxErrorOnlyFrontEndChanOpsDuringScan,
  #[error("Requested creation of a separate channel for each line is not possible when a digital port is specified as the physical channel. Specify a range of digital lines, such as \"Dev1/port0/line0:7\", as the physical channel.")]
  ErrorInvalidOptionForDigitalPortChannel =
    DAQmxErrorInvalidOptionForDigitalPortChannel,
  #[error("Signal type requested to be exported is not supported for the specified task running on the given device.")]
  ErrorUnsupportedSignalTypeExportSignal =
    DAQmxErrorUnsupportedSignalTypeExportSignal,
  #[error("Signal type requested to be exported is not valid.")]
  ErrorInvalidSignalTypeExportSignal = DAQmxErrorInvalidSignalTypeExportSignal,
  #[error("Trigger type requested to be sent as software trigger is not supported for the specified task running on the given device.")]
  ErrorUnsupportedTrigTypeSendsSWTrig =
    DAQmxErrorUnsupportedTrigTypeSendsSWTrig,
  #[error(
    "Trigger type requested to be sent as the software trigger is invalid."
  )]
  ErrorInvalidTrigTypeSendsSWTrig = DAQmxErrorInvalidTrigTypeSendsSWTrig,
  #[error("Requested multiple virtual channels that correspond to the same physical channel within a task. A task cannot contain multiple physical channels of a specified type. Use different physical channels for each virtual channel.")]
  ErrorRepeatedPhysicalChan = DAQmxErrorRepeatedPhysicalChan,
  #[error("Specified route cannot be satisfied, because it requires resources that are currently in use by another route within this task.")]
  ErrorResourcesInUseForRouteInTask = DAQmxErrorResourcesInUseForRouteInTask,
  #[error("Specified route cannot be satisfied, because it requires resources that are currently in use by another route.")]
  ErrorResourcesInUseForRoute = DAQmxErrorResourcesInUseForRoute,
  #[error("Specified route cannot be satisfied, because the hardware does not support it.")]
  ErrorRouteNotSupportedByHW = DAQmxErrorRouteNotSupportedByHW,
  #[error("Specified polarity cannot be satisfied, because it requires resources that are currently in use by another route within this task.")]
  ErrorResourcesInUseForExportSignalPolarity =
    DAQmxErrorResourcesInUseForExportSignalPolarity,
  #[error("Specified inversion cannot be satisfied, because it requires resources that are currently in use by another route within this task.")]
  ErrorResourcesInUseForInversionInTask =
    DAQmxErrorResourcesInUseForInversionInTask,
  #[error("Specified inversion cannot be satisfied, because it requires resources that are currently in use by another route.")]
  ErrorResourcesInUseForInversion = DAQmxErrorResourcesInUseForInversion,
  #[error("Specified polarity is not supported by the hardware.")]
  ErrorExportSignalPolarityNotSupportedByHW =
    DAQmxErrorExportSignalPolarityNotSupportedByHW,
  #[error("Specified inversion cannot be satisfied, because the hardware does not support it.")]
  ErrorInversionNotSupportedByHW = DAQmxErrorInversionNotSupportedByHW,
  #[error("The Overloaded Channels Exist property was not read prior to reading the specified property. The driver retrieves the overload state from the hardware when the application reads the Overloaded Channels Exist property. After the Overloaded Channels Exist property has been read, other information about overloaded channels may be read, such as which channels are overloaded.")]
  ErrorOverloadedChansExistNotRead = DAQmxErrorOverloadedChansExistNotRead,
  #[error("Onboard device memory overflow. Because of system and/or bus-bandwidth limitations, the driver could not read data from the device fast enough to keep up with the device throughput. Reduce your sample rate. If your data transfer method is interrupts, try using DMA or USB Bulk. You can also use a product with more onboard memory or reduce the number of programs your computer is executing concurrently.")]
  ErrorInputFIFOOverflow2 = DAQmxErrorInputFIFOOverflow2,
  #[error("CJC Source has been set to Channel, while the CJC channel has not been specified. Specify the CJC channel, or set CJC Source to a value other than Channel.")]
  ErrorCJCChanNotSpecd = DAQmxErrorCJCChanNotSpecd,
  #[error("Counter signals cannot be exported, because there is more than one counter channel in the task. Create separate tasks for each counter channel.")]
  ErrorCtrExportSignalNotPossible = DAQmxErrorCtrExportSignalNotPossible,
  #[error("An attempt has been made to configure a reference trigger when the sample mode of the sample clock has been configured for continuous sampling. Reference trigger is only applicable for finite sampling. Change the sample mode to finite to use a reference trigger, or do not configure a reference trigger.")]
  ErrorRefTrigWhenContinuous = DAQmxErrorRefTrigWhenContinuous,
  #[error("Measurement device cannot acquire data from the sensor in its current configuration. The voltage output range of your sensor does not overlap with the voltage input range of your measurement device. If your measurement device supports different gains or input ranges, try using a lower gain or a wider input range. If the device has a fixed gain/range, you might need to change sensor attribute settings such as Excitation Value or use a measurement device that supports a wider voltage input range.")]
  ErrorIncompatibleSensorOutputAndDeviceInputRanges =
    DAQmxErrorIncompatibleSensorOutputAndDeviceInputRanges,
  #[error("Custom scale cannot be created. A saved scale with this name already exists.")]
  ErrorCustomScaleNameUsed = DAQmxErrorCustomScaleNameUsed,
  #[error("Specified property value cannot be used, because the hardware does not support it.")]
  ErrorPropertyValNotSupportedByHW = DAQmxErrorPropertyValNotSupportedByHW,
  #[error("Specified property value is not a valid terminal name.")]
  ErrorPropertyValNotValidTermName = DAQmxErrorPropertyValNotValidTermName,
  #[error("Specified property value cannot be used, because it requires resources that are currently in use.")]
  ErrorResourcesInUseForProperty = DAQmxErrorResourcesInUseForProperty,
  #[error("Physical channel corresponding to the virtual channel specified for cold-junction compensation is already being used for a thermocouple measurement, so it cannot be used as the cold-junction compensation channel.")]
  ErrorCJCChanAlreadyUsed = DAQmxErrorCJCChanAlreadyUsed,
  #[error("Forward coefficients must be specified for the polynomial scale.")]
  ErrorForwardPolynomialCoefNotSpecd = DAQmxErrorForwardPolynomialCoefNotSpecd,
  #[error("Number of Prescaled Values needs to be equal to the number of Scaled Values in the table scale.")]
  ErrorTableScaleNumPreScaledAndScaledValsNotEqual =
    DAQmxErrorTableScaleNumPreScaledAndScaledValsNotEqual,
  #[error("Prescaled Values must be specified for the table scale.")]
  ErrorTableScalePreScaledValsNotSpecd =
    DAQmxErrorTableScalePreScaledValsNotSpecd,
  #[error("Scaled Values must be specified for the table scale.")]
  ErrorTableScaleScaledValsNotSpecd = DAQmxErrorTableScaleScaledValsNotSpecd,
  #[error("Invalid intermediate buffer size. The size of the intermediate buffer must be an integer multiple of the intermediate buffer size increment.")]
  ErrorIntermediateBufferSizeNotMultipleOfIncr =
    DAQmxErrorIntermediateBufferSizeNotMultipleOfIncr,
  #[error("Event pulse width is outside of the legal range. Change the value of the pulse width, and/or verify that the units are correct.")]
  ErrorEventPulseWidthOutOfRange = DAQmxErrorEventPulseWidthOutOfRange,
  #[error("Event delay is outside of the legal range. Change the value of the delay, and/or verify that the units are correct.")]
  ErrorEventDelayOutOfRange = DAQmxErrorEventDelayOutOfRange,
  #[error("Requested number of samples per channel is invalid. The number of samples per channel must be an integer multiple of the number of samples per channel increment.")]
  ErrorSampPerChanNotMultipleOfIncr = DAQmxErrorSampPerChanNotMultipleOfIncr,
  #[error("Driver cannot determine the number of samples to read for a continuous task that has not yet started. Start the task explicitly, or specify the number of samples to read in DAQmx Read.")]
  ErrorCannotCalculateNumSampsTaskNotStarted =
    DAQmxErrorCannotCalculateNumSampsTaskNotStarted,
  #[error("Script is not in the device memory. Make sure you are referring to a previously written script by its correct name. Also, make sure the script has not been deleted.")]
  ErrorScriptNotInMem = DAQmxErrorScriptNotInMem,
  #[error("Generation was configured to use only onboard memory, but the corresponding buffer is larger than onboard memory. Buffer size is provided implicitly when data is written or explicitly when the buffer is configured. Configure the generation so that the Use Only Onboard Memory property is false. Alternatively, you can make sure the number of samples written and/or the size of the configured buffer do not exceed the onboard memory size.")]
  ErrorOnboardMemTooSmall = DAQmxErrorOnboardMemTooSmall,
  #[error("By setting Number of Samples per Channel to -1, you indicated that all available data should be read. This is not valid for acquisitions without a buffer. Specify a value greater than or equal to zero for Number of Samples per Channel. Do not specify a value of zero for Buffer Size when configuring the input buffer.")]
  ErrorReadAllAvailableDataWithoutBuffer =
    DAQmxErrorReadAllAvailableDataWithoutBuffer,
  #[error("Pulse width measurement was started while the input signal was active, and no additional pulses were received, which caused the measurement not to complete during the specified timeout. When measuring a single pulse width, make sure the measurement counter is started before the pulse to be measured is active, or provide a timeout sufficient for at least one additional pulse to be measured.")]
  ErrorPulseActiveAtStart = DAQmxErrorPulseActiveAtStart,
  #[error("An attempt has been made to read the calibration temperature for a device without an internal temperature sensor.")]
  ErrorCalTempNotSupported = DAQmxErrorCalTempNotSupported,
  #[error("Delay from the sample clock is longer than the longest delay that can be generated using the onboard clock with a timebase suitable for generating the convert clock. For longer delays, use a slower convert clock timebase rate, if applicable.")]
  ErrorDelayFromSampClkTooLong = DAQmxErrorDelayFromSampClkTooLong,
  #[error("Delay from the sample clock is shorter than the shortest delay that can be generated using the onboard clock with a timebase suitable for generating the convert clock. For shorter delays, use a faster convert clock timebase rate, if applicable.")]
  ErrorDelayFromSampClkTooShort = DAQmxErrorDelayFromSampClkTooShort,
  #[error("Specified AI convert rate is higher than the fastest rate possible with the current timebase.")]
  ErrorAIConvRateTooHigh = DAQmxErrorAIConvRateTooHigh,
  #[error("Delay from start trigger is longer than the longest delay that can be generated using the onboard clock with a timebase suitable for generating the sample clock. For longer delays, use a slower sample clock timebase rate, if applicable.")]
  ErrorDelayFromStartTrigTooLong = DAQmxErrorDelayFromStartTrigTooLong,
  #[error("Delay from the start trigger is shorter than the shortest delay that can be generated using the onboard clock with a timebase suitable for generating the sample clock. For shorter delays, use a sample clock timebase with a higher rate, if applicable.")]
  ErrorDelayFromStartTrigTooShort = DAQmxErrorDelayFromStartTrigTooShort,
  #[error("Specified sample rate is higher than the fastest rate supported by the device.")]
  ErrorSampRateTooHigh = DAQmxErrorSampRateTooHigh,
  #[error("Specified sample rate is lower than the lowest rate that can be generated using the onboard clock. The rate has been coerced to the slowest possible sample rate. For slower rates, use an external sample clock or an external sample clock timebase.")]
  ErrorSampRateTooLow = DAQmxErrorSampRateTooLow,
  #[error("An attempt has been made to use the PFI0 terminal of the device for both an analog and digital source. Use a terminal other than PFI0 as the source of your digital signal.")]
  ErrorPFI0UsedForAnalogAndDigitalSrc =
    DAQmxErrorPFI0UsedForAnalogAndDigitalSrc,
  #[error("An error has occurred while attempting to configure the device for an analog input acquisition. If an external master timebase is being used, make sure the source is connected and generating an appropriate clock. Otherwise, contact National Instruments Technical Support.")]
  ErrorPrimingCfgFIFO = DAQmxErrorPrimingCfgFIFO,
  #[error("Switch driver cannot open the topology configuration file for the switch device. A switch device cannot function without its configuration file. The configuration file is installed with the driver. The file might have been removed, renamed, or corrupted after installation. Make sure the configuration file is available to the driver at the expected location, or reinstall the product, as that will reinstall the configuration file.")]
  ErrorCannotOpenTopologyCfgFile = DAQmxErrorCannotOpenTopologyCfgFile,
  #[error("You have specified an invalid value for dt in the waveform cluster. The value for dt must be greater than zero.")]
  ErrorInvalidDTInsideWfmDataType = DAQmxErrorInvalidDTInsideWfmDataType,
  #[error("An attempt has been made to perform a route when the source and the destination are the same terminal. In many cases, such as when configuring an external clock or a counter source, you must select a PFI, PXI Trigger, or RTSI line as the source terminal.")]
  ErrorRouteSrcAndDestSame = DAQmxErrorRouteSrcAndDestSame,
  #[error(
        "Reverse coefficients must be specified to scale your data using the polynomial scale."
    )]
  ErrorReversePolynomialCoefNotSpecd = DAQmxErrorReversePolynomialCoefNotSpecd,
  #[error("NI-DAQmx is unable to communicate with the device. Make sure the device is present in and accessible to the system, and is not currently being reset.")]
  ErrorDevAbsentOrUnavailable = DAQmxErrorDevAbsentOrUnavailable,
  #[error("Cannot perform a multidevice scan with Advance Trigger Type set to None. Without the advance trigger, the devices in the scan list cannot be synchronized.")]
  ErrorNoAdvTrigForMultiDevScan = DAQmxErrorNoAdvTrigForMultiDevScan,
  #[error("Data transfer has been stopped to prevent the computer from becoming completely unresponsive. Could not transfer enough data to satisfy the data transfer requirements with Interrupts as the Data Transfer Mechanism. Reduce your Sample Clock Rate, use DMA as your Data Transfer Mechanism, or use a different Data Transfer Request Condition.")]
  ErrorInterruptsInsufficientDataXferMech =
    DAQmxErrorInterruptsInsufficientDataXferMech,
  #[error("Attenuation Value conflicts with the specified AI Minimum and AI Maximum properties. The specified attenuation and AI Minimum and/or AI Maximum would cause the device to exceed the hardware limit. You should increase the Attenuation Value or adjust the AI Minimum and/or AI Maximum.")]
  ErrorInvalidAttentuationBasedOnMinMax =
    DAQmxErrorInvalidAttentuationBasedOnMinMax,
  #[error("Your SCXI system is not set up for analog input with simultaneous sample and hold on the given channels. The SCXI module cabled to your digitizer cannot route the signal needed for simultaneous sample and hold from the digitizer to the other modules. To perform the desired operation with multiple SCXI modules and one digitizer, cable the digitizer to one of the modules that can route the signal needed for simultaneous sample and hold. The simultaneous sample and hold module in your chassis is one such module. After cabling the digitizer to the module, update the chassis configuration in Measurement & Automation Explorer to reflect the cabling change. For detailed information about cabling, refer to the documentation.")]
  ErrorCabledModuleCannotRouteSSH = DAQmxErrorCabledModuleCannotRouteSSH,
  #[error("Your SCXI system is not set up to perform the analog input operation on given channels. The SCXI module cabled to your digitizer cannot route AI Convert Clock from the digitizer to the other modules. To perform the desired operation with multiple SCXI modules and one digitizer, cable the digitizer to one of the modules that can route the AI Convert signal, such as the module your channels are on. After cabling the digitizer to the module, update the chassis configuration in Measurement & Automation Explorer to reflect the cabling change. For detailed information about cabling, refer to the documentation.")]
  ErrorCabledModuleCannotRouteConvClk =
    DAQmxErrorCabledModuleCannotRouteConvClk,
  #[error("Invalid excitation value specified to be used for scaling with full bridge configuration. Change the excitation value if you want it to be used for scaling with full bridge configuration. Alternatively, change the bridge configuration, or do not use excitation value for scaling.")]
  ErrorInvalidExcitValForScaling = DAQmxErrorInvalidExcitValForScaling,
  #[error("There is not enough free device memory for your script. Delete waveforms or scripts not in use to free memory. If you have deleted multiple waveforms or scripts, the memory might have become fragmented. To avoid fragmentation, you can change the order in which you write/delete your waveforms and scripts.")]
  ErrorNoDevMemForScript = DAQmxErrorNoDevMemForScript,
  #[error("Device data underflow. The device was not able to move data fast enough to keep up with the sample rate for the active script. Run the operation at a lower sample rate, or look for the following in the active script: markers might be too close together, waveforms might be too small, waits might be too short, or subsets might be too small. If you are using an external clock, the provided clock might have gone away during your generation.")]
  ErrorScriptDataUnderflow = DAQmxErrorScriptDataUnderflow,
  #[error("There is not enough free device memory for your waveform. Delete waveforms or scripts not in use to free memory. If you have deleted multiple waveforms or scripts, the memory might have become fragmented. To avoid fragmentation, you can change the order in which you write/delete your waveforms and scripts.")]
  ErrorNoDevMemForWaveform = DAQmxErrorNoDevMemForWaveform,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorStreamDCMBecameUnlocked = DAQmxErrorStreamDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorStreamDCMLock = DAQmxErrorStreamDCMLock,
  #[error("Waveform is not in the device memory. Make sure you are referring to a previously allocated and/or written waveform by its correct name. Also, make sure that the waveform was not deleted.")]
  ErrorWaveformNotInMem = DAQmxErrorWaveformNotInMem,
  #[error("You cannot write data outside the boundaries of your waveform. Make sure you are not trying to write more data than your waveform can accomodate and that your write location and write offset are set correctly.")]
  ErrorWaveformWriteOutOfBounds = DAQmxErrorWaveformWriteOutOfBounds,
  #[error("The waveform you are trying to allocate on the device has been previously allocated. Make sure you are not trying to allocate the same waveform twice, or delete the existing waveform before allocating it again.")]
  ErrorWaveformPreviouslyAllocated = DAQmxErrorWaveformPreviouslyAllocated,
  #[error("Specified master timebase divisor (belonging to sample clock timebase) is not appropriate for the specified sample timebase source. Do not set the master timebase divisor when you are using an internal sample timebase source. In this case, the driver sets the master timebase divisor for you.")]
  ErrorSampClkTbMasterTbDivNotAppropriateForSampTbSrc =
    DAQmxErrorSampClkTbMasterTbDivNotAppropriateForSampTbSrc,
  #[error("Specified sample timebase rate does not match specified sample timebase source. Do not set the sample timebase rate when you are using an internal sample timebase source. In this case, the driver sets the sample timebase rate for you.")]
  ErrorSampTbRateSampTbSrcMismatch = DAQmxErrorSampTbRateSampTbSrcMismatch,
  #[error("Specified master timebase rate does not match specified master timebase source. Do not set the master timebase rate when you are using an internal master timebase source. In this case, the driver sets the master timebase rate for you.")]
  ErrorMasterTbRateMasterTbSrcMismatch =
    DAQmxErrorMasterTbRateMasterTbSrcMismatch,
  #[error("An attempt was made to set the Samples per Channel property to a value greater than the maximum supported number.")]
  ErrorSampsPerChanTooBig = DAQmxErrorSampsPerChanTooBig,
  #[error("Desired finite pulse train generation is not possible. Change the number of samples to be generated, increase the rate of the pulse train, or choose a different timebase source. Refer to the documentation for more details.")]
  ErrorFinitePulseTrainNotPossible = DAQmxErrorFinitePulseTrainNotPossible,
  #[error("External master timebase rate must be specified for this channel given the selected measurement units. Specify the master timebase rate, or use ticks as the measurements units.")]
  ErrorExtMasterTimebaseRateNotSpecified =
    DAQmxErrorExtMasterTimebaseRateNotSpecified,
  #[error(
    "External sample clock source must be specified for this application."
  )]
  ErrorExtSampClkSrcNotSpecified = DAQmxErrorExtSampClkSrcNotSpecified,
  #[error("Signal being measured is slower than the specified measurement time. Increase the measurement time, or use a different measurement method.")]
  ErrorInputSignalSlowerThanMeasTime = DAQmxErrorInputSignalSlowerThanMeasTime,
  #[error("Cannot update the Pulse Generation property. The pulse generation with previous property settings must complete a full cycle before the property can be updated.")]
  ErrorCannotUpdatePulseGenProperty = DAQmxErrorCannotUpdatePulseGenProperty,
  #[error("Invalid timing type for this channel.")]
  ErrorInvalidTimingType = DAQmxErrorInvalidTimingType,
  #[error("This property is unavailable when using onboard memory.")]
  ErrorPropertyUnavailWhenUsingOnboardMemory =
    DAQmxErrorPropertyUnavailWhenUsingOnboardMemory,
  #[error("Attempt was made to write samples after start of generation where only onboard memory was used.  In this case, all samples must be written to the device before the start of generation. No samples may be updated once the generation has started. If you wish to modify samples in the generation after the start of the generation, do not enable the onboard memory.")]
  ErrorCannotWriteAfterStartWithOnboardMemory =
    DAQmxErrorCannotWriteAfterStartWithOnboardMemory,
  #[error("Not enough samples were written to satisfy the initial data transfer request condition. To successfully start a generation, increase the number of samples initially written to the buffer before starting.  Alternatively, decrease the number of samples required to start by changing the data transfer request condition.")]
  ErrorNotEnoughSampsWrittenForInitialXferRqstCondition =
    DAQmxErrorNotEnoughSampsWrittenForInitialXferRqstCondition,
  #[error("The generation is not yet started, and not enough space is available in the buffer. Configure a larger buffer, or start the generation before writing more data than will fit in the buffer.")]
  ErrorNoMoreSpace = DAQmxErrorNoMoreSpace,
  #[error("Some or all of the samples to write could not be written to the buffer yet. More space will free up as samples currently in the buffer are generated. To wait for more space to become available, use a longer write timeout. To make the space available sooner, increase the sample rate.")]
  ErrorSamplesCanNotYetBeWritten = DAQmxErrorSamplesCanNotYetBeWritten,
  #[error("The generation has stopped because an intermediate buffer overflowed. The background was running too fast for the application to keep up, and the application was unable to write samples to the intermediate buffer fast enough to prevent regenerating old samples. To avoid this error, you can reduce the sample rate, reduce the number of applications your computer is executing concurrently, or write all samples before the generation starts.")]
  ErrorGenStoppedToPreventIntermediateBufferRegenOfOldSamples =
    DAQmxErrorGenStoppedToPreventIntermediateBufferRegenOfOldSamples,
  #[error("The generation has stopped to prevent the regeneration of old samples. Your application was unable to write samples to the background buffer fast enough to prevent old samples from being regenerated. To avoid this error, you can do any of the following: 1. Increase the size of the background buffer by configuring the buffer. 2. Increase the number of samples you write each time you invoke a write operation. 3. Write samples more often. 4. Reduce the sample rate. 5. If your data transfer method is interrupts, try using DMA or USB Bulk. 6. Reduce the number of applications your computer is executing concurrently. In addition, if you do not need to write every sample that is generated, you can configure the regeneration mode to allow regeneration, and then use the Position and Offset attributes to write the desired samples.")]
  ErrorGenStoppedToPreventRegenOfOldSamples =
    DAQmxErrorGenStoppedToPreventRegenOfOldSamples,
  #[error("Attempted to write samples that have already been generated or have already been sent to the device for generation. Increasing the buffer size or writing the data more frequently might correct the problem.")]
  ErrorSamplesNoLongerWriteable = DAQmxErrorSamplesNoLongerWriteable,
  #[error("Attempted to write a sample beyond the final sample generated. The generation has stopped, therefore the sample specified by the combination of position and offset will never be available. Specify a position and offset which selects a sample up to, but not beyond, the final sample generated. The final sample generated can be determined by querying the total samples generated after a generation has stopped.")]
  ErrorSamplesWillNeverBeGenerated = DAQmxErrorSamplesWillNeverBeGenerated,
  #[error("Attempted to write to an invalid combination of position and offset. The position and offset specified a sample prior to the first sample generated (sample 0). Make sure any negative write offset specified will select a valid sample when combined with the write position.")]
  ErrorNegativeWriteSampleNumber = DAQmxErrorNegativeWriteSampleNumber,
  #[error("No data is available to read, because no acquisition has been started. Start the acquisition before attempting to read data, either explicitly or by enabling auto start and stop.")]
  ErrorNoAcqStarted = DAQmxErrorNoAcqStarted,
  #[error("Some or all of the samples requested have not yet been acquired. To wait for the samples to become available use a longer read timeout or read later in your program. To make the samples available sooner, increase the sample rate. If your task uses a start trigger,  make sure that your start trigger is configured correctly. It is also possible that you configured the task for external timing, and no clock was supplied. If this is the case, supply an external clock.")]
  ErrorSamplesNotYetAvailable = DAQmxErrorSamplesNotYetAvailable,
  #[error("Acquisition has stopped to prevent the intermediate buffer from overflowing. The background was running too fast for the application to keep up, and the application was unable to read samples from the intermediate buffer fast enough to prevent losing samples. To avoid this error, you might reduce the sample rate, reduce the number of applications your computer is executing concurrently, or not read any samples until the acquisition is complete.")]
  ErrorAcqStoppedToPreventIntermediateBufferOverflow =
    DAQmxErrorAcqStoppedToPreventIntermediateBufferOverflow,
  #[error("Reading relative to the reference trigger or relative to the start of a pretrigger sample is not supported with the current task configuration. If you have not configured a reference trigger or if one of your devices is utilizing an onboard buffer to transfer data after an acquisition has completed, reading relative to reference trigger or relative to the first pretrigger sample is not supported. Configure a reference trigger, or configure read position differently.")]
  ErrorNoRefTrigConfigured = DAQmxErrorNoRefTrigConfigured,
  #[error("Reading relative to the reference trigger or relative to the start of pretrigger samples position before the acquisition is complete. Wait for the acquisition to complete before reading, or increase your read timeout.  Also, make sure the hardware is set up and wired correctly, the signal for the reference trigger is correct, and that the reference trigger occurs while the device is acquiring data.")]
  ErrorCannotReadRelativeToRefTrigUntilDone =
    DAQmxErrorCannotReadRelativeToRefTrigUntilDone,
  #[error("The application is not able to keep up with the hardware acquisition. Increasing the buffer size, reading the data more frequently, or specifying a fixed number of samples to read instead of reading all available samples might correct the problem.")]
  ErrorSamplesNoLongerAvailable = DAQmxErrorSamplesNoLongerAvailable,
  #[error("Attempted to read a sample beyond the final sample acquired. The acquisition has stopped, therefore the sample specified by the combination of position and offset will never be available. Specify a position and offset which selects a sample up to, but not beyond, the final sample acquired. The final sample acquired can be determined by querying the total samples acquired after an acquisition has stopped.")]
  ErrorSamplesWillNeverBeAvailable = DAQmxErrorSamplesWillNeverBeAvailable,
  #[error("Invalid combination of position and offset. The position and offset specified a sample prior to the first sample acquired (sample 0). Make sure any negative read offset specified will select a valid sample when combined with the read position.")]
  ErrorNegativeReadSampleNumber = DAQmxErrorNegativeReadSampleNumber,
  #[error("Reference clock source and sample clock source cannot be the same. Use different terminals to bring in your reference clock and sample clock, or use only one of them at a time.")]
  ErrorExternalSampClkAndRefClkThruSameTerm =
    DAQmxErrorExternalSampClkAndRefClkThruSameTerm,
  #[error("Sample rate desired is too low for an external clock being brought in through the ClkIn connector. Change the sample rate so it is within limits, or use DDC_ClkIn to bring in your sample clock.")]
  ErrorExtSampClkRateTooLowForClkIn = DAQmxErrorExtSampClkRateTooLowForClkIn,
  #[error("Sample clock rate desired is too high for an external clock being brought in through the backplane. Bring in your external sample clock through one of the higher-frequency front panel connectors, or use a lower sample rate.")]
  ErrorExtSampClkRateTooHighForBackplane =
    DAQmxErrorExtSampClkRateTooHighForBackplane,
  #[error("Sample clock rate and the sample clock divisor values are inconsistent with one another. Consider settting either the sample clock rate or the sample clock divisor, but not both. This allows the driver to automatically select an appropriate value for the other property. Alternatively, make sure the sample clock rate and sample clock divisor satisfy the following constraint: rate = timebase / divisor")]
  ErrorSampClkRateAndDivCombo = DAQmxErrorSampClkRateAndDivCombo,
  #[error("Sample clock rate requested is too low for the selected divide-down clock. Use the high resolution clock, or increase your sample rate.")]
  ErrorSampClkRateTooLowForDivDown = DAQmxErrorSampClkRateTooLowForDivDown,
  #[error("Product of AO Channel properties Minimum Value and Gain exceeds the minimum voltage for the device.")]
  ErrorProductOfAOMinAndGainTooSmall = DAQmxErrorProductOfAOMinAndGainTooSmall,
  #[error(
    "Interpolation rate specified is not possible for the given sample rate."
  )]
  ErrorInterpolationRateNotPossible = DAQmxErrorInterpolationRateNotPossible,
  #[error("Specified Offset is too large for the given AO Gain and Maximum Value. The following constraint must hold: Offset < (Gain * Maximum Value / 2)")]
  ErrorOffsetTooLarge = DAQmxErrorOffsetTooLarge,
  #[error("Specified Offset is too small given AO Gain and Minimum Value. The following constraint must hold: Offset > (Gain * Minimum Value / 2)")]
  ErrorOffsetTooSmall = DAQmxErrorOffsetTooSmall,
  #[error("Product of AO channel properties Maximum Value and Gain exceeds the maximum voltage for the device.")]
  ErrorProductOfAOMaxAndGainTooLarge = DAQmxErrorProductOfAOMaxAndGainTooLarge,
  #[error("Minimum and maximum values for the channel are not symmetric.")]
  ErrorMinAndMaxNotSymmetric = DAQmxErrorMinAndMaxNotSymmetric,
  #[error("An attempt has been made to use an invalid analog trigger source. Ensure that the trigger source you specify matches the name of the virtual channel in the task or matches the name of a non-scannable terminal that the device can use as an analog trigger source.")]
  ErrorInvalidAnalogTrigSrc = DAQmxErrorInvalidAnalogTrigSrc,
  #[error("Device supports an analog channel as the source of an analog reference trigger only when it is the only channel in the task. Remove all of the channels currently in the task except the channel that will be used as the analog trigger source, or change the analog trigger source to a terminal.")]
  ErrorTooManyChansForAnalogRefTrig = DAQmxErrorTooManyChansForAnalogRefTrig,
  #[error("Device supports an analog channel as the source of an analog pause trigger only when it is the only channel in the task. Remove all of the channels currently in the task except the channel that will be used as the analog trigger source, or change the analog trigger source to a terminal.")]
  ErrorTooManyChansForAnalogPauseTrig =
    DAQmxErrorTooManyChansForAnalogPauseTrig,
  #[error("An attempt has been made to configure a trigger without configuring the appropriate sample clock properties or when Sample Timing Type was set to On Demand. Configure the sample clock type to something other than On Demand to use a trigger.")]
  ErrorTrigWhenOnDemandSampTiming = DAQmxErrorTrigWhenOnDemandSampTiming,
  #[error("An attempt has been made to use an analog trigger in multiple situations with differing properties. Change the analog trigger properties so they are the same, or do not use an analog trigger for all situations.")]
  ErrorInconsistentAnalogTrigSettings =
    DAQmxErrorInconsistentAnalogTrigSettings,
  #[error("Memory mapping has been enabled, and the sample clock has been configured; but the buffer size has not been set, and the data transfer mechanism has either not been set or was set to something other than Programmed I/O. Set the buffer size to 0, and/or change the data transfer mechanism to Programmed I/O.")]
  ErrorMemMapDataXferModeSampTimingCombo =
    DAQmxErrorMemMapDataXferModeSampTimingCombo,
  #[error("Value selected for this jumper-controlled property must match the value specified in Measurement & Automation Explorer. Make sure the value specified in Measurement & Automation Explorer matches the value in your program and that the value corresponds to the selection made using the jumper on the device.")]
  ErrorInvalidJumperedAttr = DAQmxErrorInvalidJumperedAttr,
  #[error("Gain value conflicts with specified AI Minimum and AI Maximum properties. The specified gain and AI Minimum and/or AI Maximum would cause the device to exceed the hardware limit. Lower the gain, or adjust AI Minimum and/or AI Maximum.")]
  ErrorInvalidGainBasedOnMinMax = DAQmxErrorInvalidGainBasedOnMinMax,
  #[error("Excitation property must be the same for related physical channels. Refer to the documentation for information about setting excitation across related physical channels.")]
  ErrorInconsistentExcit = DAQmxErrorInconsistentExcit,
  #[error("Specified topology cannot be used to reset the switch, because that topology is not supported by the connected terminal block. Refer to the documentation for supported topologies for the given terminal block, or disconnect the terminal block from the switch.")]
  ErrorTopologyNotSupportedByCfgTermBlock =
    DAQmxErrorTopologyNotSupportedByCfgTermBlock,
  #[error("Built-in temperature sensor is not supported on this channel. This channel is not configured to support a built-in temperature sensor. Make sure the accessory specified in the hardware configuration is correct and that the hardware supports a built-in temperature sensor on this channel.")]
  ErrorBuiltInTempSensorNotSupported = DAQmxErrorBuiltInTempSensorNotSupported,
  #[error("Terminal for the device is invalid.")]
  ErrorInvalidTerm = DAQmxErrorInvalidTerm,
  #[error("Terminal could not be tristated because the hardware cannot tristate this terminal.")]
  ErrorCannotTristateTerm = DAQmxErrorCannotTristateTerm,
  #[error("Terminal cannot be tristated because it is busy. Disconnect any routes spanning this terminal, or stop any tasks using this terminal.")]
  ErrorCannotTristateBusyTerm = DAQmxErrorCannotTristateBusyTerm,
  #[error("No DMA channels or USB Bulk Endpoints are available. Either shut down other tasks that might be using these resources or consider changing your data transfer mechanism to Interrupts if supported.")]
  ErrorNoDMAChansAvailable = DAQmxErrorNoDMAChansAvailable,
  #[error("Length of waveform is too small for the last \"generate\" statement in a \"repeat until\" loop.")]
  ErrorInvalidWaveformLengthWithinLoopInScript =
    DAQmxErrorInvalidWaveformLengthWithinLoopInScript,
  #[error("Length of waveform subset is too small for the last \"generate\" statement in a \"repeat until\" loop.")]
  ErrorInvalidSubsetLengthWithinLoopInScript =
    DAQmxErrorInvalidSubsetLengthWithinLoopInScript,
  #[error("Specified marker position is too close to the end of the last \"generate\" statement in a \"repeat until\" loop. Move the marker position farther away from the end of the last \"generate\" statement in the \"repeat until\" loop.")]
  ErrorMarkerPosInvalidForLoopInScript =
    DAQmxErrorMarkerPosInvalidForLoopInScript,
  #[error("Integer was expected but not found in the script. Insert an appropriate integer at this location in the script.")]
  ErrorIntegerExpectedInScript = DAQmxErrorIntegerExpectedInScript,
  #[error("PLL has lost phase-lock to the external reference clock. Make sure your reference clock is connected and that it is within the jitter and voltage level specifications at all times. Also, make sure the reference clock rate is correctly specified at all times.")]
  ErrorPLLBecameUnlocked = DAQmxErrorPLLBecameUnlocked,
  #[error("PLL could not phase-lock to the external reference clock. Make sure your reference clock is connected and that it is within the jitter and voltage specifications. Also, make sure the reference clock rate is correctly specified.")]
  ErrorPLLLock = DAQmxErrorPLLLock,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDDCClkOutDCMBecameUnlocked = DAQmxErrorDDCClkOutDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorDDCClkOutDCMLock = DAQmxErrorDDCClkOutDCMLock,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorClkDoublerDCMBecameUnlocked = DAQmxErrorClkDoublerDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorClkDoublerDCMLock = DAQmxErrorClkDoublerDCMLock,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorSampClkDCMBecameUnlocked = DAQmxErrorSampClkDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external sample clock or an external reference clock, make sure it is connected and within the jitter and voltage level specifications. Also, verify that the rate of the external clock matches the specified clock rate. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorSampClkDCMLock = DAQmxErrorSampClkDCMLock,
  #[error("Hardware clocking error occurred. If you are using an external reference clock, make sure it is connected and within the jitter and voltage level specifications at all times, and its rate is correctly specified. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorSampClkTimebaseDCMBecameUnlocked =
    DAQmxErrorSampClkTimebaseDCMBecameUnlocked,
  #[error("Hardware clocking error occurred. If you are using an external reference clock, make sure it is connected and within the jitter and voltage level specifications, and its rate is correctly specified. If you are generating your clock internally, please contact National Instruments Technical Support.")]
  ErrorSampClkTimebaseDCMLock = DAQmxErrorSampClkTimebaseDCMLock,
  #[error("Property requested cannot be reset.")]
  ErrorAttrCannotBeReset = DAQmxErrorAttrCannotBeReset,
  #[error("Explanation could not be found for the requested status code. Verify that the requested status code is correct.")]
  ErrorExplanationNotFound = DAQmxErrorExplanationNotFound,
  #[error("Buffer is too small for requested samples to be written.")]
  ErrorWriteBufferTooSmall = DAQmxErrorWriteBufferTooSmall,
  #[error("Property specified is not valid for this function.")]
  ErrorSpecifiedAttrNotValid = DAQmxErrorSpecifiedAttrNotValid,
  #[error("Property requested cannot be read.")]
  ErrorAttrCannotBeRead = DAQmxErrorAttrCannotBeRead,
  #[error("Property requested cannot be set.")]
  ErrorAttrCannotBeSet = DAQmxErrorAttrCannotBeSet,
  #[error("NULL pointer was passed for a required parameter.")]
  ErrorNULLPtrForC_Api = DAQmxErrorNULLPtrForC_Api,
  #[error("Buffer is too small to fit read data.")]
  ErrorReadBufferTooSmall = DAQmxErrorReadBufferTooSmall,
  #[error("Buffer is too small to fit the string.")]
  ErrorBufferTooSmallForString = DAQmxErrorBufferTooSmallForString,
  #[error("Device does not have any free trigger lines for the device driver to choose. Although there might be trigger lines available on the respective trigger bus, the device cannot use the trigger bus because the device does not have enough free resources to do so. To free up trigger lines, you can do any of the following: 1. Stop other tasks that are connected to this device. 2. Use DAQmx Disconnect Route to stop any immediate routes that span this trigger bus and device.")]
  ErrorNoAvailTrigLinesOnDevice = DAQmxErrorNoAvailTrigLinesOnDevice,
  #[error("Trigger bus to which the device is connected does not have any free trigger lines for the driver to choose. To free up trigger lines, you can do any of the following: 1. Stop other tasks that are connected to the same trigger bus as this device. 2. Use DAQmx Disconnect Route to stop any immediate routes that span this trigger bus. 3. Make more trigger lines on this trigger bus available to the driver.")]
  ErrorTrigBusLineNotAvail = DAQmxErrorTrigBusLineNotAvail,
  #[error("Trigger line requested could not be reserved because it is already in use.")]
  ErrorCouldNotReserveRequestedTrigLine =
    DAQmxErrorCouldNotReserveRequestedTrigLine,
  #[error("No registered trigger lines could be found between the devices in the route. If you have a PXI chassis, identify the chassis correctly in MAX, and make sure it has been configured properly. If you are using PCI devices, make sure they are connected with a RTSI cable and that the RTSI cable is registered in MAX. Otherwise, make sure there is an available trigger line on the trigger bus shared between the devices.")]
  ErrorTrigLineNotFound = DAQmxErrorTrigLineNotFound,
  #[error("Specified threshold and hysteresis values for this channel create a triggering range that is not supported by your device. On the SCXI-1126, threshold minus hysteresis must be between -0.5 and 4.48.")]
  ErrorSCXI1126ThreshHystCombination = DAQmxErrorSCXI1126ThreshHystCombination,
  #[error("Acquisition has been stopped to prevent an input buffer overwrite. Your application was unable to read samples from the buffer fast enough to prevent new samples from overwriting unread data. To avoid this error, you can do any of the following: 1. Increase the size of the buffer. 2. Increase the number of samples you read each time you invoke a read operation. 3. Read samples more often. 4. Reduce the sample rate. 5. If your data transfer method is interrupts, try using DMA or USB Bulk. 6. Reduce the number of applications your computer is running concurrently. In addition, if you do not need to read every sample that is acquired, you can configure the overwrite mode to overwrite unread data, and then use the Relative To and Offset properties to read the desired samples.")]
  ErrorAcqStoppedToPreventInputBufferOverwrite =
    DAQmxErrorAcqStoppedToPreventInputBufferOverwrite,
  #[error("Amount of time allocated to perform this operation was exceeded.")]
  ErrorTimeoutExceeded = DAQmxErrorTimeoutExceeded,
  #[error("Device identifier is invalid.")]
  ErrorInvalidDeviceID = DAQmxErrorInvalidDeviceID,
  #[error("Analog output virtual channels cannot be created out of order with respect to their physical channel numbers for the type of device you are using. For example, a virtual channel using physical channel ao0 must be created before a virtual channel with physical channel ao1.")]
  ErrorInvalidAOChanOrder = DAQmxErrorInvalidAOChanOrder,
  #[error("Data Transfer Mechanism must be Programmed I/O when not using hardware timing. Set Data Transfer Mechanism to Programmed I/O, configure your sample clock timing, or set Sample Timing Type to Sample Clock.")]
  ErrorSampleTimingTypeAndDataXferMode =
    DAQmxErrorSampleTimingTypeAndDataXferMode,
  #[error("Buffered operations cannot use On Demand for Sample Timing Type. Set your buffer size to 0 for On Demand sample timing. Otherwise, configure your sample clock, or change your sample timing type for buffered operations.")]
  ErrorBufferWithOnDemandSampTiming = DAQmxErrorBufferWithOnDemandSampTiming,
  #[error("Buffered operations cannot use a Data Transfer Mechanism of Programmed I/O for this device and Channel Type. Non-buffered operations cannot use a Data Transfer Mechanism of Interrupts or DMA for this device and Channel Type.")]
  ErrorBufferAndDataXferMode = DAQmxErrorBufferAndDataXferMode,
  #[error("Memory Mapping is not supported for buffered operations. Turn Memory Mapping off, set Buffer Size to 0, or do not configure the buffer for the operation.")]
  ErrorMemMapAndBuffer = DAQmxErrorMemMapAndBuffer,
  #[error("Analog trigger circuitry unavailable on the device. Select a non-analog trigger type, or use a device with analog triggering hardware.")]
  ErrorNoAnalogTrigHW = DAQmxErrorNoAnalogTrigHW,
  #[error("Pretrigger Samples per Channel requested plus minimum number of posttrigger samples exceed the requested Number of Samples per Channel. Decrease the number of Pretrigger Samples per Channel, or increase Number of Samples per Channel.")]
  ErrorTooManyPretrigPlusMinPostTrigSamps =
    DAQmxErrorTooManyPretrigPlusMinPostTrigSamps,
  #[error("Measurement units specified for the channel are not valid for the Measurement Type of the channel.")]
  ErrorInconsistentUnitsSpecified = DAQmxErrorInconsistentUnitsSpecified,
  #[error("Multiple relay names were specified for a single relay operation.")]
  ErrorMultipleRelaysForSingleRelayOp =
    DAQmxErrorMultipleRelaysForSingleRelayOp,
  #[error("Multiple device identifiers from one chassis are not allowed in the terminal list.")]
  ErrorMultipleDevIDsPerChassisSpecifiedInList =
    DAQmxErrorMultipleDevIDsPerChassisSpecifiedInList,
  #[error("Duplicate device identifier found in the terminal list when trying to set the property. Only one instance of the device identifier is permitted.")]
  ErrorDuplicateDevIDInList = DAQmxErrorDuplicateDevIDInList,
  #[error(
    "Range statement in the list entry contains an invalid character sequence."
  )]
  ErrorInvalidRangeStatementCharInList =
    DAQmxErrorInvalidRangeStatementCharInList,
  #[error("Device identifier in the list entry is invalid.")]
  ErrorInvalidDeviceIDInList = DAQmxErrorInvalidDeviceIDInList,
  #[error("Advance trigger and Advance Complete event must use the same polarity in this particular switch device.")]
  ErrorTriggerPolarityConflict = DAQmxErrorTriggerPolarityConflict,
  #[error("Topology does not support scanning.")]
  ErrorCannotScanWithCurrentTopology = DAQmxErrorCannotScanWithCurrentTopology,
  #[error(
    "Unexpected identifier within the fully-specified path in the list."
  )]
  ErrorUnexpectedIdentifierInFullySpecifiedPathInList =
    DAQmxErrorUnexpectedIdentifierInFullySpecifiedPathInList,
  #[error("Switch hardware is incapable of driving multiple trigger lines simultaneously.")]
  ErrorSwitchCannotDriveMultipleTrigLines =
    DAQmxErrorSwitchCannotDriveMultipleTrigLines,
  #[error("Relay name is invalid.")]
  ErrorInvalidRelayName = DAQmxErrorInvalidRelayName,
  #[error(
    "Switch scan list is too large to fit in the onboard memory of the device."
  )]
  ErrorSwitchScanlistTooBig = DAQmxErrorSwitchScanlistTooBig,
  #[error(
    "Switch channel is already in exclusive use within another connection."
  )]
  ErrorSwitchChanInUse = DAQmxErrorSwitchChanInUse,
  #[error("Switch device must be reset before scanning. Reset the device by doing one of the following: 1. Call DAQmx Switch Set Topology And Reset. 2. Call DAQmx Device Reset. 3. Use Measurement & Automation Explorer.")]
  ErrorSwitchNotResetBeforeScan = DAQmxErrorSwitchNotResetBeforeScan,
  #[error("Topology specified is invalid. Make sure the spelling of the topology is correct and that the switch supports that topology.")]
  ErrorInvalidTopology = DAQmxErrorInvalidTopology,
  #[error("Device does not support this property.")]
  ErrorAttrNotSupported = DAQmxErrorAttrNotSupported,
  #[error("Action at the end of the scan list is not valid for this device.")]
  ErrorUnexpectedEndOfActionsInList = DAQmxErrorUnexpectedEndOfActionsInList,
  #[error("Switch configuration has caused the switch device to exceed its power limit because there were too many closed relays. The switch was disabled. Reset it by doing one of the following: 1. Call DAQmx Switch Set Topology And Reset. 2. Call DAQmx Device Reset. 3. Use Measurement & Automation Explorer.")]
  ErrorPowerLimitExceeded = DAQmxErrorPowerLimitExceeded,
  #[error("Hardware was unexpectedly powered off and back on. To recover, reset the device (either programmatically or by using Measurement & Automation Explorer).")]
  ErrorHWUnexpectedlyPoweredOffAndOn = DAQmxErrorHWUnexpectedlyPoweredOffAndOn,
  #[error("Switch device does not support this operation.")]
  ErrorSwitchOperationNotSupported = DAQmxErrorSwitchOperationNotSupported,
  #[error("Switch device supports continuous scanning only.")]
  ErrorOnlyContinuousScanSupported = DAQmxErrorOnlyContinuousScanSupported,
  #[error("Task was created with a topology different from the current topology. When scanning, you must use the original topology specified when the task was created.")]
  ErrorSwitchDifferentTopologyWhenScanning =
    DAQmxErrorSwitchDifferentTopologyWhenScanning,
  #[error("Disconnection path is not the same as the existing path. You can programmatically find out the existing path. Refer to your documentation for details.")]
  ErrorDisconnectPathNotSameAsExistingPath =
    DAQmxErrorDisconnectPathNotSameAsExistingPath,
  #[error(
        "Explicit connection cannot be made to a switch channel that is reserved for routing."
    )]
  ErrorConnectionNotPermittedOnChanReservedForRouting =
    DAQmxErrorConnectionNotPermittedOnChanReservedForRouting,
  #[error("Connection cannot be made between the specified channels because they are connected to different source channels.")]
  ErrorCannotConnectSrcChans = DAQmxErrorCannotConnectSrcChans,
  #[error("Channel cannot be connected to itself.")]
  ErrorCannotConnectChannelToItself = DAQmxErrorCannotConnectChannelToItself,
  #[error(
        "Channels used to make the connection between two endpoints must be reserved for routing."
    )]
  ErrorChannelNotReservedForRouting = DAQmxErrorChannelNotReservedForRouting,
  #[error(
    "Path contains a leg with two channels that cannot be directly connected."
  )]
  ErrorCannotConnectChansDirectly = DAQmxErrorCannotConnectChansDirectly,
  #[error("Leg in path cannot contain two channels that are already directly connected.")]
  ErrorChansAlreadyConnected = DAQmxErrorChansAlreadyConnected,
  #[error("Switch channel names cannot be duplicated in the path string.")]
  ErrorChanDuplicatedInPath = DAQmxErrorChanDuplicatedInPath,
  #[error(
        "Switch channels cannot be disconnected because there is no explicit path between them."
    )]
  ErrorNoPathToDisconnect = DAQmxErrorNoPathToDisconnect,
  #[error("Channel name specified is not valid for the switch device.")]
  ErrorInvalidSwitchChan = DAQmxErrorInvalidSwitchChan,
  #[error("Path between two switch channels is not available.")]
  ErrorNoPathAvailableBetween2SwitchChans =
    DAQmxErrorNoPathAvailableBetween2SwitchChans,
  #[error("Explicit connection between the channels already exists. You can make only one connection between these channels.")]
  ErrorExplicitConnectionExists = DAQmxErrorExplicitConnectionExists,
  #[error("Task was created with a settling time different from the current settling time. When scanning, you must use the original settling time specified when the task was created.")]
  ErrorSwitchDifferentSettlingTimeWhenScanning =
    DAQmxErrorSwitchDifferentSettlingTimeWhenScanning,
  #[error("Operation is permitted only while the switch device is scanning.")]
  ErrorOperationOnlyPermittedWhileScanning =
    DAQmxErrorOperationOnlyPermittedWhileScanning,
  #[error("Operation is not permitted while the switch device is scanning.")]
  ErrorOperationNotPermittedWhileScanning =
    DAQmxErrorOperationNotPermittedWhileScanning,
  #[error("Hardware is not responding. Ensure your hardware is powered on and all cables are properly connected.")]
  ErrorHardwareNotResponding = DAQmxErrorHardwareNotResponding,
  #[error("The combination of Sample Timebase Rate and Master Timebase Rate you specified is invalid. The driver computed the Sample Timebase Source Divisor by dividing the Master Timebase Rate by the Sample Timebase Rate. The resulting value for the Sample Timebase Source Divisor is not supported by your device. Refer to the documentation for more information about these three properties.")]
  ErrorInvalidSampAndMasterTimebaseRateCombo =
    DAQmxErrorInvalidSampAndMasterTimebaseRateCombo,
  #[error("Buffer size must be zero when Data Transfer Mechanism is Programmed IO. Set buffer size to zero or Data Transfer Mechanism to something other than Programmed IO.")]
  ErrorNonZeroBufferSizeInProgIOXfer = DAQmxErrorNonZeroBufferSizeInProgIOXfer,
  #[error(
        "Virtual channel cannot be created. Another virtual channel with this name already exists."
    )]
  ErrorVirtualChanNameUsed = DAQmxErrorVirtualChanNameUsed,
  #[error("Physical channel specified does not exist on this device. Refer to the documentation for channels available on this device.")]
  ErrorPhysicalChanDoesNotExist = DAQmxErrorPhysicalChanDoesNotExist,
  #[error("Memory mapping can be enabled only if Data Transfer Mechanism is Programmed IO. Enable memory mapping only when Data Transfer Mechanism is Programmed IO.")]
  ErrorMemMapOnlyForProgIOXfer = DAQmxErrorMemMapOnlyForProgIOXfer,
  #[error("Number of channels to acquire exceeds the device maximum. Reduce the number of channels. In some cases, you can access a large number of channels if they are identically configured and created consecutively. Refer to the documentation for more information.")]
  ErrorTooManyChans = DAQmxErrorTooManyChans,
  #[error("Device cannot acquire from _cjTemp and other channels in the same task. Create one task for reading _cjTemp and another task for the other channels.")]
  ErrorCannotHaveCJTempWithOtherChans =
    DAQmxErrorCannotHaveCJTempWithOtherChans,
  #[error("Output buffer underwrite. Your application was unable to write samples to the background buffer fast enough for the device to get new samples at the specified sample rate. To avoid this error, you can do any the following: 1. Increase the size of the background buffer by configuring the buffer. 2. Increase the number of samples you write each time you invoke a write operation. 3. Write samples more often. 4. Reduce your sample rate. 5. Change the data transfer mechanism from interrupts to DMA. 6. Initially write a sufficient number of samples to satisfy the specified data transfer request condition. 7. Reduce the number of applications that your computer is executing concurrently. In addition, if you do not need to ensure that each sample is generated once and only once, you can set the regeneration mode to allow regeneration.")]
  ErrorOutputBufferUnderwrite = DAQmxErrorOutputBufferUnderwrite,
  #[error(
        "Completion resistance value, R1,  cannot be zero if the circuit uses voltage excitation."
    )]
  ErrorSensorInvalidCompletionResistance =
    DAQmxErrorSensorInvalidCompletionResistance,
  #[error(
    "2-wire resistance configuration is incompatible with voltage excitation."
  )]
  ErrorVoltageExcitIncompatibleWith2WireCfg =
    DAQmxErrorVoltageExcitIncompatibleWith2WireCfg,
  #[error("Device to which the sensor is attached does not have an available internal excitation source. Select another device with an available internal excitation source or supply external excitation.")]
  ErrorIntExcitSrcNotAvailable = DAQmxErrorIntExcitSrcNotAvailable,
  #[error("Channel could not be created. All channels must be created before the task is verified. Before I/O can be performed or properties can be retrieved, tasks are verified. Channels must be created before these actions can occur.")]
  ErrorCannotCreateChannelAfterTaskVerified =
    DAQmxErrorCannotCreateChannelAfterTaskVerified,
  #[error("Requested operation could not be performed because the digital lines are being used for communication with SCXI or a TEDS carrier. For example, E Series devices use lines 0, 1, 2, and 4 on port 0 to communicate with a SCXI module. Therefore, you cannot use lines 0, 1, 2, and 4 for regular digital I/O.")]
  ErrorLinesReservedForSCXIControl = DAQmxErrorLinesReservedForSCXIControl,
  #[error("Requested operation could not be performed because the necessary digital lines could not be reserved by SCXI. Another task might have reserved these lines previously. For example, E Series devices use lines 0, 1, 2, and 4 on port 0 to communicate with the SCXI module.")]
  ErrorCouldNotReserveLinesForSCXIControl =
    DAQmxErrorCouldNotReserveLinesForSCXIControl,
  #[error("Device could not complete the calibration operation. Calibration could fail for the following reasons: 1. The actual reference signal applied for calibration was different from the value you specified. Ensure that the reference signal applied is the same as the values that were input. 2. The reference signal was not stable over the period of time that the hardware was being calibrated. Ensure that the reference signal specified is free of noise and does not drift over the duration of the calibration. 3. The device is not functioning properly.")]
  ErrorCalibrationFailed = DAQmxErrorCalibrationFailed,
  #[error("Reference frequency applied for calibration is outside the range defined for calibration of this device. Ensure that the reference frequency falls within the range specified for this device.")]
  ErrorReferenceFrequencyInvalid = DAQmxErrorReferenceFrequencyInvalid,
  #[error("Reference resistance applied for calibration is outside the range defined for calibration of this device. Ensure that the reference resistance falls within the range specified for this device.")]
  ErrorReferenceResistanceInvalid = DAQmxErrorReferenceResistanceInvalid,
  #[error("Reference current applied for calibration is outside the range defined for calibration of this device. Ensure that the reference current falls within the range specified for this device.")]
  ErrorReferenceCurrentInvalid = DAQmxErrorReferenceCurrentInvalid,
  #[error("Reference voltage applied for calibration is outside the range defined for calibration of this device. Ensure that the reference voltage falls within the range specified for this device.")]
  ErrorReferenceVoltageInvalid = DAQmxErrorReferenceVoltageInvalid,
  #[error("Data read from the EEPROM on the device is invalid. Verify that any accessories configured with this device are connected. If the problem continues, contact National Instruments Technical Support. The device might need to be recalibrated or repaired by NI.")]
  ErrorEEPROMDataInvalid = DAQmxErrorEEPROMDataInvalid,
  #[error("Your SCXI system is not set up for performing analog input operations on given channels.  The SCXI module cabled to your digitizer cannot route analog signals from other modules to the digitizer, or is not configured to route them. To perform the desired operation with multiple SCXI modules and one digitizer, cable the digitizer to one of the analog input modules.  The module your channels are on is one such module.  Then, update the chassis configuration in MAX to reflect cabling change, and ensure that the cabled module is in multiplexed mode.  Alternatively, you can use multiple digitizers and SCXI modules in parallel mode. For detailed information about cabling, refer to documentation.")]
  ErrorCabledModuleNotCapableOfRoutingAI =
    DAQmxErrorCabledModuleNotCapableOfRoutingAI,
  #[error("Channel is not available when the module is in parallel mode.")]
  ErrorChannelNotAvailableInParallelMode =
    DAQmxErrorChannelNotAvailableInParallelMode,
  #[error("External timebase rate must be specified to translate the delay into ticks. Set the external timebase rate, or set the delay in units of ticks.")]
  ErrorExternalTimebaseRateNotKnownForDelay =
    DAQmxErrorExternalTimebaseRateNotKnownForDelay,
  #[error("FREQOUT counter cannot generate the desired frequency. The FREQOUT counter is a 4-bit counter that can divide either the master timebase rate / 2 or the master timebase rate / 200 by a number between one and 16. Choose a frequency within this range.")]
  ErrorFREQOUTCannotProduceDesiredFrequency =
    DAQmxErrorFREQOUTCannotProduceDesiredFrequency,
  #[error("There cannot be multiple counters in the same task for input operations. Use a separate task for each counter.")]
  ErrorMultipleCounterInputTask = DAQmxErrorMultipleCounterInputTask,
  #[error("Pause and start triggers cannot both be active in this task.")]
  ErrorCounterStartPauseTriggerConflict =
    DAQmxErrorCounterStartPauseTriggerConflict,
  #[error("Pause trigger is only valid for event counting if sample clock is not used.")]
  ErrorCounterInputPauseTriggerAndSampleClockInvalid =
    DAQmxErrorCounterInputPauseTriggerAndSampleClockInvalid,
  #[error("Pause trigger is only valid for continuous pulse generations. Change the sample mode to continuous, or do not use the pause trigger.")]
  ErrorCounterOutputPauseTriggerInvalid =
    DAQmxErrorCounterOutputPauseTriggerInvalid,
  #[error("Counter timebase rate must be specified for external counter timebase sources in order for frequency and/or time calculations to be made correctly. Set the Counter Timebase Rate property to the appropriate value for your external source.")]
  ErrorCounterTimebaseRateNotSpecified =
    DAQmxErrorCounterTimebaseRateNotSpecified,
  #[error("Internal timebase could not be found that matches the rate specified in the Counter Timebase Rate property.")]
  ErrorCounterTimebaseRateNotFound = DAQmxErrorCounterTimebaseRateNotFound,
  #[error("Data was overwritten before it could be read by the system. If Data Transfer Mechanism is Interrupts, try using DMA or USB Bulk. Otherwise, divide the input signal before taking the measurement.")]
  ErrorCounterOverflow = DAQmxErrorCounterOverflow,
  #[error("2 consecutive active edges of the input signal occurred without a counter timebase edge. Use a faster counter timebase.")]
  ErrorCounterNoTimebaseEdgesBetweenGates =
    DAQmxErrorCounterNoTimebaseEdgesBetweenGates,
  #[error("A timebase could not be selected that covers the entire range specified in the Maximum and Minimum properties. The conflicting properties must satisfy the following constraints: Maximum <= Counter Timebase Rate / 2 Minimum >= Counter Timebase Rate / Counter Maximum Count.")]
  ErrorCounterMaxMinRangeFreq = DAQmxErrorCounterMaxMinRangeFreq,
  #[error("A timebase could not be selected that covers the entire range specified in the Maximum and Minimum properties. The conflicting properties must satisfy the following constraints: Maximum <= Counter Maximum Count / Counter Timebase Rate Minimum >= 2 / Counter Timebase Rate.")]
  ErrorCounterMaxMinRangeTime = DAQmxErrorCounterMaxMinRangeTime,
  #[error("Initial Delay, High Time, and Low Time property values are inconsistent with one or more counter timebase properties. The conflicting properties must satisfy the following restraint: 2 / Counter Timebase Rate <= X <= Counter Maximum Count/ Counter Timebase Rate where X = Initial Delay, High Time, and Low Time, and where Counter Timebase Rate = Master Timebase Rate / Counter Timebase Master Timebase Divisor or is inferred from the Counter Timebase Source selection.")]
  ErrorSuitableTimebaseNotFoundTimeCombo =
    DAQmxErrorSuitableTimebaseNotFoundTimeCombo,
  #[error("Frequency and Initial Delay property values are inconsistent with one or more counter timebase properties. The conflicting properties must satisfy the following constraint: Counter Timebase Rate / Counter Maximum Count <= X <= Counter Timebase Rate / 2 where X = Frequency and 1 / Initial Delay, and where Counter Timebase Rate = Master Timebase Rate / Counter Timebase Master Timebase Divisor or is inferred from the Counter Timebase Source selection.")]
  ErrorSuitableTimebaseNotFoundFrequencyCombo =
    DAQmxErrorSuitableTimebaseNotFoundFrequencyCombo,
  #[error("Counter timebase source and counter timebase master timebase divisor settings are inconsistent with one another. If the divisor is specified, the following must apply: Master Timebase Rate / Counter Timebase Master Timebase Divisor = Rate corresponding to Counter Timebase Source.")]
  ErrorInternalTimebaseSourceDivisorCombo =
    DAQmxErrorInternalTimebaseSourceDivisorCombo,
  #[error("Counter timebase source and counter timebase rate settings are inconsistent with one another. For internal counter timebase source selections, if the counter timebase rate is set, its value must match the rate corresponding to the counter timebase source. For example, 20 MHz corresponds to a rate of 20,000,000 Hz.")]
  ErrorInternalTimebaseSourceRateCombo =
    DAQmxErrorInternalTimebaseSourceRateCombo,
  #[error("Counter timebase source, counter timebase rate, master timebase divisor, and master timebase rate settings are inconsistent with one another. The conflicting properties must satisfy the following constraint: Master Timebase Rate / Master Timebase Divisor = Counter Timebase Rate")]
  ErrorInternalTimebaseRateDivisorSourceCombo =
    DAQmxErrorInternalTimebaseRateDivisorSourceCombo,
  #[error("External timebase rate must be specified to translate the derived clock or timebase rate into ticks. Set the external timebase rate, or set the divisor instead of the clock or timebase rate.")]
  ErrorExternalTimebaseRateNotknownForRate =
    DAQmxErrorExternalTimebaseRateNotknownForRate,
  #[error("Analog trigger source must be the first channel in the acquisition or a valid analog trigger terminal. If you explicitly named the virtual channel in DAQmx Create Channel, you must use the name assigned to that channel.")]
  ErrorAnalogTrigChanNotFirstInScanList =
    DAQmxErrorAnalogTrigChanNotFirstInScanList,
  #[error("Timebase divisor cannot be set for an external clock. You cannot divide down an externally supplied clock. If you want to divide down an external clock, specifiy an external timebase source instead and set the clock source to be internal.")]
  ErrorNoDivisorForExternalSignal = DAQmxErrorNoDivisorForExternalSignal,
  #[error("Property must have the same value for all repeated physical channels. Set the same property value for all of the channels.")]
  ErrorAttributeInconsistentAcrossRepeatedPhysicalChannels =
    DAQmxErrorAttributeInconsistentAcrossRepeatedPhysicalChannels,
  #[error("Port 0 or any of its lines cannot be used to create a handshaking task. Use port 1 or port 2 of the 8255 chip on this device for handshaking.")]
  ErrorCannotHandshakeWithPort0 = DAQmxErrorCannotHandshakeWithPort0,
  #[error("Lines on port C cannot be used for both handshaking control and static digital operations on an 8255 chip. Handshaking tasks automatically reserve some lines on port C as control lines. These lines cannot be reserved for static digital operations when the device is configured for handshaking. There are two likely causes for this error: 1. An attempt was made to reserve the lines for static digital operations when a handshaking task was previously configured. 2. An attempt was made to create a handshaking task when the lines were previously reserved for static digital operations. Refer to the documentation for information about which lines on port C are not available when the 8255 chip is in handshaking mode.")]
  ErrorControlLineConflictOnPortC = DAQmxErrorControlLineConflictOnPortC,
  #[error("Lines 4 through 7 of this port are configured for output. Cannot configure these lines for input at this time.")]
  ErrorLines4To7ConfiguredForOutput = DAQmxErrorLines4To7ConfiguredForOutput,
  #[error("Lines 4 to 7 of this port are configured for input. Cannot configure these lines for output at this time.")]
  ErrorLines4To7ConfiguredForInput = DAQmxErrorLines4To7ConfiguredForInput,
  #[error("Lines 0 through 3 of this port are configured for output. Cannot configure these lines for input at this time.")]
  ErrorLines0To3ConfiguredForOutput = DAQmxErrorLines0To3ConfiguredForOutput,
  #[error("Lines 0 through 3 of this port are configured for input. Cannot configure these lines for output at this time.")]
  ErrorLines0To3ConfiguredForInput = DAQmxErrorLines0To3ConfiguredForInput,
  #[error("Port is configured for output. Cannot configure this port or any of its lines for input at this time.")]
  ErrorPortConfiguredForOutput = DAQmxErrorPortConfiguredForOutput,
  #[error("Port is configured for input. Cannot configure this port or any of its lines for output at this time.")]
  ErrorPortConfiguredForInput = DAQmxErrorPortConfiguredForInput,
  #[error("Port is configured for static digital operations by another task. Cannot configure this port or any of its lines for handshaking at this time.")]
  ErrorPortConfiguredForStaticDigitalOps =
    DAQmxErrorPortConfiguredForStaticDigitalOps,
  #[error("Port reserved for handshaking. Cannot reserve this port or any of its lines for another task at this time.")]
  ErrorPortReservedForHandshaking = DAQmxErrorPortReservedForHandshaking,
  #[error(
    "Port C cannot be used for data input/output in a handshaking task."
  )]
  ErrorPortDoesNotSupportHandshakingDataIO =
    DAQmxErrorPortDoesNotSupportHandshakingDataIO,
  #[error("Lines on the 8255 chip for this device are configured for output. Cannot tristate these lines at this time. Read values using an input task on another port.")]
  ErrorCannotTristate8255OutputLines = DAQmxErrorCannotTristate8255OutputLines,
  #[error(
    "Device temperature is outside of the required range for calibration."
  )]
  ErrorTemperatureOutOfRangeForCalibration =
    DAQmxErrorTemperatureOutOfRangeForCalibration,
  #[error("Calibration handle is invalid. Open a calibration session to get a valid calibration handle. Use the valid calibration handle obtained when the calibration session was opened.")]
  ErrorCalibrationHandleInvalid = DAQmxErrorCalibrationHandleInvalid,
  #[error("Password is required for this operation.")]
  ErrorPasswordRequired = DAQmxErrorPasswordRequired,
  #[error("Password supplied is incorrect.")]
  ErrorIncorrectPassword = DAQmxErrorIncorrectPassword,
  #[error("Password is longer than four characters.")]
  ErrorPasswordTooLong = DAQmxErrorPasswordTooLong,
  #[error("Calibration session is already open on this device. You can have only one open calibration session for each device. Use the handle obtained when the calibration session for this device was originally opened.")]
  ErrorCalibrationSessionAlreadyOpen = DAQmxErrorCalibrationSessionAlreadyOpen,
  #[error("Module specified in the hardware configuration is not the module found. Make sure that the module specified in the hardware configuration is present in the specified slot.")]
  ErrorSCXIModuleIncorrect = DAQmxErrorSCXIModuleIncorrect,
  #[error(
    "Property must have the same value for all channels on this device."
  )]
  ErrorAttributeInconsistentAcrossChannelsOnDevice =
    DAQmxErrorAttributeInconsistentAcrossChannelsOnDevice,
  #[error("Channel is invalid for the excitation mode of the SCXI-1122. Disable multiplexed excitation, or use one of the physical channels between ai0 and ai7.")]
  ErrorSCXI1122ResistanceChanNotSupportedForCfg =
    DAQmxErrorSCXI1122ResistanceChanNotSupportedForCfg,
  #[error("Bracket character (\"[\" or \"]\") at the specified position in the list is invalid. Matching bracket cannot be found. Check for nested fully specified paths or incorrectly paired brackets.")]
  ErrorBracketPairingMismatchInList = DAQmxErrorBracketPairingMismatchInList,
  #[error("Number of samples to write must be the same for every channel.")]
  ErrorInconsistentNumSamplesToWrite = DAQmxErrorInconsistentNumSamplesToWrite,
  #[error("Pattern width specified does not match the number of lines in the digital channel.")]
  ErrorIncorrectDigitalPattern = DAQmxErrorIncorrectDigitalPattern,
  #[error(
        "Number of channels in data to write does not match the number of channels in the task."
    )]
  ErrorIncorrectNumChannelsToWrite = DAQmxErrorIncorrectNumChannelsToWrite,
  #[error("Specified DAQmx Read only can be used to read from a single channel. Use the multichannel DAQmx Read.")]
  ErrorIncorrectReadFunction = DAQmxErrorIncorrectReadFunction,
  #[error("Physical channel not specified.")]
  ErrorPhysicalChannelNotSpecified = DAQmxErrorPhysicalChannelNotSpecified,
  #[error("Number of terminals requested cannot be greater than 1.")]
  ErrorMoreThanOneTerminal = DAQmxErrorMoreThanOneTerminal,
  #[error("Attempted to retrieve channel properties from a multichannel task with more than one channel selected. You must select an individual channel to retrieve channel properties. If you are programming with LabVIEW, use the Active Channel property to specify the individual channel.")]
  ErrorMoreThanOneActiveChannelSpecified =
    DAQmxErrorMoreThanOneActiveChannelSpecified,
  #[error("Number of samples to read must be -1 or greater.")]
  ErrorInvalidNumberSamplesToRead = DAQmxErrorInvalidNumberSamplesToRead,
  #[error("Analog waveform expected as input.")]
  ErrorAnalogWaveformExpected = DAQmxErrorAnalogWaveformExpected,
  #[error("Digital waveform expected as input.")]
  ErrorDigitalWaveformExpected = DAQmxErrorDigitalWaveformExpected,
  #[error("Attempted to retrieve channel properties from a multichannel task without selecting a specific channel. Use the Active Channel property to select a specific channel from which to retrieve properties.")]
  ErrorActiveChannelNotSpecified = DAQmxErrorActiveChannelNotSpecified,
  #[error("Function supported for channel-based tasks only.")]
  ErrorFunctionNotSupportedForDeviceTasks =
    DAQmxErrorFunctionNotSupportedForDeviceTasks,
  #[error("Shared library version installed is incorrect. This error might be the result of an incorrect installation of NI-DAQmx or a related software package. Reinstall NI-DAQmx, or download the latest version of the driver from the National Instruments website at ni.com. If the error is still returned, contact NI Technical Support.")]
  ErrorFunctionNotInLibrary = DAQmxErrorFunctionNotInLibrary,
  #[error("Shared library was not found. This error might be the result of an inadvertent deletion of an NI-DAQmx component. Reinstall NI-DAQmx, or download the latest version of the driver from the National Instruments website at ni.com. If the error is still returned, contact NI Technical Support.")]
  ErrorLibraryNotPresent = DAQmxErrorLibraryNotPresent,
  #[error("Task name specified conflicts with an existing task name.")]
  ErrorDuplicateTask = DAQmxErrorDuplicateTask,
  #[error("Task specified is invalid or does not exist.")]
  ErrorInvalidTask = DAQmxErrorInvalidTask,
  #[error("Channel is not in the task, and the channel is not a valid global channel. Make sure that the channel is in the task or that the channel is a valid global channel. If you explicitly named the virtual channel in DAQmx Create Channel, you must use the name assigned to that channel. Also, check for typing errors.")]
  ErrorInvalidChannel = DAQmxErrorInvalidChannel,
  #[error("Physical channel range syntax in the input string is invalid because multiple devices were listed in the string.")]
  ErrorInvalidSyntaxForPhysicalChannelRange =
    DAQmxErrorInvalidSyntaxForPhysicalChannelRange,
  #[error("Minimum is greater than or equal to the maximum. Ensure the maximum value is greater than the minimum value. If using a custom scale, ensure that the scaled maximum is greater than the scaled minimum.")]
  ErrorMinNotLessThanMax = DAQmxErrorMinNotLessThanMax,
  #[error("Sample rate exceeds the maximum sample rate for the number of channels specified. Reduce the sample rate or the number of channels. Increasing the convert rate or reducing the sample delay might also alleviate the problem, if you set either of them.")]
  ErrorSampleRateNumChansConvertPeriodCombo =
    DAQmxErrorSampleRateNumChansConvertPeriodCombo,
  #[error("Analog output (AO) task started or committed during a counter 1 DMA acquisition. If possible, use counter 0 instead of counter 1. Otherwise, commit the AO task before starting the counter 1 DMA acquisition.")]
  ErrorAODuringCounter1DMAConflict = DAQmxErrorAODuringCounter1DMAConflict,
  #[error("Analog input (AI) task started or committed during a counter 0 DMA acquisition. If possible, use counter 1 instead of counter 0. Otherwise, start/commit the AI task before starting the counter 0 DMA acquisition.")]
  ErrorAIDuringCounter0DMAConflict = DAQmxErrorAIDuringCounter0DMAConflict,
  #[error("Requested value is not a supported value for this property. The property value may be invalid because it conflicts with another property.")]
  ErrorInvalidAttributeValue = DAQmxErrorInvalidAttributeValue,
  #[error("Current data supplied is outside of the specified range.")]
  ErrorSuppliedCurrentDataOutsideSpecifiedRange =
    DAQmxErrorSuppliedCurrentDataOutsideSpecifiedRange,
  #[error("Voltage data supplied is outside of the specified range. Change the range or the data.  Refer to the documentation for more information about possible ranges.")]
  ErrorSuppliedVoltageDataOutsideSpecifiedRange =
    DAQmxErrorSuppliedVoltageDataOutsideSpecifiedRange,
  #[error("Device unable to store calibration constants. Make sure that your hardware is properly installed, and test the regular operation of the device.")]
  ErrorCannotStoreCalConst = DAQmxErrorCannotStoreCalConst,
  #[error("SCXI module specified in the hardware configuration was not found. Make sure that the SCXI chassis is powered on, the SCXI cable is properly connected between the chassis communicator and the SCXI module, and that the cabled module specified in the hardware configuration is present in the specified slot.")]
  ErrorSCXIModuleNotFound = DAQmxErrorSCXIModuleNotFound,
  #[error(
        "Duplicate channels in the list of physical channels are not supported by this device."
    )]
  ErrorDuplicatePhysicalChansNotSupported =
    DAQmxErrorDuplicatePhysicalChansNotSupported,
  #[error("Number of physical channels is too large.")]
  ErrorTooManyPhysicalChansInList = DAQmxErrorTooManyPhysicalChansInList,
  #[error("Advance trigger type specified is not supported by the device.")]
  ErrorInvalidAdvanceEventTriggerType =
    DAQmxErrorInvalidAdvanceEventTriggerType,
  #[error("Device specified is not a valid switch device.")]
  ErrorDeviceIsNotAValidSwitch = DAQmxErrorDeviceIsNotAValidSwitch,
  #[error("Scanning is not supported by the specified device.")]
  ErrorDeviceDoesNotSupportScanning = DAQmxErrorDeviceDoesNotSupportScanning,
  #[error("Settling time constraints for the device could not be satisfied. Refer to the documentation for details about settling time constraints.")]
  ErrorScanListCannotBeTimed = DAQmxErrorScanListCannotBeTimed,
  #[error(
    "Connection operator is invalid at the designated point in the list entry."
  )]
  ErrorConnectOperatorInvalidAtPointInList =
    DAQmxErrorConnectOperatorInvalidAtPointInList,
  #[error("Action at the designated position in the scanlist is not valid for the device.")]
  ErrorUnexpectedSwitchActionInList = DAQmxErrorUnexpectedSwitchActionInList,
  #[error(
        "Unexpected connection separator \"&\" or sequence separator \"&&\" in the list entry."
    )]
  ErrorUnexpectedSeparatorInList = DAQmxErrorUnexpectedSeparatorInList,
  #[error(
    "Invalid identifier instead of an expected terminator in the list entry."
  )]
  ErrorExpectedTerminatorInList = DAQmxErrorExpectedTerminatorInList,
  #[error(
        "Invalid identifier instead of an expected connection operator \"->\" in the list entry."
    )]
  ErrorExpectedConnectOperatorInList = DAQmxErrorExpectedConnectOperatorInList,
  #[error("Invalid identifier in the list entry.  The connection separator \"&\" or sequence separator \"&&\" was expected.")]
  ErrorExpectedSeparatorInList = DAQmxErrorExpectedSeparatorInList,
  #[error("Fully specified path cannot contain a connection range.")]
  ErrorFullySpecifiedPathInListContainsRange =
    DAQmxErrorFullySpecifiedPathInListContainsRange,
  #[error("List cannot end with the connection separator \"&\".")]
  ErrorConnectionSeparatorAtEndOfList =
    DAQmxErrorConnectionSeparatorAtEndOfList,
  #[error("Identifier in the list entry is too long.")]
  ErrorIdentifierInListTooLong = DAQmxErrorIdentifierInListTooLong,
  #[error("Duplicate device identifier in the device list. This is not allowed when waiting for devices to settle.")]
  ErrorDuplicateDeviceIDInListWhenSettling =
    DAQmxErrorDuplicateDeviceIDInListWhenSettling,
  #[error("Channel name not specified in the list entry.")]
  ErrorChannelNameNotSpecifiedInList = DAQmxErrorChannelNameNotSpecifiedInList,
  #[error("Device identifier not specified in the list entry.")]
  ErrorDeviceIDNotSpecifiedInList = DAQmxErrorDeviceIDNotSpecifiedInList,
  #[error("Semicolon or a semicolon modifier must follow a connection range statement. Refer to the documentation for information on connection ranges and semicolon modifiers.")]
  ErrorSemicolonDoesNotFollowRangeInList =
    DAQmxErrorSemicolonDoesNotFollowRangeInList,
  #[error("Channels in switch actions cannot span different devices.")]
  ErrorSwitchActionInListSpansMultipleDevices =
    DAQmxErrorSwitchActionInListSpansMultipleDevices,
  #[error("Invalid identifier instead of an expected connection  operator, \"->\", in the list entry. Refer to the documentation for proper syntax for connections involving channel ranges.")]
  ErrorRangeWithoutAConnectActionInList =
    DAQmxErrorRangeWithoutAConnectActionInList,
  #[error("Invalid identifier after a separator in the list entry.")]
  ErrorInvalidIdentifierFollowingSeparatorInList =
    DAQmxErrorInvalidIdentifierFollowingSeparatorInList,
  #[error("Invalid channel name in the list entry. Refer to the documentation for valid channel names for the device in use.")]
  ErrorInvalidChannelNameInList = DAQmxErrorInvalidChannelNameInList,
  #[error("Invalid value in the \"<repeat>\" statement in the list entry. The syntax for a repeat statement is \"<repeat integer>\". Refer to the documentation for valid integer values.")]
  ErrorInvalidNumberInRepeatStatementInList =
    DAQmxErrorInvalidNumberInRepeatStatementInList,
  #[error("Invalid  trigger line in the \"<sac>\" or \"<wfa>\" statement in the list entry. Refer to the documentation for valid trigger lines.")]
  ErrorInvalidTriggerLineInList = DAQmxErrorInvalidTriggerLineInList,
  #[error("Invalid identifier after the device identifier in the list entry.")]
  ErrorInvalidIdentifierInListFollowingDeviceID =
    DAQmxErrorInvalidIdentifierInListFollowingDeviceID,
  #[error("Invalid identifier at the end of the switch action. A connection separator, sequence separator, or valid switch action terminator must follow a switch action.")]
  ErrorInvalidIdentifierInListAtEndOfSwitchAction =
    DAQmxErrorInvalidIdentifierInListAtEndOfSwitchAction,
  #[error("Device was removed or powered down between task verification and reservation. Ensure that the device is not being reset.")]
  ErrorDeviceRemoved = DAQmxErrorDeviceRemoved,
  #[error("Route cannot be made between the source and destination terminals. Either the hardware does not support this route or other routes might be interfering with this route.")]
  ErrorRoutingPathNotAvailable = DAQmxErrorRoutingPathNotAvailable,
  #[error(
    "Hardware necessary for this route is in use by another task or tasks."
  )]
  ErrorRoutingHardwareBusy = DAQmxErrorRoutingHardwareBusy,
  #[error("Inversion requested is not possible. Either the hardware between the source and destination terminals does not support the inversion, or other routes in the task might be interfering with this route.")]
  ErrorRequestedSignalInversionForRoutingNotPossible =
    DAQmxErrorRequestedSignalInversionForRoutingNotPossible,
  #[error("Destination terminal to be routed could not be found on the device. Make sure the terminal name is valid for the specified device. Refer to Measurement & Automation Explorer or your hardware documentation for valid terminal names.")]
  ErrorInvalidRoutingDestinationTerminalName =
    DAQmxErrorInvalidRoutingDestinationTerminalName,
  #[error("Source terminal to be routed could not be found on the device. Make sure the terminal name is valid for the specified device. Refer to Measurement & Automation Explorer for valid terminal names.")]
  ErrorInvalidRoutingSourceTerminalName =
    DAQmxErrorInvalidRoutingSourceTerminalName,
  #[error("Routing information associated with your device cannot be found.")]
  ErrorRoutingNotSupportedForDevice = DAQmxErrorRoutingNotSupportedForDevice,
  #[error("\"Wait\" instruction cannot be the last instruction of a \"repeat until\" loop.")]
  ErrorWaitIsLastInstructionOfLoopInScript =
    DAQmxErrorWaitIsLastInstructionOfLoopInScript,
  #[error("\"Clear trigger\" instruction cannot be the last instruction of a \"repeat\" loop.")]
  ErrorClearIsLastInstructionOfLoopInScript =
    DAQmxErrorClearIsLastInstructionOfLoopInScript,
  #[error(
    "Number of iterations specified for a finite \"repeat\" loop is invalid."
  )]
  ErrorInvalidLoopIterationsInScript = DAQmxErrorInvalidLoopIterationsInScript,
  #[error("Repeat loop is contained within too many levels of nested repeat loops. Unroll one of the \"repeat\" loops if possible, or change the script and run it several times. To unroll a loop, remove the \"repeat\" and \"end repeat\" instructions and explicitly replicate the instructions of the removed loop the desired number of times.")]
  ErrorRepeatLoopNestingTooDeepInScript =
    DAQmxErrorRepeatLoopNestingTooDeepInScript,
  #[error("Marker position exceeds the length of the subset.")]
  ErrorMarkerPositionOutsideSubsetInScript =
    DAQmxErrorMarkerPositionOutsideSubsetInScript,
  #[error(
    "Start offset of the subset is not a multiple of the alignment quantum."
  )]
  ErrorSubsetStartOffsetNotAlignedInScript =
    DAQmxErrorSubsetStartOffsetNotAlignedInScript,
  #[error("Subset length specified is not valid. Change the subset length to be longer than zero samples and a multiple of the alignment quantum.")]
  ErrorInvalidSubsetLengthInScript = DAQmxErrorInvalidSubsetLengthInScript,
  #[error("Marker position specified is not a multiple of alignment quantum.")]
  ErrorMarkerPositionNotAlignedInScript =
    DAQmxErrorMarkerPositionNotAlignedInScript,
  #[error("Subset specified in a generate instruction exceeds the waveform boundaries. Change the subset start offset and/or subset length so the subset fits within the waveform, or increase the size of the waveform.")]
  ErrorSubsetOutsideWaveformInScript = DAQmxErrorSubsetOutsideWaveformInScript,
  #[error("Marker specified in a generate instruction exceeds the waveform boundaries. Change the marker position or positions to fit within the waveform, or increase the size of the waveform.")]
  ErrorMarkerOutsideWaveformInScript = DAQmxErrorMarkerOutsideWaveformInScript,
  #[error("Waveform referenced in the script was not found in onboard memory. Write the waveform to the device before writing the script.")]
  ErrorWaveformInScriptNotInMem = DAQmxErrorWaveformInScriptNotInMem,
  #[error("Keyword was expected, but not found in the script.")]
  ErrorKeywordExpectedInScript = DAQmxErrorKeywordExpectedInScript,
  #[error("Waveform name was expected, but not found in the script.")]
  ErrorBufferNameExpectedInScript = DAQmxErrorBufferNameExpectedInScript,
  #[error("Script name was expected, but not found in the script.")]
  ErrorProcedureNameExpectedInScript = DAQmxErrorProcedureNameExpectedInScript,
  #[error("Valid identifier expected but not found in script. The identifier should specify a valid waveform or script name. Identifiers cannot start with a number.")]
  ErrorScriptHasInvalidIdentifier = DAQmxErrorScriptHasInvalidIdentifier,
  #[error("Script contains an invalid character or symbol. Replace the invalid character with a valid symbol or alphanumeric character.")]
  ErrorScriptHasInvalidCharacter = DAQmxErrorScriptHasInvalidCharacter,
  #[error("Resource requested by this task has already been reserved by a different task.")]
  ErrorResourceAlreadyReserved = DAQmxErrorResourceAlreadyReserved,
  #[error("Self-test of the device has failed.")]
  ErrorSelfTestFailed = DAQmxErrorSelfTestFailed,
  #[error("ADC conversion attempted before the prior conversion was complete. Increase the period between ADC conversions. If you are using an external clock, check your signal for the presence of noise or glitches.")]
  ErrorADCOverrun = DAQmxErrorADCOverrun,
  #[error("DAC conversion attempted before data to be converted was available. Decrease the output frequency to increase the period between DAC conversions, or reduce the size of your output buffer in order to write data more often. If you are using an external clock, check  your signal for the presence of noise or glitches.")]
  ErrorDACUnderflow = DAQmxErrorDACUnderflow,
  #[error("Onboard device memory underflow. Not enough new data has been sampled since the last read or the start of the measurement. Increase the sample rate, increase the timeout value, or decrease the number of samples to read.")]
  ErrorInputFIFOUnderflow = DAQmxErrorInputFIFOUnderflow,
  #[error("Onboard device memory underflow. Because of system and/or bus-bandwidth limitations, the driver could not write data to the device fast enough to keep up with the device output rate. Reduce your sample rate. If your data transfer method is interrupts, try using DMA or USB Bulk. You can also use a product with more onboard memory or reduce the number of programs your computer is executing concurrently.")]
  ErrorOutputFIFOUnderflow = DAQmxErrorOutputFIFOUnderflow,
  #[error("Communication with SCXI failed. The communication cable to the SCXI hardware might have been disconnected or exposed to excessive noise.")]
  ErrorSCXISerialCommunication = DAQmxErrorSCXISerialCommunication,
  #[error("Terminal cannot appear multiple times within a single digital input or output task.")]
  ErrorDigitalTerminalSpecifiedMoreThanOnce =
    DAQmxErrorDigitalTerminalSpecifiedMoreThanOnce,
  #[error("Specified physical channel does not support digital output. Change the direction of the task, use another terminal, or use another device.")]
  ErrorDigitalOutputNotSupported = DAQmxErrorDigitalOutputNotSupported,
  #[error("Task cannot contain both input and output channels. Either use channels of one direction in a task or make two separate tasks.")]
  ErrorInconsistentChannelDirections = DAQmxErrorInconsistentChannelDirections,
  #[error("Onboard device memory overflow. Because of system and/or bus-bandwidth limitations, the driver could not read data from the device fast enough to keep up with the device throughput. Reduce your sample rate. If your data transfer method is interrupts, try using DMA or USB Bulk. You can also use a product with more onboard memory or reduce the number of programs your computer is executing concurrently.")]
  ErrorInputFIFOOverflow = DAQmxErrorInputFIFOOverflow,
  #[error(
    "Timestamps have been overwritten. You can no longer read any data."
  )]
  ErrorTimeStampOverwritten = DAQmxErrorTimeStampOverwritten,
  #[error("Stop trigger has not occurred yet.")]
  ErrorStopTriggerHasNotOccurred = DAQmxErrorStopTriggerHasNotOccurred,
  #[error("Record requested has not been acquired yet .")]
  ErrorRecordNotAvailable = DAQmxErrorRecordNotAvailable,
  #[error("Record requested has been overwritten in the device memory.")]
  ErrorRecordOverwritten = DAQmxErrorRecordOverwritten,
  #[error("Data requested has not been acquired yet.")]
  ErrorDataNotAvailable = DAQmxErrorDataNotAvailable,
  #[error("Data requested has been overwritten in the device memory.")]
  ErrorDataOverwrittenInDeviceMemory = DAQmxErrorDataOverwrittenInDeviceMemory,
  #[error("Channel cannot be used more than once inside a list of channels. If you need to use the same physical channel more than once inside your list of channels, refer to that physical channel under different names.")]
  ErrorDuplicatedChannel = DAQmxErrorDuplicatedChannel,
  #[error("The installed version of the driver for this device is no longer supported. Install the current version of the driver for this device.")]
  ErrorInterfaceObsoleted_Routing = DAQmxErrorInterfaceObsoleted_Routing,
  #[error("Could not connect to the NI Route Coordinator service.  Ensure that the NI Route Coordinator service is installed and started.")]
  ErrorRoCoServiceNotAvailable_Routing =
    DAQmxErrorRoCoServiceNotAvailable_Routing,
  #[error("PXIe_DStar<n> (A, B, or C) is available as a destination terminal only for devices in a system timing slot. To use PXIe_DStar A, B, or C (without any numbers), do not specify a star line number. To use PXIe_DStar<n> (A, B, or C), move your device to a system timing slot.")]
  ErrorRoutingDestTermPXIDStarXNotInSystemTimingSlot_Routing =
    DAQmxErrorRoutingDestTermPXIDStarXNotInSystemTimingSlot_Routing,
  #[error("PXIe_DStar<n> (A, B, or C) is available as a source terminal only for devices in a system timing slot. To use PXIe_DStar A, B, or C (without any numbers), do not specify a star line number. To use PXIe_DStar<n> (A, B, or C), move your device to a system timing slot.")]
  ErrorRoutingSrcTermPXIDStarXNotInSystemTimingSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIDStarXNotInSystemTimingSlot_Routing,
  #[error("PXIe_DStar (A, B, or C) is not available as a source terminal in your device's current slot. Move your device to one of the slots connected to the DStar Trigger bus, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIDStarInNonDStarTriggerSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIDStarInNonDStarTriggerSlot_Routing,
  #[error("PXIe_DStar (A, B, or C) is not available as a destination terminal in your device's current slot. Move your device to one of the slots connected to the DStar Trigger bus, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIDStarInNonDStarTriggerSlot_Routing =
    DAQmxErrorRoutingDestTermPXIDStarInNonDStarTriggerSlot_Routing,
  #[error("PXI_Clk10In is available as a destination terminal only for devices in a star trigger slot. Move your device to a star trigger slot.")]
  ErrorRoutingDestTermPXIClk10InNotInStarTriggerSlot_Routing =
    DAQmxErrorRoutingDestTermPXIClk10InNotInStarTriggerSlot_Routing,
  #[error("PXI_Clk10In is available as a destination terminal only for devices in a system timing slot. Move your device to a system timing slot.")]
  ErrorRoutingDestTermPXIClk10InNotInSystemTimingSlot_Routing =
    DAQmxErrorRoutingDestTermPXIClk10InNotInSystemTimingSlot_Routing,
  #[error("PXI_Star<n> is available as a destination terminal only for devices in a star trigger slot. To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to a star trigger slot.")]
  ErrorRoutingDestTermPXIStarXNotInStarTriggerSlot_Routing =
    DAQmxErrorRoutingDestTermPXIStarXNotInStarTriggerSlot_Routing,
  #[error("PXI_Star<n> is available as a destination terminal only for devices in a system timing slot. To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to a system timing slot.")]
  ErrorRoutingDestTermPXIStarXNotInSystemTimingSlot_Routing =
    DAQmxErrorRoutingDestTermPXIStarXNotInSystemTimingSlot_Routing,
  #[error("PXI_Star<n> is available as a source terminal only for devices in a star trigger slot. To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to a star trigger slot.")]
  ErrorRoutingSrcTermPXIStarXNotInStarTriggerSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIStarXNotInStarTriggerSlot_Routing,
  #[error("PXI_Star<n> is available as a source terminal only for devices in a system timing slot. To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to a system timing slot.")]
  ErrorRoutingSrcTermPXIStarXNotInSystemTimingSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIStarXNotInSystemTimingSlot_Routing,
  #[error("PXI_Star is not available as a source terminal in your device's current slot. Move your device to one of the slots connected to the Star Trigger bus, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInNonStarTriggerSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIStarInNonStarTriggerSlot_Routing,
  #[error("PXI_Star is not available as a destination terminal in your device's current slot. Move your device to one of the slots connected to the Star Trigger bus, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInNonStarTriggerSlot_Routing =
    DAQmxErrorRoutingDestTermPXIStarInNonStarTriggerSlot_Routing,
  #[error("PXI_Star is not available as a destination terminal for devices in a star trigger slot. A star trigger slot has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of the other slots connected to the Star Trigger bus, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInStarTriggerSlot_Routing =
    DAQmxErrorRoutingDestTermPXIStarInStarTriggerSlot_Routing,
  #[error("PXI_Star is not available as a destination terminal for devices in a system timing slot. A system timing slot has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of the other slots connected to the Star Trigger bus, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInSystemTimingSlot_Routing =
    DAQmxErrorRoutingDestTermPXIStarInSystemTimingSlot_Routing,
  #[error("PXI_Star is not available as a source terminal for devices in a star trigger slot. A star trigger slot has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of the other slots connected to the Star Trigger bus, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInStarTriggerSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIStarInStarTriggerSlot_Routing,
  #[error("PXI_Star is not available as a source terminal for devices in a system timing slot. A system timing slot has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of the other slots connected to the Star Trigger bus, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInSystemTimingSlot_Routing =
    DAQmxErrorRoutingSrcTermPXIStarInSystemTimingSlot_Routing,
  #[error("The value specified does not correspond to any terminal connection behavior.")]
  ErrorInvalidSignalModifier_Routing = DAQmxErrorInvalidSignalModifier_Routing,
  #[error("PXI_Clk10In is available as a destination terminal only for devices in the star trigger controller slot (slot 2). Move your device to PXI slot 2.")]
  ErrorRoutingDestTermPXIClk10InNotInSlot2_Routing =
    DAQmxErrorRoutingDestTermPXIClk10InNotInSlot2_Routing,
  #[error("PXI_Star<n> is available as a destination terminal only for devices in the star trigger controller slot (slot 2). To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to slot 2.")]
  ErrorRoutingDestTermPXIStarXNotInSlot2_Routing =
    DAQmxErrorRoutingDestTermPXIStarXNotInSlot2_Routing,
  #[error("PXI_Star<n> is available as a source terminal only for devices in the star trigger controller slot (slot 2). To use PXI_Star (without any numbers), do not specify a star line number. To use PXI_Star<n>, move your device to slot 2.")]
  ErrorRoutingSrcTermPXIStarXNotInSlot2_Routing =
    DAQmxErrorRoutingSrcTermPXIStarXNotInSlot2_Routing,
  #[error("PXI_Star is not available as a source terminal for devices in PXI slots 16 and above. Move your device to one of slots 3 through 15, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInSlot16AndAbove_Routing =
    DAQmxErrorRoutingSrcTermPXIStarInSlot16AndAbove_Routing,
  #[error("PXI_Star is not available as a destination terminal for devices in PXI slots 16 and above. Move your device to one of slots 3 through 15, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInSlot16AndAbove_Routing =
    DAQmxErrorRoutingDestTermPXIStarInSlot16AndAbove_Routing,
  #[error("PXI_Star is not available as a destination terminal for devices in PXI slot 2. PXI slot 2 has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of slots 3 through 15, or select a different destination terminal.")]
  ErrorRoutingDestTermPXIStarInSlot2_Routing =
    DAQmxErrorRoutingDestTermPXIStarInSlot2_Routing,
  #[error("PXI_Star is not available as a source terminal for devices in PXI slot 2. PXI slot 2 has specific PXI_Star<n> lines, such as PXI_Star3. Move your device to one of slots 3 through 15, or select a different source terminal.")]
  ErrorRoutingSrcTermPXIStarInSlot2_Routing =
    DAQmxErrorRoutingSrcTermPXIStarInSlot2_Routing,
  #[error("Route failed because the PXI chassis is not identified. The existence of the destination terminal depends on the chassis being identified. Use the Measurements & Automation Explorer (MAX) to identify your chassis.")]
  ErrorRoutingDestTermPXIChassisNotIdentified_Routing =
    DAQmxErrorRoutingDestTermPXIChassisNotIdentified_Routing,
  #[error("Route failed because the PXI chassis is not identified. The existence of the source terminal depends on the chassis being identified. Use the Measurements & Automation Explorer (MAX) to identify your chassis.")]
  ErrorRoutingSrcTermPXIChassisNotIdentified_Routing =
    DAQmxErrorRoutingSrcTermPXIChassisNotIdentified_Routing,
  #[error("Driver cannot complete the route, because the only way to make the route requires a trigger bus line, and no trigger bus has been configured in MAX for this device. If you have a PXI chassis, make sure it has been properly identified in MAX. If you are using a PCI device, create a RTSI cable in MAX that includes your PCI device even if you are not using any RTSI cables.")]
  ErrorTrigLineNotFoundSingleDevRoute_Routing =
    DAQmxErrorTrigLineNotFoundSingleDevRoute_Routing,
  #[error("There are no shared trigger lines between the two devices which are acceptable to both devices. While each of these two devices support some shared trigger lines, none of these shared trigger lines work for both devices for the specified property and corresponding value. Consider routing the signal through the I/O connectors of the two devices, if applicable.")]
  ErrorNoCommonTrigLineForRoute_Routing =
    DAQmxErrorNoCommonTrigLineForRoute_Routing,
  #[error("Specified route cannot be satisfied, because it requires resources that are currently in use by another route within this task.")]
  ErrorResourcesInUseForRouteInTask_Routing =
    DAQmxErrorResourcesInUseForRouteInTask_Routing,
  #[error("Specified route cannot be satisfied, because it requires resources that are currently in use by another route.")]
  ErrorResourcesInUseForRoute_Routing =
    DAQmxErrorResourcesInUseForRoute_Routing,
  #[error("Specified route cannot be satisfied, because the hardware does not support it.")]
  ErrorRouteNotSupportedByHW_Routing = DAQmxErrorRouteNotSupportedByHW_Routing,
  #[error("Specified inversion cannot be satisfied, because it requires resources that are currently in use by another route within this task.")]
  ErrorResourcesInUseForInversionInTask_Routing =
    DAQmxErrorResourcesInUseForInversionInTask_Routing,
  #[error("Specified inversion cannot be satisfied, because it requires resources that are currently in use by another route.")]
  ErrorResourcesInUseForInversion_Routing =
    DAQmxErrorResourcesInUseForInversion_Routing,
  #[error("Specified inversion cannot be satisfied, because the hardware does not support it.")]
  ErrorInversionNotSupportedByHW_Routing =
    DAQmxErrorInversionNotSupportedByHW_Routing,
  #[error("Specified property value cannot be used, because it requires resources that are currently in use.")]
  ErrorResourcesInUseForProperty_Routing =
    DAQmxErrorResourcesInUseForProperty_Routing,
  #[error("An attempt has been made to perform a route when the source and the destination are the same terminal. In many cases, such as when configuring an external clock or a counter source, you must select a PFI, PXI Trigger, or RTSI line as the source terminal.")]
  ErrorRouteSrcAndDestSame_Routing = DAQmxErrorRouteSrcAndDestSame_Routing,
  #[error("Device not available for routing.  It is possible that the device needs to be reset or that the device is being reset. If you are resetting the device, wait for the reset to complete. For example, if you have used the device through Traditional NI-DAQ, you must reset the device before the requested route can be made.  For SCXI devices, you must reset the communicator DAQ device. Call the Traditional NI-DAQ Device Reset VI or the Init_DA_Brds function. To reset all devices in Traditional NI-DAQ, right-click the Traditional NI-DAQ Devices folder in MAX and select Reset Driver for Traditional NI-DAQ.")]
  ErrorDevAbsentOrUnavailable_Routing =
    DAQmxErrorDevAbsentOrUnavailable_Routing,
  #[error("Terminal for the device is invalid.")]
  ErrorInvalidTerm_Routing = DAQmxErrorInvalidTerm_Routing,
  #[error("Terminal could not be tristated because the hardware cannot tristate this terminal.")]
  ErrorCannotTristateTerm_Routing = DAQmxErrorCannotTristateTerm_Routing,
  #[error("Terminal cannot be tristated because it is busy. Disconnect any routes spanning this terminal, or stop using this terminal.")]
  ErrorCannotTristateBusyTerm_Routing =
    DAQmxErrorCannotTristateBusyTerm_Routing,
  #[error("Trigger line requested could not be reserved because it is already in use.")]
  ErrorCouldNotReserveRequestedTrigLine_Routing =
    DAQmxErrorCouldNotReserveRequestedTrigLine_Routing,
  #[error("No registered trigger lines could be found between the devices in the route. If you have a PXI chassis, identify the chassis correctly in MAX, and make sure it has been configured properly. If you are using PCI devices, make sure they are connected with a RTSI cable and that the RTSI cable is registered in MAX. Otherwise, make sure there is an available trigger line on the trigger bus shared between the devices.")]
  ErrorTrigLineNotFound_Routing = DAQmxErrorTrigLineNotFound_Routing,
  #[error("Route cannot be made between the source and destination terminals. Either the hardware does not support this route or other routes might be interfering with this route.")]
  ErrorRoutingPathNotAvailable_Routing =
    DAQmxErrorRoutingPathNotAvailable_Routing,
  #[error(
    "Hardware necessary for this route is in use by another route or routes."
  )]
  ErrorRoutingHardwareBusy_Routing = DAQmxErrorRoutingHardwareBusy_Routing,
  #[error("Inversion requested is not possible. Either the hardware between the source and destination terminals does not support the inversion, or other routes might be interfering with this route.")]
  ErrorRequestedSignalInversionForRoutingNotPossible_Routing =
    DAQmxErrorRequestedSignalInversionForRoutingNotPossible_Routing,
  #[error("Destination terminal to be routed could not be found on the device. Make sure the terminal name is valid for the specified device. Refer to Measurement & Automation Explorer or your hardware documentation for valid terminal names.")]
  ErrorInvalidRoutingDestinationTerminalName_Routing =
    DAQmxErrorInvalidRoutingDestinationTerminalName_Routing,
  #[error("Source terminal to be routed could not be found on the device. Make sure the terminal name is valid for the specified device. Refer to Measurement & Automation Explorer for valid terminal names.")]
  ErrorInvalidRoutingSourceTerminalName_Routing =
    DAQmxErrorInvalidRoutingSourceTerminalName_Routing,
  #[error("Could not register with the NI Service Locator. Ensure that the NI Service Locator service is installed and started.")]
  ErrorServiceLocatorNotAvailable_Routing =
    DAQmxErrorServiceLocatorNotAvailable_Routing,
  #[error("The specified remote target is not reachable. Ensure that the target is running.")]
  ErrorCouldNotConnectToServer_Routing =
    DAQmxErrorCouldNotConnectToServer_Routing,
  #[error("Specified device name is not valid, because it contains spaces or punctuation other than hyphens or underscores.")]
  ErrorDeviceNameContainsSpacesOrPunctuation_Routing =
    DAQmxErrorDeviceNameContainsSpacesOrPunctuation_Routing,
  #[error("Specified device name is not valid, because it contains non-printable characters.")]
  ErrorDeviceNameContainsNonprintableCharacters_Routing =
    DAQmxErrorDeviceNameContainsNonprintableCharacters_Routing,
  #[error(
    "Specified device name is not valid, because it is an empty string."
  )]
  ErrorDeviceNameIsEmpty_Routing = DAQmxErrorDeviceNameIsEmpty_Routing,
  #[error("No device by the given name was found.")]
  ErrorDeviceNameNotFound_Routing = DAQmxErrorDeviceNameNotFound_Routing,
  #[error("The requested operation failed because the versions of a driver installed on the local and the remote system are different. Update the driver so the versions on the two systems are the same.")]
  ErrorLocalRemoteDriverVersionMismatch_Routing =
    DAQmxErrorLocalRemoteDriverVersionMismatch_Routing,
  #[error("Device name specified conflicts with an existing device name.")]
  ErrorDuplicateDeviceName_Routing = DAQmxErrorDuplicateDeviceName_Routing,
  #[error("The specified operation cannot be performed because a task is in the process of being aborted or a device is in the process of being removed from the system. Wait until the abort operation is complete and attempt to perform the operation again.")]
  ErrorRuntimeAborting_Routing = DAQmxErrorRuntimeAborting_Routing,
  #[error("The specified operation cannot be performed because a task has been aborted or a device has been removed from the system. Handle this situation as required by the application and then, if appropriate, attempt to perform the operation again.")]
  ErrorRuntimeAborted_Routing = DAQmxErrorRuntimeAborted_Routing,
  #[error("The specified resource is not available. The operation could not be completed as specified.")]
  ErrorResourceNotInPool_Routing = DAQmxErrorResourceNotInPool_Routing,
  #[error("The specified device is not present or is not active in the system. The device may not be installed on this system, may have been unplugged, or may not be installed correctly.")]
  ErrorDriverDeviceGUIDNotFound_Routing =
    DAQmxErrorDriverDeviceGUIDNotFound_Routing,
  #[error("A USB transfer failed due to a transaction error reported by the USB host controller.  This may be due to a fault in the system's USB host controller, a USB cable, or a USB device.")]
  ErrorPALUSBTransactionError = DAQmxErrorPALUSBTransactionError,
  #[error("Applicable to a 1394 bus.  An isochronous buffer descriptor was not found or a link driver stream failed to be connected or disconnected.")]
  ErrorPALIsocStreamBufferError = DAQmxErrorPALIsocStreamBufferError,
  #[error("Applicable to TCP Connections.  Applicable to UDP/TCP Connections. An invalid address was used (i.e. sending to address 0). Note: The TCP stack prohibits connecting to the loopback address from a non-loopback address.")]
  ErrorPALInvalidAddressComponent = DAQmxErrorPALInvalidAddressComponent,
  #[error("Applicable to TCP Connections. This is a more specific version of kPALStatusConnectionRefused. The stream has already been opened at the same address and port.")]
  ErrorPALSharingViolation = DAQmxErrorPALSharingViolation,
  #[error("Applicable to TCP Connections.  The stream/device was setup incorrectly (i.e. invalid buffer size etc.) or the device is no longer in a good state.")]
  ErrorPALInvalidDeviceState = DAQmxErrorPALInvalidDeviceState,
  #[error("Applicable to TCP Connections.  The connection was aborted by the remote peer.")]
  ErrorPALConnectionReset = DAQmxErrorPALConnectionReset,
  #[error("Applicable to TCP Connections.  A surprise removal of the underlying device occurred and/or the connection was aborted.")]
  ErrorPALConnectionAborted = DAQmxErrorPALConnectionAborted,
  #[error("Unable to connect to remote host.  Please check that all network segments between this host and the remote host are working and that the remote host is ready to accept connections.")]
  ErrorPALConnectionRefused = DAQmxErrorPALConnectionRefused,
  #[error("Applicable to a 1394 bus.  The generation count on the packet submitted is incorrect and the entire transfer failed.")]
  ErrorPALBusResetOccurred = DAQmxErrorPALBusResetOccurred,
  #[error("The wait function was interrupted by an internal or external source and has returned.  The operation has not been completed as specified, and subsequent waits may also not complete.")]
  ErrorPALWaitInterrupted = DAQmxErrorPALWaitInterrupted,
  #[error("No messages could be read because the message queue is empty. The operation could not be completed as specified.")]
  ErrorPALMessageUnderflow = DAQmxErrorPALMessageUnderflow,
  #[error("The message could not be written because the message queue is full. The operation could not be completed as specified.")]
  ErrorPALMessageOverflow = DAQmxErrorPALMessageOverflow,
  #[error("The specified thread could not be joined because the thread has already exited or has been terminated. The operation could not be completed as specified.")]
  ErrorPALThreadAlreadyDead = DAQmxErrorPALThreadAlreadyDead,
  #[error("The specified stack size is not supported. The operation could not be completed as specified.")]
  ErrorPALThreadStackSizeNotSupported =
    DAQmxErrorPALThreadStackSizeNotSupported,
  #[error("The wrong instance of iThreadController was used while attempting to join to a thread. The operation could not be completed as specified.")]
  ErrorPALThreadControllerIsNotThreadCreator =
    DAQmxErrorPALThreadControllerIsNotThreadCreator,
  #[error("An internal thread data structure has been corrupted. The operation could not be completed as specified.")]
  ErrorPALThreadHasNoThreadObject = DAQmxErrorPALThreadHasNoThreadObject,
  #[error(
        "The specified thread is not runnable. The operation could not be completed as specified."
    )]
  ErrorPALThreadCouldNotRun = DAQmxErrorPALThreadCouldNotRun,
  #[error("The thread that acquired the synchronization object has exited or has been terminated. This error applies to mutex-like synchronization objects, where the thread that acquires the synchronization object owns it until the thread releases it. The resource that the synchronization object protects may be in an unknown state.")]
  ErrorPALSyncAbandoned = DAQmxErrorPALSyncAbandoned,
  #[error("A synchronization object was not acquired within the time limit. The operation could not be completed as specified.")]
  ErrorPALSyncTimedOut = DAQmxErrorPALSyncTimedOut,
  #[error("The Receiver Socket is invalid. The operation could not be completed as specified.")]
  ErrorPALReceiverSocketInvalid = DAQmxErrorPALReceiverSocketInvalid,
  #[error("The Socket Listener specified is invalid. The operation could not be completed as specified.")]
  ErrorPALSocketListenerInvalid = DAQmxErrorPALSocketListenerInvalid,
  #[error("The Socket Listener is already registered. The operation could not be completed as specified.")]
  ErrorPALSocketListenerAlreadyRegistered =
    DAQmxErrorPALSocketListenerAlreadyRegistered,
  #[error("The dispatch function has already been exported.")]
  ErrorPALDispatcherAlreadyExported = DAQmxErrorPALDispatcherAlreadyExported,
  #[error("A DMA link event was missed - probably because the interrupt was not serviced in time. The data being moved is incomplete as a result. The operation could not be completed as specified.")]
  ErrorPALDMALinkEventMissed = DAQmxErrorPALDMALinkEventMissed,
  #[error("A transfer was aborted because of a hardware error on the bus on which the transfer was occurring.")]
  ErrorPALBusError = DAQmxErrorPALBusError,
  #[error("A transfer was aborted because it could not be completed within the maximum number of retry attempts allowed by the bus on which the transfer was occurring.")]
  ErrorPALRetryLimitExceeded = DAQmxErrorPALRetryLimitExceeded,
  #[error("There was no more data in the buffer when new data was being read. The data returned by the operation is stale as a result.")]
  ErrorPALTransferOverread = DAQmxErrorPALTransferOverread,
  #[error("There was no more space in the buffer when new data was written. The oldest unread data in the buffer was lost as a result.")]
  ErrorPALTransferOverwritten = DAQmxErrorPALTransferOverwritten,
  #[error("A hardware buffer has overflowed.")]
  ErrorPALPhysicalBufferFull = DAQmxErrorPALPhysicalBufferFull,
  #[error("A hardware buffer has underflowed.")]
  ErrorPALPhysicalBufferEmpty = DAQmxErrorPALPhysicalBufferEmpty,
  #[error("There is no more space in the buffer.")]
  ErrorPALLogicalBufferFull = DAQmxErrorPALLogicalBufferFull,
  #[error("There is no more data in the buffer")]
  ErrorPALLogicalBufferEmpty = DAQmxErrorPALLogicalBufferEmpty,
  #[error("No transfer is in progress because the transfer was aborted by the client. The operation could not be completed as specified.")]
  ErrorPALTransferAborted = DAQmxErrorPALTransferAborted,
  #[error("No transfer is in progress because the transfer was halted by the system. The operation could not be completed as specified.")]
  ErrorPALTransferStopped = DAQmxErrorPALTransferStopped,
  #[error(
        "A transfer is already in progress. The operation could not be completed as specified."
    )]
  ErrorPALTransferInProgress = DAQmxErrorPALTransferInProgress,
  #[error("No transfer is in progress because no transfer was initiated. The operation could not be completed as specified.")]
  ErrorPALTransferNotInProgress = DAQmxErrorPALTransferNotInProgress,
  #[error("An unknown or unexpected communication fault occurred while attempting to read data from or write data to a device.")]
  ErrorPALCommunicationsFault = DAQmxErrorPALCommunicationsFault,
  #[error("The transfer did not complete within the timeout period or within the specified number of retries.")]
  ErrorPALTransferTimedOut = DAQmxErrorPALTransferTimedOut,
  #[error("The Memory Manager detected that one or more allocated chunks were not freed. The operation was completed but the state of the system may be unreliable.")]
  ErrorPALMemoryHeapNotEmpty = DAQmxErrorPALMemoryHeapNotEmpty,
  #[error("The Memory Manager detected corruption in its internal data structures. The state of the system may be unreliable.")]
  ErrorPALMemoryBlockCheckFailed = DAQmxErrorPALMemoryBlockCheckFailed,
  #[error("One or more memory pages in the specified logical buffer could not be locked into physical memory. The operation could not be completed as specified.")]
  ErrorPALMemoryPageLockFailed = DAQmxErrorPALMemoryPageLockFailed,
  #[error("The requested memory could not be allocated.")]
  ErrorPALMemoryFull = DAQmxErrorPALMemoryFull,
  #[error("The specified buffer is aligned to an address boundary that prevents moving data to or from the specified device. The operation could not be completed as specified.")]
  ErrorPALMemoryAlignmentFault = DAQmxErrorPALMemoryAlignmentFault,
  #[error("A memory configuration problem is interfering with a background transfer mechanism. This could mean that a buffer failed to be allocated or that a buffer was allocated but is not in the address range of the targeted bus master. The operation could not be completed as specified.")]
  ErrorPALMemoryConfigurationFault = DAQmxErrorPALMemoryConfigurationFault,
  #[error("The specified device could not be initialized. The operation could not be completed as specified.")]
  ErrorPALDeviceInitializationFault = DAQmxErrorPALDeviceInitializationFault,
  #[error("The specified device is managed by this software stack but a resource or some other sort of functionality associated with the device is not currently supported by this software stack. The operation could not be completed as specified.")]
  ErrorPALDeviceNotSupported = DAQmxErrorPALDeviceNotSupported,
  #[error("The specified device is not managed by this software stack. The operation could not be completed as specified.")]
  ErrorPALDeviceUnknown = DAQmxErrorPALDeviceUnknown,
  #[error("The specified device is not present or is deactivated. The operation could not be completed as specified.")]
  ErrorPALDeviceNotFound = DAQmxErrorPALDeviceNotFound,
  #[error("The specified feature is implemented but has been disabled in the current environment. The operation could not be completed as specified.")]
  ErrorPALFeatureDisabled = DAQmxErrorPALFeatureDisabled,
  #[error("The specified software component is busy or still in use. The operation could not be completed as specified.")]
  ErrorPALComponentBusy = DAQmxErrorPALComponentBusy,
  #[error("The specified software component has already been installed. The operation could not be completed as specified.")]
  ErrorPALComponentAlreadyInstalled = DAQmxErrorPALComponentAlreadyInstalled,
  #[error("The specified software component cannot be unloaded. The operation could not be completed as specified.")]
  ErrorPALComponentNotUnloadable = DAQmxErrorPALComponentNotUnloadable,
  #[error("The specified software component is not currently loaded. The operation could not be completed as specified.")]
  ErrorPALComponentNeverLoaded = DAQmxErrorPALComponentNeverLoaded,
  #[error("The specified software component has already been loaded. The operation could not be completed as specified.")]
  ErrorPALComponentAlreadyLoaded = DAQmxErrorPALComponentAlreadyLoaded,
  #[error("The specified software component has circular dependency. The operation could not be completed as specified.")]
  ErrorPALComponentCircularDependency =
    DAQmxErrorPALComponentCircularDependency,
  #[error("The specified software component failed to initialize. The operation could not be completed as specified.")]
  ErrorPALComponentInitializationFault =
    DAQmxErrorPALComponentInitializationFault,
  #[error("The specified software component could not be loaded because its image is corrupt. The operation could not be completed as specified.")]
  ErrorPALComponentImageCorrupt = DAQmxErrorPALComponentImageCorrupt,
  #[error("The specified feature has not been implemented for this environment. The operation could not be completed as specified.")]
  ErrorPALFeatureNotSupported = DAQmxErrorPALFeatureNotSupported,
  #[error("A function imported from another library could not be found. The operation could not be completed as specified.")]
  ErrorPALFunctionNotFound = DAQmxErrorPALFunctionNotFound,
  #[error("The specified function is no longer supported. The operation could not be completed as specified.")]
  ErrorPALFunctionObsolete = DAQmxErrorPALFunctionObsolete,
  #[error("The specified software component is not backward compatible with the client that was attempting to load it. The client was not loaded.")]
  ErrorPALComponentTooNew = DAQmxErrorPALComponentTooNew,
  #[error("The specified software component is not forward compatible with the client that was attempting to load it. The client was not loaded.")]
  ErrorPALComponentTooOld = DAQmxErrorPALComponentTooOld,
  #[error("The specified software component is not available. The component was not loaded.")]
  ErrorPALComponentNotFound = DAQmxErrorPALComponentNotFound,
  #[error("2 or more components of the package being loaded are unable to operate together due to a version mismatch. A failure of this sort is generally indicative of an installation problem. The package was not loaded.")]
  ErrorPALVersionMismatch = DAQmxErrorPALVersionMismatch,
  #[error("An undetermined failure occurred while operating on the specified file. The operation could not be completed as specified.")]
  ErrorPALFileFault = DAQmxErrorPALFileFault,
  #[error("A failure occurred while attempting to write a buffer to the specified file. The operation could not be completed as specified.")]
  ErrorPALFileWriteFault = DAQmxErrorPALFileWriteFault,
  #[error("A failure occurred while attempting to read a buffer from the specified file. The operation could not be completed as specified.")]
  ErrorPALFileReadFault = DAQmxErrorPALFileReadFault,
  #[error("The specified offset in the specified file could not be found. The operation could not be completed as specified.")]
  ErrorPALFileSeekFault = DAQmxErrorPALFileSeekFault,
  #[error("The specified file could not be closed. The operation could not be completed as specified.")]
  ErrorPALFileCloseFault = DAQmxErrorPALFileCloseFault,
  #[error("The specified file could not be opened. The operation could not be completed as specified.")]
  ErrorPALFileOpenFault = DAQmxErrorPALFileOpenFault,
  #[error("There is no more space on the targeted disk. The operation could not be completed as specified.")]
  ErrorPALDiskFull = DAQmxErrorPALDiskFull,
  #[error("A system call returned an error. The operation could not be completed as specified.")]
  ErrorPALOSFault = DAQmxErrorPALOSFault,
  #[error("A system call returned an error while NI Platform Services was launching. The NI Platform Services launch could not be completed.")]
  ErrorPALOSInitializationFault = DAQmxErrorPALOSInitializationFault,
  #[error("This function has not been implemented for this operating system. The operation could not be completed as specified.")]
  ErrorPALOSUnsupported = DAQmxErrorPALOSUnsupported,
  #[error("The specified mathematical operation results in an overflow.")]
  ErrorPALCalculationOverflow = DAQmxErrorPALCalculationOverflow,
  #[error("A hardware failure has occurred. The operation could not be completed as specified.")]
  ErrorPALHardwareFault = DAQmxErrorPALHardwareFault,
  #[error("The firmware - which is software resident on a device - has entered an unknown state usually as a result of a cascade failure induced by an unexpected series of state inputs. The operation could not be completed as specified, and you should immediately terminate all further transactions if you are able to do so.")]
  ErrorPALFirmwareFault = DAQmxErrorPALFirmwareFault,
  #[error("The software has entered an unknown state - usually as a result of a cascade failure induced by an unexpected series of state inputs. The operation could not be completed as specified and you should immediately terminate all further transactions if you are able to do so.")]
  ErrorPALSoftwareFault = DAQmxErrorPALSoftwareFault,
  #[error("The message-transport queue for the implied arena is full. The operation could not be completed as specified.")]
  ErrorPALMessageQueueFull = DAQmxErrorPALMessageQueueFull,
  #[error("The specified resource is ambiguous or a unique resource could not be determined based on the qualifying attributes. The operation could not be completed as specified.")]
  ErrorPALResourceAmbiguous = DAQmxErrorPALResourceAmbiguous,
  #[error("The resource is in use. The operation could not be completed as specified.")]
  ErrorPALResourceBusy = DAQmxErrorPALResourceBusy,
  #[error("The resource was already initialized and cannot be initialized again. The operation could not be completed as specified.")]
  ErrorPALResourceInitialized = DAQmxErrorPALResourceInitialized,
  #[error("The specified resource has not been initialized. The operation could not be completed as specified.")]
  ErrorPALResourceNotInitialized = DAQmxErrorPALResourceNotInitialized,
  #[error(
        "The specified resource is reserved. The operation could not be completed as specified."
    )]
  ErrorPALResourceReserved = DAQmxErrorPALResourceReserved,
  #[error("The specified resource is not reserved. The operation could not be completed as specified.")]
  ErrorPALResourceNotReserved = DAQmxErrorPALResourceNotReserved,
  #[error("The requested resource does not exist or exists but is in a state which prevents its reservation. The operation could not be completed as specified.")]
  ErrorPALResourceNotAvailable = DAQmxErrorPALResourceNotAvailable,
  #[error("The resource is controlled by the OS or a related system and should not be manipulated directly. The operation could not be completed as specified.")]
  ErrorPALResourceOwnedBySystem = DAQmxErrorPALResourceOwnedBySystem,
  #[error(
        "The specified token is not valid. The operation could not be completed as specified."
    )]
  ErrorPALBadToken = DAQmxErrorPALBadToken,
  #[error("The specified multitasking mode is invalid. The operation could not be completed as specified.")]
  ErrorPALBadThreadMultitask = DAQmxErrorPALBadThreadMultitask,
  #[error(
        "The library specifier is invalid. The operation could not be completed as specified."
    )]
  ErrorPALBadLibrarySpecifier = DAQmxErrorPALBadLibrarySpecifier,
  #[error("The specified address space is not a valid address space. The operation could not be completed as specified.")]
  ErrorPALBadAddressSpace = DAQmxErrorPALBadAddressSpace,
  #[error("The specified window type is not valid or is inappropriate in the context of the current device configuration. The operation could not be completed as specified.")]
  ErrorPALBadWindowType = DAQmxErrorPALBadWindowType,
  #[error("The specified address class is not valid in the context of the current device configuration. The operation could not be completed as specified.")]
  ErrorPALBadAddressClass = DAQmxErrorPALBadAddressClass,
  #[error("The specified count is too small or large or of the wrong granularity for write operations. In cases where an offset is also specified the maximum count may be derived from the offset. The operation could not be completed as specified.")]
  ErrorPALBadWriteCount = DAQmxErrorPALBadWriteCount,
  #[error("The specified offset is out of range for write operations. The operation could not be completed as specified.")]
  ErrorPALBadWriteOffset = DAQmxErrorPALBadWriteOffset,
  #[error("The specified mode is not supported for write operations. The operation could not be completed as specified.")]
  ErrorPALBadWriteMode = DAQmxErrorPALBadWriteMode,
  #[error("The specified count is too small or large or of the wrong granularity for read operations. In cases where an offset is also specified the maximum count may be derived from the offset. The operation could not be completed as specified.")]
  ErrorPALBadReadCount = DAQmxErrorPALBadReadCount,
  #[error("The specified offset is out of range for read operations. The operation could not be completed as specified.")]
  ErrorPALBadReadOffset = DAQmxErrorPALBadReadOffset,
  #[error("The specified mode is not supported for read operations. The operation could not be completed as specified.")]
  ErrorPALBadReadMode = DAQmxErrorPALBadReadMode,
  #[error("The specified count is too small or large or of the wrong granularity. In cases where an offset is also specified the maximum count may be derived from the offset. The operation could not be completed as specified.")]
  ErrorPALBadCount = DAQmxErrorPALBadCount,
  #[error(
        "The specified offset is out of range. The operation could not be completed as specified."
    )]
  ErrorPALBadOffset = DAQmxErrorPALBadOffset,
  #[error(
        "The specified mode is not supported. The operation could not be completed as specified."
    )]
  ErrorPALBadMode = DAQmxErrorPALBadMode,
  #[error("The specified data width is not supported or cannot be transferred on the bus. The operation could not be completed as specified.")]
  ErrorPALBadDataSize = DAQmxErrorPALBadDataSize,
  #[error("A pointer-type parameter is invalid. This may mean that the pointer is NULL when it should not be or not NULL when it should be or is not NULL but does not appear to point to the appropriate type of object or data. The operation could not be completed as specified.")]
  ErrorPALBadPointer = DAQmxErrorPALBadPointer,
  #[error("A selector - usually of an enumerated type - is inappropriate or out of range. The operation could not be completed as specified.")]
  ErrorPALBadSelector = DAQmxErrorPALBadSelector,
  #[error("The specified device is not a valid device. The operation could not be completed as specified.")]
  ErrorPALBadDevice = DAQmxErrorPALBadDevice,
  #[error("An attribute whether explicit or implicit is not relevant or is not relevant given the current system state. The operation could not be completed as specified.")]
  ErrorPALIrrelevantAttribute = DAQmxErrorPALIrrelevantAttribute,
  #[error("The value of one of the specified parameters conflicts with the state or the value of another parameter. The operation could not be completed as specified.")]
  ErrorPALValueConflict = DAQmxErrorPALValueConflict,
}
