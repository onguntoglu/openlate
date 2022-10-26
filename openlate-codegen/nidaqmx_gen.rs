pub enum StrainGageRosetteMeasurementType {
    #[doc=" The maximum tensile strain coplanar to the surface of the material under stress."]
    PrincipalStrain1,
    #[doc=" The minimum tensile strain coplanar to the surface of the material under stress."]
    PrincipalStrain2,
    #[doc=" The angle at which the principal strains of the rosette occur."]
    PrincipalStrainAngle,
    #[doc=" The tensile strain coplanar to the surface of the material under stress in the  X coordinate direction."]
    CartesianStrainX,
    #[doc=" The tensile strain coplanar to the surface of the material under stress in the  Y coordinate direction."]
    CartesianStrainY,
    #[doc=" The tensile strain coplanar to the surface of the material under stress in the  XY coordinate direction."]
    CartesianShearStrainXy,
    #[doc=" The maximum strain coplanar to the cross section of the material under stress."]
    MaxShearStrain,
    #[doc=" The angle at which the maximum shear strain of the rosette occurs."]
    MaxShearStrainAngle,
}

impl From<i32> for StrainGageRosetteMeasurementType {
    fn from(val: i32) -> StrainGageRosetteMeasurementType {
        match val {
            15971_i32 => StrainGageRosetteMeasurementType::PrincipalStrain1,
            15972_i32 => StrainGageRosetteMeasurementType::PrincipalStrain2,
            15973_i32 => StrainGageRosetteMeasurementType::PrincipalStrainAngle,
            15974_i32 => StrainGageRosetteMeasurementType::CartesianStrainX,
            15975_i32 => StrainGageRosetteMeasurementType::CartesianStrainY,
            15976_i32 => StrainGageRosetteMeasurementType::CartesianShearStrainXy,
            15977_i32 => StrainGageRosetteMeasurementType::MaxShearStrain,
            15978_i32 => StrainGageRosetteMeasurementType::MaxShearStrainAngle,
        }
    }
}

impl From<StrainGageRosetteMeasurementType> for i32 {
    fn from(val: StrainGageRosetteMeasurementType) -> i32 {
        match val {
            StrainGageRosetteMeasurementType::PrincipalStrain1 => 15971_i32,
            StrainGageRosetteMeasurementType::PrincipalStrain2 => 15972_i32,
            StrainGageRosetteMeasurementType::PrincipalStrainAngle => 15973_i32,
            StrainGageRosetteMeasurementType::CartesianStrainX => 15974_i32,
            StrainGageRosetteMeasurementType::CartesianStrainY => 15975_i32,
            StrainGageRosetteMeasurementType::CartesianShearStrainXy => 15976_i32,
            StrainGageRosetteMeasurementType::MaxShearStrain => 15977_i32,
            StrainGageRosetteMeasurementType::MaxShearStrainAngle => 15978_i32,
        }
    }
}

pub enum DataJustification1 {
    #[doc="Samples occupy the lower bits of the integer."]
    RightJustified,
    #[doc="Samples occupy the higher bits of the integer."]
    LeftJustified,
}

impl From<i32> for DataJustification1 {
    fn from(val: i32) -> DataJustification1 {
        match val {
            10279_i32 => DataJustification1::RightJustified,
            10209_i32 => DataJustification1::LeftJustified,
        }
    }
}

impl From<DataJustification1> for i32 {
    fn from(val: DataJustification1) -> i32 {
        match val {
            DataJustification1::RightJustified => 10279_i32,
            DataJustification1::LeftJustified => 10209_i32,
        }
    }
}

pub enum Coupling2 {
    #[doc="Alternating Current."]
    Ac,
    #[doc="Direct Current."]
    Dc,
}

impl From<i32> for Coupling2 {
    fn from(val: i32) -> Coupling2 {
        match val {
            10045_i32 => Coupling2::Ac,
            10050_i32 => Coupling2::Dc,
        }
    }
}

impl From<Coupling2> for i32 {
    fn from(val: Coupling2) -> i32 {
        match val {
            Coupling2::Ac => 10045_i32,
            Coupling2::Dc => 10050_i32,
        }
    }
}

pub enum TriggerType5 {
    #[doc=" Advance to the next entry in a scan list on the rising or falling edge of a  digital signal."]
    DigEdge,
    #[doc=" Advance to the next entry in a scan list when you call  DAQmxSendSoftwareTrigger()."]
    Software,
    #[doc=" Advance through all entries in the scan list as fast as possible."]
    None,
}

impl From<i32> for TriggerType5 {
    fn from(val: i32) -> TriggerType5 {
        match val {
            10150_i32 => TriggerType5::DigEdge,
            10292_i32 => TriggerType5::Software,
            10230_i32 => TriggerType5::None,
        }
    }
}

impl From<TriggerType5> for i32 {
    fn from(val: TriggerType5) -> i32 {
        match val {
            TriggerType5::DigEdge => 10150_i32,
            TriggerType5::Software => 10292_i32,
            TriggerType5::None => 10230_i32,
        }
    }
}

pub enum WindowTriggerCondition1 {
    #[doc="Trigger when the signal enters the window."]
    EnteringWin,
    #[doc="Trigger when the signal leaves the window."]
    LeavingWin,
}

impl From<i32> for WindowTriggerCondition1 {
    fn from(val: i32) -> WindowTriggerCondition1 {
        match val {
            10163_i32 => WindowTriggerCondition1::EnteringWin,
            10208_i32 => WindowTriggerCondition1::LeavingWin,
        }
    }
}

impl From<WindowTriggerCondition1> for i32 {
    fn from(val: WindowTriggerCondition1) -> i32 {
        match val {
            WindowTriggerCondition1::EnteringWin => 10163_i32,
            WindowTriggerCondition1::LeavingWin => 10208_i32,
        }
    }
}

pub enum ProductCategory {
    #[doc="M Series DAQ."]
    MSeriesDaq,
    #[doc="X Series DAQ."]
    XSeriesDaq,
    #[doc="E Series DAQ."]
    ESeriesDaq,
    #[doc="S Series DAQ."]
    SSeriesDaq,
    #[doc="B Series DAQ."]
    BSeriesDaq,
    #[doc="SC Series DAQ."]
    ScSeriesDaq,
    #[doc="USB DAQ."]
    Usbdaq,
    #[doc="AO Series."]
    AoSeries,
    #[doc="Digital I/O."]
    DigitalIo,
    #[doc="TIO Series."]
    TioSeries,
    #[doc="Dynamic Signal Acquisition."]
    DynamicSignalAcquisition,
    #[doc="Switches."]
    Switches,
    #[doc="CompactDAQ chassis."]
    CompactDaqChassis,
    #[doc="CompactRIO Chassis."]
    CompactRioChassis,
    #[doc="C Series I/O module."]
    CSeriesModule,
    #[doc="SCXI module."]
    ScxiModule,
    #[doc="SCC Connector Block."]
    SccConnectorBlock,
    #[doc="SCC Module."]
    SccModule,
    #[doc="NI ELVIS."]
    Nielvis,
    #[doc="Network DAQ."]
    NetworkDaq,
    #[doc="SC Express."]
    ScExpress,
    #[doc="FieldDAQ."]
    FieldDaq,
    #[doc="Unknown category."]
    Unknown,
}

impl From<i32> for ProductCategory {
    fn from(val: i32) -> ProductCategory {
        match val {
            14643_i32 => ProductCategory::MSeriesDaq,
            15858_i32 => ProductCategory::XSeriesDaq,
            14642_i32 => ProductCategory::ESeriesDaq,
            14644_i32 => ProductCategory::SSeriesDaq,
            14662_i32 => ProductCategory::BSeriesDaq,
            14645_i32 => ProductCategory::ScSeriesDaq,
            14646_i32 => ProductCategory::Usbdaq,
            14647_i32 => ProductCategory::AoSeries,
            14648_i32 => ProductCategory::DigitalIo,
            14661_i32 => ProductCategory::TioSeries,
            14649_i32 => ProductCategory::DynamicSignalAcquisition,
            14650_i32 => ProductCategory::Switches,
            14658_i32 => ProductCategory::CompactDaqChassis,
            16144_i32 => ProductCategory::CompactRioChassis,
            14659_i32 => ProductCategory::CSeriesModule,
            14660_i32 => ProductCategory::ScxiModule,
            14704_i32 => ProductCategory::SccConnectorBlock,
            14705_i32 => ProductCategory::SccModule,
            14755_i32 => ProductCategory::Nielvis,
            14829_i32 => ProductCategory::NetworkDaq,
            15886_i32 => ProductCategory::ScExpress,
            16151_i32 => ProductCategory::FieldDaq,
            12588_i32 => ProductCategory::Unknown,
        }
    }
}

impl From<ProductCategory> for i32 {
    fn from(val: ProductCategory) -> i32 {
        match val {
            ProductCategory::MSeriesDaq => 14643_i32,
            ProductCategory::XSeriesDaq => 15858_i32,
            ProductCategory::ESeriesDaq => 14642_i32,
            ProductCategory::SSeriesDaq => 14644_i32,
            ProductCategory::BSeriesDaq => 14662_i32,
            ProductCategory::ScSeriesDaq => 14645_i32,
            ProductCategory::Usbdaq => 14646_i32,
            ProductCategory::AoSeries => 14647_i32,
            ProductCategory::DigitalIo => 14648_i32,
            ProductCategory::TioSeries => 14661_i32,
            ProductCategory::DynamicSignalAcquisition => 14649_i32,
            ProductCategory::Switches => 14650_i32,
            ProductCategory::CompactDaqChassis => 14658_i32,
            ProductCategory::CompactRioChassis => 16144_i32,
            ProductCategory::CSeriesModule => 14659_i32,
            ProductCategory::ScxiModule => 14660_i32,
            ProductCategory::SccConnectorBlock => 14704_i32,
            ProductCategory::SccModule => 14705_i32,
            ProductCategory::Nielvis => 14755_i32,
            ProductCategory::NetworkDaq => 14829_i32,
            ProductCategory::ScExpress => 15886_i32,
            ProductCategory::FieldDaq => 16151_i32,
            ProductCategory::Unknown => 12588_i32,
        }
    }
}

pub enum AngularVelocityUnits {
    #[doc="Revolutions per minute."]
    Rpm,
    #[doc="Radians per second."]
    RadiansPerSecond,
    #[doc="Degrees per second."]
    DegreesPerSecond,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for AngularVelocityUnits {
    fn from(val: i32) -> AngularVelocityUnits {
        match val {
            16080_i32 => AngularVelocityUnits::Rpm,
            16081_i32 => AngularVelocityUnits::RadiansPerSecond,
            16082_i32 => AngularVelocityUnits::DegreesPerSecond,
            10065_i32 => AngularVelocityUnits::FromCustomScale,
        }
    }
}

impl From<AngularVelocityUnits> for i32 {
    fn from(val: AngularVelocityUnits) -> i32 {
        match val {
            AngularVelocityUnits::Rpm => 16080_i32,
            AngularVelocityUnits::RadiansPerSecond => 16081_i32,
            AngularVelocityUnits::DegreesPerSecond => 16082_i32,
            AngularVelocityUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum SourceSelection {
    #[doc="Internal to the device."]
    Internal,
    #[doc="External to the device."]
    External,
}

impl From<i32> for SourceSelection {
    fn from(val: i32) -> SourceSelection {
        match val {
            10200_i32 => SourceSelection::Internal,
            10167_i32 => SourceSelection::External,
        }
    }
}

impl From<SourceSelection> for i32 {
    fn from(val: SourceSelection) -> i32 {
        match val {
            SourceSelection::Internal => 10200_i32,
            SourceSelection::External => 10167_i32,
        }
    }
}

pub enum InputTermCfgWithDefault {
    CfgDefault,
    Rse,
    Nrse,
    Diff,
    PseudoDiff,
}

impl From<i32> for InputTermCfgWithDefault {
    fn from(val: i32) -> InputTermCfgWithDefault {
        match val {
            -1_i32 => InputTermCfgWithDefault::CfgDefault,
            10083_i32 => InputTermCfgWithDefault::Rse,
            10078_i32 => InputTermCfgWithDefault::Nrse,
            10106_i32 => InputTermCfgWithDefault::Diff,
            12529_i32 => InputTermCfgWithDefault::PseudoDiff,
        }
    }
}

impl From<InputTermCfgWithDefault> for i32 {
    fn from(val: InputTermCfgWithDefault) -> i32 {
        match val {
            InputTermCfgWithDefault::CfgDefault => -1_i32,
            InputTermCfgWithDefault::Rse => 10083_i32,
            InputTermCfgWithDefault::Nrse => 10078_i32,
            InputTermCfgWithDefault::Diff => 10106_i32,
            InputTermCfgWithDefault::PseudoDiff => 12529_i32,
        }
    }
}

pub enum AutoZeroType1 {
    #[doc="Do not perform an autozero."]
    None,
    #[doc=" Perform an auto zero at the beginning of the acquisition. This auto zero task  might not run if you have used DAQmx Control Task previously in your task."]
    Once,
    #[doc="Perform an auto zero at every sample of the acquisition."]
    EverySample,
}

impl From<i32> for AutoZeroType1 {
    fn from(val: i32) -> AutoZeroType1 {
        match val {
            10230_i32 => AutoZeroType1::None,
            10244_i32 => AutoZeroType1::Once,
            10164_i32 => AutoZeroType1::EverySample,
        }
    }
}

impl From<AutoZeroType1> for i32 {
    fn from(val: AutoZeroType1) -> i32 {
        match val {
            AutoZeroType1::None => 10230_i32,
            AutoZeroType1::Once => 10244_i32,
            AutoZeroType1::EverySample => 10164_i32,
        }
    }
}

pub enum SoundPressureUnits1 {
    #[doc="Pascals."]
    Pascals,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for SoundPressureUnits1 {
    fn from(val: i32) -> SoundPressureUnits1 {
        match val {
            10081_i32 => SoundPressureUnits1::Pascals,
            10065_i32 => SoundPressureUnits1::FromCustomScale,
        }
    }
}

impl From<SoundPressureUnits1> for i32 {
    fn from(val: SoundPressureUnits1) -> i32 {
        match val {
            SoundPressureUnits1::Pascals => 10081_i32,
            SoundPressureUnits1::FromCustomScale => 10065_i32,
        }
    }
}

pub enum ExportActions3 {
    #[doc=" The exported Sample Clock pulses at the beginning of each sample."]
    Pulse,
    #[doc=" The exported Sample Clock goes high at the beginning of the sample and goes low  when the last AI Convert begins."]
    Lvl,
}

impl From<i32> for ExportActions3 {
    fn from(val: i32) -> ExportActions3 {
        match val {
            10265_i32 => ExportActions3::Pulse,
            10210_i32 => ExportActions3::Lvl,
        }
    }
}

impl From<ExportActions3> for i32 {
    fn from(val: ExportActions3) -> i32 {
        match val {
            ExportActions3::Pulse => 10265_i32,
            ExportActions3::Lvl => 10210_i32,
        }
    }
}

pub enum OverflowBehavior {
    #[doc="Stop task and return an error."]
    StopTaskAndError,
    #[doc=" NI-DAQmx ignores Sample Clock overruns, and the task continues to run."]
    IgnoreOverruns,
}

impl From<i32> for OverflowBehavior {
    fn from(val: i32) -> OverflowBehavior {
        match val {
            15862_i32 => OverflowBehavior::StopTaskAndError,
            15863_i32 => OverflowBehavior::IgnoreOverruns,
        }
    }
}

impl From<OverflowBehavior> for i32 {
    fn from(val: OverflowBehavior) -> i32 {
        match val {
            OverflowBehavior::StopTaskAndError => 15862_i32,
            OverflowBehavior::IgnoreOverruns => 15863_i32,
        }
    }
}

pub enum Timescale2 {
    #[doc="Use the host device."]
    HostTime,
    #[doc="Use the I/O device."]
    IoDeviceTime,
}

impl From<i32> for Timescale2 {
    fn from(val: i32) -> Timescale2 {
        match val {
            16126_i32 => Timescale2::HostTime,
            16127_i32 => Timescale2::IoDeviceTime,
        }
    }
}

impl From<Timescale2> for i32 {
    fn from(val: Timescale2) -> i32 {
        match val {
            Timescale2::HostTime => 16126_i32,
            Timescale2::IoDeviceTime => 16127_i32,
        }
    }
}

pub enum InputTermCfg {
    #[doc="Referenced Single-Ended."]
    Rse,
    #[doc="Non-Referenced Single-Ended."]
    Nrse,
    #[doc="Differential."]
    Diff,
    #[doc="Pseudodifferential."]
    PseudoDiff,
}

impl From<i32> for InputTermCfg {
    fn from(val: i32) -> InputTermCfg {
        match val {
            10083_i32 => InputTermCfg::Rse,
            10078_i32 => InputTermCfg::Nrse,
            10106_i32 => InputTermCfg::Diff,
            12529_i32 => InputTermCfg::PseudoDiff,
        }
    }
}

impl From<InputTermCfg> for i32 {
    fn from(val: InputTermCfg) -> i32 {
        match val {
            InputTermCfg::Rse => 10083_i32,
            InputTermCfg::Nrse => 10078_i32,
            InputTermCfg::Diff => 10106_i32,
            InputTermCfg::PseudoDiff => 12529_i32,
        }
    }
}

pub enum ShuntCalSelect {
    #[doc="Switch A."]
    A,
    #[doc="Switch B."]
    B,
    #[doc="Switches A and B."]
    AandB,
}

impl From<i32> for ShuntCalSelect {
    fn from(val: i32) -> ShuntCalSelect {
        match val {
            12513_i32 => ShuntCalSelect::A,
            12514_i32 => ShuntCalSelect::B,
            12515_i32 => ShuntCalSelect::AandB,
        }
    }
}

impl From<ShuntCalSelect> for i32 {
    fn from(val: ShuntCalSelect) -> i32 {
        match val {
            ShuntCalSelect::A => 12513_i32,
            ShuntCalSelect::B => 12514_i32,
            ShuntCalSelect::AandB => 12515_i32,
        }
    }
}

pub enum FrequencyUnits3 {
    #[doc="Hertz."]
    Hz,
    #[doc="Timebase ticks."]
    Ticks,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for FrequencyUnits3 {
    fn from(val: i32) -> FrequencyUnits3 {
        match val {
            10373_i32 => FrequencyUnits3::Hz,
            10304_i32 => FrequencyUnits3::Ticks,
            10065_i32 => FrequencyUnits3::FromCustomScale,
        }
    }
}

impl From<FrequencyUnits3> for i32 {
    fn from(val: FrequencyUnits3) -> i32 {
        match val {
            FrequencyUnits3::Hz => 10373_i32,
            FrequencyUnits3::Ticks => 10304_i32,
            FrequencyUnits3::FromCustomScale => 10065_i32,
        }
    }
}

pub enum RTDType1 {
    #[doc="Pt3750."]
    Pt3750,
    #[doc="Pt3851."]
    Pt3851,
    #[doc="Pt3911."]
    Pt3911,
    #[doc="Pt3916."]
    Pt3916,
    #[doc="Pt3920."]
    Pt3920,
    #[doc="Pt3928."]
    Pt3928,
    #[doc=" You must use DAQmx_AI_RTD_A, DAQmx_AI_RTD_B, and DAQmx_AI_RTD_C to supply the  coefficients for the Callendar-Van Dusen equation."]
    Custom,
}

impl From<i32> for RTDType1 {
    fn from(val: i32) -> RTDType1 {
        match val {
            12481_i32 => RTDType1::Pt3750,
            10071_i32 => RTDType1::Pt3851,
            12482_i32 => RTDType1::Pt3911,
            10069_i32 => RTDType1::Pt3916,
            10053_i32 => RTDType1::Pt3920,
            12483_i32 => RTDType1::Pt3928,
            10137_i32 => RTDType1::Custom,
        }
    }
}

impl From<RTDType1> for i32 {
    fn from(val: RTDType1) -> i32 {
        match val {
            RTDType1::Pt3750 => 12481_i32,
            RTDType1::Pt3851 => 10071_i32,
            RTDType1::Pt3911 => 12482_i32,
            RTDType1::Pt3916 => 10069_i32,
            RTDType1::Pt3920 => 10053_i32,
            RTDType1::Pt3928 => 12483_i32,
            RTDType1::Custom => 10137_i32,
        }
    }
}

pub enum InvertPolarity {
    DoNotInvertPolarity,
    InvertPolarity,
}

impl From<i32> for InvertPolarity {
    fn from(val: i32) -> InvertPolarity {
        match val {
            0_i32 => InvertPolarity::DoNotInvertPolarity,
            1_i32 => InvertPolarity::InvertPolarity,
        }
    }
}

impl From<InvertPolarity> for i32 {
    fn from(val: InvertPolarity) -> i32 {
        match val {
            InvertPolarity::DoNotInvertPolarity => 0_i32,
            InvertPolarity::InvertPolarity => 1_i32,
        }
    }
}

pub enum OutputTermCfg {
    #[doc="Referenced Single-Ended."]
    Rse,
    #[doc="Differential."]
    Diff,
    #[doc="Pseudodifferential."]
    PseudoDiff,
}

impl From<i32> for OutputTermCfg {
    fn from(val: i32) -> OutputTermCfg {
        match val {
            10083_i32 => OutputTermCfg::Rse,
            10106_i32 => OutputTermCfg::Diff,
            12529_i32 => OutputTermCfg::PseudoDiff,
        }
    }
}

impl From<OutputTermCfg> for i32 {
    fn from(val: OutputTermCfg) -> i32 {
        match val {
            OutputTermCfg::Rse => 10083_i32,
            OutputTermCfg::Diff => 10106_i32,
            OutputTermCfg::PseudoDiff => 12529_i32,
        }
    }
}

pub enum WaitMode3 {
    #[doc=" Check for Sample Clock pulses when the system receives an interrupt service  request. This mode is the most CPU efficient, but results in lower possible  sampling rates."]
    WaitForInterrupt,
    #[doc=" Repeatedly check for Sample Clock pulses as fast as possible. This mode allows  for the highest sampling rates at the expense of CPU efficiency."]
    Poll,
}

impl From<i32> for WaitMode3 {
    fn from(val: i32) -> WaitMode3 {
        match val {
            12523_i32 => WaitMode3::WaitForInterrupt,
            12524_i32 => WaitMode3::Poll,
        }
    }
}

impl From<WaitMode3> for i32 {
    fn from(val: WaitMode3) -> i32 {
        match val {
            WaitMode3::WaitForInterrupt => 12523_i32,
            WaitMode3::Poll => 12524_i32,
        }
    }
}

pub enum VelocityUnits {
    #[doc="Meters per second."]
    MetersPerSecond,
    #[doc="Inches per second."]
    InchesPerSecond,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for VelocityUnits {
    fn from(val: i32) -> VelocityUnits {
        match val {
            15959_i32 => VelocityUnits::MetersPerSecond,
            15960_i32 => VelocityUnits::InchesPerSecond,
            10065_i32 => VelocityUnits::FromCustomScale,
        }
    }
}

impl From<VelocityUnits> for i32 {
    fn from(val: VelocityUnits) -> i32 {
        match val {
            VelocityUnits::MetersPerSecond => 15959_i32,
            VelocityUnits::InchesPerSecond => 15960_i32,
            VelocityUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum TimeUnits2 {
    #[doc="Seconds."]
    Seconds,
}

impl From<i32> for TimeUnits2 {
    fn from(val: i32) -> TimeUnits2 {
        match val {
            10364_i32 => TimeUnits2::Seconds,
        }
    }
}

impl From<TimeUnits2> for i32 {
    fn from(val: TimeUnits2) -> i32 {
        match val {
            TimeUnits2::Seconds => 10364_i32,
        }
    }
}

pub enum WriteBasicTEDSOptions {
    WriteToEeprom,
    WriteToProm,
    DoNotWrite,
}

impl From<i32> for WriteBasicTEDSOptions {
    fn from(val: i32) -> WriteBasicTEDSOptions {
        match val {
            12538_i32 => WriteBasicTEDSOptions::WriteToEeprom,
            12539_i32 => WriteBasicTEDSOptions::WriteToProm,
            12540_i32 => WriteBasicTEDSOptions::DoNotWrite,
        }
    }
}

impl From<WriteBasicTEDSOptions> for i32 {
    fn from(val: WriteBasicTEDSOptions) -> i32 {
        match val {
            WriteBasicTEDSOptions::WriteToEeprom => 12538_i32,
            WriteBasicTEDSOptions::WriteToProm => 12539_i32,
            WriteBasicTEDSOptions::DoNotWrite => 12540_i32,
        }
    }
}

pub enum ForceUnits {
    #[doc="Newtons."]
    Newtons,
    #[doc="Pounds."]
    Pounds,
    #[doc="Kilograms-force."]
    KilogramForce,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for ForceUnits {
    fn from(val: i32) -> ForceUnits {
        match val {
            15875_i32 => ForceUnits::Newtons,
            15876_i32 => ForceUnits::Pounds,
            15877_i32 => ForceUnits::KilogramForce,
            10065_i32 => ForceUnits::FromCustomScale,
        }
    }
}

impl From<ForceUnits> for i32 {
    fn from(val: ForceUnits) -> i32 {
        match val {
            ForceUnits::Newtons => 15875_i32,
            ForceUnits::Pounds => 15876_i32,
            ForceUnits::KilogramForce => 15877_i32,
            ForceUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum WatchdogCOExpirState {
    #[doc="Low logic."]
    Low,
    #[doc="High logic."]
    High,
    #[doc=" Expiration does not affect the state of the counter output. The channels retain  their states at the time of the watchdog timer expiration, and no further  counter generation runs."]
    NoChange,
}

impl From<i32> for WatchdogCOExpirState {
    fn from(val: i32) -> WatchdogCOExpirState {
        match val {
            10214_i32 => WatchdogCOExpirState::Low,
            10192_i32 => WatchdogCOExpirState::High,
            10160_i32 => WatchdogCOExpirState::NoChange,
        }
    }
}

impl From<WatchdogCOExpirState> for i32 {
    fn from(val: WatchdogCOExpirState) -> i32 {
        match val {
            WatchdogCOExpirState::Low => 10214_i32,
            WatchdogCOExpirState::High => 10192_i32,
            WatchdogCOExpirState::NoChange => 10160_i32,
        }
    }
}

pub enum DigitalDriveType {
    #[doc=" Drive the output pin to approximately 0 V for logic low and +3.3 V or +5 V,  depending on the device, for logic high."]
    ActiveDrive,
    #[doc=" Drive the output pin to 0 V for logic low. For logic high, the output driver  assumes a high-impedance state and does not drive a voltage."]
    OpenCollector,
}

impl From<i32> for DigitalDriveType {
    fn from(val: i32) -> DigitalDriveType {
        match val {
            12573_i32 => DigitalDriveType::ActiveDrive,
            12574_i32 => DigitalDriveType::OpenCollector,
        }
    }
}

impl From<DigitalDriveType> for i32 {
    fn from(val: DigitalDriveType) -> i32 {
        match val {
            DigitalDriveType::ActiveDrive => 12573_i32,
            DigitalDriveType::OpenCollector => 12574_i32,
        }
    }
}

pub enum LineGrouping {
    ChanPerLine,
    ChanForAllLines,
}

impl From<i32> for LineGrouping {
    fn from(val: i32) -> LineGrouping {
        match val {
            0_i32 => LineGrouping::ChanPerLine,
            1_i32 => LineGrouping::ChanForAllLines,
        }
    }
}

impl From<LineGrouping> for i32 {
    fn from(val: LineGrouping) -> i32 {
        match val {
            LineGrouping::ChanPerLine => 0_i32,
            LineGrouping::ChanForAllLines => 1_i32,
        }
    }
}

pub enum AngleUnits2 {
    #[doc="Degrees."]
    Degrees,
    #[doc="Radians."]
    Radians,
    #[doc="Ticks."]
    Ticks,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for AngleUnits2 {
    fn from(val: i32) -> AngleUnits2 {
        match val {
            10146_i32 => AngleUnits2::Degrees,
            10273_i32 => AngleUnits2::Radians,
            10304_i32 => AngleUnits2::Ticks,
            10065_i32 => AngleUnits2::FromCustomScale,
        }
    }
}

impl From<AngleUnits2> for i32 {
    fn from(val: AngleUnits2) -> i32 {
        match val {
            AngleUnits2::Degrees => 10146_i32,
            AngleUnits2::Radians => 10273_i32,
            AngleUnits2::Ticks => 10304_i32,
            AngleUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum EncoderZIndexPhase1 {
    #[doc="Reset the measurement when signal A and signal B are high."]
    AHighBHigh,
    #[doc=" Reset the measurement when signal A is high and signal B is low."]
    AHighBLow,
    #[doc=" Reset the measurement when signal A is low and signal B high."]
    ALowBHigh,
    #[doc="Reset the measurement when signal A and signal B are low."]
    ALowBLow,
}

impl From<i32> for EncoderZIndexPhase1 {
    fn from(val: i32) -> EncoderZIndexPhase1 {
        match val {
            10040_i32 => EncoderZIndexPhase1::AHighBHigh,
            10041_i32 => EncoderZIndexPhase1::AHighBLow,
            10042_i32 => EncoderZIndexPhase1::ALowBHigh,
            10043_i32 => EncoderZIndexPhase1::ALowBLow,
        }
    }
}

impl From<EncoderZIndexPhase1> for i32 {
    fn from(val: EncoderZIndexPhase1) -> i32 {
        match val {
            EncoderZIndexPhase1::AHighBHigh => 10040_i32,
            EncoderZIndexPhase1::AHighBLow => 10041_i32,
            EncoderZIndexPhase1::ALowBHigh => 10042_i32,
            EncoderZIndexPhase1::ALowBLow => 10043_i32,
        }
    }
}

pub enum Signal2 {
    #[doc=" Timed Loop executes each time the Sample Complete Event occurs."]
    SampleCompleteEvent,
    #[doc=" Timed Loop executes each time the Counter Output Event occurs."]
    CounterOutputEvent,
    #[doc=" Timed Loop executes each time the Change Detection Event occurs."]
    ChangeDetectionEvent,
    #[doc="Timed Loop executes on each active edge of the Sample Clock."]
    SampleClock,
}

impl From<i32> for Signal2 {
    fn from(val: i32) -> Signal2 {
        match val {
            12530_i32 => Signal2::SampleCompleteEvent,
            12494_i32 => Signal2::CounterOutputEvent,
            12511_i32 => Signal2::ChangeDetectionEvent,
            12487_i32 => Signal2::SampleClock,
        }
    }
}

impl From<Signal2> for i32 {
    fn from(val: Signal2) -> i32 {
        match val {
            Signal2::SampleCompleteEvent => 12530_i32,
            Signal2::CounterOutputEvent => 12494_i32,
            Signal2::ChangeDetectionEvent => 12511_i32,
            Signal2::SampleClock => 12487_i32,
        }
    }
}

pub enum DigitalLineState {
    #[doc="Logic high."]
    High,
    #[doc="Logic low."]
    Low,
    #[doc=" High-impedance state. You can select this state only on devices with  bidirectional lines.  You cannot select this state for dedicated digital output  lines. On some devices, you can select this value only for entire ports."]
    Tristate,
    #[doc=" Do not change the state of the lines. On some devices, you can select this  value only for entire ports."]
    NoChange,
}

impl From<i32> for DigitalLineState {
    fn from(val: i32) -> DigitalLineState {
        match val {
            10192_i32 => DigitalLineState::High,
            10214_i32 => DigitalLineState::Low,
            10310_i32 => DigitalLineState::Tristate,
            10160_i32 => DigitalLineState::NoChange,
        }
    }
}

impl From<DigitalLineState> for i32 {
    fn from(val: DigitalLineState) -> i32 {
        match val {
            DigitalLineState::High => 10192_i32,
            DigitalLineState::Low => 10214_i32,
            DigitalLineState::Tristate => 10310_i32,
            DigitalLineState::NoChange => 10160_i32,
        }
    }
}

pub enum ResistanceConfiguration {
    #[doc="2-wire mode."]
    D2Wire,
    #[doc="3-wire mode."]
    D3Wire,
    #[doc="4-wire mode."]
    D4Wire,
}

impl From<i32> for ResistanceConfiguration {
    fn from(val: i32) -> ResistanceConfiguration {
        match val {
            2_i32 => ResistanceConfiguration::D2Wire,
            3_i32 => ResistanceConfiguration::D3Wire,
            4_i32 => ResistanceConfiguration::D4Wire,
        }
    }
}

impl From<ResistanceConfiguration> for i32 {
    fn from(val: ResistanceConfiguration) -> i32 {
        match val {
            ResistanceConfiguration::D2Wire => 2_i32,
            ResistanceConfiguration::D3Wire => 3_i32,
            ResistanceConfiguration::D4Wire => 4_i32,
        }
    }
}

pub enum VelocityIEPESensorSensitivityUnits {
    #[doc="Millivolts per millimeter per second."]
    MillivoltsPerMillimeterPerSecond,
    #[doc="Millivolts per inch per second."]
    MilliVoltsPerInchPerSecond,
}

impl From<i32> for VelocityIEPESensorSensitivityUnits {
    fn from(val: i32) -> VelocityIEPESensorSensitivityUnits {
        match val {
            15963_i32 => VelocityIEPESensorSensitivityUnits::MillivoltsPerMillimeterPerSecond,
            15964_i32 => VelocityIEPESensorSensitivityUnits::MilliVoltsPerInchPerSecond,
        }
    }
}

impl From<VelocityIEPESensorSensitivityUnits> for i32 {
    fn from(val: VelocityIEPESensorSensitivityUnits) -> i32 {
        match val {
            VelocityIEPESensorSensitivityUnits::MillivoltsPerMillimeterPerSecond => 15963_i32,
            VelocityIEPESensorSensitivityUnits::MilliVoltsPerInchPerSecond => 15964_i32,
        }
    }
}

pub enum AOOutputChannelType {
    #[doc="Voltage generation."]
    Voltage,
    #[doc="Current generation."]
    Current,
    #[doc="Function generation."]
    FuncGen,
}

impl From<i32> for AOOutputChannelType {
    fn from(val: i32) -> AOOutputChannelType {
        match val {
            10322_i32 => AOOutputChannelType::Voltage,
            10134_i32 => AOOutputChannelType::Current,
            14750_i32 => AOOutputChannelType::FuncGen,
        }
    }
}

impl From<AOOutputChannelType> for i32 {
    fn from(val: AOOutputChannelType) -> i32 {
        match val {
            AOOutputChannelType::Voltage => 10322_i32,
            AOOutputChannelType::Current => 10134_i32,
            AOOutputChannelType::FuncGen => 14750_i32,
        }
    }
}

pub enum LengthUnits3 {
    #[doc="Meters."]
    Meters,
    #[doc="Inches."]
    Inches,
    #[doc="Ticks."]
    Ticks,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for LengthUnits3 {
    fn from(val: i32) -> LengthUnits3 {
        match val {
            10219_i32 => LengthUnits3::Meters,
            10379_i32 => LengthUnits3::Inches,
            10304_i32 => LengthUnits3::Ticks,
            10065_i32 => LengthUnits3::FromCustomScale,
        }
    }
}

impl From<LengthUnits3> for i32 {
    fn from(val: LengthUnits3) -> i32 {
        match val {
            LengthUnits3::Meters => 10219_i32,
            LengthUnits3::Inches => 10379_i32,
            LengthUnits3::Ticks => 10304_i32,
            LengthUnits3::FromCustomScale => 10065_i32,
        }
    }
}

pub enum UnderflowBehavior {
    #[doc="Stop generating samples and return an error."]
    HaltOutputAndError,
    #[doc="Pause the task until samples are available in the FIFO."]
    PauseUntilDataAvailable,
}

impl From<i32> for UnderflowBehavior {
    fn from(val: i32) -> UnderflowBehavior {
        match val {
            14615_i32 => UnderflowBehavior::HaltOutputAndError,
            14616_i32 => UnderflowBehavior::PauseUntilDataAvailable,
        }
    }
}

impl From<UnderflowBehavior> for i32 {
    fn from(val: UnderflowBehavior) -> i32 {
        match val {
            UnderflowBehavior::HaltOutputAndError => 14615_i32,
            UnderflowBehavior::PauseUntilDataAvailable => 14616_i32,
        }
    }
}

pub enum AccelUnits2 {
    #[doc="1 g is approximately equal to 9.81 m/s/s."]
    AccelUnitG,
    #[doc="Meters per second per second."]
    MetersPerSecondSquared,
    #[doc="Inches per second per second."]
    InchesPerSecondSquared,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for AccelUnits2 {
    fn from(val: i32) -> AccelUnits2 {
        match val {
            10186_i32 => AccelUnits2::AccelUnitG,
            12470_i32 => AccelUnits2::MetersPerSecondSquared,
            12471_i32 => AccelUnits2::InchesPerSecondSquared,
            10065_i32 => AccelUnits2::FromCustomScale,
        }
    }
}

impl From<AccelUnits2> for i32 {
    fn from(val: AccelUnits2) -> i32 {
        match val {
            AccelUnits2::AccelUnitG => 10186_i32,
            AccelUnits2::MetersPerSecondSquared => 12470_i32,
            AccelUnits2::InchesPerSecondSquared => 12471_i32,
            AccelUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum VelocityUnits2 {
    MetersPerSecond,
    KilometersPerHour,
    FeetPerSecond,
    MilesPerHour,
    Knots,
    FromCustomScale,
}

impl From<i32> for VelocityUnits2 {
    fn from(val: i32) -> VelocityUnits2 {
        match val {
            15959_i32 => VelocityUnits2::MetersPerSecond,
            16007_i32 => VelocityUnits2::KilometersPerHour,
            16008_i32 => VelocityUnits2::FeetPerSecond,
            16009_i32 => VelocityUnits2::MilesPerHour,
            16010_i32 => VelocityUnits2::Knots,
            10065_i32 => VelocityUnits2::FromCustomScale,
        }
    }
}

impl From<VelocityUnits2> for i32 {
    fn from(val: VelocityUnits2) -> i32 {
        match val {
            VelocityUnits2::MetersPerSecond => 15959_i32,
            VelocityUnits2::KilometersPerHour => 16007_i32,
            VelocityUnits2::FeetPerSecond => 16008_i32,
            VelocityUnits2::MilesPerHour => 16009_i32,
            VelocityUnits2::Knots => 16010_i32,
            VelocityUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum WaitMode4 {
    #[doc=" Attempt to recover when the system receives an interrupt service request. This  mode is the most CPU efficient and best suited for recovery at lower pulse  train frequencies."]
    WaitForInterrupt,
    #[doc=" Repeatedly attempt to recover as fast as possible. This mode has the highest  probability of recovery success at the expense of CPU efficiency."]
    Poll,
}

impl From<i32> for WaitMode4 {
    fn from(val: i32) -> WaitMode4 {
        match val {
            12523_i32 => WaitMode4::WaitForInterrupt,
            12524_i32 => WaitMode4::Poll,
        }
    }
}

impl From<WaitMode4> for i32 {
    fn from(val: WaitMode4) -> i32 {
        match val {
            WaitMode4::WaitForInterrupt => 12523_i32,
            WaitMode4::Poll => 12524_i32,
        }
    }
}

pub enum ForceIEPEUnits {
    Newtons,
    Pounds,
    FromCustomScale,
}

impl From<i32> for ForceIEPEUnits {
    fn from(val: i32) -> ForceIEPEUnits {
        match val {
            15875_i32 => ForceIEPEUnits::Newtons,
            15876_i32 => ForceIEPEUnits::Pounds,
            10065_i32 => ForceIEPEUnits::FromCustomScale,
        }
    }
}

impl From<ForceIEPEUnits> for i32 {
    fn from(val: ForceIEPEUnits) -> i32 {
        match val {
            ForceIEPEUnits::Newtons => 15875_i32,
            ForceIEPEUnits::Pounds => 15876_i32,
            ForceIEPEUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum WatchdogAOExpirState {
    #[doc="Voltage output."]
    Voltage,
    #[doc="Current output."]
    Current,
    #[doc=" Expiration does not affect the port. Do not change the state of any lines in  the port, and do not lock the port."]
    NoChange,
}

impl From<i32> for WatchdogAOExpirState {
    fn from(val: i32) -> WatchdogAOExpirState {
        match val {
            10322_i32 => WatchdogAOExpirState::Voltage,
            10134_i32 => WatchdogAOExpirState::Current,
            10160_i32 => WatchdogAOExpirState::NoChange,
        }
    }
}

impl From<WatchdogAOExpirState> for i32 {
    fn from(val: WatchdogAOExpirState) -> i32 {
        match val {
            WatchdogAOExpirState::Voltage => 10322_i32,
            WatchdogAOExpirState::Current => 10134_i32,
            WatchdogAOExpirState::NoChange => 10160_i32,
        }
    }
}

pub enum SyncUnlockBehavior {
    #[doc="Stop task and return an error."]
    StopTaskAndError,
    #[doc="Ignore the loss of synchronization and do nothing."]
    IgnoreLostSyncLock,
}

impl From<i32> for SyncUnlockBehavior {
    fn from(val: i32) -> SyncUnlockBehavior {
        match val {
            15862_i32 => SyncUnlockBehavior::StopTaskAndError,
            16129_i32 => SyncUnlockBehavior::IgnoreLostSyncLock,
        }
    }
}

impl From<SyncUnlockBehavior> for i32 {
    fn from(val: SyncUnlockBehavior) -> i32 {
        match val {
            SyncUnlockBehavior::StopTaskAndError => 15862_i32,
            SyncUnlockBehavior::IgnoreLostSyncLock => 16129_i32,
        }
    }
}

pub enum RawDataCompressionType {
    #[doc="Do not compress samples."]
    None,
    #[doc="Remove unused bits from samples. No resolution is lost."]
    LosslessPacking,
    #[doc=" Remove unused bits from samples. Then, if necessary, remove bits from samples  until the samples are the size specified with  DAQmx_AI_LossyLSBRemoval_CompressedSampSize. This compression type limits  resolution to the specified sample size."]
    LossyLsbRemoval,
}

impl From<i32> for RawDataCompressionType {
    fn from(val: i32) -> RawDataCompressionType {
        match val {
            10230_i32 => RawDataCompressionType::None,
            12555_i32 => RawDataCompressionType::LosslessPacking,
            12556_i32 => RawDataCompressionType::LossyLsbRemoval,
        }
    }
}

impl From<RawDataCompressionType> for i32 {
    fn from(val: RawDataCompressionType) -> i32 {
        match val {
            RawDataCompressionType::None => 10230_i32,
            RawDataCompressionType::LosslessPacking => 12555_i32,
            RawDataCompressionType::LossyLsbRemoval => 12556_i32,
        }
    }
}

pub enum LengthUnits4 {
    Meters,
    Feet,
    FromCustomScale,
}

impl From<i32> for LengthUnits4 {
    fn from(val: i32) -> LengthUnits4 {
        match val {
            10219_i32 => LengthUnits4::Meters,
            10380_i32 => LengthUnits4::Feet,
            10065_i32 => LengthUnits4::FromCustomScale,
        }
    }
}

impl From<LengthUnits4> for i32 {
    fn from(val: LengthUnits4) -> i32 {
        match val {
            LengthUnits4::Meters => 10219_i32,
            LengthUnits4::Feet => 10380_i32,
            LengthUnits4::FromCustomScale => 10065_i32,
        }
    }
}

pub enum ResistanceUnits2 {
    Ohms,
    FromCustomScale,
}

impl From<i32> for ResistanceUnits2 {
    fn from(val: i32) -> ResistanceUnits2 {
        match val {
            10384_i32 => ResistanceUnits2::Ohms,
            10065_i32 => ResistanceUnits2::FromCustomScale,
        }
    }
}

impl From<ResistanceUnits2> for i32 {
    fn from(val: ResistanceUnits2) -> i32 {
        match val {
            ResistanceUnits2::Ohms => 10384_i32,
            ResistanceUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum ExportActions {
    Pulse,
    Toggle,
    Lvl,
}

impl From<i32> for ExportActions {
    fn from(val: i32) -> ExportActions {
        match val {
            10265_i32 => ExportActions::Pulse,
            10307_i32 => ExportActions::Toggle,
            10210_i32 => ExportActions::Lvl,
        }
    }
}

impl From<ExportActions> for i32 {
    fn from(val: ExportActions) -> i32 {
        match val {
            ExportActions::Pulse => 10265_i32,
            ExportActions::Toggle => 10307_i32,
            ExportActions::Lvl => 10210_i32,
        }
    }
}

pub enum LogicLvlBehavior {
    #[doc="High logic."]
    LogicLevelPullUp,
    #[doc="Supply no excitation to the channel."]
    None,
}

impl From<i32> for LogicLvlBehavior {
    fn from(val: i32) -> LogicLvlBehavior {
        match val {
            16064_i32 => LogicLvlBehavior::LogicLevelPullUp,
            10230_i32 => LogicLvlBehavior::None,
        }
    }
}

impl From<LogicLvlBehavior> for i32 {
    fn from(val: LogicLvlBehavior) -> i32 {
        match val {
            LogicLvlBehavior::LogicLevelPullUp => 16064_i32,
            LogicLvlBehavior::None => 10230_i32,
        }
    }
}

pub enum TimeUnits {
    #[doc="Seconds."]
    Seconds,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for TimeUnits {
    fn from(val: i32) -> TimeUnits {
        match val {
            10364_i32 => TimeUnits::Seconds,
            10065_i32 => TimeUnits::FromCustomScale,
        }
    }
}

impl From<TimeUnits> for i32 {
    fn from(val: TimeUnits) -> i32 {
        match val {
            TimeUnits::Seconds => 10364_i32,
            TimeUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum BridgeShuntCalSource {
    #[doc="Use the internal shunt."]
    BuiltIn,
    #[doc="Use an external shunt."]
    UserProvided,
}

impl From<i32> for BridgeShuntCalSource {
    fn from(val: i32) -> BridgeShuntCalSource {
        match val {
            10200_i32 => BridgeShuntCalSource::BuiltIn,
            10167_i32 => BridgeShuntCalSource::UserProvided,
        }
    }
}

impl From<BridgeShuntCalSource> for i32 {
    fn from(val: BridgeShuntCalSource) -> i32 {
        match val {
            BridgeShuntCalSource::BuiltIn => 10200_i32,
            BridgeShuntCalSource::UserProvided => 10167_i32,
        }
    }
}

pub enum BridgeConfiguration1 {
    #[doc=" Sensor is a full bridge. If you set DAQmx_AI_Excit_UseForScaling to TRUE,  NI-DAQmx divides the measurement by the excitation value. Many sensors scale  data to native units using scaling of volts per excitation."]
    FullBridge,
    #[doc=" Sensor is a half bridge. If you set DAQmx_AI_Excit_UseForScaling to TRUE,  NI-DAQmx divides the measurement by the excitation value. Many sensors scale  data to native units using scaling of volts per excitation."]
    HalfBridge,
    #[doc=" Sensor is a quarter bridge. If you set DAQmx_AI_Excit_UseForScaling to TRUE,  NI-DAQmx divides the measurement by the excitation value. Many sensors scale  data to native units using scaling of volts per excitation."]
    QuarterBridge,
    #[doc="Sensor is not a Wheatstone bridge."]
    NoBridge,
}

impl From<i32> for BridgeConfiguration1 {
    fn from(val: i32) -> BridgeConfiguration1 {
        match val {
            10182_i32 => BridgeConfiguration1::FullBridge,
            10187_i32 => BridgeConfiguration1::HalfBridge,
            10270_i32 => BridgeConfiguration1::QuarterBridge,
            10228_i32 => BridgeConfiguration1::NoBridge,
        }
    }
}

impl From<BridgeConfiguration1> for i32 {
    fn from(val: BridgeConfiguration1) -> i32 {
        match val {
            BridgeConfiguration1::FullBridge => 10182_i32,
            BridgeConfiguration1::HalfBridge => 10187_i32,
            BridgeConfiguration1::QuarterBridge => 10270_i32,
            BridgeConfiguration1::NoBridge => 10228_i32,
        }
    }
}

pub enum ChargeUnits {
    #[doc="Coulombs."]
    Coulombs,
    #[doc="PicoCoulombs."]
    PicoCoulombs,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for ChargeUnits {
    fn from(val: i32) -> ChargeUnits {
        match val {
            16102_i32 => ChargeUnits::Coulombs,
            16103_i32 => ChargeUnits::PicoCoulombs,
            10065_i32 => ChargeUnits::FromCustomScale,
        }
    }
}

impl From<ChargeUnits> for i32 {
    fn from(val: ChargeUnits) -> i32 {
        match val {
            ChargeUnits::Coulombs => 16102_i32,
            ChargeUnits::PicoCoulombs => 16103_i32,
            ChargeUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum ResistanceUnits1 {
    #[doc="Ohms."]
    Ohms,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
    #[doc=" Units defined by TEDS information associated with the channel."]
    FromTeds,
}

impl From<i32> for ResistanceUnits1 {
    fn from(val: i32) -> ResistanceUnits1 {
        match val {
            10384_i32 => ResistanceUnits1::Ohms,
            10065_i32 => ResistanceUnits1::FromCustomScale,
            12516_i32 => ResistanceUnits1::FromTeds,
        }
    }
}

impl From<ResistanceUnits1> for i32 {
    fn from(val: ResistanceUnits1) -> i32 {
        match val {
            ResistanceUnits1::Ohms => 10384_i32,
            ResistanceUnits1::FromCustomScale => 10065_i32,
            ResistanceUnits1::FromTeds => 12516_i32,
        }
    }
}

pub enum ACExcitWireMode {
    #[doc="4-wire."]
    D4Wire,
    #[doc="5-wire."]
    D5Wire,
    #[doc="6-wire."]
    D6Wire,
}

impl From<i32> for ACExcitWireMode {
    fn from(val: i32) -> ACExcitWireMode {
        match val {
            4_i32 => ACExcitWireMode::D4Wire,
            5_i32 => ACExcitWireMode::D5Wire,
            6_i32 => ACExcitWireMode::D6Wire,
        }
    }
}

impl From<ACExcitWireMode> for i32 {
    fn from(val: ACExcitWireMode) -> i32 {
        match val {
            ACExcitWireMode::D4Wire => 4_i32,
            ACExcitWireMode::D5Wire => 5_i32,
            ACExcitWireMode::D6Wire => 6_i32,
        }
    }
}

pub enum BusType {
    #[doc="PCI."]
    Pci,
    #[doc="PCI Express."]
    PcIe,
    #[doc="PXI."]
    Pxi,
    #[doc="PXI Express."]
    PxIe,
    #[doc="SCXI."]
    Scxi,
    #[doc="SCC."]
    Scc,
    #[doc="PC Card/PCMCIA."]
    PcCard,
    #[doc="USB."]
    Usb,
    #[doc="CompactDAQ."]
    CompactDaq,
    #[doc="CompactRIO."]
    CompactRio,
    #[doc="TCP/IP."]
    Tcpip,
    #[doc="Unknown bus type."]
    Unknown,
    #[doc="SwitchBlock."]
    SwitchBlock,
}

impl From<i32> for BusType {
    fn from(val: i32) -> BusType {
        match val {
            12582_i32 => BusType::Pci,
            13612_i32 => BusType::PcIe,
            12583_i32 => BusType::Pxi,
            14706_i32 => BusType::PxIe,
            12584_i32 => BusType::Scxi,
            14707_i32 => BusType::Scc,
            12585_i32 => BusType::PcCard,
            12586_i32 => BusType::Usb,
            14637_i32 => BusType::CompactDaq,
            16143_i32 => BusType::CompactRio,
            14828_i32 => BusType::Tcpip,
            12588_i32 => BusType::Unknown,
            15870_i32 => BusType::SwitchBlock,
        }
    }
}

impl From<BusType> for i32 {
    fn from(val: BusType) -> i32 {
        match val {
            BusType::Pci => 12582_i32,
            BusType::PcIe => 13612_i32,
            BusType::Pxi => 12583_i32,
            BusType::PxIe => 14706_i32,
            BusType::Scxi => 12584_i32,
            BusType::Scc => 14707_i32,
            BusType::PcCard => 12585_i32,
            BusType::Usb => 12586_i32,
            BusType::CompactDaq => 14637_i32,
            BusType::CompactRio => 16143_i32,
            BusType::Tcpip => 14828_i32,
            BusType::Unknown => 12588_i32,
            BusType::SwitchBlock => 15870_i32,
        }
    }
}

pub enum WatchdogAOOutputType {
    Voltage,
    Current,
    NoChange,
}

impl From<i32> for WatchdogAOOutputType {
    fn from(val: i32) -> WatchdogAOOutputType {
        match val {
            10322_i32 => WatchdogAOOutputType::Voltage,
            10134_i32 => WatchdogAOOutputType::Current,
            10160_i32 => WatchdogAOOutputType::NoChange,
        }
    }
}

impl From<WatchdogAOOutputType> for i32 {
    fn from(val: WatchdogAOOutputType) -> i32 {
        match val {
            WatchdogAOOutputType::Voltage => 10322_i32,
            WatchdogAOOutputType::Current => 10134_i32,
            WatchdogAOOutputType::NoChange => 10160_i32,
        }
    }
}

pub enum ThermocoupleType1 {
    #[doc="J-type thermocouple."]
    JTypeTc,
    #[doc="K-type thermocouple."]
    KTypeTc,
    #[doc="N-type thermocouple."]
    NTypeTc,
    #[doc="R-type thermocouple."]
    RTypeTc,
    #[doc="S-type thermocouple."]
    STypeTc,
    #[doc="T-type thermocouple."]
    TTypeTc,
    #[doc="B-type thermocouple."]
    BTypeTc,
    #[doc="E-type thermocouple."]
    ETypeTc,
}

impl From<i32> for ThermocoupleType1 {
    fn from(val: i32) -> ThermocoupleType1 {
        match val {
            10072_i32 => ThermocoupleType1::JTypeTc,
            10073_i32 => ThermocoupleType1::KTypeTc,
            10077_i32 => ThermocoupleType1::NTypeTc,
            10082_i32 => ThermocoupleType1::RTypeTc,
            10085_i32 => ThermocoupleType1::STypeTc,
            10086_i32 => ThermocoupleType1::TTypeTc,
            10047_i32 => ThermocoupleType1::BTypeTc,
            10055_i32 => ThermocoupleType1::ETypeTc,
        }
    }
}

impl From<ThermocoupleType1> for i32 {
    fn from(val: ThermocoupleType1) -> i32 {
        match val {
            ThermocoupleType1::JTypeTc => 10072_i32,
            ThermocoupleType1::KTypeTc => 10073_i32,
            ThermocoupleType1::NTypeTc => 10077_i32,
            ThermocoupleType1::RTypeTc => 10082_i32,
            ThermocoupleType1::STypeTc => 10085_i32,
            ThermocoupleType1::TTypeTc => 10086_i32,
            ThermocoupleType1::BTypeTc => 10047_i32,
            ThermocoupleType1::ETypeTc => 10055_i32,
        }
    }
}

pub enum CurrentShuntResistorLocationWithDefault {
    Default,
    Internal,
    External,
}

impl From<i32> for CurrentShuntResistorLocationWithDefault {
    fn from(val: i32) -> CurrentShuntResistorLocationWithDefault {
        match val {
            -1_i32 => CurrentShuntResistorLocationWithDefault::Default,
            10200_i32 => CurrentShuntResistorLocationWithDefault::Internal,
            10167_i32 => CurrentShuntResistorLocationWithDefault::External,
        }
    }
}

impl From<CurrentShuntResistorLocationWithDefault> for i32 {
    fn from(val: CurrentShuntResistorLocationWithDefault) -> i32 {
        match val {
            CurrentShuntResistorLocationWithDefault::Default => -1_i32,
            CurrentShuntResistorLocationWithDefault::Internal => 10200_i32,
            CurrentShuntResistorLocationWithDefault::External => 10167_i32,
        }
    }
}

pub enum ForceIEPESensorSensitivityUnits {
    #[doc="Millivolts per newton."]
    MVoltsPerNewton,
    #[doc="Millivolts per pound."]
    MVoltsPerPound,
}

impl From<i32> for ForceIEPESensorSensitivityUnits {
    fn from(val: i32) -> ForceIEPESensorSensitivityUnits {
        match val {
            15891_i32 => ForceIEPESensorSensitivityUnits::MVoltsPerNewton,
            15892_i32 => ForceIEPESensorSensitivityUnits::MVoltsPerPound,
        }
    }
}

impl From<ForceIEPESensorSensitivityUnits> for i32 {
    fn from(val: ForceIEPESensorSensitivityUnits) -> i32 {
        match val {
            ForceIEPESensorSensitivityUnits::MVoltsPerNewton => 15891_i32,
            ForceIEPESensorSensitivityUnits::MVoltsPerPound => 15892_i32,
        }
    }
}

pub enum TaskControlAction {
    TaskStart,
    TaskStop,
    TaskVerify,
    TaskCommit,
    TaskReserve,
    TaskUnreserve,
    TaskAbort,
}

impl From<i32> for TaskControlAction {
    fn from(val: i32) -> TaskControlAction {
        match val {
            0_i32 => TaskControlAction::TaskStart,
            1_i32 => TaskControlAction::TaskStop,
            2_i32 => TaskControlAction::TaskVerify,
            3_i32 => TaskControlAction::TaskCommit,
            4_i32 => TaskControlAction::TaskReserve,
            5_i32 => TaskControlAction::TaskUnreserve,
            6_i32 => TaskControlAction::TaskAbort,
        }
    }
}

impl From<TaskControlAction> for i32 {
    fn from(val: TaskControlAction) -> i32 {
        match val {
            TaskControlAction::TaskStart => 0_i32,
            TaskControlAction::TaskStop => 1_i32,
            TaskControlAction::TaskVerify => 2_i32,
            TaskControlAction::TaskCommit => 3_i32,
            TaskControlAction::TaskReserve => 4_i32,
            TaskControlAction::TaskUnreserve => 5_i32,
            TaskControlAction::TaskAbort => 6_i32,
        }
    }
}

pub enum PressureUnits {
    #[doc="Pascals."]
    Pascals,
    #[doc="Pounds per square inch."]
    PoundsPerSquareInch,
    #[doc="Bar."]
    Bar,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for PressureUnits {
    fn from(val: i32) -> PressureUnits {
        match val {
            10081_i32 => PressureUnits::Pascals,
            15879_i32 => PressureUnits::PoundsPerSquareInch,
            15880_i32 => PressureUnits::Bar,
            10065_i32 => PressureUnits::FromCustomScale,
        }
    }
}

impl From<PressureUnits> for i32 {
    fn from(val: PressureUnits) -> i32 {
        match val {
            PressureUnits::Pascals => 10081_i32,
            PressureUnits::PoundsPerSquareInch => 15879_i32,
            PressureUnits::Bar => 15880_i32,
            PressureUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum BridgeElectricalUnits {
    #[doc="Volts per volt."]
    VoltsPerVolt,
    #[doc="Millivolts per volt."]
    MVoltsPerVolt,
}

impl From<i32> for BridgeElectricalUnits {
    fn from(val: i32) -> BridgeElectricalUnits {
        match val {
            15896_i32 => BridgeElectricalUnits::VoltsPerVolt,
            15897_i32 => BridgeElectricalUnits::MVoltsPerVolt,
        }
    }
}

impl From<BridgeElectricalUnits> for i32 {
    fn from(val: BridgeElectricalUnits) -> i32 {
        match val {
            BridgeElectricalUnits::VoltsPerVolt => 15896_i32,
            BridgeElectricalUnits::MVoltsPerVolt => 15897_i32,
        }
    }
}

pub enum Edge1 {
    #[doc="Rising edge(s)."]
    Rising,
    #[doc="Falling edge(s)."]
    Falling,
}

impl From<i32> for Edge1 {
    fn from(val: i32) -> Edge1 {
        match val {
            10280_i32 => Edge1::Rising,
            10171_i32 => Edge1::Falling,
        }
    }
}

impl From<Edge1> for i32 {
    fn from(val: Edge1) -> i32 {
        match val {
            Edge1::Rising => 10280_i32,
            Edge1::Falling => 10171_i32,
        }
    }
}

pub enum ExcitationSource {
    #[doc=" Use the built-in excitation source of the device. If you select this value, you  must specify the amount of excitation."]
    Internal,
    #[doc=" Use an excitation source other than the built-in excitation source of the  device. If you select this value, you must specify the amount of excitation."]
    External,
    #[doc="Supply no excitation to the channel."]
    None,
}

impl From<i32> for ExcitationSource {
    fn from(val: i32) -> ExcitationSource {
        match val {
            10200_i32 => ExcitationSource::Internal,
            10167_i32 => ExcitationSource::External,
            10230_i32 => ExcitationSource::None,
        }
    }
}

impl From<ExcitationSource> for i32 {
    fn from(val: ExcitationSource) -> i32 {
        match val {
            ExcitationSource::Internal => 10200_i32,
            ExcitationSource::External => 10167_i32,
            ExcitationSource::None => 10230_i32,
        }
    }
}

pub enum CounterFrequencyMethod {
    #[doc=" Use one counter that uses a constant timebase to measure the input signal."]
    LowFreq1Ctr,
    #[doc=" Use two counters, one of which counts pulses of the signal to measure during  the specified measurement time."]
    HighFreq2Ctr,
    #[doc=" Use one counter to divide the frequency of the input signal to create a  lower-frequency signal that the second counter can more easily measure."]
    LargeRng2Ctr,
    #[doc=" Uses one counter with configuration options to control the amount of averaging  or filtering applied to the counter measurements. Set filtering options to  balance measurement accuracy and noise versus latency."]
    DynAvg,
}

impl From<i32> for CounterFrequencyMethod {
    fn from(val: i32) -> CounterFrequencyMethod {
        match val {
            10105_i32 => CounterFrequencyMethod::LowFreq1Ctr,
            10157_i32 => CounterFrequencyMethod::HighFreq2Ctr,
            10205_i32 => CounterFrequencyMethod::LargeRng2Ctr,
            16065_i32 => CounterFrequencyMethod::DynAvg,
        }
    }
}

impl From<CounterFrequencyMethod> for i32 {
    fn from(val: CounterFrequencyMethod) -> i32 {
        match val {
            CounterFrequencyMethod::LowFreq1Ctr => 10105_i32,
            CounterFrequencyMethod::HighFreq2Ctr => 10157_i32,
            CounterFrequencyMethod::LargeRng2Ctr => 10205_i32,
            CounterFrequencyMethod::DynAvg => 16065_i32,
        }
    }
}

pub enum OverwriteMode1 {
    #[doc=" When an acquisition encounters unread data in the buffer, the acquisition  continues and overwrites the unread samples with new ones. You can read the new  samples by setting DAQmx_Read_RelativeTo to DAQmx_Val_MostRecentSamp and  setting DAQmx_Read_Offset to the appropriate number of samples."]
    OverwriteUnreadSamps,
    #[doc=" The acquisition stops when it encounters a sample in the buffer that you have  not read."]
    DoNotOverwriteUnreadSamps,
}

impl From<i32> for OverwriteMode1 {
    fn from(val: i32) -> OverwriteMode1 {
        match val {
            10252_i32 => OverwriteMode1::OverwriteUnreadSamps,
            10159_i32 => OverwriteMode1::DoNotOverwriteUnreadSamps,
        }
    }
}

impl From<OverwriteMode1> for i32 {
    fn from(val: OverwriteMode1) -> i32 {
        match val {
            OverwriteMode1::OverwriteUnreadSamps => 10252_i32,
            OverwriteMode1::DoNotOverwriteUnreadSamps => 10159_i32,
        }
    }
}

pub enum LVDTSensitivityUnits1 {
    #[doc="mVolts/Volt/mMeter."]
    MVoltsPerVoltPerMillimeter,
    #[doc="mVolts/Volt/0.001 Inch."]
    MVoltsPerVoltPerMilliInch,
}

impl From<i32> for LVDTSensitivityUnits1 {
    fn from(val: i32) -> LVDTSensitivityUnits1 {
        match val {
            12506_i32 => LVDTSensitivityUnits1::MVoltsPerVoltPerMillimeter,
            12505_i32 => LVDTSensitivityUnits1::MVoltsPerVoltPerMilliInch,
        }
    }
}

impl From<LVDTSensitivityUnits1> for i32 {
    fn from(val: LVDTSensitivityUnits1) -> i32 {
        match val {
            LVDTSensitivityUnits1::MVoltsPerVoltPerMillimeter => 12506_i32,
            LVDTSensitivityUnits1::MVoltsPerVoltPerMilliInch => 12505_i32,
        }
    }
}

pub enum LengthUnits2 {
    #[doc="Meters."]
    Meters,
    #[doc="Inches."]
    Inches,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for LengthUnits2 {
    fn from(val: i32) -> LengthUnits2 {
        match val {
            10219_i32 => LengthUnits2::Meters,
            10379_i32 => LengthUnits2::Inches,
            10065_i32 => LengthUnits2::FromCustomScale,
        }
    }
}

impl From<LengthUnits2> for i32 {
    fn from(val: LengthUnits2) -> i32 {
        match val {
            LengthUnits2::Meters => 10219_i32,
            LengthUnits2::Inches => 10379_i32,
            LengthUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum Polarity2 {
    #[doc="High state is the active state."]
    ActiveHigh,
    #[doc="Low state is the active state."]
    ActiveLow,
}

impl From<i32> for Polarity2 {
    fn from(val: i32) -> Polarity2 {
        match val {
            10095_i32 => Polarity2::ActiveHigh,
            10096_i32 => Polarity2::ActiveLow,
        }
    }
}

impl From<Polarity2> for i32 {
    fn from(val: Polarity2) -> i32 {
        match val {
            Polarity2::ActiveHigh => 10095_i32,
            Polarity2::ActiveLow => 10096_i32,
        }
    }
}

pub enum InputDataTransferCondition {
    #[doc=" Transfer data from the device when more than half of the onboard memory of the  device fills."]
    OnBrdMemMoreThanHalfFull,
    #[doc=" Transfer data from the device when there is data in the onboard memory."]
    OnBrdMemNotEmpty,
    #[doc=" Transfer data from the device when the number of samples specified with  DAQmx_AI_DataXferCustomThreshold are in the device FIFO."]
    OnbrdMemCustomThreshold,
    #[doc="Transfer data when the acquisition is complete."]
    WhenAcqComplete,
}

impl From<i32> for InputDataTransferCondition {
    fn from(val: i32) -> InputDataTransferCondition {
        match val {
            10237_i32 => InputDataTransferCondition::OnBrdMemMoreThanHalfFull,
            10241_i32 => InputDataTransferCondition::OnBrdMemNotEmpty,
            12577_i32 => InputDataTransferCondition::OnbrdMemCustomThreshold,
            12546_i32 => InputDataTransferCondition::WhenAcqComplete,
        }
    }
}

impl From<InputDataTransferCondition> for i32 {
    fn from(val: InputDataTransferCondition) -> i32 {
        match val {
            InputDataTransferCondition::OnBrdMemMoreThanHalfFull => 10237_i32,
            InputDataTransferCondition::OnBrdMemNotEmpty => 10241_i32,
            InputDataTransferCondition::OnbrdMemCustomThreshold => 12577_i32,
            InputDataTransferCondition::WhenAcqComplete => 12546_i32,
        }
    }
}

pub enum Sense {
    #[doc="Local."]
    Local,
    #[doc="Remote."]
    Remote,
}

impl From<i32> for Sense {
    fn from(val: i32) -> Sense {
        match val {
            16095_i32 => Sense::Local,
            16096_i32 => Sense::Remote,
        }
    }
}

impl From<Sense> for i32 {
    fn from(val: Sense) -> i32 {
        match val {
            Sense::Local => 16095_i32,
            Sense::Remote => 16096_i32,
        }
    }
}

pub enum TriggerType4 {
    #[doc="Trigger on a rising or falling edge of a digital signal."]
    DigEdge,
    #[doc="Trigger when a specified time is reached."]
    Time,
    #[doc="Disable the trigger."]
    None,
}

impl From<i32> for TriggerType4 {
    fn from(val: i32) -> TriggerType4 {
        match val {
            10150_i32 => TriggerType4::DigEdge,
            15996_i32 => TriggerType4::Time,
            10230_i32 => TriggerType4::None,
        }
    }
}

impl From<TriggerType4> for i32 {
    fn from(val: TriggerType4) -> i32 {
        match val {
            TriggerType4::DigEdge => 10150_i32,
            TriggerType4::Time => 15996_i32,
            TriggerType4::None => 10230_i32,
        }
    }
}

pub enum RegenerationMode1 {
    #[doc=" Allow NI-DAQmx to regenerate samples that the device previously generated. When  you choose this value, the write marker returns to the beginning of the buffer  after the device generates all samples currently in the buffer."]
    AllowRegen,
    #[doc=" Do not allow NI-DAQmx to regenerate samples the device previously generated.  When you choose this value, NI-DAQmx waits for you to write more samples to the  buffer or until the timeout expires."]
    DoNotAllowRegen,
}

impl From<i32> for RegenerationMode1 {
    fn from(val: i32) -> RegenerationMode1 {
        match val {
            10097_i32 => RegenerationMode1::AllowRegen,
            10158_i32 => RegenerationMode1::DoNotAllowRegen,
        }
    }
}

impl From<RegenerationMode1> for i32 {
    fn from(val: RegenerationMode1) -> i32 {
        match val {
            RegenerationMode1::AllowRegen => 10097_i32,
            RegenerationMode1::DoNotAllowRegen => 10158_i32,
        }
    }
}

pub enum WindowTriggerCondition2 {
    #[doc=" Pause the measurement or generation while the trigger is inside the window."]
    InsideWin,
    #[doc=" Pause the measurement or generation while the signal is outside the window."]
    OutsideWin,
}

impl From<i32> for WindowTriggerCondition2 {
    fn from(val: i32) -> WindowTriggerCondition2 {
        match val {
            10199_i32 => WindowTriggerCondition2::InsideWin,
            10251_i32 => WindowTriggerCondition2::OutsideWin,
        }
    }
}

impl From<WindowTriggerCondition2> for i32 {
    fn from(val: WindowTriggerCondition2) -> i32 {
        match val {
            WindowTriggerCondition2::InsideWin => 10199_i32,
            WindowTriggerCondition2::OutsideWin => 10251_i32,
        }
    }
}

pub enum Level1 {
    #[doc="High state."]
    High,
    #[doc="Low state."]
    Low,
}

impl From<i32> for Level1 {
    fn from(val: i32) -> Level1 {
        match val {
            10192_i32 => Level1::High,
            10214_i32 => Level1::Low,
        }
    }
}

impl From<Level1> for i32 {
    fn from(val: Level1) -> i32 {
        match val {
            Level1::High => 10192_i32,
            Level1::Low => 10214_i32,
        }
    }
}

pub enum NavMode {
    Mobile,
    StationaryWithSurvey,
    StationaryWithPresetLocation,
}

impl From<i32> for NavMode {
    fn from(val: i32) -> NavMode {
        match val {
            15989_i32 => NavMode::Mobile,
            15990_i32 => NavMode::StationaryWithSurvey,
            15991_i32 => NavMode::StationaryWithPresetLocation,
        }
    }
}

impl From<NavMode> for i32 {
    fn from(val: NavMode) -> i32 {
        match val {
            NavMode::Mobile => 15989_i32,
            NavMode::StationaryWithSurvey => 15990_i32,
            NavMode::StationaryWithPresetLocation => 15991_i32,
        }
    }
}

pub enum DigitalWidthUnits4 {
    #[doc="Seconds."]
    Seconds,
    #[doc="Sample Clock Periods."]
    SampleClkPeriods,
}

impl From<i32> for DigitalWidthUnits4 {
    fn from(val: i32) -> DigitalWidthUnits4 {
        match val {
            10364_i32 => DigitalWidthUnits4::Seconds,
            10286_i32 => DigitalWidthUnits4::SampleClkPeriods,
        }
    }
}

impl From<DigitalWidthUnits4> for i32 {
    fn from(val: DigitalWidthUnits4) -> i32 {
        match val {
            DigitalWidthUnits4::Seconds => 10364_i32,
            DigitalWidthUnits4::SampleClkPeriods => 10286_i32,
        }
    }
}

pub enum SensorPowerType {
    #[doc="Sensor power supply generates a single DC voltage level."]
    Dc,
    #[doc="Sensor power supply generates an AC voltage."]
    Ac,
    #[doc="Sensor power supply generates a pair of DC voltage levels."]
    BipolarDc,
}

impl From<i32> for SensorPowerType {
    fn from(val: i32) -> SensorPowerType {
        match val {
            10050_i32 => SensorPowerType::Dc,
            10045_i32 => SensorPowerType::Ac,
            16147_i32 => SensorPowerType::BipolarDc,
        }
    }
}

impl From<SensorPowerType> for i32 {
    fn from(val: SensorPowerType) -> i32 {
        match val {
            SensorPowerType::Dc => 10050_i32,
            SensorPowerType::Ac => 10045_i32,
            SensorPowerType::BipolarDc => 16147_i32,
        }
    }
}

pub enum ScaleType4 {
    #[doc="Do not scale electrical values to physical units."]
    None,
    #[doc=" You provide two pairs of electrical values and their corresponding physical  values. NI-DAQmx uses those values to calculate the slope and y-intercept of a  linear equation and uses that equation to scale electrical values to physical  values."]
    TwoPointLinear,
    #[doc=" Map an array of electrical values to an array of corresponding physical values,  with all other values scaled proportionally. If you specify this scaling type,  DAQmx_AI_Max and DAQmx_AI_Min must be within the smallest and largest physical  values. For any data outside those endpoints, NI-DAQmx coerces that data to the  endpoints."]
    Table,
    #[doc="Scale values by using an Nth order polynomial equation."]
    Polynomial,
}

impl From<i32> for ScaleType4 {
    fn from(val: i32) -> ScaleType4 {
        match val {
            10230_i32 => ScaleType4::None,
            15898_i32 => ScaleType4::TwoPointLinear,
            10450_i32 => ScaleType4::Table,
            10449_i32 => ScaleType4::Polynomial,
        }
    }
}

impl From<ScaleType4> for i32 {
    fn from(val: ScaleType4) -> i32 {
        match val {
            ScaleType4::None => 10230_i32,
            ScaleType4::TwoPointLinear => 15898_i32,
            ScaleType4::Table => 10450_i32,
            ScaleType4::Polynomial => 10449_i32,
        }
    }
}

pub enum SyncType {
    #[doc="Disables trigger skew correction."]
    None,
    #[doc="Device is the source for shared clocks and triggers."]
    Master,
    #[doc="Device uses clocks and triggers from the master device."]
    Slave,
}

impl From<i32> for SyncType {
    fn from(val: i32) -> SyncType {
        match val {
            10230_i32 => SyncType::None,
            15888_i32 => SyncType::Master,
            15889_i32 => SyncType::Slave,
        }
    }
}

impl From<SyncType> for i32 {
    fn from(val: SyncType) -> i32 {
        match val {
            SyncType::None => 10230_i32,
            SyncType::Master => 15888_i32,
            SyncType::Slave => 15889_i32,
        }
    }
}

pub enum FilterResponse {
    #[doc="Constant group delay filter response."]
    ConstantGroupDelay,
    #[doc="Butterworth filter response."]
    Butterworth,
    #[doc="Elliptical filter response."]
    Elliptical,
    #[doc="Use the hardware-defined filter response."]
    HardwareDefined,
}

impl From<i32> for FilterResponse {
    fn from(val: i32) -> FilterResponse {
        match val {
            16075_i32 => FilterResponse::ConstantGroupDelay,
            16076_i32 => FilterResponse::Butterworth,
            16077_i32 => FilterResponse::Elliptical,
            10191_i32 => FilterResponse::HardwareDefined,
        }
    }
}

impl From<FilterResponse> for i32 {
    fn from(val: FilterResponse) -> i32 {
        match val {
            FilterResponse::ConstantGroupDelay => 16075_i32,
            FilterResponse::Butterworth => 16076_i32,
            FilterResponse::Elliptical => 16077_i32,
            FilterResponse::HardwareDefined => 10191_i32,
        }
    }
}

pub enum ResistorState {
    PullUp,
    PullDown,
}

impl From<i32> for ResistorState {
    fn from(val: i32) -> ResistorState {
        match val {
            15950_i32 => ResistorState::PullUp,
            15951_i32 => ResistorState::PullDown,
        }
    }
}

impl From<ResistorState> for i32 {
    fn from(val: ResistorState) -> i32 {
        match val {
            ResistorState::PullUp => 15950_i32,
            ResistorState::PullDown => 15951_i32,
        }
    }
}

pub enum WatchdogControlAction {
    ResetTimer,
    ClearExpiration,
}

impl From<i32> for WatchdogControlAction {
    fn from(val: i32) -> WatchdogControlAction {
        match val {
            0_i32 => WatchdogControlAction::ResetTimer,
            1_i32 => WatchdogControlAction::ClearExpiration,
        }
    }
}

impl From<WatchdogControlAction> for i32 {
    fn from(val: WatchdogControlAction) -> i32 {
        match val {
            WatchdogControlAction::ResetTimer => 0_i32,
            WatchdogControlAction::ClearExpiration => 1_i32,
        }
    }
}

pub enum HandshakeStartCondition {
    #[doc=" Device is waiting for space in the FIFO (for acquisition) or waiting for  samples (for generation)."]
    Immediate,
    #[doc="Device is waiting for the Handshake Trigger to assert."]
    WaitForHandshakeTriggerAssert,
    #[doc="Device is waiting for the Handshake Trigger to deassert."]
    WaitForHandshakeTriggerDeassert,
}

impl From<i32> for HandshakeStartCondition {
    fn from(val: i32) -> HandshakeStartCondition {
        match val {
            10198_i32 => HandshakeStartCondition::Immediate,
            12550_i32 => HandshakeStartCondition::WaitForHandshakeTriggerAssert,
            12551_i32 => HandshakeStartCondition::WaitForHandshakeTriggerDeassert,
        }
    }
}

impl From<HandshakeStartCondition> for i32 {
    fn from(val: HandshakeStartCondition) -> i32 {
        match val {
            HandshakeStartCondition::Immediate => 10198_i32,
            HandshakeStartCondition::WaitForHandshakeTriggerAssert => 12550_i32,
            HandshakeStartCondition::WaitForHandshakeTriggerDeassert => 12551_i32,
        }
    }
}

pub enum ActiveLevel {
    #[doc=" Pause the measurement or generation while the signal is above the threshold."]
    AboveLvl,
    #[doc=" Pause the measurement or generation while the signal is below the threshold."]
    BelowLvl,
}

impl From<i32> for ActiveLevel {
    fn from(val: i32) -> ActiveLevel {
        match val {
            10093_i32 => ActiveLevel::AboveLvl,
            10107_i32 => ActiveLevel::BelowLvl,
        }
    }
}

impl From<ActiveLevel> for i32 {
    fn from(val: ActiveLevel) -> i32 {
        match val {
            ActiveLevel::AboveLvl => 10093_i32,
            ActiveLevel::BelowLvl => 10107_i32,
        }
    }
}

pub enum SampleTimingType {
    #[doc=" Acquire or generate samples on the specified edge of the sample clock."]
    SampClk,
    #[doc=" Determine sample timing using burst handshaking between the device and a  peripheral device."]
    BurstHandshake,
    #[doc=" Determine sample timing by using digital handshaking between the device and a  peripheral device."]
    Handshake,
    #[doc="Configure only the duration of the task."]
    Implicit,
    #[doc=" Acquire or generate a sample on each read or write operation. This timing type  is also referred to as static or software-timed."]
    OnDemand,
    #[doc=" Acquire samples when a change occurs in the state of one or more digital input  lines. The lines must be contained within a digital input channel."]
    ChangeDetection,
    #[doc=" Device acquires or generates samples on each sample clock edge, but does not  respond to certain triggers until a few sample clock edges later. Pipelining  allows higher data transfer rates at the cost of increased trigger response  latency.  Refer to the device documentation for information about which  triggers pipelining affects. This timing type allows handshaking with some  devices using the Pause trigger, the Ready for Transfer event, or the Data  Active event. Refer to the device documentation for more information."]
    PipelinedSampClk,
}

impl From<i32> for SampleTimingType {
    fn from(val: i32) -> SampleTimingType {
        match val {
            10388_i32 => SampleTimingType::SampClk,
            12548_i32 => SampleTimingType::BurstHandshake,
            10389_i32 => SampleTimingType::Handshake,
            10451_i32 => SampleTimingType::Implicit,
            10390_i32 => SampleTimingType::OnDemand,
            12504_i32 => SampleTimingType::ChangeDetection,
            14668_i32 => SampleTimingType::PipelinedSampClk,
        }
    }
}

impl From<SampleTimingType> for i32 {
    fn from(val: SampleTimingType) -> i32 {
        match val {
            SampleTimingType::SampClk => 10388_i32,
            SampleTimingType::BurstHandshake => 12548_i32,
            SampleTimingType::Handshake => 10389_i32,
            SampleTimingType::Implicit => 10451_i32,
            SampleTimingType::OnDemand => 10390_i32,
            SampleTimingType::ChangeDetection => 12504_i32,
            SampleTimingType::PipelinedSampClk => 14668_i32,
        }
    }
}

pub enum CurrentUnits2 {
    Amps,
    FromCustomScale,
}

impl From<i32> for CurrentUnits2 {
    fn from(val: i32) -> CurrentUnits2 {
        match val {
            10342_i32 => CurrentUnits2::Amps,
            10065_i32 => CurrentUnits2::FromCustomScale,
        }
    }
}

impl From<CurrentUnits2> for i32 {
    fn from(val: CurrentUnits2) -> i32 {
        match val {
            CurrentUnits2::Amps => 10342_i32,
            CurrentUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum TemperatureUnits1 {
    #[doc="Degrees Celsius."]
    DegC,
    #[doc="Degrees Fahrenheit."]
    DegF,
    #[doc="Kelvins."]
    Kelvins,
    #[doc="Degrees Rankine."]
    DegR,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for TemperatureUnits1 {
    fn from(val: i32) -> TemperatureUnits1 {
        match val {
            10143_i32 => TemperatureUnits1::DegC,
            10144_i32 => TemperatureUnits1::DegF,
            10325_i32 => TemperatureUnits1::Kelvins,
            10145_i32 => TemperatureUnits1::DegR,
            10065_i32 => TemperatureUnits1::FromCustomScale,
        }
    }
}

impl From<TemperatureUnits1> for i32 {
    fn from(val: TemperatureUnits1) -> i32 {
        match val {
            TemperatureUnits1::DegC => 10143_i32,
            TemperatureUnits1::DegF => 10144_i32,
            TemperatureUnits1::Kelvins => 10325_i32,
            TemperatureUnits1::DegR => 10145_i32,
            TemperatureUnits1::FromCustomScale => 10065_i32,
        }
    }
}

pub enum WriteRelativeTo {
    #[doc="Write samples relative to the first sample."]
    FirstSample,
    #[doc=" Write samples relative to the current position in the buffer."]
    CurrWritePos,
}

impl From<i32> for WriteRelativeTo {
    fn from(val: i32) -> WriteRelativeTo {
        match val {
            10424_i32 => WriteRelativeTo::FirstSample,
            10430_i32 => WriteRelativeTo::CurrWritePos,
        }
    }
}

impl From<WriteRelativeTo> for i32 {
    fn from(val: WriteRelativeTo) -> i32 {
        match val {
            WriteRelativeTo::FirstSample => 10424_i32,
            WriteRelativeTo::CurrWritePos => 10430_i32,
        }
    }
}

pub enum UnitsPreScaled {
    #[doc="Volts."]
    Volts,
    #[doc="Amperes."]
    Amps,
    #[doc="Degrees Fahrenheit."]
    DegF,
    #[doc="Degrees Celsius."]
    DegC,
    #[doc="Degrees Rankine."]
    DegR,
    #[doc="Kelvins."]
    Kelvins,
    #[doc="Strain."]
    Strain,
    #[doc="Ohms."]
    Ohms,
    #[doc="Hertz."]
    Hz,
    #[doc="Seconds."]
    Seconds,
    #[doc="Meters."]
    Meters,
    #[doc="Inches."]
    Inches,
    #[doc="Degrees."]
    Degrees,
    #[doc="Radians."]
    Radians,
    #[doc="Ticks."]
    Ticks,
    #[doc="Revolutions per minute."]
    Rpm,
    #[doc="Radians per second."]
    RadiansPerSecond,
    #[doc="Degrees per second."]
    DegreesPerSecond,
    #[doc="1 g is approximately equal to 9.81 m/s/s."]
    G,
    #[doc="Meters per second per second."]
    MetersPerSecondSquared,
    #[doc="Inches per second per second."]
    InchesPerSecondSquared,
    #[doc="Meters per second."]
    MetersPerSecond,
    #[doc="Inches per second."]
    InchesPerSecond,
    #[doc="Pascals."]
    Pascals,
    #[doc="Newtons."]
    Newtons,
    #[doc="Pounds."]
    Pounds,
    #[doc="Kilograms-force."]
    KilogramForce,
    #[doc="Pounds per square inch."]
    PoundsPerSquareInch,
    #[doc="Bar."]
    Bar,
    #[doc="Newton meters."]
    NewtonMeters,
    #[doc="Ounce-inches."]
    InchOunces,
    #[doc="Pound-inches."]
    InchPounds,
    #[doc="Pound-feet."]
    FootPounds,
    #[doc="Volts per volt."]
    VoltsPerVolt,
    #[doc="Millivolts per volt."]
    MVoltsPerVolt,
    #[doc="Coulombs."]
    Coulombs,
    #[doc="PicoCoulombs."]
    PicoCoulombs,
    #[doc=" Units defined by TEDS information associated with the channel."]
    FromTeds,
}

impl From<i32> for UnitsPreScaled {
    fn from(val: i32) -> UnitsPreScaled {
        match val {
            10348_i32 => UnitsPreScaled::Volts,
            10342_i32 => UnitsPreScaled::Amps,
            10144_i32 => UnitsPreScaled::DegF,
            10143_i32 => UnitsPreScaled::DegC,
            10145_i32 => UnitsPreScaled::DegR,
            10325_i32 => UnitsPreScaled::Kelvins,
            10299_i32 => UnitsPreScaled::Strain,
            10384_i32 => UnitsPreScaled::Ohms,
            10373_i32 => UnitsPreScaled::Hz,
            10364_i32 => UnitsPreScaled::Seconds,
            10219_i32 => UnitsPreScaled::Meters,
            10379_i32 => UnitsPreScaled::Inches,
            10146_i32 => UnitsPreScaled::Degrees,
            10273_i32 => UnitsPreScaled::Radians,
            10304_i32 => UnitsPreScaled::Ticks,
            16080_i32 => UnitsPreScaled::Rpm,
            16081_i32 => UnitsPreScaled::RadiansPerSecond,
            16082_i32 => UnitsPreScaled::DegreesPerSecond,
            10186_i32 => UnitsPreScaled::G,
            12470_i32 => UnitsPreScaled::MetersPerSecondSquared,
            12471_i32 => UnitsPreScaled::InchesPerSecondSquared,
            15959_i32 => UnitsPreScaled::MetersPerSecond,
            15960_i32 => UnitsPreScaled::InchesPerSecond,
            10081_i32 => UnitsPreScaled::Pascals,
            15875_i32 => UnitsPreScaled::Newtons,
            15876_i32 => UnitsPreScaled::Pounds,
            15877_i32 => UnitsPreScaled::KilogramForce,
            15879_i32 => UnitsPreScaled::PoundsPerSquareInch,
            15880_i32 => UnitsPreScaled::Bar,
            15881_i32 => UnitsPreScaled::NewtonMeters,
            15882_i32 => UnitsPreScaled::InchOunces,
            15883_i32 => UnitsPreScaled::InchPounds,
            15884_i32 => UnitsPreScaled::FootPounds,
            15896_i32 => UnitsPreScaled::VoltsPerVolt,
            15897_i32 => UnitsPreScaled::MVoltsPerVolt,
            16102_i32 => UnitsPreScaled::Coulombs,
            16103_i32 => UnitsPreScaled::PicoCoulombs,
            12516_i32 => UnitsPreScaled::FromTeds,
        }
    }
}

impl From<UnitsPreScaled> for i32 {
    fn from(val: UnitsPreScaled) -> i32 {
        match val {
            UnitsPreScaled::Volts => 10348_i32,
            UnitsPreScaled::Amps => 10342_i32,
            UnitsPreScaled::DegF => 10144_i32,
            UnitsPreScaled::DegC => 10143_i32,
            UnitsPreScaled::DegR => 10145_i32,
            UnitsPreScaled::Kelvins => 10325_i32,
            UnitsPreScaled::Strain => 10299_i32,
            UnitsPreScaled::Ohms => 10384_i32,
            UnitsPreScaled::Hz => 10373_i32,
            UnitsPreScaled::Seconds => 10364_i32,
            UnitsPreScaled::Meters => 10219_i32,
            UnitsPreScaled::Inches => 10379_i32,
            UnitsPreScaled::Degrees => 10146_i32,
            UnitsPreScaled::Radians => 10273_i32,
            UnitsPreScaled::Ticks => 10304_i32,
            UnitsPreScaled::Rpm => 16080_i32,
            UnitsPreScaled::RadiansPerSecond => 16081_i32,
            UnitsPreScaled::DegreesPerSecond => 16082_i32,
            UnitsPreScaled::G => 10186_i32,
            UnitsPreScaled::MetersPerSecondSquared => 12470_i32,
            UnitsPreScaled::InchesPerSecondSquared => 12471_i32,
            UnitsPreScaled::MetersPerSecond => 15959_i32,
            UnitsPreScaled::InchesPerSecond => 15960_i32,
            UnitsPreScaled::Pascals => 10081_i32,
            UnitsPreScaled::Newtons => 15875_i32,
            UnitsPreScaled::Pounds => 15876_i32,
            UnitsPreScaled::KilogramForce => 15877_i32,
            UnitsPreScaled::PoundsPerSquareInch => 15879_i32,
            UnitsPreScaled::Bar => 15880_i32,
            UnitsPreScaled::NewtonMeters => 15881_i32,
            UnitsPreScaled::InchOunces => 15882_i32,
            UnitsPreScaled::InchPounds => 15883_i32,
            UnitsPreScaled::FootPounds => 15884_i32,
            UnitsPreScaled::VoltsPerVolt => 15896_i32,
            UnitsPreScaled::MVoltsPerVolt => 15897_i32,
            UnitsPreScaled::Coulombs => 16102_i32,
            UnitsPreScaled::PicoCoulombs => 16103_i32,
            UnitsPreScaled::FromTeds => 12516_i32,
        }
    }
}

pub enum GroupBy {
    GroupByChannel,
    GroupByScanNumber,
}

impl From<i32> for GroupBy {
    fn from(val: i32) -> GroupBy {
        match val {
            0_i32 => GroupBy::GroupByChannel,
            1_i32 => GroupBy::GroupByScanNumber,
        }
    }
}

impl From<GroupBy> for i32 {
    fn from(val: GroupBy) -> i32 {
        match val {
            GroupBy::GroupByChannel => 0_i32,
            GroupBy::GroupByScanNumber => 1_i32,
        }
    }
}

pub enum AngleUnits3 {
    Degrees,
    FromCustomScale,
}

impl From<i32> for AngleUnits3 {
    fn from(val: i32) -> AngleUnits3 {
        match val {
            10146_i32 => AngleUnits3::Degrees,
            10065_i32 => AngleUnits3::FromCustomScale,
        }
    }
}

impl From<AngleUnits3> for i32 {
    fn from(val: AngleUnits3) -> i32 {
        match val {
            AngleUnits3::Degrees => 10146_i32,
            AngleUnits3::FromCustomScale => 10065_i32,
        }
    }
}

pub enum AccelSensitivityUnits1 {
    #[doc="mVolts/g."]
    MVoltsPerG,
    #[doc="Volts/g."]
    VoltsPerG,
}

impl From<i32> for AccelSensitivityUnits1 {
    fn from(val: i32) -> AccelSensitivityUnits1 {
        match val {
            12509_i32 => AccelSensitivityUnits1::MVoltsPerG,
            12510_i32 => AccelSensitivityUnits1::VoltsPerG,
        }
    }
}

impl From<AccelSensitivityUnits1> for i32 {
    fn from(val: AccelSensitivityUnits1) -> i32 {
        match val {
            AccelSensitivityUnits1::MVoltsPerG => 12509_i32,
            AccelSensitivityUnits1::VoltsPerG => 12510_i32,
        }
    }
}

pub enum DigitalWidthUnits2 {
    #[doc="Seconds."]
    Seconds,
    #[doc="Timebase ticks."]
    Ticks,
}

impl From<i32> for DigitalWidthUnits2 {
    fn from(val: i32) -> DigitalWidthUnits2 {
        match val {
            10364_i32 => DigitalWidthUnits2::Seconds,
            10304_i32 => DigitalWidthUnits2::Ticks,
        }
    }
}

impl From<DigitalWidthUnits2> for i32 {
    fn from(val: DigitalWidthUnits2) -> i32 {
        match val {
            DigitalWidthUnits2::Seconds => 10364_i32,
            DigitalWidthUnits2::Ticks => 10304_i32,
        }
    }
}

pub enum AngleUnits1 {
    #[doc="Degrees."]
    Degrees,
    #[doc="Radians."]
    Radians,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for AngleUnits1 {
    fn from(val: i32) -> AngleUnits1 {
        match val {
            10146_i32 => AngleUnits1::Degrees,
            10273_i32 => AngleUnits1::Radians,
            10065_i32 => AngleUnits1::FromCustomScale,
        }
    }
}

impl From<AngleUnits1> for i32 {
    fn from(val: AngleUnits1) -> i32 {
        match val {
            AngleUnits1::Degrees => 10146_i32,
            AngleUnits1::Radians => 10273_i32,
            AngleUnits1::FromCustomScale => 10065_i32,
        }
    }
}

pub enum BridgeUnits {
    #[doc="Volts per volt."]
    VoltsPerVolt,
    #[doc="Millivolts per volt."]
    MVoltsPerVolt,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
    #[doc=" Units defined by TEDS information associated with the channel."]
    FromTeds,
}

impl From<i32> for BridgeUnits {
    fn from(val: i32) -> BridgeUnits {
        match val {
            15896_i32 => BridgeUnits::VoltsPerVolt,
            15897_i32 => BridgeUnits::MVoltsPerVolt,
            10065_i32 => BridgeUnits::FromCustomScale,
            12516_i32 => BridgeUnits::FromTeds,
        }
    }
}

impl From<BridgeUnits> for i32 {
    fn from(val: BridgeUnits) -> i32 {
        match val {
            BridgeUnits::VoltsPerVolt => 15896_i32,
            BridgeUnits::MVoltsPerVolt => 15897_i32,
            BridgeUnits::FromCustomScale => 10065_i32,
            BridgeUnits::FromTeds => 12516_i32,
        }
    }
}

pub enum DigitalWidthUnits3 {
    #[doc="Seconds."]
    Seconds,
}

impl From<i32> for DigitalWidthUnits3 {
    fn from(val: i32) -> DigitalWidthUnits3 {
        match val {
            10364_i32 => DigitalWidthUnits3::Seconds,
        }
    }
}

impl From<DigitalWidthUnits3> for i32 {
    fn from(val: DigitalWidthUnits3) -> i32 {
        match val {
            DigitalWidthUnits3::Seconds => 10364_i32,
        }
    }
}

pub enum LogicFamily {
    #[doc="Compatible with 2.5 V CMOS signals."]
    D2Point5V,
    #[doc="Compatible with LVTTL signals."]
    D3Point3V,
    #[doc="Compatible with TTL and 5 V CMOS signals."]
    D5V,
}

impl From<i32> for LogicFamily {
    fn from(val: i32) -> LogicFamily {
        match val {
            14620_i32 => LogicFamily::D2Point5V,
            14621_i32 => LogicFamily::D3Point3V,
            14619_i32 => LogicFamily::D5V,
        }
    }
}

impl From<LogicFamily> for i32 {
    fn from(val: LogicFamily) -> i32 {
        match val {
            LogicFamily::D2Point5V => 14620_i32,
            LogicFamily::D3Point3V => 14621_i32,
            LogicFamily::D5V => 14619_i32,
        }
    }
}

pub enum AccelChargeSensitivityUnits {
    #[doc="PicoCoulombs per g."]
    PicoCoulombsPerG,
    #[doc="PicoCoulombs per m/s^2."]
    PicoCoulombsPerMetersPerSecondSquared,
    #[doc="PicoCoulombs per in/s^2."]
    PicoCoulombsPerInchesPerSecondSquared,
}

impl From<i32> for AccelChargeSensitivityUnits {
    fn from(val: i32) -> AccelChargeSensitivityUnits {
        match val {
            16099_i32 => AccelChargeSensitivityUnits::PicoCoulombsPerG,
            16100_i32 => AccelChargeSensitivityUnits::PicoCoulombsPerMetersPerSecondSquared,
            16101_i32 => AccelChargeSensitivityUnits::PicoCoulombsPerInchesPerSecondSquared,
        }
    }
}

impl From<AccelChargeSensitivityUnits> for i32 {
    fn from(val: AccelChargeSensitivityUnits) -> i32 {
        match val {
            AccelChargeSensitivityUnits::PicoCoulombsPerG => 16099_i32,
            AccelChargeSensitivityUnits::PicoCoulombsPerMetersPerSecondSquared => 16100_i32,
            AccelChargeSensitivityUnits::PicoCoulombsPerInchesPerSecondSquared => 16101_i32,
        }
    }
}

pub enum CIMeasurementType {
    #[doc="Count edges of a digital signal."]
    CountEdges,
    #[doc="Measure the frequency of a digital signal."]
    Freq,
    #[doc="Measure the period of a digital signal."]
    Period,
    #[doc="Measure the width of a pulse of a digital signal."]
    PulseWidth,
    #[doc=" Measure the time between state transitions of a digital signal."]
    SemiPeriod,
    #[doc=" Pulse measurement, returning the result as frequency and duty cycle."]
    PulseFrequency,
    #[doc=" Pulse measurement, returning the result as high time and low time."]
    PulseTime,
    #[doc=" Pulse measurement, returning the result as high ticks and low ticks."]
    PulseTicks,
    #[doc="Measure the duty cycle of a digital signal."]
    DutyCycle,
    #[doc="Angular position measurement using an angular encoder."]
    PositionAngEncoder,
    #[doc="Linear position measurement using a linear encoder."]
    PositionLinEncoder,
    #[doc="Angular velocity measurement using an angular encoder."]
    VelocityAngEncoder,
    #[doc="Linear velocity measurement using a linear encoder."]
    VelocityLinEncoder,
    #[doc="Measure time between edges of two digital signals."]
    TwoEdgeSep,
    #[doc=" Timestamp measurement, synchronizing the counter to a GPS receiver."]
    GpsTimestamp,
}

impl From<i32> for CIMeasurementType {
    fn from(val: i32) -> CIMeasurementType {
        match val {
            10125_i32 => CIMeasurementType::CountEdges,
            10179_i32 => CIMeasurementType::Freq,
            10256_i32 => CIMeasurementType::Period,
            10359_i32 => CIMeasurementType::PulseWidth,
            10289_i32 => CIMeasurementType::SemiPeriod,
            15864_i32 => CIMeasurementType::PulseFrequency,
            15865_i32 => CIMeasurementType::PulseTime,
            15866_i32 => CIMeasurementType::PulseTicks,
            16070_i32 => CIMeasurementType::DutyCycle,
            10360_i32 => CIMeasurementType::PositionAngEncoder,
            10361_i32 => CIMeasurementType::PositionLinEncoder,
            16078_i32 => CIMeasurementType::VelocityAngEncoder,
            16079_i32 => CIMeasurementType::VelocityLinEncoder,
            10267_i32 => CIMeasurementType::TwoEdgeSep,
            10362_i32 => CIMeasurementType::GpsTimestamp,
        }
    }
}

impl From<CIMeasurementType> for i32 {
    fn from(val: CIMeasurementType) -> i32 {
        match val {
            CIMeasurementType::CountEdges => 10125_i32,
            CIMeasurementType::Freq => 10179_i32,
            CIMeasurementType::Period => 10256_i32,
            CIMeasurementType::PulseWidth => 10359_i32,
            CIMeasurementType::SemiPeriod => 10289_i32,
            CIMeasurementType::PulseFrequency => 15864_i32,
            CIMeasurementType::PulseTime => 15865_i32,
            CIMeasurementType::PulseTicks => 15866_i32,
            CIMeasurementType::DutyCycle => 16070_i32,
            CIMeasurementType::PositionAngEncoder => 10360_i32,
            CIMeasurementType::PositionLinEncoder => 10361_i32,
            CIMeasurementType::VelocityAngEncoder => 16078_i32,
            CIMeasurementType::VelocityLinEncoder => 16079_i32,
            CIMeasurementType::TwoEdgeSep => 10267_i32,
            CIMeasurementType::GpsTimestamp => 10362_i32,
        }
    }
}

pub enum COOutputType {
    #[doc=" Generate pulses defined by the time the pulse is at a low state and the time  the pulse is at a high state."]
    PulseTime,
    #[doc="Generate digital pulses defined by frequency and duty cycle."]
    PulseFreq,
    #[doc=" Generate digital pulses defined by the number of timebase ticks that the pulse  is at a low state and the number of timebase ticks that the pulse is at a high  state."]
    PulseTicks,
}

impl From<i32> for COOutputType {
    fn from(val: i32) -> COOutputType {
        match val {
            10269_i32 => COOutputType::PulseTime,
            10119_i32 => COOutputType::PulseFreq,
            10268_i32 => COOutputType::PulseTicks,
        }
    }
}

impl From<COOutputType> for i32 {
    fn from(val: COOutputType) -> i32 {
        match val {
            COOutputType::PulseTime => 10269_i32,
            COOutputType::PulseFreq => 10119_i32,
            COOutputType::PulseTicks => 10268_i32,
        }
    }
}

pub enum CountDirection1 {
    #[doc="Increment counter."]
    CountUp,
    #[doc="Decrement counter."]
    CountDown,
    #[doc=" The state of a digital line controls the count direction. Each counter has a  default count direction terminal."]
    ExtControlled,
}

impl From<i32> for CountDirection1 {
    fn from(val: i32) -> CountDirection1 {
        match val {
            10128_i32 => CountDirection1::CountUp,
            10124_i32 => CountDirection1::CountDown,
            10326_i32 => CountDirection1::ExtControlled,
        }
    }
}

impl From<CountDirection1> for i32 {
    fn from(val: CountDirection1) -> i32 {
        match val {
            CountDirection1::CountUp => 10128_i32,
            CountDirection1::CountDown => 10124_i32,
            CountDirection1::ExtControlled => 10326_i32,
        }
    }
}

pub enum NavMeasurementType {
    Altitude,
    Longitude,
    Latitude,
    SpeedOverGround,
    Track,
    Timestamp,
    VertVelocity,
}

impl From<i32> for NavMeasurementType {
    fn from(val: i32) -> NavMeasurementType {
        match val {
            15997_i32 => NavMeasurementType::Altitude,
            15998_i32 => NavMeasurementType::Longitude,
            15999_i32 => NavMeasurementType::Latitude,
            16000_i32 => NavMeasurementType::SpeedOverGround,
            16001_i32 => NavMeasurementType::Track,
            15986_i32 => NavMeasurementType::Timestamp,
            16003_i32 => NavMeasurementType::VertVelocity,
        }
    }
}

impl From<NavMeasurementType> for i32 {
    fn from(val: NavMeasurementType) -> i32 {
        match val {
            NavMeasurementType::Altitude => 15997_i32,
            NavMeasurementType::Longitude => 15998_i32,
            NavMeasurementType::Latitude => 15999_i32,
            NavMeasurementType::SpeedOverGround => 16000_i32,
            NavMeasurementType::Track => 16001_i32,
            NavMeasurementType::Timestamp => 15986_i32,
            NavMeasurementType::VertVelocity => 16003_i32,
        }
    }
}

pub enum SampleInputDataWhen {
    #[doc="Latch data when the Handshake Trigger asserts."]
    HandshakeTriggerAsserts,
    #[doc="Latch data when the Handshake Trigger deasserts."]
    HandshakeTriggerDeasserts,
}

impl From<i32> for SampleInputDataWhen {
    fn from(val: i32) -> SampleInputDataWhen {
        match val {
            12552_i32 => SampleInputDataWhen::HandshakeTriggerAsserts,
            12553_i32 => SampleInputDataWhen::HandshakeTriggerDeasserts,
        }
    }
}

impl From<SampleInputDataWhen> for i32 {
    fn from(val: SampleInputDataWhen) -> i32 {
        match val {
            SampleInputDataWhen::HandshakeTriggerAsserts => 12552_i32,
            SampleInputDataWhen::HandshakeTriggerDeasserts => 12553_i32,
        }
    }
}

pub enum TEDSUnits {
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
    #[doc=" Units defined by TEDS information associated with the channel."]
    FromTeds,
}

impl From<i32> for TEDSUnits {
    fn from(val: i32) -> TEDSUnits {
        match val {
            10065_i32 => TEDSUnits::FromCustomScale,
            12516_i32 => TEDSUnits::FromTeds,
        }
    }
}

impl From<TEDSUnits> for i32 {
    fn from(val: TEDSUnits) -> i32 {
        match val {
            TEDSUnits::FromCustomScale => 10065_i32,
            TEDSUnits::FromTeds => 12516_i32,
        }
    }
}

pub enum SaveOptions {
    Overwrite,
    AllowInteractiveEditing,
    AllowInteractiveDeletion,
}

impl From<i32> for SaveOptions {
    fn from(val: i32) -> SaveOptions {
        match val {
            1_i32 => SaveOptions::Overwrite,
            2_i32 => SaveOptions::AllowInteractiveEditing,
            4_i32 => SaveOptions::AllowInteractiveDeletion,
        }
    }
}

impl From<SaveOptions> for i32 {
    fn from(val: SaveOptions) -> i32 {
        match val {
            SaveOptions::Overwrite => 1_i32,
            SaveOptions::AllowInteractiveEditing => 2_i32,
            SaveOptions::AllowInteractiveDeletion => 4_i32,
        }
    }
}

pub enum TorqueUnits {
    #[doc="Newton meters."]
    NewtonMeters,
    #[doc="Ounce-inches."]
    InchOunces,
    #[doc="Pound-inches."]
    InchPounds,
    #[doc="Pound-feet."]
    FootPounds,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for TorqueUnits {
    fn from(val: i32) -> TorqueUnits {
        match val {
            15881_i32 => TorqueUnits::NewtonMeters,
            15882_i32 => TorqueUnits::InchOunces,
            15883_i32 => TorqueUnits::InchPounds,
            15884_i32 => TorqueUnits::FootPounds,
            10065_i32 => TorqueUnits::FromCustomScale,
        }
    }
}

impl From<TorqueUnits> for i32 {
    fn from(val: TorqueUnits) -> i32 {
        match val {
            TorqueUnits::NewtonMeters => 15881_i32,
            TorqueUnits::InchOunces => 15882_i32,
            TorqueUnits::InchPounds => 15883_i32,
            TorqueUnits::FootPounds => 15884_i32,
            TorqueUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum WaitMode {
    #[doc=" Check for available samples when the system receives an interrupt service  request. This mode is the most CPU efficient, but results in lower possible  sampling rates."]
    WaitForInterrupt,
    #[doc=" Repeatedly check for available samples as fast as possible. This mode allows  for the highest sampling rates at the expense of CPU efficiency."]
    Poll,
    #[doc=" Repeatedly check for available samples, but yield control to other threads  after each check. This mode offers a balance between sampling rate and CPU  efficiency."]
    Yield,
    #[doc=" Check for available samples once per the amount of time specified in  DAQmx_Read_SleepTime."]
    Sleep,
}

impl From<i32> for WaitMode {
    fn from(val: i32) -> WaitMode {
        match val {
            12523_i32 => WaitMode::WaitForInterrupt,
            12524_i32 => WaitMode::Poll,
            12525_i32 => WaitMode::Yield,
            12547_i32 => WaitMode::Sleep,
        }
    }
}

impl From<WaitMode> for i32 {
    fn from(val: WaitMode) -> i32 {
        match val {
            WaitMode::WaitForInterrupt => 12523_i32,
            WaitMode::Poll => 12524_i32,
            WaitMode::Yield => 12525_i32,
            WaitMode::Sleep => 12547_i32,
        }
    }
}

pub enum SampleClockActiveOrInactiveEdgeSelection {
    #[doc="Active edges."]
    SampClkActiveEdge,
    #[doc="Inactive edges."]
    SampClkInactiveEdge,
}

impl From<i32> for SampleClockActiveOrInactiveEdgeSelection {
    fn from(val: i32) -> SampleClockActiveOrInactiveEdgeSelection {
        match val {
            14617_i32 => SampleClockActiveOrInactiveEdgeSelection::SampClkActiveEdge,
            14618_i32 => SampleClockActiveOrInactiveEdgeSelection::SampClkInactiveEdge,
        }
    }
}

impl From<SampleClockActiveOrInactiveEdgeSelection> for i32 {
    fn from(val: SampleClockActiveOrInactiveEdgeSelection) -> i32 {
        match val {
            SampleClockActiveOrInactiveEdgeSelection::SampClkActiveEdge => 14617_i32,
            SampleClockActiveOrInactiveEdgeSelection::SampClkInactiveEdge => 14618_i32,
        }
    }
}

pub enum ExcitationIdleOutputBehavior {
    #[doc="Drive excitation output to zero."]
    ZeroVoltsOrAmps,
    #[doc="Continue generating the current value."]
    MaintainExistingValue,
}

impl From<i32> for ExcitationIdleOutputBehavior {
    fn from(val: i32) -> ExcitationIdleOutputBehavior {
        match val {
            12526_i32 => ExcitationIdleOutputBehavior::ZeroVoltsOrAmps,
            12528_i32 => ExcitationIdleOutputBehavior::MaintainExistingValue,
        }
    }
}

impl From<ExcitationIdleOutputBehavior> for i32 {
    fn from(val: ExcitationIdleOutputBehavior) -> i32 {
        match val {
            ExcitationIdleOutputBehavior::ZeroVoltsOrAmps => 12526_i32,
            ExcitationIdleOutputBehavior::MaintainExistingValue => 12528_i32,
        }
    }
}

pub enum ScaleType3 {
    #[doc="Scale values by using an Nth order polynomial equation."]
    Polynomial,
    #[doc=" Map an array of prescaled values to an array of corresponding scaled values,  with all other values scaled proportionally."]
    Table,
    None,
}

impl From<i32> for ScaleType3 {
    fn from(val: i32) -> ScaleType3 {
        match val {
            10449_i32 => ScaleType3::Polynomial,
            10450_i32 => ScaleType3::Table,
            10230_i32 => ScaleType3::None,
        }
    }
}

impl From<ScaleType3> for i32 {
    fn from(val: ScaleType3) -> i32 {
        match val {
            ScaleType3::Polynomial => 10449_i32,
            ScaleType3::Table => 10450_i32,
            ScaleType3::None => 10230_i32,
        }
    }
}

pub enum CJCSource1 {
    #[doc=" Use a cold-junction compensation channel built into the terminal block."]
    BuiltIn,
    #[doc="You must specify the cold-junction temperature."]
    ConstVal,
    #[doc="Use a channel for cold-junction compensation."]
    Chan,
}

impl From<i32> for CJCSource1 {
    fn from(val: i32) -> CJCSource1 {
        match val {
            10200_i32 => CJCSource1::BuiltIn,
            10116_i32 => CJCSource1::ConstVal,
            10113_i32 => CJCSource1::Chan,
        }
    }
}

impl From<CJCSource1> for i32 {
    fn from(val: CJCSource1) -> i32 {
        match val {
            CJCSource1::BuiltIn => 10200_i32,
            CJCSource1::ConstVal => 10116_i32,
            CJCSource1::Chan => 10113_i32,
        }
    }
}

pub enum BridgePhysicalUnits {
    #[doc="Newtons."]
    Newtons,
    #[doc="Pounds."]
    Pounds,
    #[doc="kilograms-force."]
    KilogramForce,
    #[doc="Pascals."]
    Pascals,
    #[doc="Pounds per square inch."]
    PoundsPerSquareInch,
    #[doc="Bar."]
    Bar,
    #[doc="Newton metres."]
    NewtonMeters,
    #[doc="Ounce-inches."]
    InchOunces,
    #[doc="Pound-inches."]
    InchPounds,
    #[doc="Pound-feet."]
    FootPounds,
}

impl From<i32> for BridgePhysicalUnits {
    fn from(val: i32) -> BridgePhysicalUnits {
        match val {
            15875_i32 => BridgePhysicalUnits::Newtons,
            15876_i32 => BridgePhysicalUnits::Pounds,
            15877_i32 => BridgePhysicalUnits::KilogramForce,
            10081_i32 => BridgePhysicalUnits::Pascals,
            15879_i32 => BridgePhysicalUnits::PoundsPerSquareInch,
            15880_i32 => BridgePhysicalUnits::Bar,
            15881_i32 => BridgePhysicalUnits::NewtonMeters,
            15882_i32 => BridgePhysicalUnits::InchOunces,
            15883_i32 => BridgePhysicalUnits::InchPounds,
            15884_i32 => BridgePhysicalUnits::FootPounds,
        }
    }
}

impl From<BridgePhysicalUnits> for i32 {
    fn from(val: BridgePhysicalUnits) -> i32 {
        match val {
            BridgePhysicalUnits::Newtons => 15875_i32,
            BridgePhysicalUnits::Pounds => 15876_i32,
            BridgePhysicalUnits::KilogramForce => 15877_i32,
            BridgePhysicalUnits::Pascals => 10081_i32,
            BridgePhysicalUnits::PoundsPerSquareInch => 15879_i32,
            BridgePhysicalUnits::Bar => 15880_i32,
            BridgePhysicalUnits::NewtonMeters => 15881_i32,
            BridgePhysicalUnits::InchOunces => 15882_i32,
            BridgePhysicalUnits::InchPounds => 15883_i32,
            BridgePhysicalUnits::FootPounds => 15884_i32,
        }
    }
}

pub enum EncoderType2 {
    #[doc=" If signal A leads signal B, count the rising edges of signal A. If signal B  leads signal A, count the falling edges of signal A."]
    X1,
    #[doc="Count the rising and falling edges of signal A."]
    X2,
    #[doc="Count the rising and falling edges of signal A and signal B."]
    X4,
    #[doc=" Increment the count on rising edges of signal A. Decrement the count on rising  edges of signal B."]
    TwoPulseCounting,
}

impl From<i32> for EncoderType2 {
    fn from(val: i32) -> EncoderType2 {
        match val {
            10090_i32 => EncoderType2::X1,
            10091_i32 => EncoderType2::X2,
            10092_i32 => EncoderType2::X4,
            10313_i32 => EncoderType2::TwoPulseCounting,
        }
    }
}

impl From<EncoderType2> for i32 {
    fn from(val: EncoderType2) -> i32 {
        match val {
            EncoderType2::X1 => 10090_i32,
            EncoderType2::X2 => 10091_i32,
            EncoderType2::X4 => 10092_i32,
            EncoderType2::TwoPulseCounting => 10313_i32,
        }
    }
}

pub enum EveryNSamplesEventType {
    AcquiredIntoBuffer,
    TransferredFromBuffer,
}

impl From<i32> for EveryNSamplesEventType {
    fn from(val: i32) -> EveryNSamplesEventType {
        match val {
            1_i32 => EveryNSamplesEventType::AcquiredIntoBuffer,
            2_i32 => EveryNSamplesEventType::TransferredFromBuffer,
        }
    }
}

impl From<EveryNSamplesEventType> for i32 {
    fn from(val: EveryNSamplesEventType) -> i32 {
        match val {
            EveryNSamplesEventType::AcquiredIntoBuffer => 1_i32,
            EveryNSamplesEventType::TransferredFromBuffer => 2_i32,
        }
    }
}

pub enum FilterType1 {
    HardwareDefined,
}

impl From<i32> for FilterType1 {
    fn from(val: i32) -> FilterType1 {
        match val {
            10191_i32 => FilterType1::HardwareDefined,
        }
    }
}

impl From<FilterType1> for i32 {
    fn from(val: FilterType1) -> i32 {
        match val {
            FilterType1::HardwareDefined => 10191_i32,
        }
    }
}

pub enum GpsSignalType1 {
    #[doc=" Use the IRIG-B synchronization method. The GPS receiver sends one  synchronization pulse per second, as well as information about the number of  days, hours, minutes, and seconds that elapsed since the beginning of the  current year."]
    Irigb,
    #[doc=" Use the PPS synchronization method. The GPS receiver sends one synchronization  pulse per second, but does not send any timing information. The timestamp  measurement returns the number of seconds that elapsed since the device powered  up unless you set DAQmx_CI_Timestamp_InitialSeconds."]
    Pps,
    #[doc=" Do not synchronize the counter to a GPS receiver. The timestamp measurement  returns the number of seconds that elapsed since the device powered up unless  you set  DAQmx_CI_Timestamp_InitialSeconds."]
    None,
}

impl From<i32> for GpsSignalType1 {
    fn from(val: i32) -> GpsSignalType1 {
        match val {
            10070_i32 => GpsSignalType1::Irigb,
            10080_i32 => GpsSignalType1::Pps,
            10230_i32 => GpsSignalType1::None,
        }
    }
}

impl From<GpsSignalType1> for i32 {
    fn from(val: GpsSignalType1) -> i32 {
        match val {
            GpsSignalType1::Irigb => 10070_i32,
            GpsSignalType1::Pps => 10080_i32,
            GpsSignalType1::None => 10230_i32,
        }
    }
}

pub enum OutputDataTransferCondition {
    #[doc=" Transfer data to the device only when there is no data in the onboard memory of  the device."]
    OnBrdMemEmpty,
    #[doc=" Transfer data to the device any time the onboard memory is less than half full."]
    OnBrdMemHalfFullOrLess,
    #[doc=" Transfer data to the device any time the onboard memory of the device is not  full."]
    OnBrdMemNotFull,
}

impl From<i32> for OutputDataTransferCondition {
    fn from(val: i32) -> OutputDataTransferCondition {
        match val {
            10235_i32 => OutputDataTransferCondition::OnBrdMemEmpty,
            10239_i32 => OutputDataTransferCondition::OnBrdMemHalfFullOrLess,
            10242_i32 => OutputDataTransferCondition::OnBrdMemNotFull,
        }
    }
}

impl From<OutputDataTransferCondition> for i32 {
    fn from(val: OutputDataTransferCondition) -> i32 {
        match val {
            OutputDataTransferCondition::OnBrdMemEmpty => 10235_i32,
            OutputDataTransferCondition::OnBrdMemHalfFullOrLess => 10239_i32,
            OutputDataTransferCondition::OnBrdMemNotFull => 10242_i32,
        }
    }
}

pub enum TemperatureUnits {
    DegC,
    DegF,
    Kelvins,
    DegR,
}

impl From<i32> for TemperatureUnits {
    fn from(val: i32) -> TemperatureUnits {
        match val {
            10143_i32 => TemperatureUnits::DegC,
            10144_i32 => TemperatureUnits::DegF,
            10325_i32 => TemperatureUnits::Kelvins,
            10145_i32 => TemperatureUnits::DegR,
        }
    }
}

impl From<TemperatureUnits> for i32 {
    fn from(val: TemperatureUnits) -> i32 {
        match val {
            TemperatureUnits::DegC => 10143_i32,
            TemperatureUnits::DegF => 10144_i32,
            TemperatureUnits::Kelvins => 10325_i32,
            TemperatureUnits::DegR => 10145_i32,
        }
    }
}

pub enum ExportActions5 {
    #[doc=" Handshake Event deasserts after the Handshake Trigger asserts, plus the amount  of time specified with DAQmx_Exported_HshkEvent_Interlocked_DeassertDelay."]
    Interlocked,
    #[doc=" Handshake Event pulses with the pulse width specified in  DAQmx_Exported_HshkEvent_Pulse_Width."]
    Pulse,
}

impl From<i32> for ExportActions5 {
    fn from(val: i32) -> ExportActions5 {
        match val {
            12549_i32 => ExportActions5::Interlocked,
            10265_i32 => ExportActions5::Pulse,
        }
    }
}

impl From<ExportActions5> for i32 {
    fn from(val: ExportActions5) -> i32 {
        match val {
            ExportActions5::Interlocked => 12549_i32,
            ExportActions5::Pulse => 10265_i32,
        }
    }
}

pub enum FrequencyUnits2 {
    #[doc="Hertz."]
    Hz,
}

impl From<i32> for FrequencyUnits2 {
    fn from(val: i32) -> FrequencyUnits2 {
        match val {
            10373_i32 => FrequencyUnits2::Hz,
        }
    }
}

impl From<FrequencyUnits2> for i32 {
    fn from(val: FrequencyUnits2) -> i32 {
        match val {
            FrequencyUnits2::Hz => 10373_i32,
        }
    }
}

pub enum ScaleType2 {
    #[doc="Scale values by using an Nth order polynomial equation."]
    Polynomial,
    #[doc=" Map an array of prescaled values to an array of corresponding scaled values,  with all other values scaled proportionally."]
    Table,
}

impl From<i32> for ScaleType2 {
    fn from(val: i32) -> ScaleType2 {
        match val {
            10449_i32 => ScaleType2::Polynomial,
            10450_i32 => ScaleType2::Table,
        }
    }
}

impl From<ScaleType2> for i32 {
    fn from(val: ScaleType2) -> i32 {
        match val {
            ScaleType2::Polynomial => 10449_i32,
            ScaleType2::Table => 10450_i32,
        }
    }
}

pub enum InputTermCfg2 {
    #[doc="Differential."]
    Diff,
    #[doc="Referenced Single-Ended."]
    Rse,
}

impl From<i32> for InputTermCfg2 {
    fn from(val: i32) -> InputTermCfg2 {
        match val {
            10106_i32 => InputTermCfg2::Diff,
            10083_i32 => InputTermCfg2::Rse,
        }
    }
}

impl From<InputTermCfg2> for i32 {
    fn from(val: InputTermCfg2) -> i32 {
        match val {
            InputTermCfg2::Diff => 10106_i32,
            InputTermCfg2::Rse => 10083_i32,
        }
    }
}

pub enum PowerUpChannelType {
    ChannelVoltage,
    ChannelCurrent,
    ChannelHighImpedance,
}

impl From<i32> for PowerUpChannelType {
    fn from(val: i32) -> PowerUpChannelType {
        match val {
            0_i32 => PowerUpChannelType::ChannelVoltage,
            1_i32 => PowerUpChannelType::ChannelCurrent,
            2_i32 => PowerUpChannelType::ChannelHighImpedance,
        }
    }
}

impl From<PowerUpChannelType> for i32 {
    fn from(val: PowerUpChannelType) -> i32 {
        match val {
            PowerUpChannelType::ChannelVoltage => 0_i32,
            PowerUpChannelType::ChannelCurrent => 1_i32,
            PowerUpChannelType::ChannelHighImpedance => 2_i32,
        }
    }
}

pub enum TriggerType9 {
    #[doc=" Use the Handshake Trigger as a control signal for asynchronous handshaking,  such as 8255 handshaking."]
    Interlocked,
    #[doc=" Start the measurement or generation immediately when you start the task."]
    None,
}

impl From<i32> for TriggerType9 {
    fn from(val: i32) -> TriggerType9 {
        match val {
            12549_i32 => TriggerType9::Interlocked,
            10230_i32 => TriggerType9::None,
        }
    }
}

impl From<TriggerType9> for i32 {
    fn from(val: TriggerType9) -> i32 {
        match val {
            TriggerType9::Interlocked => 12549_i32,
            TriggerType9::None => 10230_i32,
        }
    }
}

pub enum ADCTimingMode {
    #[doc=" Uses the most appropriate supported timing mode based on the Sample Clock Rate."]
    Automatic,
    #[doc=" Increases resolution and noise rejection while decreasing conversion rate."]
    HighResolution,
    #[doc="Increases conversion rate while decreasing resolution."]
    HighSpeed,
    #[doc=" Improves 50 Hz noise rejection while decreasing noise rejection at other  frequencies."]
    Best50HzRejection,
    #[doc=" Improves 60 Hz noise rejection while decreasing noise rejection at other  frequencies."]
    Best60HzRejection,
    #[doc=" Use DAQmx_AI_ADCCustomTimingMode to specify a custom value controlling the  tradeoff between speed and resolution."]
    Custom,
}

impl From<i32> for ADCTimingMode {
    fn from(val: i32) -> ADCTimingMode {
        match val {
            16097_i32 => ADCTimingMode::Automatic,
            10195_i32 => ADCTimingMode::HighResolution,
            14712_i32 => ADCTimingMode::HighSpeed,
            14713_i32 => ADCTimingMode::Best50HzRejection,
            14714_i32 => ADCTimingMode::Best60HzRejection,
            10137_i32 => ADCTimingMode::Custom,
        }
    }
}

impl From<ADCTimingMode> for i32 {
    fn from(val: ADCTimingMode) -> i32 {
        match val {
            ADCTimingMode::Automatic => 16097_i32,
            ADCTimingMode::HighResolution => 10195_i32,
            ADCTimingMode::HighSpeed => 14712_i32,
            ADCTimingMode::Best50HzRejection => 14713_i32,
            ADCTimingMode::Best60HzRejection => 14714_i32,
            ADCTimingMode::Custom => 10137_i32,
        }
    }
}

pub enum ChannelType {
    #[doc="Analog input channel."]
    Ai,
    #[doc="Analog output channel."]
    Ao,
    #[doc="Digital input channel."]
    Di,
    #[doc="Digital output channel."]
    Do,
    #[doc="Counter input channel."]
    Ci,
    #[doc="Counter output channel."]
    Co,
}

impl From<i32> for ChannelType {
    fn from(val: i32) -> ChannelType {
        match val {
            10100_i32 => ChannelType::Ai,
            10102_i32 => ChannelType::Ao,
            10151_i32 => ChannelType::Di,
            10153_i32 => ChannelType::Do,
            10131_i32 => ChannelType::Ci,
            10132_i32 => ChannelType::Co,
        }
    }
}

impl From<ChannelType> for i32 {
    fn from(val: ChannelType) -> i32 {
        match val {
            ChannelType::Ai => 10100_i32,
            ChannelType::Ao => 10102_i32,
            ChannelType::Di => 10151_i32,
            ChannelType::Do => 10153_i32,
            ChannelType::Ci => 10131_i32,
            ChannelType::Co => 10132_i32,
        }
    }
}

pub enum CurrentUnits1 {
    #[doc="Amperes."]
    Amps,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
    #[doc=" Units defined by TEDS information associated with the channel."]
    FromTeds,
}

impl From<i32> for CurrentUnits1 {
    fn from(val: i32) -> CurrentUnits1 {
        match val {
            10342_i32 => CurrentUnits1::Amps,
            10065_i32 => CurrentUnits1::FromCustomScale,
            12516_i32 => CurrentUnits1::FromTeds,
        }
    }
}

impl From<CurrentUnits1> for i32 {
    fn from(val: CurrentUnits1) -> i32 {
        match val {
            CurrentUnits1::Amps => 10342_i32,
            CurrentUnits1::FromCustomScale => 10065_i32,
            CurrentUnits1::FromTeds => 12516_i32,
        }
    }
}

pub enum FrequencyUnits {
    #[doc="Hertz."]
    Hz,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for FrequencyUnits {
    fn from(val: i32) -> FrequencyUnits {
        match val {
            10373_i32 => FrequencyUnits::Hz,
            10065_i32 => FrequencyUnits::FromCustomScale,
        }
    }
}

impl From<FrequencyUnits> for i32 {
    fn from(val: FrequencyUnits) -> i32 {
        match val {
            FrequencyUnits::Hz => 10373_i32,
            FrequencyUnits::FromCustomScale => 10065_i32,
        }
    }
}

pub enum SampClkOverrunBehavior {
    #[doc="Repeat the last sample."]
    RepeatedData,
    #[doc="Return the sentinel value."]
    SentinelValue,
}

impl From<i32> for SampClkOverrunBehavior {
    fn from(val: i32) -> SampClkOverrunBehavior {
        match val {
            16062_i32 => SampClkOverrunBehavior::RepeatedData,
            16063_i32 => SampClkOverrunBehavior::SentinelValue,
        }
    }
}

impl From<SampClkOverrunBehavior> for i32 {
    fn from(val: SampClkOverrunBehavior) -> i32 {
        match val {
            SampClkOverrunBehavior::RepeatedData => 16062_i32,
            SampClkOverrunBehavior::SentinelValue => 16063_i32,
        }
    }
}

pub enum StrainGageBridgeType1 {
    #[doc=" Four active gages with two pairs subjected to equal and opposite strains."]
    FullBridgeI,
    #[doc=" Four active gages with two aligned with maximum principal strain and two  Poisson gages in adjacent arms."]
    FullBridgeIi,
    #[doc=" Four active gages with two aligned with maximum principal strain and two  Poisson gages in opposite arms."]
    FullBridgeIii,
    #[doc=" Two active gages with one aligned with maximum principal strain and one Poisson  gage."]
    HalfBridgeI,
    #[doc="Two active gages with equal and opposite strains."]
    HalfBridgeIi,
    #[doc="Single active gage."]
    QuarterBridgeI,
    #[doc="Single active gage and one dummy gage."]
    QuarterBridgeIi,
}

impl From<i32> for StrainGageBridgeType1 {
    fn from(val: i32) -> StrainGageBridgeType1 {
        match val {
            10183_i32 => StrainGageBridgeType1::FullBridgeI,
            10184_i32 => StrainGageBridgeType1::FullBridgeIi,
            10185_i32 => StrainGageBridgeType1::FullBridgeIii,
            10188_i32 => StrainGageBridgeType1::HalfBridgeI,
            10189_i32 => StrainGageBridgeType1::HalfBridgeIi,
            10271_i32 => StrainGageBridgeType1::QuarterBridgeI,
            10272_i32 => StrainGageBridgeType1::QuarterBridgeIi,
        }
    }
}

impl From<StrainGageBridgeType1> for i32 {
    fn from(val: StrainGageBridgeType1) -> i32 {
        match val {
            StrainGageBridgeType1::FullBridgeI => 10183_i32,
            StrainGageBridgeType1::FullBridgeIi => 10184_i32,
            StrainGageBridgeType1::FullBridgeIii => 10185_i32,
            StrainGageBridgeType1::HalfBridgeI => 10188_i32,
            StrainGageBridgeType1::HalfBridgeIi => 10189_i32,
            StrainGageBridgeType1::QuarterBridgeI => 10271_i32,
            StrainGageBridgeType1::QuarterBridgeIi => 10272_i32,
        }
    }
}

pub enum TimeUnits3 {
    #[doc="Seconds."]
    Seconds,
    #[doc="Timebase ticks."]
    Ticks,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for TimeUnits3 {
    fn from(val: i32) -> TimeUnits3 {
        match val {
            10364_i32 => TimeUnits3::Seconds,
            10304_i32 => TimeUnits3::Ticks,
            10065_i32 => TimeUnits3::FromCustomScale,
        }
    }
}

impl From<TimeUnits3> for i32 {
    fn from(val: TimeUnits3) -> i32 {
        match val {
            TimeUnits3::Seconds => 10364_i32,
            TimeUnits3::Ticks => 10304_i32,
            TimeUnits3::FromCustomScale => 10065_i32,
        }
    }
}

pub enum TriggerType6 {
    #[doc=" Pause the measurement or generation while an analog signal is above or below a  level."]
    AnlgLvl,
    #[doc=" Pause the measurement or generation while an analog signal is either inside or  outside of a range of values."]
    AnlgWin,
    #[doc=" Pause the measurement or generation while a digital signal is at either a high  or low state."]
    DigLvl,
    #[doc=" Pause the measurement or generation while digital physical channels either  match or do not match a digital pattern."]
    DigPattern,
    #[doc="Do not pause the measurement or generation."]
    None,
}

impl From<i32> for TriggerType6 {
    fn from(val: i32) -> TriggerType6 {
        match val {
            10101_i32 => TriggerType6::AnlgLvl,
            10103_i32 => TriggerType6::AnlgWin,
            10152_i32 => TriggerType6::DigLvl,
            10398_i32 => TriggerType6::DigPattern,
            10230_i32 => TriggerType6::None,
        }
    }
}

impl From<TriggerType6> for i32 {
    fn from(val: TriggerType6) -> i32 {
        match val {
            TriggerType6::AnlgLvl => 10101_i32,
            TriggerType6::AnlgWin => 10103_i32,
            TriggerType6::DigLvl => 10152_i32,
            TriggerType6::DigPattern => 10398_i32,
            TriggerType6::None => 10230_i32,
        }
    }
}

pub enum RVDTSensitivityUnits1 {
    #[doc="mVolts/Volt/Degree."]
    MVoltsPerVoltPerDegree,
    #[doc="mVolts/Volt/Radian."]
    MVoltsPerVoltPerRadian,
}

impl From<i32> for RVDTSensitivityUnits1 {
    fn from(val: i32) -> RVDTSensitivityUnits1 {
        match val {
            12507_i32 => RVDTSensitivityUnits1::MVoltsPerVoltPerDegree,
            12508_i32 => RVDTSensitivityUnits1::MVoltsPerVoltPerRadian,
        }
    }
}

impl From<RVDTSensitivityUnits1> for i32 {
    fn from(val: RVDTSensitivityUnits1) -> i32 {
        match val {
            RVDTSensitivityUnits1::MVoltsPerVoltPerDegree => 12507_i32,
            RVDTSensitivityUnits1::MVoltsPerVoltPerRadian => 12508_i32,
        }
    }
}

pub enum ScaleType {
    #[doc=" Scale values by using the equation y=mx+b, where x is a prescaled value and y  is a scaled value."]
    Linear,
    #[doc=" Scale values proportionally from a range of pre-scaled values to a range of  scaled values."]
    MapRanges,
    #[doc="Scale values by using an Nth order polynomial equation."]
    Polynomial,
    #[doc=" Map an array of pre-scaled values to an array of corresponding scaled values,  with all other values scaled proportionally."]
    Table,
}

impl From<i32> for ScaleType {
    fn from(val: i32) -> ScaleType {
        match val {
            10447_i32 => ScaleType::Linear,
            10448_i32 => ScaleType::MapRanges,
            10449_i32 => ScaleType::Polynomial,
            10450_i32 => ScaleType::Table,
        }
    }
}

impl From<ScaleType> for i32 {
    fn from(val: ScaleType) -> i32 {
        match val {
            ScaleType::Linear => 10447_i32,
            ScaleType::MapRanges => 10448_i32,
            ScaleType::Polynomial => 10449_i32,
            ScaleType::Table => 10450_i32,
        }
    }
}

pub enum FilterType2 {
    #[doc="Lowpass filter."]
    Lowpass,
    #[doc="Highpass filter."]
    Highpass,
    #[doc="Bandpass filter."]
    Bandpass,
    #[doc="Notch filter."]
    Notch,
    #[doc="Custom filter."]
    Custom,
}

impl From<i32> for FilterType2 {
    fn from(val: i32) -> FilterType2 {
        match val {
            16071_i32 => FilterType2::Lowpass,
            16072_i32 => FilterType2::Highpass,
            16073_i32 => FilterType2::Bandpass,
            16074_i32 => FilterType2::Notch,
            10137_i32 => FilterType2::Custom,
        }
    }
}

impl From<FilterType2> for i32 {
    fn from(val: FilterType2) -> i32 {
        match val {
            FilterType2::Lowpass => 16071_i32,
            FilterType2::Highpass => 16072_i32,
            FilterType2::Bandpass => 16073_i32,
            FilterType2::Notch => 16074_i32,
            FilterType2::Custom => 10137_i32,
        }
    }
}

pub enum FilterResponse1 {
    #[doc="Comb filter response."]
    Comb,
    #[doc="Bessel filter response."]
    Bessel,
    #[doc="Brickwall filter response."]
    Brickwall,
    #[doc="Butterworth filter response."]
    Butterworth,
}

impl From<i32> for FilterResponse1 {
    fn from(val: i32) -> FilterResponse1 {
        match val {
            16152_i32 => FilterResponse1::Comb,
            16153_i32 => FilterResponse1::Bessel,
            16155_i32 => FilterResponse1::Brickwall,
            16076_i32 => FilterResponse1::Butterworth,
        }
    }
}

impl From<FilterResponse1> for i32 {
    fn from(val: FilterResponse1) -> i32 {
        match val {
            FilterResponse1::Comb => 16152_i32,
            FilterResponse1::Bessel => 16153_i32,
            FilterResponse1::Brickwall => 16155_i32,
            FilterResponse1::Butterworth => 16076_i32,
        }
    }
}

pub enum DigitalWidthUnits1 {
    #[doc="Complete periods of the Sample Clock."]
    SampClkPeriods,
    #[doc="Seconds."]
    Seconds,
    #[doc="Timebase ticks."]
    Ticks,
}

impl From<i32> for DigitalWidthUnits1 {
    fn from(val: i32) -> DigitalWidthUnits1 {
        match val {
            10286_i32 => DigitalWidthUnits1::SampClkPeriods,
            10364_i32 => DigitalWidthUnits1::Seconds,
            10304_i32 => DigitalWidthUnits1::Ticks,
        }
    }
}

impl From<DigitalWidthUnits1> for i32 {
    fn from(val: DigitalWidthUnits1) -> i32 {
        match val {
            DigitalWidthUnits1::SampClkPeriods => 10286_i32,
            DigitalWidthUnits1::Seconds => 10364_i32,
            DigitalWidthUnits1::Ticks => 10304_i32,
        }
    }
}

pub enum EddyCurrentProxProbeSensitivityUnits {
    #[doc="mVolts/mil."]
    MVoltsPerMil,
    #[doc="Volts/mil."]
    VoltsPerMil,
    #[doc="mVolts/mMeter."]
    MVoltsPerMillimeter,
    #[doc="Volts/mMeter."]
    VoltsPerMillimeter,
    #[doc="mVolts/micron."]
    MVoltsPerMicron,
}

impl From<i32> for EddyCurrentProxProbeSensitivityUnits {
    fn from(val: i32) -> EddyCurrentProxProbeSensitivityUnits {
        match val {
            14836_i32 => EddyCurrentProxProbeSensitivityUnits::MVoltsPerMil,
            14837_i32 => EddyCurrentProxProbeSensitivityUnits::VoltsPerMil,
            14838_i32 => EddyCurrentProxProbeSensitivityUnits::MVoltsPerMillimeter,
            14839_i32 => EddyCurrentProxProbeSensitivityUnits::VoltsPerMillimeter,
            14840_i32 => EddyCurrentProxProbeSensitivityUnits::MVoltsPerMicron,
        }
    }
}

impl From<EddyCurrentProxProbeSensitivityUnits> for i32 {
    fn from(val: EddyCurrentProxProbeSensitivityUnits) -> i32 {
        match val {
            EddyCurrentProxProbeSensitivityUnits::MVoltsPerMil => 14836_i32,
            EddyCurrentProxProbeSensitivityUnits::VoltsPerMil => 14837_i32,
            EddyCurrentProxProbeSensitivityUnits::MVoltsPerMillimeter => 14838_i32,
            EddyCurrentProxProbeSensitivityUnits::VoltsPerMillimeter => 14839_i32,
            EddyCurrentProxProbeSensitivityUnits::MVoltsPerMicron => 14840_i32,
        }
    }
}

pub enum AltRef {
    Msl,
    Hae,
}

impl From<i32> for AltRef {
    fn from(val: i32) -> AltRef {
        match val {
            16005_i32 => AltRef::Msl,
            16006_i32 => AltRef::Hae,
        }
    }
}

impl From<AltRef> for i32 {
    fn from(val: AltRef) -> i32 {
        match val {
            AltRef::Msl => 16005_i32,
            AltRef::Hae => 16006_i32,
        }
    }
}

pub enum AntStatus {
    Unknown,
    Normal,
    Absent,
    Overcurrent,
}

impl From<i32> for AntStatus {
    fn from(val: i32) -> AntStatus {
        match val {
            12588_i32 => AntStatus::Unknown,
            10459_i32 => AntStatus::Normal,
            15994_i32 => AntStatus::Absent,
            15995_i32 => AntStatus::Overcurrent,
        }
    }
}

impl From<AntStatus> for i32 {
    fn from(val: AntStatus) -> i32 {
        match val {
            AntStatus::Unknown => 12588_i32,
            AntStatus::Normal => 10459_i32,
            AntStatus::Absent => 15994_i32,
            AntStatus::Overcurrent => 15995_i32,
        }
    }
}

pub enum Coupling1 {
    #[doc="Remove the DC offset from the signal."]
    Ac,
    #[doc="Allow NI-DAQmx to measure all of the signal."]
    Dc,
    #[doc=" Remove the signal from the measurement and measure only ground."]
    Gnd,
}

impl From<i32> for Coupling1 {
    fn from(val: i32) -> Coupling1 {
        match val {
            10045_i32 => Coupling1::Ac,
            10050_i32 => Coupling1::Dc,
            10066_i32 => Coupling1::Gnd,
        }
    }
}

impl From<Coupling1> for i32 {
    fn from(val: Coupling1) -> i32 {
        match val {
            Coupling1::Ac => 10045_i32,
            Coupling1::Dc => 10050_i32,
            Coupling1::Gnd => 10066_i32,
        }
    }
}

pub enum Slope1 {
    #[doc="Trigger on the rising slope of the signal."]
    RisingSlope,
    #[doc="Trigger on the falling slope of the signal."]
    FallingSlope,
}

impl From<i32> for Slope1 {
    fn from(val: i32) -> Slope1 {
        match val {
            10280_i32 => Slope1::RisingSlope,
            10171_i32 => Slope1::FallingSlope,
        }
    }
}

impl From<Slope1> for i32 {
    fn from(val: Slope1) -> i32 {
        match val {
            Slope1::RisingSlope => 10280_i32,
            Slope1::FallingSlope => 10171_i32,
        }
    }
}

pub enum SyncPulseType {
    #[doc="Use the synchronization pulse type specified by the device."]
    Onboard,
    #[doc="Digital Edge synchronization."]
    DigEdge,
    #[doc="Time synchronization."]
    Time,
}

impl From<i32> for SyncPulseType {
    fn from(val: i32) -> SyncPulseType {
        match val {
            16128_i32 => SyncPulseType::Onboard,
            10150_i32 => SyncPulseType::DigEdge,
            15996_i32 => SyncPulseType::Time,
        }
    }
}

impl From<SyncPulseType> for i32 {
    fn from(val: SyncPulseType) -> i32 {
        match val {
            SyncPulseType::Onboard => 16128_i32,
            SyncPulseType::DigEdge => 10150_i32,
            SyncPulseType::Time => 15996_i32,
        }
    }
}

pub enum TriggerType10 {
    #[doc="Trigger when an analog signal signal crosses a threshold."]
    AnlgEdge,
    #[doc=" Trigger when any of the configured analog signals cross their respective  thresholds."]
    AnlgMultiEdge,
    #[doc="Trigger on the rising or falling edge of a digital signal."]
    DigEdge,
    #[doc=" Trigger when digital physical channels match a digital pattern."]
    DigPattern,
    #[doc=" Trigger when an analog signal enters or leaves a range of values. The range is  in the units of the measurement."]
    AnlgWin,
    #[doc="Trigger when a specified time is reached."]
    Time,
    #[doc="Disable triggering for the task."]
    None,
}

impl From<i32> for TriggerType10 {
    fn from(val: i32) -> TriggerType10 {
        match val {
            10099_i32 => TriggerType10::AnlgEdge,
            16108_i32 => TriggerType10::AnlgMultiEdge,
            10150_i32 => TriggerType10::DigEdge,
            10398_i32 => TriggerType10::DigPattern,
            10103_i32 => TriggerType10::AnlgWin,
            15996_i32 => TriggerType10::Time,
            10230_i32 => TriggerType10::None,
        }
    }
}

impl From<TriggerType10> for i32 {
    fn from(val: TriggerType10) -> i32 {
        match val {
            TriggerType10::AnlgEdge => 10099_i32,
            TriggerType10::AnlgMultiEdge => 16108_i32,
            TriggerType10::DigEdge => 10150_i32,
            TriggerType10::DigPattern => 10398_i32,
            TriggerType10::AnlgWin => 10103_i32,
            TriggerType10::Time => 15996_i32,
            TriggerType10::None => 10230_i32,
        }
    }
}

pub enum VoltageUnits1 {
    #[doc="Volts."]
    Volts,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
    #[doc=" Units defined by TEDS information associated with the channel."]
    FromTeds,
}

impl From<i32> for VoltageUnits1 {
    fn from(val: i32) -> VoltageUnits1 {
        match val {
            10348_i32 => VoltageUnits1::Volts,
            10065_i32 => VoltageUnits1::FromCustomScale,
            12516_i32 => VoltageUnits1::FromTeds,
        }
    }
}

impl From<VoltageUnits1> for i32 {
    fn from(val: VoltageUnits1) -> i32 {
        match val {
            VoltageUnits1::Volts => 10348_i32,
            VoltageUnits1::FromCustomScale => 10065_i32,
            VoltageUnits1::FromTeds => 12516_i32,
        }
    }
}

pub enum AIMeasurementType {
    #[doc="Voltage measurement."]
    Voltage,
    #[doc="Voltage RMS measurement."]
    VoltageRms,
    #[doc="Current measurement."]
    Current,
    #[doc="Current RMS measurement."]
    CurrentRms,
    #[doc=" Voltage measurement with an excitation source. You can use this measurement  type for custom sensors that require excitation, but you must use a custom  scale to scale the measured voltage."]
    VoltageCustomWithExcitation,
    #[doc="Measure voltage ratios from a Wheatstone bridge."]
    Bridge,
    #[doc=" Frequency measurement using a frequency to voltage converter."]
    FreqVoltage,
    #[doc="Resistance measurement."]
    Resistance,
    #[doc="Temperature measurement using a thermocouple."]
    TempTc,
    #[doc="Temperature measurement using a thermistor."]
    TempThrmstr,
    #[doc="Temperature measurement using an RTD."]
    TempRtd,
    #[doc=" Temperature measurement using a built-in sensor on a terminal block or device.  On SCXI modules, for example, this could be the CJC sensor."]
    TempBuiltInSensor,
    #[doc="Strain measurement."]
    StrainGage,
    #[doc="Strain measurement using a rosette strain gage."]
    RosetteStrainGage,
    #[doc="Position measurement using an LVDT."]
    PositionLvdt,
    #[doc="Position measurement using an RVDT."]
    PositionRvdt,
    #[doc="Position measurement using an eddy current proximity probe."]
    PositionEddyCurrentProximityProbe,
    #[doc="Acceleration measurement using an accelerometer."]
    Accelerometer,
    #[doc="Acceleration measurement using a charge-based sensor."]
    AccelerationCharge,
    #[doc=" Acceleration measurement using a 4 wire DC voltage based sensor."]
    Acceleration4WireDcVoltage,
    #[doc="Velocity measurement using an IEPE Sensor."]
    VelocityIepeSensor,
    #[doc="Force measurement using a bridge-based sensor."]
    ForceBridge,
    #[doc="Force measurement using an IEPE Sensor."]
    ForceIepeSensor,
    #[doc="Pressure measurement using a bridge-based sensor."]
    PressureBridge,
    #[doc="Sound pressure measurement using a microphone."]
    SoundPressureMicrophone,
    #[doc="Torque measurement using a bridge-based sensor."]
    TorqueBridge,
    #[doc="Measurement type defined by TEDS."]
    TedsSensor,
    #[doc="Charge measurement."]
    Charge,
}

impl From<i32> for AIMeasurementType {
    fn from(val: i32) -> AIMeasurementType {
        match val {
            10322_i32 => AIMeasurementType::Voltage,
            10350_i32 => AIMeasurementType::VoltageRms,
            10134_i32 => AIMeasurementType::Current,
            10351_i32 => AIMeasurementType::CurrentRms,
            10323_i32 => AIMeasurementType::VoltageCustomWithExcitation,
            15908_i32 => AIMeasurementType::Bridge,
            10181_i32 => AIMeasurementType::FreqVoltage,
            10278_i32 => AIMeasurementType::Resistance,
            10303_i32 => AIMeasurementType::TempTc,
            10302_i32 => AIMeasurementType::TempThrmstr,
            10301_i32 => AIMeasurementType::TempRtd,
            10311_i32 => AIMeasurementType::TempBuiltInSensor,
            10300_i32 => AIMeasurementType::StrainGage,
            15980_i32 => AIMeasurementType::RosetteStrainGage,
            10352_i32 => AIMeasurementType::PositionLvdt,
            10353_i32 => AIMeasurementType::PositionRvdt,
            14835_i32 => AIMeasurementType::PositionEddyCurrentProximityProbe,
            10356_i32 => AIMeasurementType::Accelerometer,
            16104_i32 => AIMeasurementType::AccelerationCharge,
            16106_i32 => AIMeasurementType::Acceleration4WireDcVoltage,
            15966_i32 => AIMeasurementType::VelocityIepeSensor,
            15899_i32 => AIMeasurementType::ForceBridge,
            15895_i32 => AIMeasurementType::ForceIepeSensor,
            15902_i32 => AIMeasurementType::PressureBridge,
            10354_i32 => AIMeasurementType::SoundPressureMicrophone,
            15905_i32 => AIMeasurementType::TorqueBridge,
            12531_i32 => AIMeasurementType::TedsSensor,
            16105_i32 => AIMeasurementType::Charge,
        }
    }
}

impl From<AIMeasurementType> for i32 {
    fn from(val: AIMeasurementType) -> i32 {
        match val {
            AIMeasurementType::Voltage => 10322_i32,
            AIMeasurementType::VoltageRms => 10350_i32,
            AIMeasurementType::Current => 10134_i32,
            AIMeasurementType::CurrentRms => 10351_i32,
            AIMeasurementType::VoltageCustomWithExcitation => 10323_i32,
            AIMeasurementType::Bridge => 15908_i32,
            AIMeasurementType::FreqVoltage => 10181_i32,
            AIMeasurementType::Resistance => 10278_i32,
            AIMeasurementType::TempTc => 10303_i32,
            AIMeasurementType::TempThrmstr => 10302_i32,
            AIMeasurementType::TempRtd => 10301_i32,
            AIMeasurementType::TempBuiltInSensor => 10311_i32,
            AIMeasurementType::StrainGage => 10300_i32,
            AIMeasurementType::RosetteStrainGage => 15980_i32,
            AIMeasurementType::PositionLvdt => 10352_i32,
            AIMeasurementType::PositionRvdt => 10353_i32,
            AIMeasurementType::PositionEddyCurrentProximityProbe => 14835_i32,
            AIMeasurementType::Accelerometer => 10356_i32,
            AIMeasurementType::AccelerationCharge => 16104_i32,
            AIMeasurementType::Acceleration4WireDcVoltage => 16106_i32,
            AIMeasurementType::VelocityIepeSensor => 15966_i32,
            AIMeasurementType::ForceBridge => 15899_i32,
            AIMeasurementType::ForceIepeSensor => 15895_i32,
            AIMeasurementType::PressureBridge => 15902_i32,
            AIMeasurementType::SoundPressureMicrophone => 10354_i32,
            AIMeasurementType::TorqueBridge => 15905_i32,
            AIMeasurementType::TedsSensor => 12531_i32,
            AIMeasurementType::Charge => 16105_i32,
        }
    }
}

pub enum VoltageUnits2 {
    #[doc="Volts."]
    Volts,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for VoltageUnits2 {
    fn from(val: i32) -> VoltageUnits2 {
        match val {
            10348_i32 => VoltageUnits2::Volts,
            10065_i32 => VoltageUnits2::FromCustomScale,
        }
    }
}

impl From<VoltageUnits2> for i32 {
    fn from(val: VoltageUnits2) -> i32 {
        match val {
            VoltageUnits2::Volts => 10348_i32,
            VoltageUnits2::FromCustomScale => 10065_i32,
        }
    }
}

pub enum ConstrainedGenMode {
    #[doc="Counter has no restrictions on pulse generation."]
    Unconstrained,
    #[doc=" Pulse frequency must be above 7.63 Hz and cannot change while the task runs. In  this mode, the duty cycle has 8 bits of resolution."]
    FixedHighFreq,
    #[doc=" Pulse frequency must be below 366.21 Hz and cannot change while the task runs.  In this mode, the duty cycle has 16 bits of resolution."]
    FixedLowFreq,
    #[doc=" Pulse duty cycle must be 50 percent. The frequency can change while the task  runs."]
    Fixed50PercentDutyCycle,
}

impl From<i32> for ConstrainedGenMode {
    fn from(val: i32) -> ConstrainedGenMode {
        match val {
            14708_i32 => ConstrainedGenMode::Unconstrained,
            14709_i32 => ConstrainedGenMode::FixedHighFreq,
            14710_i32 => ConstrainedGenMode::FixedLowFreq,
            14711_i32 => ConstrainedGenMode::Fixed50PercentDutyCycle,
        }
    }
}

impl From<ConstrainedGenMode> for i32 {
    fn from(val: ConstrainedGenMode) -> i32 {
        match val {
            ConstrainedGenMode::Unconstrained => 14708_i32,
            ConstrainedGenMode::FixedHighFreq => 14709_i32,
            ConstrainedGenMode::FixedLowFreq => 14710_i32,
            ConstrainedGenMode::Fixed50PercentDutyCycle => 14711_i32,
        }
    }
}

pub enum CurrentShuntResistorLocation1 {
    #[doc="Use the built-in shunt resistor of the device."]
    Internal,
    #[doc=" Use a shunt resistor external to the device. You must specify the value of the  shunt resistor by using DAQmx_AI_CurrentShunt_Resistance."]
    External,
}

impl From<i32> for CurrentShuntResistorLocation1 {
    fn from(val: i32) -> CurrentShuntResistorLocation1 {
        match val {
            10200_i32 => CurrentShuntResistorLocation1::Internal,
            10167_i32 => CurrentShuntResistorLocation1::External,
        }
    }
}

impl From<CurrentShuntResistorLocation1> for i32 {
    fn from(val: CurrentShuntResistorLocation1) -> i32 {
        match val {
            CurrentShuntResistorLocation1::Internal => 10200_i32,
            CurrentShuntResistorLocation1::External => 10167_i32,
        }
    }
}

pub enum Signal {
    AiConvertClock,
    D10MhzRefClock,
    D20MhzTimebaseClock,
    SampleClock,
    AdvanceTrigger,
    ReferenceTrigger,
    StartTrigger,
    AdvCmpltEvent,
    AiHoldCmpltEvent,
    CounterOutputEvent,
    ChangeDetectionEvent,
    WdtExpiredEvent,
}

impl From<i32> for Signal {
    fn from(val: i32) -> Signal {
        match val {
            12484_i32 => Signal::AiConvertClock,
            12536_i32 => Signal::D10MhzRefClock,
            12486_i32 => Signal::D20MhzTimebaseClock,
            12487_i32 => Signal::SampleClock,
            12488_i32 => Signal::AdvanceTrigger,
            12490_i32 => Signal::ReferenceTrigger,
            12491_i32 => Signal::StartTrigger,
            12492_i32 => Signal::AdvCmpltEvent,
            12493_i32 => Signal::AiHoldCmpltEvent,
            12494_i32 => Signal::CounterOutputEvent,
            12511_i32 => Signal::ChangeDetectionEvent,
            12512_i32 => Signal::WdtExpiredEvent,
        }
    }
}

impl From<Signal> for i32 {
    fn from(val: Signal) -> i32 {
        match val {
            Signal::AiConvertClock => 12484_i32,
            Signal::D10MhzRefClock => 12536_i32,
            Signal::D20MhzTimebaseClock => 12486_i32,
            Signal::SampleClock => 12487_i32,
            Signal::AdvanceTrigger => 12488_i32,
            Signal::ReferenceTrigger => 12490_i32,
            Signal::StartTrigger => 12491_i32,
            Signal::AdvCmpltEvent => 12492_i32,
            Signal::AiHoldCmpltEvent => 12493_i32,
            Signal::CounterOutputEvent => 12494_i32,
            Signal::ChangeDetectionEvent => 12511_i32,
            Signal::WdtExpiredEvent => 12512_i32,
        }
    }
}

pub enum TimestampEvent {
    StartTrigger,
    ReferenceTrigger,
    ArmStartTrigger,
    FirstSampleTimestamp,
}

impl From<i32> for TimestampEvent {
    fn from(val: i32) -> TimestampEvent {
        match val {
            12491_i32 => TimestampEvent::StartTrigger,
            12490_i32 => TimestampEvent::ReferenceTrigger,
            14641_i32 => TimestampEvent::ArmStartTrigger,
            16130_i32 => TimestampEvent::FirstSampleTimestamp,
        }
    }
}

impl From<TimestampEvent> for i32 {
    fn from(val: TimestampEvent) -> i32 {
        match val {
            TimestampEvent::StartTrigger => 12491_i32,
            TimestampEvent::ReferenceTrigger => 12490_i32,
            TimestampEvent::ArmStartTrigger => 14641_i32,
            TimestampEvent::FirstSampleTimestamp => 16130_i32,
        }
    }
}

pub enum TriggerType8 {
    #[doc="Trigger when an analog signal signal crosses a threshold."]
    AnlgEdge,
    #[doc=" Trigger when any of the configured analog signals cross their respective  thresholds."]
    AnlgMultiEdge,
    #[doc="Trigger on the rising or falling edge of a digital signal."]
    DigEdge,
    #[doc=" Trigger when digital physical channels match a digital pattern."]
    DigPattern,
    #[doc=" Trigger when an analog signal enters or leaves a range of values. The range is  in the units of the measurement."]
    AnlgWin,
    #[doc="Trigger when a specified time is reached."]
    Time,
    #[doc="Disable triggering for the task."]
    None,
}

impl From<i32> for TriggerType8 {
    fn from(val: i32) -> TriggerType8 {
        match val {
            10099_i32 => TriggerType8::AnlgEdge,
            16108_i32 => TriggerType8::AnlgMultiEdge,
            10150_i32 => TriggerType8::DigEdge,
            10398_i32 => TriggerType8::DigPattern,
            10103_i32 => TriggerType8::AnlgWin,
            15996_i32 => TriggerType8::Time,
            10230_i32 => TriggerType8::None,
        }
    }
}

impl From<TriggerType8> for i32 {
    fn from(val: TriggerType8) -> i32 {
        match val {
            TriggerType8::AnlgEdge => 10099_i32,
            TriggerType8::AnlgMultiEdge => 16108_i32,
            TriggerType8::DigEdge => 10150_i32,
            TriggerType8::DigPattern => 10398_i32,
            TriggerType8::AnlgWin => 10103_i32,
            TriggerType8::Time => 15996_i32,
            TriggerType8::None => 10230_i32,
        }
    }
}

pub enum ResolutionType1 {
    #[doc="Bits."]
    Bits,
}

impl From<i32> for ResolutionType1 {
    fn from(val: i32) -> ResolutionType1 {
        match val {
            10109_i32 => ResolutionType1::Bits,
        }
    }
}

impl From<ResolutionType1> for i32 {
    fn from(val: ResolutionType1) -> i32 {
        match val {
            ResolutionType1::Bits => 10109_i32,
        }
    }
}

pub enum SensorPowerCfg {
    #[doc="Sensor power supply configuration is not changed."]
    NoChange,
    #[doc="Sensor power supply is turned on."]
    Enabled,
    #[doc="Sensor power supply is turned off."]
    Disabled,
}

impl From<i32> for SensorPowerCfg {
    fn from(val: i32) -> SensorPowerCfg {
        match val {
            10160_i32 => SensorPowerCfg::NoChange,
            16145_i32 => SensorPowerCfg::Enabled,
            16146_i32 => SensorPowerCfg::Disabled,
        }
    }
}

impl From<SensorPowerCfg> for i32 {
    fn from(val: SensorPowerCfg) -> i32 {
        match val {
            SensorPowerCfg::NoChange => 10160_i32,
            SensorPowerCfg::Enabled => 16145_i32,
            SensorPowerCfg::Disabled => 16146_i32,
        }
    }
}

pub enum AOIdleOutputBehavior {
    #[doc="Generate 0 V."]
    ZeroVolts,
    #[doc=" Set the channel to high-impedance, effectively disconnecting the analog output  circuitry from the I/O connector."]
    HighImpedance,
    #[doc="Continue generating the current value."]
    MaintainExistingValue,
}

impl From<i32> for AOIdleOutputBehavior {
    fn from(val: i32) -> AOIdleOutputBehavior {
        match val {
            12526_i32 => AOIdleOutputBehavior::ZeroVolts,
            12527_i32 => AOIdleOutputBehavior::HighImpedance,
            12528_i32 => AOIdleOutputBehavior::MaintainExistingValue,
        }
    }
}

impl From<AOIdleOutputBehavior> for i32 {
    fn from(val: AOIdleOutputBehavior) -> i32 {
        match val {
            AOIdleOutputBehavior::ZeroVolts => 12526_i32,
            AOIdleOutputBehavior::HighImpedance => 12527_i32,
            AOIdleOutputBehavior::MaintainExistingValue => 12528_i32,
        }
    }
}

pub enum ExcitationDCorAC {
    #[doc="DC excitation."]
    Dc,
    #[doc="AC excitation."]
    Ac,
}

impl From<i32> for ExcitationDCorAC {
    fn from(val: i32) -> ExcitationDCorAC {
        match val {
            10050_i32 => ExcitationDCorAC::Dc,
            10045_i32 => ExcitationDCorAC::Ac,
        }
    }
}

impl From<ExcitationDCorAC> for i32 {
    fn from(val: ExcitationDCorAC) -> i32 {
        match val {
            ExcitationDCorAC::Dc => 10050_i32,
            ExcitationDCorAC::Ac => 10045_i32,
        }
    }
}

pub enum LoggingOperation {
    #[doc=" Open an existing TDMS file, and append data to that file. If the file does not  exist, NI-DAQmx returns an error."]
    Open,
    #[doc=" Open an existing TDMS file, and append data to that file. If the file does not  exist, NI-DAQmx creates a new TDMS file."]
    OpenOrCreate,
    #[doc="Create a new TDMS file, or replace an existing TDMS file."]
    CreateOrReplace,
    #[doc=" Create a new TDMS file. If the file already exists, NI-DAQmx returns an error."]
    Create,
}

impl From<i32> for LoggingOperation {
    fn from(val: i32) -> LoggingOperation {
        match val {
            10437_i32 => LoggingOperation::Open,
            15846_i32 => LoggingOperation::OpenOrCreate,
            15847_i32 => LoggingOperation::CreateOrReplace,
            15848_i32 => LoggingOperation::Create,
        }
    }
}

impl From<LoggingOperation> for i32 {
    fn from(val: LoggingOperation) -> i32 {
        match val {
            LoggingOperation::Open => 10437_i32,
            LoggingOperation::OpenOrCreate => 15846_i32,
            LoggingOperation::CreateOrReplace => 15847_i32,
            LoggingOperation::Create => 15848_i32,
        }
    }
}

pub enum TriggerUsage {
    Advance,
    Pause,
    Reference,
    Start,
    Handshake,
    ArmStart,
}

impl From<i32> for TriggerUsage {
    fn from(val: i32) -> TriggerUsage {
        match val {
            12488_i32 => TriggerUsage::Advance,
            12489_i32 => TriggerUsage::Pause,
            12490_i32 => TriggerUsage::Reference,
            12491_i32 => TriggerUsage::Start,
            10389_i32 => TriggerUsage::Handshake,
            14641_i32 => TriggerUsage::ArmStart,
        }
    }
}

impl From<TriggerUsage> for i32 {
    fn from(val: TriggerUsage) -> i32 {
        match val {
            TriggerUsage::Advance => 12488_i32,
            TriggerUsage::Pause => 12489_i32,
            TriggerUsage::Reference => 12490_i32,
            TriggerUsage::Start => 12491_i32,
            TriggerUsage::Handshake => 10389_i32,
            TriggerUsage::ArmStart => 14641_i32,
        }
    }
}

pub enum ExportActions2 {
    #[doc="Send a pulse to the terminal."]
    Pulse,
    #[doc=" Toggle the state of the terminal from low to high or from high to low."]
    Toggle,
}

impl From<i32> for ExportActions2 {
    fn from(val: i32) -> ExportActions2 {
        match val {
            10265_i32 => ExportActions2::Pulse,
            10307_i32 => ExportActions2::Toggle,
        }
    }
}

impl From<ExportActions2> for i32 {
    fn from(val: ExportActions2) -> i32 {
        match val {
            ExportActions2::Pulse => 10265_i32,
            ExportActions2::Toggle => 10307_i32,
        }
    }
}

pub enum ReadRelativeTo {
    #[doc="Start reading samples relative to the first sample acquired."]
    FirstSample,
    #[doc=" Start reading samples relative to the last sample returned by the previous  read. For the first read operation, this position is the first sample acquired  or the first pretrigger sample if you configured a reference trigger for the  task."]
    CurrReadPos,
    #[doc=" Start reading samples relative to the first sample after the reference trigger  occurred."]
    RefTrig,
    #[doc=" Start reading samples relative to the first pretrigger sample. You specify the  number of pretrigger samples to acquire when you configure a reference trigger."]
    FirstPretrigSamp,
    #[doc=" Start reading samples relative to the next sample acquired. For example, use  this value and set DAQmx_Read_Offset to -1 to read the last sample acquired."]
    MostRecentSamp,
}

impl From<i32> for ReadRelativeTo {
    fn from(val: i32) -> ReadRelativeTo {
        match val {
            10424_i32 => ReadRelativeTo::FirstSample,
            10425_i32 => ReadRelativeTo::CurrReadPos,
            10426_i32 => ReadRelativeTo::RefTrig,
            10427_i32 => ReadRelativeTo::FirstPretrigSamp,
            10428_i32 => ReadRelativeTo::MostRecentSamp,
        }
    }
}

impl From<ReadRelativeTo> for i32 {
    fn from(val: ReadRelativeTo) -> i32 {
        match val {
            ReadRelativeTo::FirstSample => 10424_i32,
            ReadRelativeTo::CurrReadPos => 10425_i32,
            ReadRelativeTo::RefTrig => 10426_i32,
            ReadRelativeTo::FirstPretrigSamp => 10427_i32,
            ReadRelativeTo::MostRecentSamp => 10428_i32,
        }
    }
}

pub enum WaitMode2 {
    #[doc=" Repeatedly check for available buffer space as fast as possible. This mode  allows for the highest sampling rates at the expense of CPU efficiency."]
    Poll,
    #[doc=" Repeatedly check for available buffer space, but yield control to other threads  after each check. This mode offers a balance between sampling rate and CPU  efficiency."]
    Yield,
    #[doc=" Check for available buffer space once per the amount of time specified in  DAQmx_Write_SleepTime."]
    Sleep,
}

impl From<i32> for WaitMode2 {
    fn from(val: i32) -> WaitMode2 {
        match val {
            12524_i32 => WaitMode2::Poll,
            12525_i32 => WaitMode2::Yield,
            12547_i32 => WaitMode2::Sleep,
        }
    }
}

impl From<WaitMode2> for i32 {
    fn from(val: WaitMode2) -> i32 {
        match val {
            WaitMode2::Poll => 12524_i32,
            WaitMode2::Yield => 12525_i32,
            WaitMode2::Sleep => 12547_i32,
        }
    }
}

pub enum DigitalPatternCondition1 {
    #[doc=" Trigger when the physical channels match the specified pattern."]
    PatternMatches,
    #[doc=" Trigger when the physical channels do not match the specified pattern."]
    PatternDoesNotMatch,
}

impl From<i32> for DigitalPatternCondition1 {
    fn from(val: i32) -> DigitalPatternCondition1 {
        match val {
            10254_i32 => DigitalPatternCondition1::PatternMatches,
            10253_i32 => DigitalPatternCondition1::PatternDoesNotMatch,
        }
    }
}

impl From<DigitalPatternCondition1> for i32 {
    fn from(val: DigitalPatternCondition1) -> i32 {
        match val {
            DigitalPatternCondition1::PatternMatches => 10254_i32,
            DigitalPatternCondition1::PatternDoesNotMatch => 10253_i32,
        }
    }
}

pub enum AOPowerUpOutputBehavior {
    #[doc="Voltage output."]
    D10322,
    #[doc="Current output."]
    D10134,
    #[doc="High-impedance state."]
    D12527,
}

impl From<i32> for AOPowerUpOutputBehavior {
    fn from(val: i32) -> AOPowerUpOutputBehavior {
        match val {
            10322_i32 => AOPowerUpOutputBehavior::D10322,
            10134_i32 => AOPowerUpOutputBehavior::D10134,
            12527_i32 => AOPowerUpOutputBehavior::D12527,
        }
    }
}

impl From<AOPowerUpOutputBehavior> for i32 {
    fn from(val: AOPowerUpOutputBehavior) -> i32 {
        match val {
            AOPowerUpOutputBehavior::D10322 => 10322_i32,
            AOPowerUpOutputBehavior::D10134 => 10134_i32,
            AOPowerUpOutputBehavior::D12527 => 12527_i32,
        }
    }
}

pub enum ModulationType {
    #[doc="Amplitude modulation."]
    Am,
    #[doc="Frequency modulation."]
    Fm,
    #[doc="No modulation."]
    None,
}

impl From<i32> for ModulationType {
    fn from(val: i32) -> ModulationType {
        match val {
            14756_i32 => ModulationType::Am,
            14757_i32 => ModulationType::Fm,
            10230_i32 => ModulationType::None,
        }
    }
}

impl From<ModulationType> for i32 {
    fn from(val: ModulationType) -> i32 {
        match val {
            ModulationType::Am => 14756_i32,
            ModulationType::Fm => 14757_i32,
            ModulationType::None => 10230_i32,
        }
    }
}

pub enum StrainGageRosetteType {
    #[doc=" A rectangular rosette consists of three strain gages, each separated by a 45  degree angle."]
    RectangularRosette,
    #[doc=" A delta rosette consists of three strain gages, each separated by a 60 degree  angle."]
    DeltaRosette,
    #[doc=" A tee rosette consists of two gages oriented at 90 degrees with respect to each  other."]
    TeeRosette,
}

impl From<i32> for StrainGageRosetteType {
    fn from(val: i32) -> StrainGageRosetteType {
        match val {
            15968_i32 => StrainGageRosetteType::RectangularRosette,
            15969_i32 => StrainGageRosetteType::DeltaRosette,
            15970_i32 => StrainGageRosetteType::TeeRosette,
        }
    }
}

impl From<StrainGageRosetteType> for i32 {
    fn from(val: StrainGageRosetteType) -> i32 {
        match val {
            StrainGageRosetteType::RectangularRosette => 15968_i32,
            StrainGageRosetteType::DeltaRosette => 15969_i32,
            StrainGageRosetteType::TeeRosette => 15970_i32,
        }
    }
}

pub enum ExcitationVoltageOrCurrent {
    #[doc="Voltage excitation."]
    Voltage,
    #[doc="Current excitation."]
    Current,
}

impl From<i32> for ExcitationVoltageOrCurrent {
    fn from(val: i32) -> ExcitationVoltageOrCurrent {
        match val {
            10322_i32 => ExcitationVoltageOrCurrent::Voltage,
            10134_i32 => ExcitationVoltageOrCurrent::Current,
        }
    }
}

impl From<ExcitationVoltageOrCurrent> for i32 {
    fn from(val: ExcitationVoltageOrCurrent) -> i32 {
        match val {
            ExcitationVoltageOrCurrent::Voltage => 10322_i32,
            ExcitationVoltageOrCurrent::Current => 10134_i32,
        }
    }
}

pub enum MIOAIConvertTbSrc {
    #[doc="Use the same source as Sample Clock timebase."]
    SameAsSampTimebase,
    #[doc="Use the same source as the Master Timebase."]
    SameAsMasterTimebase,
    #[doc="Use the onboard 100 MHz timebase."]
    D100MhzTimebase,
    #[doc="Use the onboard 80 MHz timebase."]
    D80MhzTimebase,
    #[doc="Use the onboard 20 MHz timebase."]
    D20MhzTimebase,
    #[doc="Use the onboard 8 MHz timebase."]
    D8MhzTimebase,
}

impl From<i32> for MIOAIConvertTbSrc {
    fn from(val: i32) -> MIOAIConvertTbSrc {
        match val {
            10284_i32 => MIOAIConvertTbSrc::SameAsSampTimebase,
            10282_i32 => MIOAIConvertTbSrc::SameAsMasterTimebase,
            15857_i32 => MIOAIConvertTbSrc::D100MhzTimebase,
            14636_i32 => MIOAIConvertTbSrc::D80MhzTimebase,
            12537_i32 => MIOAIConvertTbSrc::D20MhzTimebase,
            16023_i32 => MIOAIConvertTbSrc::D8MhzTimebase,
        }
    }
}

impl From<MIOAIConvertTbSrc> for i32 {
    fn from(val: MIOAIConvertTbSrc) -> i32 {
        match val {
            MIOAIConvertTbSrc::SameAsSampTimebase => 10284_i32,
            MIOAIConvertTbSrc::SameAsMasterTimebase => 10282_i32,
            MIOAIConvertTbSrc::D100MhzTimebase => 15857_i32,
            MIOAIConvertTbSrc::D80MhzTimebase => 14636_i32,
            MIOAIConvertTbSrc::D20MhzTimebase => 12537_i32,
            MIOAIConvertTbSrc::D8MhzTimebase => 16023_i32,
        }
    }
}

pub enum StrainUnits1 {
    #[doc="Strain."]
    Strain,
    #[doc=" Units a custom scale specifies. If you select this value, you must specify a  custom scale name."]
    FromCustomScale,
}

impl From<i32> for StrainUnits1 {
    fn from(val: i32) -> StrainUnits1 {
        match val {
            10299_i32 => StrainUnits1::Strain,
            10065_i32 => StrainUnits1::FromCustomScale,
        }
    }
}

impl From<StrainUnits1> for i32 {
    fn from(val: StrainUnits1) -> i32 {
        match val {
            StrainUnits1::Strain => 10299_i32,
            StrainUnits1::FromCustomScale => 10065_i32,
        }
    }
}

pub enum AcquisitionType {
    #[doc="Acquire or generate a finite number of samples."]
    FiniteSamps,
    #[doc="Acquire or generate samples until you stop the task."]
    ContSamps,
    #[doc=" Acquire or generate samples continuously using hardware timing without a  buffer. Hardware timed single point sample mode is supported only for the  sample clock and change detection timing types."]
    HwTimedSinglePoint,
}

impl From<i32> for AcquisitionType {
    fn from(val: i32) -> AcquisitionType {
        match val {
            10178_i32 => AcquisitionType::FiniteSamps,
            10123_i32 => AcquisitionType::ContSamps,
            12522_i32 => AcquisitionType::HwTimedSinglePoint,
        }
    }
}

impl From<AcquisitionType> for i32 {
    fn from(val: AcquisitionType) -> i32 {
        match val {
            AcquisitionType::FiniteSamps => 10178_i32,
            AcquisitionType::ContSamps => 10123_i32,
            AcquisitionType::HwTimedSinglePoint => 12522_i32,
        }
    }
}

pub enum PowerUpStates {
    High,
    Low,
    Tristate,
}

impl From<i32> for PowerUpStates {
    fn from(val: i32) -> PowerUpStates {
        match val {
            10192_i32 => PowerUpStates::High,
            10214_i32 => PowerUpStates::Low,
            10310_i32 => PowerUpStates::Tristate,
        }
    }
}

impl From<PowerUpStates> for i32 {
    fn from(val: PowerUpStates) -> i32 {
        match val {
            PowerUpStates::High => 10192_i32,
            PowerUpStates::Low => 10214_i32,
            PowerUpStates::Tristate => 10310_i32,
        }
    }
}

pub enum DeassertCondition {
    #[doc=" Deassert the signal when more than half of the onboard memory of the device  fills."]
    OnbrdMemMoreThanHalfFull,
    #[doc="Deassert the signal when the onboard memory fills."]
    OnbrdMemFull,
    #[doc=" Deassert the signal when the amount of space available in the onboard memory is  below the value specified with  DAQmx_Exported_RdyForXferEvent_DeassertCondCustomThreshold."]
    OnbrdMemCustomThreshold,
}

impl From<i32> for DeassertCondition {
    fn from(val: i32) -> DeassertCondition {
        match val {
            10237_i32 => DeassertCondition::OnbrdMemMoreThanHalfFull,
            10236_i32 => DeassertCondition::OnbrdMemFull,
            12577_i32 => DeassertCondition::OnbrdMemCustomThreshold,
        }
    }
}

impl From<DeassertCondition> for i32 {
    fn from(val: DeassertCondition) -> i32 {
        match val {
            DeassertCondition::OnbrdMemMoreThanHalfFull => 10237_i32,
            DeassertCondition::OnbrdMemFull => 10236_i32,
            DeassertCondition::OnbrdMemCustomThreshold => 12577_i32,
        }
    }
}

pub enum LoggingMode {
    #[doc="Disable logging for the task."]
    Off,
    #[doc=" Enable logging for the task. You cannot read data using an NI-DAQmx Read  function when using this mode. If you require access to the data, read from the  TDMS file."]
    Log,
    #[doc=" Enable both logging and reading data for the task. You must use an NI-DAQmx  Read function to read samples for NI-DAQmx to stream them to disk."]
    LogAndRead,
}

impl From<i32> for LoggingMode {
    fn from(val: i32) -> LoggingMode {
        match val {
            10231_i32 => LoggingMode::Off,
            15844_i32 => LoggingMode::Log,
            15842_i32 => LoggingMode::LogAndRead,
        }
    }
}

impl From<LoggingMode> for i32 {
    fn from(val: LoggingMode) -> i32 {
        match val {
            LoggingMode::Off => 10231_i32,
            LoggingMode::Log => 15844_i32,
            LoggingMode::LogAndRead => 15842_i32,
        }
    }
}

pub enum DataTransferMechanism {
    #[doc=" Direct Memory Access. Data transfers take place independently from the  application."]
    Dma,
    #[doc=" Data transfers take place independently from the application. Using interrupts  increases CPU usage because the CPU must service interrupt requests. Typically,  you should use interrupts if the device is out of DMA channels."]
    Interrupts,
    #[doc=" Data transfers take place when you call an NI-DAQmx Read function or an  NI-DAQmx Write function."]
    ProgrammedIo,
    #[doc=" Data transfers take place independently from the application using a USB bulk  pipe."]
    UsBbulk,
}

impl From<i32> for DataTransferMechanism {
    fn from(val: i32) -> DataTransferMechanism {
        match val {
            10054_i32 => DataTransferMechanism::Dma,
            10204_i32 => DataTransferMechanism::Interrupts,
            10264_i32 => DataTransferMechanism::ProgrammedIo,
            12590_i32 => DataTransferMechanism::UsBbulk,
        }
    }
}

impl From<DataTransferMechanism> for i32 {
    fn from(val: DataTransferMechanism) -> i32 {
        match val {
            DataTransferMechanism::Dma => 10054_i32,
            DataTransferMechanism::Interrupts => 10204_i32,
            DataTransferMechanism::ProgrammedIo => 10264_i32,
            DataTransferMechanism::UsBbulk => 12590_i32,
        }
    }
}

pub enum Timescale {
    Tai,
    Utc,
}

impl From<i32> for Timescale {
    fn from(val: i32) -> Timescale {
        match val {
            15988_i32 => Timescale::Tai,
            15987_i32 => Timescale::Utc,
        }
    }
}

impl From<Timescale> for i32 {
    fn from(val: Timescale) -> i32 {
        match val {
            Timescale::Tai => 15988_i32,
            Timescale::Utc => 15987_i32,
        }
    }
}

pub enum FuncGenType {
    #[doc="Sine wave."]
    Sine,
    #[doc="Triangle wave."]
    Triangle,
    #[doc="Square wave."]
    Square,
    #[doc="Sawtooth wave."]
    Sawtooth,
}

impl From<i32> for FuncGenType {
    fn from(val: i32) -> FuncGenType {
        match val {
            14751_i32 => FuncGenType::Sine,
            14752_i32 => FuncGenType::Triangle,
            14753_i32 => FuncGenType::Square,
            14754_i32 => FuncGenType::Sawtooth,
        }
    }
}

impl From<FuncGenType> for i32 {
    fn from(val: FuncGenType) -> i32 {
        match val {
            FuncGenType::Sine => 14751_i32,
            FuncGenType::Triangle => 14752_i32,
            FuncGenType::Square => 14753_i32,
            FuncGenType::Sawtooth => 14754_i32,
        }
    }
}

pub enum Timing {
    SyncPulseTerm,
    TimingSyncPulseForce,
    SyncPulseType,
    SampClkDigFltrEnable,
    AiConvActiveEdge,
    FirstSampTimestampEnable,
    SampClkOverrunBehavior,
    SampClkDigFltrTimebaseRate,
    AiConvDigSyncEnable,
    SyncPulseMinDelayToStart,
    HshkSampleInputDataWhen,
    AiConvDigFltrTimebaseRate,
    ChangeDetectDiRisingEdgePhysicalChans,
    RefClkRate,
    SampClkTimebaseDiv,
    SampTimingEngine,
    SyncPulseResetDelay,
    SyncPulseTimeTimescale,
    FirstSampTimestampVal,
    SampClkTimebaseActiveEdge,
    MasterTimebaseRate,
    SyncPulseResetTime,
    AiConvRate,
    DelayFromSampClkDelay,
    AiConvDigFltrTimebaseSrc,
    FirstSampClkTimescale,
    DelayFromSampClkDelayUnits,
    FirstSampTimestampTimescale,
    HshkStartCond,
    SampClkDigSyncEnable,
    OnDemandSimultaneousAoEnable,
    SyncClkInterval,
    RefClkSrc,
    SampClkDigFltrTimebaseSrc,
    SyncPulseTimeWhen,
    SampClkTimebaseMasterTimebaseDiv,
    AiConvSrc,
    ChangeDetectDiTristate,
    MasterTimebaseSrc,
    HshkDelayAfterXfer,
    SampClkMaxRate,
    AiConvDigFltrEnable,
    SampClkWriteWfmUseInitialWfmDt,
    SampClkTimebaseTerm,
    SampClkRate,
    ChangeDetectDiFallingEdgePhysicalChans,
    SampClkTimebaseRate,
    AiConvTimebaseDiv,
    SampClkTimebaseSrc,
    SampQuantSampPerChan,
    AiConvDigFltrMinPulseWidth,
    FirstSampClkWhen,
    SyncPulseSrc,
    FirstSampClkOffset,
    SampTimingType,
    SampClkActiveEdge,
    AiConvMaxRate,
    SampClkTerm,
    AiConvTimebaseSrc,
    SampQuantSampMode,
    SampClkSrc,
    SampClkUnderflowBehavior,
    ImplicitUnderflowBehavior,
    SampClkDigFltrMinPulseWidth,
    SyncPulseSyncTime,
}

impl Timing {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Timing::*;
            match self {
                SyncPulseTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                TimingSyncPulseForce => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                SyncPulseType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiConvActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                FirstSampTimestampEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                SampClkOverrunBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiConvDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                SyncPulseMinDelayToStart => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                HshkSampleInputDataWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiConvDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                ChangeDetectDiRisingEdgePhysicalChans => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                RefClkRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SampClkTimebaseDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                SampTimingEngine => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                SyncPulseResetDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SyncPulseTimeTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                FirstSampTimestampVal => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [CVIAbsoluteTime;0] },
                SampClkTimebaseActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                MasterTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SyncPulseResetTime => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                AiConvRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DelayFromSampClkDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiConvDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                FirstSampClkTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DelayFromSampClkDelayUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                FirstSampTimestampTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                HshkStartCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                OnDemandSimultaneousAoEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                SyncClkInterval => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                RefClkSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                SampClkDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                SyncPulseTimeWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [CVIAbsoluteTime;0] },
                SampClkTimebaseMasterTimebaseDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiConvSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                ChangeDetectDiTristate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                MasterTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                HshkDelayAfterXfer => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SampClkMaxRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                AiConvDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                SampClkWriteWfmUseInitialWfmDt => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                SampClkTimebaseTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                SampClkRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                ChangeDetectDiFallingEdgePhysicalChans => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                SampClkTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiConvTimebaseDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                SampClkTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                SampQuantSampPerChan => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u64;0] },
                AiConvDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                FirstSampClkWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [CVIAbsoluteTime;0] },
                SyncPulseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                FirstSampClkOffset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SampTimingType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiConvMaxRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                SampClkTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AiConvTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampQuantSampMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                SampClkUnderflowBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ImplicitUnderflowBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SyncPulseSyncTime => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
            }
        }
    }
}

pub enum Scale {
    PolyForwardCoeff,
    MapScaledMin,
    MapPreScaledMin,
    Descr,
    LinSlope,
    LinYIntercept,
    TableScaledVals,
    PolyReverseCoeff,
    TablePreScaledVals,
    MapScaledMax,
    PreScaledUnits,
    ScaledUnits,
    Type,
    MapPreScaledMax,
}

impl Scale {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Scale::*;
            match self {
                PolyForwardCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;N] },
                MapScaledMin => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                MapPreScaledMin => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                Descr => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i8;N] },
                LinSlope => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                LinYIntercept => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                TableScaledVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;N] },
                PolyReverseCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;N] },
                TablePreScaledVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;N] },
                MapScaledMax => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                PreScaledUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i32;0] },
                ScaledUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i8;N] },
                Type => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                MapPreScaledMax => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
            }
        }
    }
}

pub enum ExportSignal {
    RdyForXferEventOutputTerm,
    PauseTrigLvlActiveLvl,
    SampClkPulsePolarity,
    AiConvClkOutputTerm,
    AdvCmpltEventOutputTerm,
    D10MhzRefClkOutputTerm,
    HshkEventOutputBehavior,
    HshkEventInterlockedAssertOnStart,
    CtrOutEventLvlPolarity,
    AdvCmpltEventPulseWidth,
    StartTrigOutputTerm,
    StartTrigPulseWidth,
    CtrOutEventPulsePolarity,
    RdyForXferEventLvlActiveLvl,
    RefTrigOutputTerm,
    StartTrigPulseWidthUnits,
    RdyForStartEventOutputTerm,
    HshkEventDelay,
    AdvTrigPulseWidthUnits,
    CtrOutEventOutputTerm,
    HshkEventInterlockedDeassertDelay,
    RdyForXferEventDeassertCondCustomThreshold,
    AdvCmpltEventDelay,
    AdvCmpltEventPulseWidthUnits,
    RdyForRefEventPulsePolarity,
    FreqOutClkPulseWidthUnits,
    AiConvClkPulseWidth,
    RefClkPulseWidthUnits,
    RefClkPulseWidth,
    DividedSampClkTimebaseOutputTerm,
    HshkEventPulseWidth,
    AiConvClkPulseWidthUnits,
    D20MhzTimebaseOutputTerm,
    WatchdogExpiredEventOutputTerm,
    AiHoldCmpltEventOutputTerm,
    DataActiveEventLvlActiveLvl,
    AiConvClkPulsePolarity,
    RefClkPulsePolarity,
    StartTrigToggleInitialState,
    RdyForXferEventDeassertCond,
    SampClkOutputBehavior,
    SampClkPulseWidth,
    D20MhzTimebaseDivideDownByN,
    HshkEventInterlockedAssertedLvl,
    HshkEventPulsePolarity,
    RdyForRefEventPulseWidth,
    CtrOutEventPulseWidthUnits,
    SampClkOutputTerm,
    StartTrigOutputBehavior,
    D20MhzTimebasePulseWidth,
    DataActiveEventOutputTerm,
    CtrOutEventOutputBehavior,
    CtrOutEventToggleIdleState,
    SyncPulseEventOutputTerm,
    ChangeDetectEventPulsePolarity,
    FreqOutClkPulsePolarity,
    StartTrigDelayUnits,
    CtrOutEventPulseWidth,
    StartTrigDelay,
    SampClkTimebaseOutputTerm,
    RdyForStartEventLvlActiveLvl,
    StartTrigPulsePolarity,
    AdvCmpltEventPulsePolarity,
    RdyForRefEventPulseWidthUnits,
    HshkEventOutputTerm,
    AdvTrigPulseWidth,
    SampClkPulseWidthUnits,
    FreqOutClkPulseWidth,
    PauseTrigOutputTerm,
    D20MhzTimebasePulseWidthUnits,
    SampClkDelayOffset,
    ChangeDetectEventOutputTerm,
    AdvCmpltEventEnable,
    D20MhzTimebasePulsePolarity,
    AiHoldCmpltEventPulsePolarity,
    AdvTrigPulsePolarity,
    RefTrigPulsePolarity,
    AdvTrigOutputTerm,
}

impl ExportSignal {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use ExportSignal::*;
            match self {
                RdyForXferEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                PauseTrigLvlActiveLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiConvClkOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AdvCmpltEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                D10MhzRefClkOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                HshkEventOutputBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                HshkEventInterlockedAssertOnStart => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CtrOutEventLvlPolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AdvCmpltEventPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                StartTrigOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                StartTrigPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CtrOutEventPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RdyForXferEventLvlActiveLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RefTrigOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                StartTrigPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RdyForStartEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                HshkEventDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AdvTrigPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CtrOutEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                HshkEventInterlockedDeassertDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                RdyForXferEventDeassertCondCustomThreshold => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AdvCmpltEventDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AdvCmpltEventPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RdyForRefEventPulsePolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                FreqOutClkPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiConvClkPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                RefClkPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RefClkPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                DividedSampClkTimebaseOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                HshkEventPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiConvClkPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                D20MhzTimebaseOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                WatchdogExpiredEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiHoldCmpltEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DataActiveEventLvlActiveLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiConvClkPulsePolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                RefClkPulsePolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                StartTrigToggleInitialState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RdyForXferEventDeassertCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkOutputBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                D20MhzTimebaseDivideDownByN => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                HshkEventInterlockedAssertedLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                HshkEventPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RdyForRefEventPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CtrOutEventPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                StartTrigOutputBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                D20MhzTimebasePulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                DataActiveEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CtrOutEventOutputBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CtrOutEventToggleIdleState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SyncPulseEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                ChangeDetectEventPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                FreqOutClkPulsePolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                StartTrigDelayUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CtrOutEventPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                StartTrigDelay => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                SampClkTimebaseOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                RdyForStartEventLvlActiveLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                StartTrigPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AdvCmpltEventPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RdyForRefEventPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                HshkEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AdvTrigPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SampClkPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                FreqOutClkPulseWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                PauseTrigOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                D20MhzTimebasePulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                SampClkDelayOffset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                ChangeDetectEventOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AdvCmpltEventEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                D20MhzTimebasePulsePolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiHoldCmpltEventPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AdvTrigPulsePolarity => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                RefTrigPulsePolarity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AdvTrigOutputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
            }
        }
    }
}

pub enum System {
    GlobalChans,
    NidaqMinorVersion,
    DevNames,
    Tasks,
    NidaqMajorVersion,
    NidaqUpdateVersion,
    Scales,
}

impl System {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use System::*;
            match self {
                GlobalChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NidaqMinorVersion => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                DevNames => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                Tasks => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NidaqMajorVersion => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                NidaqUpdateVersion => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                Scales => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
            }
        }
    }
}

pub enum CalibrationInfo {
    SelfCalLastTemp,
    CalUserDefinedInfoMaxSize,
    ExtCalLastTemp,
    CalRecommendedAccConnectionCountLimit,
    SelfCalSupported,
    CalUserDefinedInfo,
    ExtCalRecommendedInterval,
    CalDevTemp,
    CalAccConnectionCount,
}

impl CalibrationInfo {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use CalibrationInfo::*;
            match self {
                SelfCalLastTemp => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CalUserDefinedInfoMaxSize => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                ExtCalLastTemp => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CalRecommendedAccConnectionCountLimit => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                SelfCalSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                CalUserDefinedInfo => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i8;N] },
                ExtCalRecommendedInterval => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CalDevTemp => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CalAccConnectionCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [u32;0] },
            }
        }
    }
}

pub enum Buffer {
    InputBufSize,
    InputOnbrdBufSize,
    OutputBufSize,
    OutputOnbrdBufSize,
}

impl Buffer {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Buffer::*;
            match self {
                InputBufSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                InputOnbrdBufSize => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                OutputBufSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                OutputOnbrdBufSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
            }
        }
    }
}

pub enum PersistedTask {
    Author,
    AllowInteractiveEditing,
    ActiveTask,
    AllowInteractiveDeletion,
}

impl PersistedTask {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use PersistedTask::*;
            match self {
                Author => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AllowInteractiveEditing => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                ActiveTask => AttrStruct { access: AttrAccess::Write, resettable: false, ty: [i8;N] },
                AllowInteractiveDeletion => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
            }
        }
    }
}

pub enum RealTime {
    WriteRecoveryMode,
    ConvLateErrorsToWarnings,
    NumOfWarmupIters,
    WaitForNextSampClkWaitMode,
    ReportMissedSamp,
}

impl RealTime {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use RealTime::*;
            match self {
                WriteRecoveryMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ConvLateErrorsToWarnings => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                NumOfWarmupIters => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                WaitForNextSampClkWaitMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ReportMissedSamp => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
            }
        }
    }
}

pub enum Watchdog {
    Timeout,
    DigEdgeWatchdogExpirTrigSrc,
    HasExpired,
    ExpirTrigTrigOnNetworkConnLoss,
    CoExpirState,
    AoOutputType,
    DoExpirState,
    AoExpirState,
    ExpirTrigType,
    DigEdgeWatchdogExpirTrigEdge,
}

impl Watchdog {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Watchdog::*;
            match self {
                Timeout => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeWatchdogExpirTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                HasExpired => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                ExpirTrigTrigOnNetworkConnLoss => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CoExpirState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoOutputType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoExpirState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoExpirState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                ExpirTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigEdgeWatchdogExpirTrigEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
            }
        }
    }
}

pub enum Device {
    DiNumSampTimingEngines,
    AiMaxMultiChanRate,
    NavCurrentUtcOffset,
    NumTimestampEngines,
    CompactRioChassisDevName,
    AoCurrentRngs,
    FieldDaqDevName,
    NavVdop,
    NavRemainingSurveyFixes,
    NavPresetLong,
    CompactDaqSlotNum,
    TcpipHostname,
    NavAntStatus,
    AiMaxSingleChanRate,
    NavTrigUsage,
    CoPhysicalChans,
    FieldDaqBankDevNames,
    CoTrigUsage,
    AoGains,
    AiChargeRngs,
    AoVoltageRngs,
    NavHdop,
    DiTrigUsage,
    ProductType,
    DiPorts,
    CiMaxTimebase,
    CompactDaqChassisDevName,
    AiSimultaneousSamplingSupported,
    AoSupportedOutputTypes,
    CiPhysicalChans,
    CiMaxSize,
    NavPpsCompen,
    AiVoltageRngs,
    AoNumSampTimingEngines,
    NavPhysicalChans,
    CarrierSerialNum,
    CiCurrentUtcOffset,
    NavAltRef,
    CoSampModes,
    AoPhysicalChans,
    TimeTrigSupported,
    TcpipWirelessIp,
    NavPresetAlt,
    NumDmaChans,
    CompactRioSlotNum,
    DoLines,
    AiNumSampTimingEngines,
    AccessoryProductTypes,
    AiTrigUsage,
    BusType,
    AoSampModes,
    PciDevNum,
    NavPresetLat,
    NavHasFix,
    AiDigFltrTypes,
    NumTimeTrigs,
    AccessorySerialNums,
    AiLowpassCutoffFreqRangeVals,
    IsSimulated,
    PxiSlotNum,
    AiResistanceRngs,
    AiDigFltrLowpassCutoffFreqDiscreteVals,
    CoMaxTimebase,
    AiNumSyncPulseSrcs,
    AiVoltageIntExcitRangeVals,
    AiSampModes,
    AoNumSyncPulseSrcs,
    CiTrigUsage,
    AiDigFltrLowpassCutoffFreqRangeVals,
    AnlgTrigSupported,
    AiGains,
    DoNumSampTimingEngines,
    PciBusNum,
    ProductCategory,
    AiMinRate,
    CiSampModes,
    CiSampClkSupported,
    AiFreqRngs,
    AoTrigUsage,
    SerialNum,
    AiCurrentRngs,
    AccessoryProductNums,
    Terminals,
    ChassisModuleDevNames,
    DigTrigSupported,
    AiCouplings,
    NavNumSurveyFixes,
    TcpipEthernetIp,
    AiBridgeRngs,
    CiUtcOffsetReady,
    AiVoltageIntExcitDiscreteVals,
    DoTrigUsage,
    AiSupportedMeasTypes,
    NavUtcOffsetReady,
    PxiChassisNum,
    DiMaxRate,
    TedsHwtedsSupported,
    AoMinRate,
    AiLowpassCutoffFreqDiscreteVals,
    CoMaxSize,
    DoMaxRate,
    AoMaxRate,
    CoSupportedOutputTypes,
    NavMode,
    NavNumSats,
    AiCurrentIntExcitDiscreteVals,
    CiSupportedMeasTypes,
    NavPdop,
    DoPorts,
    ProductNum,
    AiPhysicalChans,
    NavSupportedMeasTypes,
    AoSampClkSupported,
    DiLines,
    CoSampClkSupported,
}

impl Device {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Device::*;
            match self {
                DiNumSampTimingEngines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiMaxMultiChanRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                NavCurrentUtcOffset => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                NumTimestampEngines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CompactRioChassisDevName => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AoCurrentRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                FieldDaqDevName => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NavVdop => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                NavRemainingSurveyFixes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                NavPresetLong => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                CompactDaqSlotNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                TcpipHostname => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NavAntStatus => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiMaxSingleChanRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                NavTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                CoPhysicalChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                FieldDaqBankDevNames => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CoTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AoGains => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AiChargeRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AoVoltageRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                NavHdop => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                DiTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                ProductType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                DiPorts => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CiMaxTimebase => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CompactDaqChassisDevName => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AiSimultaneousSamplingSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AoSupportedOutputTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                CiPhysicalChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CiMaxSize => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                NavPpsCompen => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                AiVoltageRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AoNumSampTimingEngines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                NavPhysicalChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CarrierSerialNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CiCurrentUtcOffset => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                NavAltRef => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i32;0] },
                CoSampModes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AoPhysicalChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                TimeTrigSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                TcpipWirelessIp => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NavPresetAlt => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                NumDmaChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CompactRioSlotNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                DoLines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AiNumSampTimingEngines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AccessoryProductTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AiTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                BusType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AoSampModes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PciDevNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                NavPresetLat => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [f64;0] },
                NavHasFix => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiDigFltrTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                NumTimeTrigs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AccessorySerialNums => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;N] },
                AiLowpassCutoffFreqRangeVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                IsSimulated => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PxiSlotNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiResistanceRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AiDigFltrLowpassCutoffFreqDiscreteVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                CoMaxTimebase => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                AiNumSyncPulseSrcs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiVoltageIntExcitRangeVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AiSampModes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AoNumSyncPulseSrcs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CiTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AiDigFltrLowpassCutoffFreqRangeVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AnlgTrigSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiGains => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                DoNumSampTimingEngines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                PciBusNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                ProductCategory => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiMinRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CiSampModes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                CiSampClkSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiFreqRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AoTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                SerialNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiCurrentRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                AccessoryProductNums => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;N] },
                Terminals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                ChassisModuleDevNames => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                DigTrigSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiCouplings => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                NavNumSurveyFixes => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [u32;0] },
                TcpipEthernetIp => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AiBridgeRngs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                CiUtcOffsetReady => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiVoltageIntExcitDiscreteVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                DoTrigUsage => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AiSupportedMeasTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                NavUtcOffsetReady => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PxiChassisNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                DiMaxRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                TedsHwtedsSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AoMinRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                AiLowpassCutoffFreqDiscreteVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                CoMaxSize => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                DoMaxRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                AoMaxRate => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CoSupportedOutputTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                NavMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i32;0] },
                NavNumSats => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiCurrentIntExcitDiscreteVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                CiSupportedMeasTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                NavPdop => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                DoPorts => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                ProductNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiPhysicalChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NavSupportedMeasTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AoSampClkSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                DiLines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CoSampClkSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
            }
        }
    }
}

pub enum PersistedScale {
    AllowInteractiveEditing,
    ActiveScale,
    Author,
    AllowInteractiveDeletion,
}

impl PersistedScale {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use PersistedScale::*;
            match self {
                AllowInteractiveEditing => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                ActiveScale => AttrStruct { access: AttrAccess::Write, resettable: false, ty: [i8;N] },
                Author => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AllowInteractiveDeletion => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
            }
        }
    }
}

pub enum Task {
    NumDevices,
    Devices,
    Channels,
    Complete,
    Name,
    NumChans,
}

impl Task {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Task::*;
            match self {
                NumDevices => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                Devices => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                Channels => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                Complete => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                Name => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NumChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
            }
        }
    }
}

pub enum Trigger {
    AnlgEdgeRefTrigDigFltrTimebaseSrc,
    DigPatternRefTrigPattern,
    AnlgMultiEdgeStartTrigSlopes,
    AnlgWinRefTrigDigFltrTimebaseRate,
    AnlgEdgeStartTrigSlope,
    StartTrigDelay,
    DigEdgeAdvTrigSrc,
    DigPatternStartTrigPattern,
    AnlgWinStartTrigTop,
    PauseTrigTerm,
    StartTrigMaxNumTrigsToDetect,
    StartTrigTimescale,
    AnlgWinPauseTrigWhen,
    DigLvlPauseTrigWhen,
    AnlgWinPauseTrigTop,
    RefTrigPretrigSamples,
    AnlgEdgeRefTrigDigFltrEnable,
    AnlgWinStartTrigCoupling,
    RefTrigRetriggerable,
    AnlgMultiEdgeStartTrigCouplings,
    ArmStartTrigTimestampEnable,
    DigPatternStartTrigSrc,
    AnlgMultiEdgeRefTrigSrcs,
    ArmStartTrigTrigWhen,
    InterlockedHshkTrigSrc,
    DigLvlPauseTrigDigFltrEnable,
    AnlgWinRefTrigDigFltrTimebaseSrc,
    AnlgMultiEdgeRefTrigLvls,
    AnlgEdgeStartTrigDigFltrTimebaseSrc,
    DigPatternRefTrigWhen,
    DigLvlPauseTrigDigFltrTimebaseSrc,
    DigLvlPauseTrigSrc,
    DigEdgeArmStartTrigDigFltrTimebaseRate,
    AnlgWinPauseTrigDigFltrTimebaseRate,
    StartTrigRetriggerWin,
    DigEdgeArmStartTrigDigFltrMinPulseWidth,
    AnlgMultiEdgeRefTrigSlopes,
    DigLvlPauseTrigDigFltrTimebaseRate,
    DigEdgeRefTrigDigFltrTimebaseSrc,
    StartTrigTrigWhen,
    HshkTrigType,
    AnlgWinRefTrigWhen,
    RefTrigTimestampEnable,
    RefTrigTerm,
    RefTrigDelay,
    AnlgWinPauseTrigSrc,
    StartTrigType,
    AnlgMultiEdgeRefTrigCouplings,
    AnlgEdgeStartTrigSrc,
    AnlgWinRefTrigDigFltrMinPulseWidth,
    StartTrigTimestampVal,
    PauseTrigType,
    RefTrigType,
    AnlgWinRefTrigCoupling,
    DigEdgeArmStartTrigEdge,
    AnlgLvlPauseTrigCoupling,
    TriggerSyncType,
    DigPatternPauseTrigSrc,
    DigEdgeStartTrigDigFltrTimebaseRate,
    DigPatternRefTrigSrc,
    AnlgMultiEdgeStartTrigLvls,
    ArmStartTrigTimescale,
    DigPatternStartTrigWhen,
    AnlgLvlPauseTrigDigFltrEnable,
    StartTrigTimestampEnable,
    ArmStartTrigTimestampTimescale,
    StartTrigTimestampTimescale,
    ArmStartTrigType,
    DigEdgeStartTrigDigFltrEnable,
    DigPatternPauseTrigPattern,
    DigEdgeRefTrigDigFltrEnable,
    AnlgEdgeRefTrigDigFltrTimebaseRate,
    AnlgWinPauseTrigDigFltrEnable,
    DigEdgeStartTrigDigSyncEnable,
    AnlgWinStartTrigDigFltrTimebaseRate,
    AnlgEdgeRefTrigCoupling,
    AnlgEdgeStartTrigDigFltrEnable,
    StartTrigTrigWin,
    AnlgWinRefTrigSrc,
    AnlgLvlPauseTrigLvl,
    DigEdgeRefTrigDigFltrMinPulseWidth,
    AnlgMultiEdgeStartTrigHysts,
    RefTrigMaxNumTrigsToDetect,
    AnlgEdgeStartTrigHyst,
    DigLvlPauseTrigDigSyncEnable,
    AnlgWinRefTrigBtm,
    AnlgLvlPauseTrigSrc,
    RefTrigAutoTrigEnable,
    AnlgEdgeStartTrigDigFltrMinPulseWidth,
    DigEdgeAdvTrigEdge,
    RefTrigAutoTriggered,
    AnlgLvlPauseTrigDigFltrTimebaseRate,
    AnlgWinStartTrigSrc,
    DigEdgeStartTrigSrc,
    AnlgWinStartTrigBtm,
    AnlgWinPauseTrigBtm,
    AnlgWinPauseTrigDigSyncEnable,
    DigEdgeArmStartTrigDigFltrTimebaseSrc,
    AnlgWinStartTrigDigSyncEnable,
    ArmStartTerm,
    RefTrigTimestampTimescale,
    ArmStartTrigTimestampVal,
    DigEdgeArmStartTrigDigSyncEnable,
    AnlgEdgeRefTrigLvl,
    DigEdgeRefTrigEdge,
    StartTrigDelayUnits,
    AnlgLvlPauseTrigDigFltrTimebaseSrc,
    RefTrigTimestampVal,
    DigLvlPauseTrigDigFltrMinPulseWidth,
    DigEdgeStartTrigEdge,
    AnlgWinStartTrigDigFltrTimebaseSrc,
    AnlgEdgeStartTrigLvl,
    AnlgMultiEdgeStartTrigSrcs,
    AnlgLvlPauseTrigDigFltrMinPulseWidth,
    AnlgEdgeRefTrigHyst,
    AnlgMultiEdgeRefTrigHysts,
    AdvTrigType,
    AnlgLvlPauseTrigDigSyncEnable,
    AnlgEdgeRefTrigDigFltrMinPulseWidth,
    DigEdgeStartTrigDigFltrTimebaseSrc,
    RefTrigRetriggerWin,
    StartTrigRetriggerable,
    InterlockedHshkTrigAssertedLvl,
    AnlgLvlPauseTrigWhen,
    AnlgEdgeRefTrigSlope,
    DigEdgeAdvTrigDigFltrEnable,
    AnlgEdgeRefTrigDigSyncEnable,
    AnlgEdgeStartTrigCoupling,
    DigEdgeRefTrigDigSyncEnable,
    DigEdgeArmStartTrigSrc,
    AnlgWinRefTrigTop,
    AnlgWinStartTrigDigFltrEnable,
    StartTrigTerm,
    DigEdgeRefTrigSrc,
    DigEdgeArmStartTrigDigFltrEnable,
    AnlgLvlPauseTrigHyst,
    AnlgEdgeStartTrigDigFltrTimebaseRate,
    AnlgWinStartTrigWhen,
    AnlgEdgeStartTrigDigSyncEnable,
    AnlgEdgeRefTrigSrc,
    AnlgWinPauseTrigCoupling,
    AnlgWinRefTrigDigFltrEnable,
    DigPatternPauseTrigWhen,
    DigEdgeRefTrigDigFltrTimebaseRate,
    AnlgWinPauseTrigDigFltrMinPulseWidth,
    AnlgWinStartTrigDigFltrMinPulseWidth,
    RefTrigTrigWin,
    DigEdgeStartTrigDigFltrMinPulseWidth,
    AnlgWinRefTrigDigSyncEnable,
    AnlgWinPauseTrigDigFltrTimebaseSrc,
    TimeStartTrigSrc,
}

impl Trigger {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Trigger::*;
            match self {
                AnlgEdgeRefTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigPatternRefTrigPattern => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgMultiEdgeStartTrigSlopes => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;N] },
                AnlgWinRefTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgEdgeStartTrigSlope => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                StartTrigDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeAdvTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigPatternStartTrigPattern => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgWinStartTrigTop => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                PauseTrigTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                StartTrigMaxNumTrigsToDetect => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                StartTrigTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgWinPauseTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigLvlPauseTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgWinPauseTrigTop => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                RefTrigPretrigSamples => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AnlgEdgeRefTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgWinStartTrigCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RefTrigRetriggerable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgMultiEdgeStartTrigCouplings => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;N] },
                ArmStartTrigTimestampEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DigPatternStartTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgMultiEdgeRefTrigSrcs => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                ArmStartTrigTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [CVIAbsoluteTime;0] },
                InterlockedHshkTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigLvlPauseTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgWinRefTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgMultiEdgeRefTrigLvls => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AnlgEdgeStartTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigPatternRefTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigLvlPauseTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigLvlPauseTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigEdgeArmStartTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinPauseTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                StartTrigRetriggerWin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeArmStartTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgMultiEdgeRefTrigSlopes => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;N] },
                DigLvlPauseTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeRefTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                StartTrigTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [CVIAbsoluteTime;0] },
                HshkTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgWinRefTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RefTrigTimestampEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                RefTrigTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                RefTrigDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinPauseTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                StartTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgMultiEdgeRefTrigCouplings => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;N] },
                AnlgEdgeStartTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgWinRefTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                StartTrigTimestampVal => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [CVIAbsoluteTime;0] },
                PauseTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RefTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgWinRefTrigCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigEdgeArmStartTrigEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgLvlPauseTrigCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                TriggerSyncType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigPatternPauseTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigEdgeStartTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigPatternRefTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgMultiEdgeStartTrigLvls => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                ArmStartTrigTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigPatternStartTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgLvlPauseTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                StartTrigTimestampEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                ArmStartTrigTimestampTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                StartTrigTimestampTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ArmStartTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigEdgeStartTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DigPatternPauseTrigPattern => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigEdgeRefTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeRefTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinPauseTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DigEdgeStartTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgWinStartTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgEdgeRefTrigCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgEdgeStartTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                StartTrigTrigWin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinRefTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgLvlPauseTrigLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeRefTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgMultiEdgeStartTrigHysts => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                RefTrigMaxNumTrigsToDetect => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AnlgEdgeStartTrigHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigLvlPauseTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgWinRefTrigBtm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgLvlPauseTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                RefTrigAutoTrigEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeStartTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeAdvTrigEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                RefTrigAutoTriggered => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AnlgLvlPauseTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinStartTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigEdgeStartTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgWinStartTrigBtm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinPauseTrigBtm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinPauseTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DigEdgeArmStartTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgWinStartTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                ArmStartTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                RefTrigTimestampTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ArmStartTrigTimestampVal => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [CVIAbsoluteTime;0] },
                DigEdgeArmStartTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeRefTrigLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeRefTrigEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                StartTrigDelayUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgLvlPauseTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                RefTrigTimestampVal => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [CVIAbsoluteTime;0] },
                DigLvlPauseTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeStartTrigEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgWinStartTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgEdgeStartTrigLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgMultiEdgeStartTrigSrcs => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgLvlPauseTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgEdgeRefTrigHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgMultiEdgeRefTrigHysts => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AdvTrigType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgLvlPauseTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeRefTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeStartTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                RefTrigRetriggerWin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                StartTrigRetriggerable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                InterlockedHshkTrigAssertedLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgLvlPauseTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgEdgeRefTrigSlope => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigEdgeAdvTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeRefTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeStartTrigCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigEdgeRefTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DigEdgeArmStartTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgWinRefTrigTop => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinStartTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                StartTrigTerm => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                DigEdgeRefTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DigEdgeArmStartTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgLvlPauseTrigHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgEdgeStartTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinStartTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgEdgeStartTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgEdgeRefTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AnlgWinPauseTrigCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AnlgWinRefTrigDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DigPatternPauseTrigWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DigEdgeRefTrigDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinPauseTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinStartTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                RefTrigTrigWin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DigEdgeStartTrigDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AnlgWinRefTrigDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AnlgWinPauseTrigDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                TimeStartTrigSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
            }
        }
    }
}

pub enum Write {
    SleepTime,
    Offset,
    NumChans,
    CurrWritePos,
    SyncUnlockedChans,
    DevsWithInsertedOrRemovedAccessories,
    TotalSampPerChanGenerated,
    DigitalLinesBytesPerChan,
    RawDataWidth,
    PowerSupplyFaultChans,
    OvertemperatureChansExist,
    OvertemperatureChans,
    OverloadedChansExist,
    SpaceAvail,
    OpenCurrentLoopChansExist,
    OvercurrentChansExist,
    OverloadedChans,
    PowerSupplyFaultChansExist,
    RelativeTo,
    WaitMode,
    NextWriteIsLast,
    ExternalOvervoltageChansExist,
    ExternalOvervoltageChans,
    OvercurrentChans,
    OpenCurrentLoopChans,
    SyncUnlockedChansExist,
    RegenMode,
    AccessoryInsertionOrRemovalDetected,
}

impl Write {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Write::*;
            match self {
                SleepTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                Offset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                NumChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CurrWritePos => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u64;0] },
                SyncUnlockedChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                DevsWithInsertedOrRemovedAccessories => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                TotalSampPerChanGenerated => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u64;0] },
                DigitalLinesBytesPerChan => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                RawDataWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                PowerSupplyFaultChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OvertemperatureChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                OvertemperatureChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OverloadedChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                SpaceAvail => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                OpenCurrentLoopChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                OvercurrentChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                OverloadedChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                PowerSupplyFaultChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                RelativeTo => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                WaitMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                NextWriteIsLast => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                ExternalOvervoltageChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                ExternalOvervoltageChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OvercurrentChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OpenCurrentLoopChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                SyncUnlockedChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                RegenMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AccessoryInsertionOrRemovalDetected => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
            }
        }
    }
}

pub enum Channel {
    CiUsbXferReqSize,
    CiFreqDigFltrMinPulseWidth,
    CiEncoderBInputDigFltrEnable,
    CiEncoderBInputLogicLvlBehavior,
    AiCurrentUnits,
    ChanIsGlobal,
    CiPulseWidthDigFltrMinPulseWidth,
    CiTwoEdgeSepSecondLogicLvlBehavior,
    CiPulseTimeLogicLvlBehavior,
    DiDigFltrTimebaseRate,
    CiVelocityEncoderBInputDigFltrTimebaseSrc,
    AiAccelChargeSensitivityUnits,
    CiEncoderZInputDigFltrTimebaseRate,
    AiInputLimitsFaultDetectLowerLimit,
    CiSemiPeriodStartingEdge,
    AiBridgeInitialRatio,
    CiFreqMeasMeth,
    AoDacRefExtSrc,
    AoResolution,
    CiVelocityEncoderDecodingType,
    CiEncoderBInputTermCfg,
    CiSemiPeriodUnits,
    CiPulseFreqLogicLvlBehavior,
    CiPulseFreqStartEdge,
    CiPeriodStartingEdge,
    AiRtdB,
    CiVelocityEncoderAInputLogicLvlBehavior,
    CoPulseFreqInitialDelay,
    CiCountEdgesGateDigFltrTimebaseRate,
    AiEddyCurrentProxProbeSensitivityUnits,
    DoOvercurrentLimit,
    CiLinEncoderDistPerPulse,
    CiCountEdgesCountDirDigFltrTimebaseSrc,
    AiFreqHyst,
    CiPrescaler,
    AiEddyCurrentProxProbeSensitivity,
    NavLongUnits,
    CiPeriodTerm,
    CoUseOnlyOnBrdMem,
    CiPeriodMeasTime,
    CiEncoderAInputDigFltrTimebaseSrc,
    AiRngLow,
    CiPulseFreqDigSyncEnable,
    AiVelocityIepeSensorSensitivity,
    CoCtrTimebaseDigFltrEnable,
    AiSoundPressureUnits,
    AiAcExcitWireMode,
    AiBridgePhysicalUnits,
    AoDacOffsetExtSrc,
    DiAcquireOn,
    CiPulseTicksLogicLvlBehavior,
    AiBridgeElectricalUnits,
    AiChanCalVerifAcqVals,
    AiBridgeShuntCalShuntCalAResistance,
    CiVelocityLinEncoderDistPerPulse,
    NavVertVelocityUnits,
    AiChargeUnits,
    CiPeriodDigFltrTimebaseSrc,
    CoCtrTimebaseMasterTimebaseDiv,
    AiChanCalTablePreScaledVals,
    CiPulseTicksTerm,
    AiRosetteStrainGageRosetteMeasType,
    DoUsbXferReqCount,
    CiPeriodHyst,
    CiLinEncoderUnits,
    CiPeriodDigFltrEnable,
    CiCountEdgesCountResetDigFltrEnable,
    AiBridgeCfg,
    AiBandpassType,
    CiTwoEdgeSepFirstLogicLvlBehavior,
    AoEnhancedImageRejectionEnable,
    AiVelocityUnits,
    AiStrainGageCfg,
    CiCtrTimebaseRate,
    CiPulseWidthUnits,
    CiDataXferReqCond,
    CoCtrTimebaseDigSyncEnable,
    AiDigFltrLowpassCutoffFreq,
    AiResolution,
    AiBridgePolyReverseCoeff,
    AoDacOffsetVal,
    AoResolutionUnits,
    AiAveragingWinSize,
    CiDutyCycleTermCfg,
    AiDataXferMaxRate,
    AiBridgeTwoPointLinSecondPhysicalVal,
    CoOutputState,
    CiCountEdgesTerm,
    CiVelocityEncoderBInputLogicLvlBehavior,
    CiDupCountPrevent,
    AiBridgeTableElectricalVals,
    CiSemiPeriodTermCfg,
    AiRtdC,
    CiPeriodUnits,
    AiChanCalScaleType,
    AoFuncGenModulationType,
    CiTwoEdgeSepSecondDigFltrEnable,
    CiOutputState,
    AiThrmcplType,
    AiThrmcplCjcSrc,
    CiFreqEnableAveraging,
    CiEncoderZInputDigFltrMinPulseWidth,
    CiFreqDigFltrEnable,
    CiPulseTicksDigFltrEnable,
    CiPulseTicksDigFltrTimebaseSrc,
    CoUsbXferReqCount,
    AiBridgeShuntCalShuntCalBActualResistance,
    CiCountEdgesGateThreshVoltage,
    AoFilterDelayAdjustment,
    AiBridgeTwoPointLinFirstElectricalVal,
    CoPulseLowTicks,
    AiMin,
    AiLowpassSwitchCapClkSrc,
    CiMemMapEnable,
    CiPulseWidthLogicLvlBehavior,
    CiEncoderZIndexVal,
    CiPulseWidthDigFltrTimebaseSrc,
    CiCountEdgesLogicLvlBehavior,
    AiBridgeScaleType,
    AiFilterOrder,
    CiPulseWidthStartingEdge,
    CoPulseDone,
    AiAtten,
    CiEncoderBInputDigSyncEnable,
    CiPulseWidthDigFltrEnable,
    CiEncoderBInputTerm,
    AiLvdtUnits,
    AiPowerSupplyFaultDetectEnable,
    AiNotchWidth,
    CoUsbXferReqSize,
    AiChanCalOperatorName,
    CoCount,
    AiBridgeShuntCalShuntCalBResistance,
    AoCurrentUnits,
    AoMax,
    AiBridgeBalanceCoarsePot,
    AiBridgeShuntCalShuntCalAActualResistance,
    AiThrmstrR1,
    CiCountEdgesCountResetTerm,
    CoDataXferMech,
    CiEncoderDecodingType,
    CiSemiPeriodDigFltrEnable,
    CiVelocityLinEncoderUnits,
    CoPulseLowTime,
    DoGenerateOn,
    CiPulseFreqDigFltrTimebaseSrc,
    AiBandpassCenterFreq,
    CiFreqDigFltrTimebaseSrc,
    AiForceUnits,
    AiSensorPowerVoltage,
    NavLatUnits,
    AiAccelChargeSensitivity,
    DoOutputDriveType,
    CiCountEdgesHyst,
    CiTwoEdgeSepFirstDigFltrTimebaseRate,
    AoVoltageUnits,
    DiDigFltrMinPulseWidth,
    AiSampAndHoldEnable,
    DoOvercurrentAutoReenable,
    CiTimestampTerm,
    CiSemiPeriodLogicLvlBehavior,
    AoDacRngHigh,
    AoFuncGenOffset,
    CiFilterDelay,
    CiThreshVoltage,
    AiDataXferMech,
    AiLowpassSwitchCapExtClkFreq,
    CiFreqHyst,
    AoLoadImpedance,
    CiCountEdgesDigFltrTimebaseRate,
    CiCountEdgesGateDigFltrTimebaseSrc,
    CiPulseTimeDigFltrTimebaseSrc,
    CiPulseWidthDigFltrTimebaseRate,
    CiPulseTimeStartEdge,
    CiCountEdgesGateEnable,
    CiTwoEdgeSepFirstTerm,
    CiCountEdgesGateTerm,
    CoAutoIncrCnt,
    CiPulseTicksDigFltrMinPulseWidth,
    AiExcitVal,
    CiCountEdgesCountDirThreshVoltage,
    CiTwoEdgeSepFirstTermCfg,
    CiTwoEdgeSepSecondEdge,
    AoVoltageCurrentLimit,
    AiBandpassEnable,
    AiChanCalPolyReverseCoeff,
    AiLeadWireResistance,
    DiDataXferReqCond,
    CiCountEdgesCountResetDigFltrMinPulseWidth,
    CiVelocityEncoderBInputTermCfg,
    AiExcitDCorAc,
    AoMemMapEnable,
    CiPeriodDiv,
    CiCountEdgesCountResetResetCount,
    AiImpedance,
    AiTermCfg,
    AoFuncGenStartPhase,
    DiDataXferMech,
    AiCoupling,
    AiResistanceCfg,
    AoDataXferMech,
    AiThrmstrC,
    CiNumPossiblyInvalidSamps,
    CiEncoderZInputDigSyncEnable,
    AoFuncGenFreq,
    CiVelocityEncoderAInputDigFltrMinPulseWidth,
    AiLowpassEnable,
    AiTempUnits,
    AiInputSrc,
    AiChanCalApplyCalIfExp,
    CiPulseTimeDigSyncEnable,
    AiAcExcitSyncEnable,
    ChanSyncUnlockBehavior,
    DoLineStatesPausedState,
    AoDataXferReqCond,
    CiPulseTimeDigFltrEnable,
    CiVelocityEncoderAInputDigFltrTimebaseRate,
    AiBridgeShuntCalEnable,
    CiCustomScaleName,
    CiCountEdgesDigSyncEnable,
    CoCtrTimebaseDigFltrTimebaseRate,
    DiNumLines,
    CiPulseFreqTermCfg,
    CiPulseFreqTerm,
    AoFuncGenFmDeviation,
    CiCtrTimebaseDigFltrEnable,
    CiDutyCycleDigFltrTimebaseSrc,
    AiDigFltrOrder,
    CiUsbXferReqCount,
    AiRawDataCompressionType,
    DoTristate,
    AiLvdtSensitivityUnits,
    AiBridgeUnits,
    CiPulseTimeUnits,
    AiFilterDelayUnits,
    AiHighpassEnable,
    CiPulseWidthTermCfg,
    CoEnableInitialDelayOnRetrigger,
    AiInputLimitsFaultDetectUpperLimit,
    CiEncoderZIndexEnable,
    CiPeriodMeasMeth,
    DiInvertLines,
    CiVelocityAngEncoderUnits,
    CiCountEdgesCountResetHyst,
    AiAcExcitFreq,
    CiSemiPeriodDigSyncEnable,
    AiExcitIdleOutputBehavior,
    CiFreqMeasTime,
    CiGpsSyncMethod,
    DiUsbXferReqSize,
    CiCountEdgesGateDigFltrEnable,
    CiSampClkOverrunSentinelVal,
    AiThrmcplLeadOffsetVoltage,
    AiDigFltrBandpassCenterFreq,
    CiFreqDigFltrTimebaseRate,
    AiDigFltrBandpassWidth,
    AiExcitSrc,
    AiRawSampSize,
    DoUsbXferReqSize,
    AiAdcCustomTimingMode,
    CiSemiPeriodDigFltrTimebaseRate,
    AiCustomScaleName,
    AiIsTeds,
    AiMax,
    CiEncoderAInputDigSyncEnable,
    CiPulseTicksStartEdge,
    CiCountEdgesCountDirDigFltrEnable,
    AiHighpassOrder,
    CiDataXferMech,
    CiPulseFreqDigFltrEnable,
    AiCurrentAcrmsUnits,
    AoFilterDelayUnits,
    AiDitherEnable,
    CiCtrTimebaseDigFltrTimebaseRate,
    AiStrainGagePoissonRatio,
    AiDigFltrResponse,
    CiDutyCycleDigFltrTimebaseRate,
    AiRvdtSensitivityUnits,
    AiThrmstrA,
    CiCountEdgesInitialCnt,
    AiChanCalTableScaledVals,
    AoUsbXferReqSize,
    CiAngEncoderPulsesPerRev,
    CiGpsSyncSrc,
    AiResolutionUnits,
    AiOpenThrmcplDetectEnable,
    CoCtrTimebaseDigFltrMinPulseWidth,
    CiFilterOrder,
    AoFuncGenAmplitude,
    AiAccel4WireDcVoltageSensitivityUnits,
    CiTwoEdgeSepSecondDigFltrMinPulseWidth,
    CiMeasType,
    AiDevScalingCoeff,
    CiSemiPeriodDigFltrTimebaseSrc,
    CiPulseTimeTerm,
    AiStrainGageForceReadFromChan,
    DoDataXferMech,
    CiAngEncoderUnits,
    CiPulseFreqUnits,
    AiExcitSense,
    AiRtdType,
    CoCtrTimebaseSrc,
    CiDutyCycleDigFltrMinPulseWidth,
    AiAccelSensitivity,
    CiPulseTimeDigFltrTimebaseRate,
    AoCommonModeOffset,
    DoOvercurrentReenablePeriod,
    CiTwoEdgeSepSecondTermCfg,
    CiCountEdgesCountResetEnable,
    CiVelocityEncoderBInputTerm,
    AiBandpassWidth,
    DiDigFltrEnable,
    CiDutyCycleDigFltrEnable,
    AiMicrophoneSensitivity,
    CiFreqStartingEdge,
    AiForceReadFromChan,
    CiTimestampInitialSeconds,
    CiFilterFreq,
    AiFreqThreshVoltage,
    CoDataXferReqCond,
    CiSampClkOverrunBehavior,
    CiCountEdgesCountResetTermCfg,
    AiChanCalPolyForwardCoeff,
    AiRawSampJustification,
    CiEncoderZInputLogicLvlBehavior,
    CoPulseTimeInitialDelay,
    AiOpenChanDetectEnable,
    CiCountEdgesThreshVoltage,
    AiCurrentShuntLoc,
    CiEncoderZInputDigFltrTimebaseSrc,
    CiEncoderAInputLogicLvlBehavior,
    CiCountEdgesDigFltrEnable,
    CiEncoderAInputDigFltrEnable,
    CiTwoEdgeSepUnits,
    CoRdyForNewVal,
    CiPeriodEnableAveraging,
    CiVelocityMeasTime,
    AiLowpassCutoffFreq,
    CiFilterDelayUnits,
    CiCtrTimebaseDigFltrTimebaseSrc,
    CiTimestampEdge,
    DoUseOnlyOnBrdMem,
    CiPulseTimeTermCfg,
    CiMin,
    AiChanCalDesc,
    AiRtdR0,
    AiDigFltrHighpassCutoffFreq,
    DoInvertLines,
    ChanType,
    CoPulseHighTime,
    CoPulseTimeUnits,
    AiFreqUnits,
    DiUsbXferReqCount,
    CiSemiPeriodDigFltrMinPulseWidth,
    CiCountEdgesCountDirTermCfg,
    AiRvdtSensitivity,
    AiThrmcplCjcChan,
    AiEnhancedAliasRejectionEnable,
    CoConstrainedGenMode,
    AiDigFltrType,
    CiPulseWidthDigSyncEnable,
    CiTwoEdgeSepSecondDigSyncEnable,
    AiChanCalEnableCal,
    AiChanCalHasValidCalInfo,
    AoFilterDelay,
    AiAccelUnits,
    CiEncoderAInputTerm,
    AiBridgeShuntCalShuntCalASrc,
    AiFilterFreq,
    CiTwoEdgeSepSecondDigFltrTimebaseRate,
    CiFreqThreshVoltage,
    CiEncoderZInputDigFltrEnable,
    AiSoundPressureDbRef,
    CiPulseTicksDigSyncEnable,
    AoTermCfg,
    AiUsbXferReqCount,
    AiNotchCenterFreq,
    NavMeasType,
    CiCountEdgesGateHyst,
    AiForceIepeSensorSensitivity,
    DiTristate,
    CiCountEdgesDirTerm,
    CiDutyCycleLogicLvlBehavior,
    CiTwoEdgeSepFirstDigFltrTimebaseSrc,
    CiTwoEdgeSepSecondTerm,
    CiCountEdgesCountDirLogicLvlBehavior,
    CiEncoderBInputDigFltrTimebaseSrc,
    CiVelocityEncoderAInputDigFltrTimebaseSrc,
    DiDigFltrEnableBusMode,
    CiCountEdgesCountResetLogicLvlBehavior,
    AiNotchType,
    CiPulseTicksDigFltrTimebaseRate,
    CiCountEdgesCountDirDigFltrMinPulseWidth,
    CiTwoEdgeSepFirstDigFltrMinPulseWidth,
    CoPulseHighTicks,
    CiCountEdgesCountResetThreshVoltage,
    CoPulseTerm,
    CiEncoderZInputTerm,
    AiForceIepeSensorSensitivityUnits,
    CiFreqTerm,
    CiTwoEdgeSepFirstDigSyncEnable,
    CiCountEdgesDir,
    AiMeasType,
    CiAngEncoderInitialAngle,
    AoDacRefAllowConnToGnd,
    CiEncoderBInputDigFltrMinPulseWidth,
    CiCountEdgesCountResetDigFltrTimebaseRate,
    CiEncoderZIndexPhase,
    AoDacRefVal,
    CiEncoderBInputDigFltrTimebaseRate,
    NavTimestampUnits,
    CiCountEdgesCountResetActiveEdge,
    CiFreqTermCfg,
    DiLogicFamily,
    AiStrainUnits,
    DoLineStatesDoneState,
    CiTimestampUnits,
    AiFilterResponse,
    AiRngHigh,
    AiChopEnable,
    CiPulseTicksTermCfg,
    AiChanCalVerifRefVals,
    NavCustomScaleName,
    AiMemMapEnable,
    AiDigFltrEnable,
    AoDacRefConnToGnd,
    AiLowpassSwitchCapOutClkDiv,
    CiVelocityAngEncoderPulsesPerRev,
    AiBridgeShuntCalGainAdjust,
    AiFilterEnable,
    AiVoltageAcrmsUnits,
    AiLossyLsbRemovalCompressedSampSize,
    CiPeriodDigFltrTimebaseRate,
    CiPeriodLogicLvlBehavior,
    CiCtrTimebaseDigSyncEnable,
    CiPeriodTermCfg,
    AiInputLimitsFaultDetectEnable,
    AiExcitUseForScaling,
    AoDacRngLow,
    CiCtrTimebaseDigFltrMinPulseWidth,
    AiVelocityIepeSensorDbRef,
    DoDataXferReqCond,
    CoPulseIdleState,
    AiBridgeTwoPointLinFirstPhysicalVal,
    AoDacOffsetSrc,
    DoMemMapEnable,
    AiAccelSensitivityUnits,
    CiCountEdgesDigFltrMinPulseWidth,
    AiBridgePolyForwardCoeff,
    CiFreqDiv,
    CiMaxMeasPeriod,
    AiVoltageUnits,
    CiPeriodDigFltrMinPulseWidth,
    CoPrescaler,
    CoOutputType,
    CiTwoEdgeSepFirstEdge,
    CiVelocityEncoderAInputDigFltrEnable,
    AiSensorPowerType,
    AiVelocityIepeSensorSensitivityUnits,
    AiDataXferCustomThreshold,
    AiPressureUnits,
    AiBridgeTwoPointLinSecondElectricalVal,
    DiMemMapEnable,
    CiCountEdgesGateTermCfg,
    CiEncoderAInputDigFltrMinPulseWidth,
    DiDigSyncEnable,
    CoPulseFreqUnits,
    CiMax,
    ChanDescr,
    CiCountEdgesCountDirDigSyncEnable,
    AoUseOnlyOnBrdMem,
    AiBridgeBalanceFinePot,
    DoLineStatesStartState,
    AiVoltageDbRef,
    CiEncoderAInputDigFltrTimebaseRate,
    AoIdleOutputBehavior,
    AiBridgeTablePhysicalVals,
    AiRosetteStrainGageOrientation,
    CiVelocityEncoderAInputTermCfg,
    AiCurrentShuntResistance,
    CiCountEdgesGateWhen,
    CiCountEdgesActiveEdge,
    CiCountEdgesCountDirHyst,
    AoFuncGenType,
    CoMemMapEnable,
    CiPulseFreqDigFltrMinPulseWidth,
    DoLogicFamily,
    NavTimestampTimescale,
    AoDevScalingCoeff,
    CoCtrTimebaseDigFltrTimebaseSrc,
    NavAltUnits,
    CiVelocityDiv,
    AoDacRefSrc,
    NavTrackUnits,
    AiOvercurrentDetectEnable,
    AiExcitVoltageOrCurrent,
    AiRvdtUnits,
    AiUsbXferReqSize,
    AiFilterDelayAdjustment,
    AiDigFltrNotchCenterFreq,
    AiAccel4WireDcVoltageSensitivity,
    CiCountEdgesGateLogicLvlBehavior,
    AiGain,
    CiCountEdgesCountResetDigSyncEnable,
    AiAdcTimingMode,
    AiDigFltrNotchWidth,
    AiHighpassType,
    AiDataXferReqCond,
    AiStrainGageGageFactor,
    AiBridgeInitialVoltage,
    AiTedsUnits,
    CiDutyCycleStartingEdge,
    CiFreqLogicLvlBehavior,
    CoCtrTimebaseRate,
    CiCtrTimebaseMasterTimebaseDiv,
    AoFuncGenSquareDutyCycle,
    AoUsbXferReqCount,
    AiLvdtSensitivity,
    CiEncoderAInputTermCfg,
    CiVelocityEncoderAInputTerm,
    CiCountEdgesGateDigFltrMinPulseWidth,
    CiPeriodThreshVoltage,
    AiProbeAtten,
    DiDigFltrTimebaseSrc,
    CiCountEdgesCountResetDigFltrTimebaseSrc,
    CiFilterResponse,
    AiHighpassCutoffFreq,
    DoNumLines,
    AoOutputType,
    CoPulseFreq,
    CiDutyCycleTerm,
    CiPulseFreqDigFltrTimebaseRate,
    CiTcReached,
    CiEncoderZInputTermCfg,
    CiVelocityEncoderBInputDigFltrEnable,
    CiCount,
    AiTorqueUnits,
    PhysicalChanName,
    AiRosetteStrainGageRosetteType,
    AiDigFltrCoeff,
    CiVelocityEncoderBInputDigFltrMinPulseWidth,
    CiPulseWidthTerm,
    CiFreqDigSyncEnable,
    AiAccelDbRef,
    AoCustomScaleName,
    AiDcOffset,
    AiSensorPowerCfg,
    AiThrmstrB,
    AoMin,
    AiAutoZeroMode,
    AiBridgeShuntCalSelect,
    AiSoundPressureMaxSoundPressureLvl,
    AiThrmcplScaleType,
    NavSpeedOverGroundUnits,
    AiBridgeShuntCalShuntCalBSrc,
    CiFilterEnable,
    AoGain,
    CiLinEncoderInitialPos,
    CiSemiPeriodTerm,
    AiResistanceUnits,
    AiLowpassSwitchCapExtClkDiv,
    CoPulseDutyCyc,
    CoPulseTicksInitialDelay,
    CiPeriodDigSyncEnable,
    AoReglitchEnable,
    CiCtrTimebaseActiveEdge,
    AiExcitActualVal,
    CiVelocityEncoderBInputDigFltrTimebaseRate,
    CoCtrTimebaseActiveEdge,
    CiPulseTimeDigFltrMinPulseWidth,
    AiRtdA,
    AiExcitUseMultiplexed,
    AoOutputImpedance,
    AiRosetteStrainGageStrainChans,
    CiCountEdgesCountDirDigFltrTimebaseRate,
    AiBridgeNomResistance,
    CiTwoEdgeSepFirstDigFltrEnable,
    AiRemoveFilterDelay,
    CiCtrTimebaseSrc,
    AiEddyCurrentProxProbeUnits,
    CiTimestampTimescale,
    AiThrmcplCjcVal,
    CiCountEdgesDigFltrTimebaseSrc,
    CiTwoEdgeSepSecondDigFltrTimebaseSrc,
    AiFilterDelay,
    CiCountEdgesTermCfg,
    CiFreqUnits,
}

impl Channel {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Channel::*;
            match self {
                CiUsbXferReqSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiFreqDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderBInputDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiEncoderBInputLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiCurrentUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ChanIsGlobal => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                CiPulseWidthDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepSecondLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseTimeLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DiDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityEncoderBInputDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiAccelChargeSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderZInputDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiInputLimitsFaultDetectLowerLimit => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiSemiPeriodStartingEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeInitialRatio => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqMeasMeth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDacRefExtSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AoResolution => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CiVelocityEncoderDecodingType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderBInputTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiSemiPeriodUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseFreqLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseFreqStartEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPeriodStartingEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRtdB => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityEncoderAInputLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoPulseFreqInitialDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesGateDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiEddyCurrentProxProbeSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoOvercurrentLimit => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiLinEncoderDistPerPulse => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountDirDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiFreqHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPrescaler => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiEddyCurrentProxProbeSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                NavLongUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPeriodTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CoUseOnlyOnBrdMem => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPeriodMeasTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderAInputDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiRngLow => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseFreqDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiVelocityIepeSensorSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoCtrTimebaseDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiSoundPressureUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiAcExcitWireMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgePhysicalUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDacOffsetExtSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DiAcquireOn => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseTicksLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeElectricalUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiChanCalVerifAcqVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AiBridgeShuntCalShuntCalAResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityLinEncoderDistPerPulse => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                NavVertVelocityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiChargeUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPeriodDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CoCtrTimebaseMasterTimebaseDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiChanCalTablePreScaledVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                CiPulseTicksTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiRosetteStrainGageRosetteMeasType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoUsbXferReqCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiPeriodHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiLinEncoderUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPeriodDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCountEdgesCountResetDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiBridgeCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBandpassType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTwoEdgeSepFirstLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoEnhancedImageRejectionEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiVelocityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiStrainGageCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCtrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseWidthUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiDataXferReqCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoCtrTimebaseDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiDigFltrLowpassCutoffFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiResolution => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                AiBridgePolyReverseCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AoDacOffsetVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoResolutionUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiAveragingWinSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiDutyCycleTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiDataXferMaxRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgeTwoPointLinSecondPhysicalVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoOutputState => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CiCountEdgesTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiVelocityEncoderBInputLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiDupCountPrevent => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiBridgeTableElectricalVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                CiSemiPeriodTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRtdC => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPeriodUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiChanCalScaleType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoFuncGenModulationType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTwoEdgeSepSecondDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiOutputState => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiThrmcplType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiThrmcplCjcSrc => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CiFreqEnableAveraging => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiEncoderZInputDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseTicksDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseTicksDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CoUsbXferReqCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiBridgeShuntCalShuntCalBActualResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesGateThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoFilterDelayAdjustment => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgeTwoPointLinFirstElectricalVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoPulseLowTicks => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiMin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiLowpassSwitchCapClkSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiMemMapEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseWidthLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderZIndexVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseWidthDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeScaleType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiFilterOrder => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiPulseWidthStartingEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoPulseDone => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiAtten => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderBInputDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseWidthDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiEncoderBInputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiLvdtUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiPowerSupplyFaultDetectEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiNotchWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoUsbXferReqSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiChanCalOperatorName => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CoCount => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiBridgeShuntCalShuntCalBResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoCurrentUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoMax => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgeBalanceCoarsePot => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeShuntCalShuntCalAActualResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiThrmstrR1 => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountResetTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CoDataXferMech => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderDecodingType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiSemiPeriodDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiVelocityLinEncoderUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoPulseLowTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoGenerateOn => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseFreqDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiBandpassCenterFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiForceUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiSensorPowerVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                NavLatUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiAccelChargeSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoOutputDriveType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepFirstDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoVoltageUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DiDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiSampAndHoldEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DoOvercurrentAutoReenable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiTimestampTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiSemiPeriodLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDacRngHigh => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoFuncGenOffset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFilterDelay => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CiThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiDataXferMech => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiLowpassSwitchCapExtClkFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoLoadImpedance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesGateDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiPulseTimeDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiPulseWidthDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseTimeStartEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesGateEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiTwoEdgeSepFirstTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesGateTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CoAutoIncrCnt => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiPulseTicksDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiExcitVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountDirThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepFirstTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTwoEdgeSepSecondEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoVoltageCurrentLimit => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBandpassEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiChanCalPolyReverseCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AiLeadWireResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiDataXferReqCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountResetDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityEncoderBInputTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiExcitDCorAc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoMemMapEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPeriodDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiCountEdgesCountResetResetCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiImpedance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoFuncGenStartPhase => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiDataXferMech => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiCoupling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiResistanceCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDataXferMech => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiThrmstrC => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiNumPossiblyInvalidSamps => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CiEncoderZInputDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoFuncGenFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityEncoderAInputDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiLowpassEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiTempUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiInputSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiChanCalApplyCalIfExp => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseTimeDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiAcExcitSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                ChanSyncUnlockBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoLineStatesPausedState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDataXferReqCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseTimeDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiVelocityEncoderAInputDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgeShuntCalEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCustomScaleName => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CoCtrTimebaseDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiNumLines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                CiPulseFreqTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseFreqTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AoFuncGenFmDeviation => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCtrTimebaseDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiDutyCycleDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiDigFltrOrder => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiUsbXferReqCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiRawDataCompressionType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoTristate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiLvdtSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseTimeUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiFilterDelayUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiHighpassEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseWidthTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoEnableInitialDelayOnRetrigger => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiInputLimitsFaultDetectUpperLimit => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderZIndexEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPeriodMeasMeth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DiInvertLines => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiVelocityAngEncoderUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountResetHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiAcExcitFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiSemiPeriodDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiExcitIdleOutputBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiFreqMeasTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiGpsSyncMethod => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DiUsbXferReqSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiCountEdgesGateDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiSampClkOverrunSentinelVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiThrmcplLeadOffsetVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiDigFltrBandpassCenterFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiDigFltrBandpassWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiExcitSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRawSampSize => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                DoUsbXferReqSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiAdcCustomTimingMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiSemiPeriodDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiCustomScaleName => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiIsTeds => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AiMax => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderAInputDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseTicksStartEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountDirDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiHighpassOrder => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiDataXferMech => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseFreqDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiCurrentAcrmsUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoFilterDelayUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiDitherEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCtrTimebaseDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiStrainGagePoissonRatio => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiDigFltrResponse => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiDutyCycleDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiRvdtSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiThrmstrA => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesInitialCnt => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiChanCalTableScaledVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AoUsbXferReqSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiAngEncoderPulsesPerRev => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiGpsSyncSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiResolutionUnits => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiOpenThrmcplDetectEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CoCtrTimebaseDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFilterOrder => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AoFuncGenAmplitude => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiAccel4WireDcVoltageSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTwoEdgeSepSecondDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiMeasType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiDevScalingCoeff => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                CiSemiPeriodDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiPulseTimeTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiStrainGageForceReadFromChan => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                DoDataXferMech => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiAngEncoderUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseFreqUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiExcitSense => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRtdType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoCtrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiDutyCycleDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiAccelSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseTimeDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoCommonModeOffset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoOvercurrentReenablePeriod => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepSecondTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountResetEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiVelocityEncoderBInputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiBandpassWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiDutyCycleDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiMicrophoneSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqStartingEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiForceReadFromChan => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiTimestampInitialSeconds => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiFilterFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiFreqThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoDataXferReqCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiSampClkOverrunBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountResetTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiChanCalPolyForwardCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AiRawSampJustification => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CiEncoderZInputLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoPulseTimeInitialDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiOpenChanDetectEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCountEdgesThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiCurrentShuntLoc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderZInputDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiEncoderAInputLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiEncoderAInputDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiTwoEdgeSepUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoRdyForNewVal => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                CiPeriodEnableAveraging => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiVelocityMeasTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiLowpassCutoffFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFilterDelayUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCtrTimebaseDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiTimestampEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoUseOnlyOnBrdMem => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseTimeTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiMin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiChanCalDesc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiRtdR0 => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiDigFltrHighpassCutoffFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoInvertLines => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                ChanType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CoPulseHighTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoPulseTimeUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiFreqUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DiUsbXferReqCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiSemiPeriodDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountDirTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRvdtSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiThrmcplCjcChan => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AiEnhancedAliasRejectionEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CoConstrainedGenMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiDigFltrType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseWidthDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiTwoEdgeSepSecondDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiChanCalEnableCal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiChanCalHasValidCalInfo => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AoFilterDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiAccelUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderAInputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiBridgeShuntCalShuntCalASrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiFilterFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepSecondDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiFreqThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderZInputDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiSoundPressureDbRef => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseTicksDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiUsbXferReqCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiNotchCenterFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                NavMeasType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CiCountEdgesGateHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiForceIepeSensorSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiTristate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCountEdgesDirTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiDutyCycleLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTwoEdgeSepFirstDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiTwoEdgeSepSecondTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesCountDirLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderBInputDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiVelocityEncoderAInputDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                DiDigFltrEnableBusMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCountEdgesCountResetLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiNotchType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseTicksDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountDirDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepFirstDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoPulseHighTicks => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiCountEdgesCountResetThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoPulseTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiEncoderZInputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiForceIepeSensorSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiFreqTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiTwoEdgeSepFirstDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCountEdgesDir => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiMeasType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CiAngEncoderInitialAngle => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoDacRefAllowConnToGnd => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiEncoderBInputDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountResetDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderZIndexPhase => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDacRefVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderBInputDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                NavTimestampUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountResetActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiFreqTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DiLogicFamily => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiStrainUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoLineStatesDoneState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTimestampUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiFilterResponse => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRngHigh => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiChopEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseTicksTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiChanCalVerifRefVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                NavCustomScaleName => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiMemMapEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoDacRefConnToGnd => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiLowpassSwitchCapOutClkDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiVelocityAngEncoderPulsesPerRev => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiBridgeShuntCalGainAdjust => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiFilterEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiVoltageAcrmsUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiLossyLsbRemovalCompressedSampSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiPeriodDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPeriodLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCtrTimebaseDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPeriodTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiInputLimitsFaultDetectEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiExcitUseForScaling => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoDacRngLow => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCtrTimebaseDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiVelocityIepeSensorDbRef => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoDataXferReqCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoPulseIdleState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeTwoPointLinFirstPhysicalVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoDacOffsetSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoMemMapEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiAccelSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgePolyForwardCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                CiFreqDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiMaxMeasPeriod => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiVoltageUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPeriodDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoPrescaler => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CoOutputType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CiTwoEdgeSepFirstEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiVelocityEncoderAInputDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiSensorPowerType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiVelocityIepeSensorSensitivityUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiDataXferCustomThreshold => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiPressureUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeTwoPointLinSecondElectricalVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiMemMapEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCountEdgesGateTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiEncoderAInputDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CoPulseFreqUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiMax => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                ChanDescr => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesCountDirDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoUseOnlyOnBrdMem => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiBridgeBalanceFinePot => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                DoLineStatesStartState => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiVoltageDbRef => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderAInputDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoIdleOutputBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeTablePhysicalVals => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                AiRosetteStrainGageOrientation => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityEncoderAInputTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiCurrentShuntResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesGateWhen => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiCountEdgesCountDirHyst => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoFuncGenType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoMemMapEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiPulseFreqDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoLogicFamily => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                NavTimestampTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AoDevScalingCoeff => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                CoCtrTimebaseDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                NavAltUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiVelocityDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AoDacRefSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                NavTrackUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiOvercurrentDetectEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiExcitVoltageOrCurrent => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiRvdtUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiUsbXferReqSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiFilterDelayAdjustment => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiDigFltrNotchCenterFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiAccel4WireDcVoltageSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesGateLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiGain => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesCountResetDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiAdcTimingMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiDigFltrNotchWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiHighpassType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiDataXferReqCond => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiStrainGageGageFactor => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgeInitialVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiTedsUnits => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CiDutyCycleStartingEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiFreqLogicLvlBehavior => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CoCtrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCtrTimebaseMasterTimebaseDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AoFuncGenSquareDutyCycle => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoUsbXferReqCount => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                AiLvdtSensitivity => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiEncoderAInputTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiVelocityEncoderAInputTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesGateDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPeriodThreshVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiProbeAtten => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DiDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiCountEdgesCountResetDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiFilterResponse => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiHighpassCutoffFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                DoNumLines => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AoOutputType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                CoPulseFreq => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiDutyCycleTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiPulseFreqDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTcReached => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                CiEncoderZInputTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiVelocityEncoderBInputDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCount => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AiTorqueUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                PhysicalChanName => AttrStruct { access: AttrAccess::ReadWrite, resettable: false, ty: [i8;N] },
                AiRosetteStrainGageRosetteType => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;0] },
                AiDigFltrCoeff => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;N] },
                CiVelocityEncoderBInputDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiPulseWidthTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiFreqDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiAccelDbRef => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoCustomScaleName => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiDcOffset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiSensorPowerCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiThrmstrB => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AoMin => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiAutoZeroMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeShuntCalSelect => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiSoundPressureMaxSoundPressureLvl => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiThrmcplScaleType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                NavSpeedOverGroundUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiBridgeShuntCalShuntCalBSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiFilterEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoGain => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiLinEncoderInitialPos => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiSemiPeriodTerm => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiResistanceUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiLowpassSwitchCapExtClkDiv => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CoPulseDutyCyc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoPulseTicksInitialDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                CiPeriodDigSyncEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoReglitchEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCtrTimebaseActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiExcitActualVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiVelocityEncoderBInputDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CoCtrTimebaseActiveEdge => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiPulseTimeDigFltrMinPulseWidth => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiRtdA => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiExcitUseMultiplexed => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoOutputImpedance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiRosetteStrainGageStrainChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                CiCountEdgesCountDirDigFltrTimebaseRate => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                AiBridgeNomResistance => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiTwoEdgeSepFirstDigFltrEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AiRemoveFilterDelay => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                CiCtrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiEddyCurrentProxProbeUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiTimestampTimescale => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AiThrmcplCjcVal => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                CiCountEdgesDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                CiTwoEdgeSepSecondDigFltrTimebaseSrc => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                AiFilterDelay => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                CiCountEdgesTermCfg => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CiFreqUnits => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
            }
        }
    }
}

pub enum PhysicalChannel {
    PhysicalChanAiSensorPowerOpenChan,
    PhysicalChanAiSensorPowerOvercurrent,
    PhysicalChanTedsModelNum,
    PhysicalChanAiSensorPowerTypes,
    PhysicalChanAoTermCfgs,
    PhysicalChanAiInputSrcs,
    AoPowerAmpChannelEnable,
    AoPowerAmpGain,
    PhysicalChanDiSampModes,
    PhysicalChanTedsBitStream,
    PhysicalChanAiPowerControlVoltage,
    PhysicalChanTedsSerialNum,
    AoPowerAmpOffset,
    PhysicalChanTedsMfgId,
    PhysicalChanDiSampClkSupported,
    PhysicalChanCiSupportedMeasTypes,
    PhysicalChanAoSupportedOutputTypes,
    PhysicalChanNavSupportedMeasTypes,
    PhysicalChanAiPowerControlEnable,
    PhysicalChanDoSampModes,
    PhysicalChanAiSensorPowerVoltageRangeVals,
    PhysicalChanAoManualControlShortDetected,
    PhysicalChanTedsVersionNum,
    PhysicalChanAiPowerControlType,
    PhysicalChanAiSupportedMeasTypes,
    PhysicalChanTedsTemplateIDs,
    PhysicalChanAoManualControlFreq,
    PhysicalChanTedsVersionLetter,
    PhysicalChanCoSupportedOutputTypes,
    AoPowerAmpScalingCoeff,
    PhysicalChanAoManualControlAmplitude,
    PhysicalChanDiPortWidth,
    AoPowerAmpOvercurrent,
    PhysicalChanDoSampClkSupported,
    PhysicalChanAiTermCfgs,
    PhysicalChanDiChangeDetectSupported,
    PhysicalChanDoPortWidth,
    PhysicalChanAoManualControlEnable,
    PhysicalChanAoSupportedPowerUpOutputTypes,
}

impl PhysicalChannel {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use PhysicalChannel::*;
            match self {
                PhysicalChanAiSensorPowerOpenChan => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanAiSensorPowerOvercurrent => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanTedsModelNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                PhysicalChanAiSensorPowerTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanAoTermCfgs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanAiInputSrcs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                AoPowerAmpChannelEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                AoPowerAmpGain => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                PhysicalChanDiSampModes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanTedsBitStream => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u8;N] },
                PhysicalChanAiPowerControlVoltage => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                PhysicalChanTedsSerialNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AoPowerAmpOffset => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                PhysicalChanTedsMfgId => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                PhysicalChanDiSampClkSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanCiSupportedMeasTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanAoSupportedOutputTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanNavSupportedMeasTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanAiPowerControlEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                PhysicalChanDoSampModes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanAiSensorPowerVoltageRangeVals => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                PhysicalChanAoManualControlShortDetected => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanTedsVersionNum => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                PhysicalChanAiPowerControlType => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                PhysicalChanAiSupportedMeasTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanTedsTemplateIDs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;N] },
                PhysicalChanAoManualControlFreq => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                PhysicalChanTedsVersionLetter => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                PhysicalChanCoSupportedOutputTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                AoPowerAmpScalingCoeff => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;N] },
                PhysicalChanAoManualControlAmplitude => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [f64;0] },
                PhysicalChanDiPortWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AoPowerAmpOvercurrent => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanDoSampClkSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanAiTermCfgs => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
                PhysicalChanDiChangeDetectSupported => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PhysicalChanDoPortWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                PhysicalChanAoManualControlEnable => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                PhysicalChanAoSupportedPowerUpOutputTypes => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i32;N] },
            }
        }
    }
}

pub enum PersistedChannel {
    AllowInteractiveEditing,
    ActiveChan,
    AllowInteractiveDeletion,
    Author,
}

impl PersistedChannel {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use PersistedChannel::*;
            match self {
                AllowInteractiveEditing => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                ActiveChan => AttrStruct { access: AttrAccess::Write, resettable: false, ty: [i8;N] },
                AllowInteractiveDeletion => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                Author => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
            }
        }
    }
}

pub enum Read {
    OverloadedChans,
    LoggingFilePath,
    ExcitFaultChansExist,
    PllUnlockedChansExist,
    LoggingTdmsGroupName,
    PllUnlockedChans,
    ExcitFaultChans,
    OpenChansExist,
    InputLimitsFaultChansExist,
    TotalSampPerChanAcquired,
    LoggingPause,
    InputLimitsFaultChans,
    NavFixLost,
    LoggingMode,
    ReadAllAvailSamp,
    OverloadedChansExist,
    PowerSupplyFaultChansExist,
    ChangeDetectHasOverflowed,
    Overwrite,
    CurrReadPos,
    NumChans,
    AutoStart,
    OpenCurrentLoopChans,
    LoggingFilePreallocationSize,
    OpenChansDetails,
    SleepTime,
    SyncUnlockedChans,
    OpenThrmcplChansExist,
    DevsWithInsertedOrRemovedAccessories,
    OpenCurrentLoopChansExist,
    OpenThrmcplChans,
    RelativeTo,
    AccessoryInsertionOrRemovalDetected,
    PowerSupplyFaultChans,
    RawDataWidth,
    DigitalLinesBytesPerChan,
    WaitMode,
    CommonModeRangeErrorChansExist,
    AvailSampPerChan,
    OvertemperatureChans,
    OpenChans,
    SyncUnlockedChansExist,
    Offset,
    CommonModeRangeErrorChans,
    OvercurrentChansExist,
    LoggingTdmsOperation,
    ChannelsToRead,
    LoggingSampsPerFile,
    OvertemperatureChansExist,
    LoggingFileWriteSize,
    OvercurrentChans,
}

impl Read {
    const fn value<T, N>(self) -> AttrStruct {
        {
            use Read::*;
            match self {
                OverloadedChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                LoggingFilePath => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                ExcitFaultChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PllUnlockedChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                LoggingTdmsGroupName => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                PllUnlockedChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                ExcitFaultChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OpenChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                InputLimitsFaultChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                TotalSampPerChanAcquired => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u64;0] },
                LoggingPause => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                InputLimitsFaultChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                NavFixLost => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                LoggingMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ReadAllAvailSamp => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                OverloadedChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PowerSupplyFaultChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                ChangeDetectHasOverflowed => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                Overwrite => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CurrReadPos => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u64;0] },
                NumChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                AutoStart => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [bool;0] },
                OpenCurrentLoopChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                LoggingFilePreallocationSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u64;0] },
                OpenChansDetails => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                SleepTime => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [f64;0] },
                SyncUnlockedChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OpenThrmcplChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                DevsWithInsertedOrRemovedAccessories => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OpenCurrentLoopChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                OpenThrmcplChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                RelativeTo => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                AccessoryInsertionOrRemovalDetected => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                PowerSupplyFaultChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                RawDataWidth => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                DigitalLinesBytesPerChan => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                WaitMode => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CommonModeRangeErrorChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                AvailSampPerChan => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [u32;0] },
                OvertemperatureChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OpenChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                SyncUnlockedChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                Offset => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                CommonModeRangeErrorChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
                OvercurrentChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                LoggingTdmsOperation => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i32;0] },
                ChannelsToRead => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [i8;N] },
                LoggingSampsPerFile => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u64;0] },
                OvertemperatureChansExist => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [bool;0] },
                LoggingFileWriteSize => AttrStruct { access: AttrAccess::ReadWrite, resettable: true, ty: [u32;0] },
                OvercurrentChans => AttrStruct { access: AttrAccess::Read, resettable: false, ty: [i8;N] },
            }
        }
    }
}

enum AttrAccess {
    Read,
    Write,
    ReadWrite,
}

struct AttrStruct<T, const N: usize> {
    access: AttrAccess,
    resettable: bool,
    ty: [T; N],
}