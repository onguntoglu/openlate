pub mod DaqmxValues {
    
    #[derive(Debug)]
    pub enum ACExcitWireMode {
        [object]
        4_WIRE,
        [object]
        5_WIRE,
        [object]
        6_WIRE,
        
    }
    
    #[derive(Debug)]
    pub enum ADCTimingMode {
        [object]
        AUTOMATIC,
        [object]
        HIGH_RESOLUTION,
        [object]
        HIGH_SPEED,
        [object]
        BEST_50_HZ_REJECTION,
        [object]
        BEST_60_HZ_REJECTION,
        [object]
        CUSTOM,
        
    }
    
    #[derive(Debug)]
    pub enum AIMeasurementType {
        [object]
        VOLTAGE,
        [object]
        VOLTAGE_RMS,
        [object]
        CURRENT,
        [object]
        CURRENT_RMS,
        [object]
        VOLTAGE_CUSTOM_WITH_EXCITATION,
        [object]
        BRIDGE,
        [object]
        FREQ_VOLTAGE,
        [object]
        RESISTANCE,
        [object]
        TEMP_TC,
        [object]
        TEMP_THRMSTR,
        [object]
        TEMP_RTD,
        [object]
        TEMP_BUILT_IN_SENSOR,
        [object]
        STRAIN_GAGE,
        [object]
        ROSETTE_STRAIN_GAGE,
        [object]
        POSITION_LVDT,
        [object]
        POSITION_RVDT,
        [object]
        POSITION_EDDY_CURRENT_PROXIMITY_PROBE,
        [object]
        ACCELEROMETER,
        [object]
        ACCELERATION_CHARGE,
        [object]
        ACCELERATION_4_WIRE_DC_VOLTAGE,
        [object]
        VELOCITY_IEPE_SENSOR,
        [object]
        FORCE_BRIDGE,
        [object]
        FORCE_IEPE_SENSOR,
        [object]
        PRESSURE_BRIDGE,
        [object]
        SOUND_PRESSURE_MICROPHONE,
        [object]
        TORQUE_BRIDGE,
        [object]
        TEDS_SENSOR,
        [object]
        CHARGE,
        
    }
    
    #[derive(Debug)]
    pub enum AOIdleOutputBehavior {
        [object]
        ZERO_VOLTS,
        [object]
        HIGH_IMPEDANCE,
        [object]
        MAINTAIN_EXISTING_VALUE,
        
    }
    
    #[derive(Debug)]
    pub enum AOOutputChannelType {
        [object]
        VOLTAGE,
        [object]
        CURRENT,
        [object]
        FUNC_GEN,
        
    }
    
    #[derive(Debug)]
    pub enum AOPowerUpOutputBehavior {
        [object]
        10322,
        [object]
        10134,
        [object]
        12527,
        
    }
    
    #[derive(Debug)]
    pub enum AccelChargeSensitivityUnits {
        [object]
        PICO_COULOMBS_PER_G,
        [object]
        PICO_COULOMBS_PER_METERS_PER_SECOND_SQUARED,
        [object]
        PICO_COULOMBS_PER_INCHES_PER_SECOND_SQUARED,
        
    }
    
    #[derive(Debug)]
    pub enum AccelSensitivityUnits1 {
        [object]
        M_VOLTS_PER_G,
        [object]
        VOLTS_PER_G,
        
    }
    
    #[derive(Debug)]
    pub enum AccelUnits2 {
        [object]
        ACCEL_UNIT_G,
        [object]
        METERS_PER_SECOND_SQUARED,
        [object]
        INCHES_PER_SECOND_SQUARED,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum AcquisitionType {
        [object]
        FINITE_SAMPS,
        [object]
        CONT_SAMPS,
        [object]
        HW_TIMED_SINGLE_POINT,
        
    }
    
    #[derive(Debug)]
    pub enum ActiveLevel {
        [object]
        ABOVE_LVL,
        [object]
        BELOW_LVL,
        
    }
    
    #[derive(Debug)]
    pub enum AltRef {
        
        MSL,
        
        HAE,
        
    }
    
    #[derive(Debug)]
    pub enum AngleUnits1 {
        [object]
        DEGREES,
        [object]
        RADIANS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum AngleUnits2 {
        [object]
        DEGREES,
        [object]
        RADIANS,
        [object]
        TICKS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum AngleUnits3 {
        
        DEGREES,
        
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum AngularVelocityUnits {
        [object]
        RPM,
        [object]
        RADIANS_PER_SECOND,
        [object]
        DEGREES_PER_SECOND,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum AntStatus {
        
        UNKNOWN,
        
        NORMAL,
        
        ABSENT,
        
        OVERCURRENT,
        
    }
    
    #[derive(Debug)]
    pub enum AutoZeroType1 {
        [object]
        NONE,
        [object]
        ONCE,
        [object]
        EVERY_SAMPLE,
        
    }
    
    #[derive(Debug)]
    pub enum BridgeConfiguration1 {
        [object]
        FULL_BRIDGE,
        [object]
        HALF_BRIDGE,
        [object]
        QUARTER_BRIDGE,
        [object]
        NO_BRIDGE,
        
    }
    
    #[derive(Debug)]
    pub enum BridgeElectricalUnits {
        [object]
        VOLTS_PER_VOLT,
        [object]
        M_VOLTS_PER_VOLT,
        
    }
    
    #[derive(Debug)]
    pub enum BridgePhysicalUnits {
        [object]
        NEWTONS,
        [object]
        POUNDS,
        [object]
        KILOGRAM_FORCE,
        [object]
        PASCALS,
        [object]
        POUNDS_PER_SQUARE_INCH,
        [object]
        BAR,
        [object]
        NEWTON_METERS,
        [object]
        INCH_OUNCES,
        [object]
        INCH_POUNDS,
        [object]
        FOOT_POUNDS,
        
    }
    
    #[derive(Debug)]
    pub enum BridgeShuntCalSource {
        [object]
        BUILT_IN,
        [object]
        USER_PROVIDED,
        
    }
    
    #[derive(Debug)]
    pub enum BridgeUnits {
        [object]
        VOLTS_PER_VOLT,
        [object]
        M_VOLTS_PER_VOLT,
        [object]
        FROM_CUSTOM_SCALE,
        [object]
        FROM_TEDS,
        
    }
    
    #[derive(Debug)]
    pub enum BusType {
        [object]
        PCI,
        [object]
        PC_IE,
        [object]
        PXI,
        [object]
        PX_IE,
        [object]
        SCXI,
        [object]
        SCC,
        [object]
        PC_CARD,
        [object]
        USB,
        [object]
        COMPACT_DAQ,
        [object]
        COMPACT_RIO,
        [object]
        TCPIP,
        [object]
        UNKNOWN,
        [object]
        SWITCH_BLOCK,
        
    }
    
    #[derive(Debug)]
    pub enum CIMeasurementType {
        [object]
        COUNT_EDGES,
        [object]
        FREQ,
        [object]
        PERIOD,
        [object]
        PULSE_WIDTH,
        [object]
        SEMI_PERIOD,
        [object]
        PULSE_FREQUENCY,
        [object]
        PULSE_TIME,
        [object]
        PULSE_TICKS,
        [object]
        DUTY_CYCLE,
        [object]
        POSITION_ANG_ENCODER,
        [object]
        POSITION_LIN_ENCODER,
        [object]
        VELOCITY_ANG_ENCODER,
        [object]
        VELOCITY_LIN_ENCODER,
        [object]
        TWO_EDGE_SEP,
        [object]
        GPS_TIMESTAMP,
        
    }
    
    #[derive(Debug)]
    pub enum CJCSource1 {
        [object]
        BUILT_IN,
        [object]
        CONST_VAL,
        [object]
        CHAN,
        
    }
    
    #[derive(Debug)]
    pub enum COOutputType {
        [object]
        PULSE_TIME,
        [object]
        PULSE_FREQ,
        [object]
        PULSE_TICKS,
        
    }
    
    #[derive(Debug)]
    pub enum ChannelType {
        [object]
        AI,
        [object]
        AO,
        [object]
        DI,
        [object]
        DO,
        [object]
        CI,
        [object]
        CO,
        
    }
    
    #[derive(Debug)]
    pub enum ChargeUnits {
        [object]
        COULOMBS,
        [object]
        PICO_COULOMBS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum ConstrainedGenMode {
        [object]
        UNCONSTRAINED,
        [object]
        FIXED_HIGH_FREQ,
        [object]
        FIXED_LOW_FREQ,
        [object]
        FIXED_50_PERCENT_DUTY_CYCLE,
        
    }
    
    #[derive(Debug)]
    pub enum CountDirection1 {
        [object]
        COUNT_UP,
        [object]
        COUNT_DOWN,
        [object]
        EXT_CONTROLLED,
        
    }
    
    #[derive(Debug)]
    pub enum CounterFrequencyMethod {
        [object]
        LOW_FREQ_1_CTR,
        [object]
        HIGH_FREQ_2_CTR,
        [object]
        LARGE_RNG_2_CTR,
        [object]
        DYN_AVG,
        
    }
    
    #[derive(Debug)]
    pub enum Coupling1 {
        [object]
        AC,
        [object]
        DC,
        [object]
        GND,
        
    }
    
    #[derive(Debug)]
    pub enum Coupling2 {
        [object]
        AC,
        [object]
        DC,
        
    }
    
    #[derive(Debug)]
    pub enum CurrentShuntResistorLocation1 {
        [object]
        INTERNAL,
        [object]
        EXTERNAL,
        
    }
    
    #[derive(Debug)]
    pub enum CurrentShuntResistorLocationWithDefault {
        
        DEFAULT,
        
        INTERNAL,
        
        EXTERNAL,
        
    }
    
    #[derive(Debug)]
    pub enum CurrentUnits1 {
        [object]
        AMPS,
        [object]
        FROM_CUSTOM_SCALE,
        [object]
        FROM_TEDS,
        
    }
    
    #[derive(Debug)]
    pub enum CurrentUnits2 {
        
        AMPS,
        
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum DataJustification1 {
        [object]
        RIGHT_JUSTIFIED,
        [object]
        LEFT_JUSTIFIED,
        
    }
    
    #[derive(Debug)]
    pub enum DataTransferMechanism {
        [object]
        DMA,
        [object]
        INTERRUPTS,
        [object]
        PROGRAMMED_IO,
        [object]
        US_BBULK,
        
    }
    
    #[derive(Debug)]
    pub enum DeassertCondition {
        [object]
        ONBRD_MEM_MORE_THAN_HALF_FULL,
        [object]
        ONBRD_MEM_FULL,
        [object]
        ONBRD_MEM_CUSTOM_THRESHOLD,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalDriveType {
        [object]
        ACTIVE_DRIVE,
        [object]
        OPEN_COLLECTOR,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalLineState {
        [object]
        HIGH,
        [object]
        LOW,
        [object]
        TRISTATE,
        [object]
        NO_CHANGE,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalPatternCondition1 {
        [object]
        PATTERN_MATCHES,
        [object]
        PATTERN_DOES_NOT_MATCH,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalWidthUnits1 {
        [object]
        SAMP_CLK_PERIODS,
        [object]
        SECONDS,
        [object]
        TICKS,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalWidthUnits2 {
        [object]
        SECONDS,
        [object]
        TICKS,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalWidthUnits3 {
        [object]
        SECONDS,
        
    }
    
    #[derive(Debug)]
    pub enum DigitalWidthUnits4 {
        [object]
        SECONDS,
        [object]
        SAMPLE_CLK_PERIODS,
        
    }
    
    #[derive(Debug)]
    pub enum EddyCurrentProxProbeSensitivityUnits {
        [object]
        M_VOLTS_PER_MIL,
        [object]
        VOLTS_PER_MIL,
        [object]
        M_VOLTS_PER_MILLIMETER,
        [object]
        VOLTS_PER_MILLIMETER,
        [object]
        M_VOLTS_PER_MICRON,
        
    }
    
    #[derive(Debug)]
    pub enum Edge1 {
        [object]
        RISING,
        [object]
        FALLING,
        
    }
    
    #[derive(Debug)]
    pub enum EncoderType2 {
        [object]
        X1,
        [object]
        X2,
        [object]
        X4,
        [object]
        TWO_PULSE_COUNTING,
        
    }
    
    #[derive(Debug)]
    pub enum EncoderZIndexPhase1 {
        [object]
        A_HIGH_B_HIGH,
        [object]
        A_HIGH_B_LOW,
        [object]
        A_LOW_B_HIGH,
        [object]
        A_LOW_B_LOW,
        
    }
    
    #[derive(Debug)]
    pub enum EveryNSamplesEventType {
        
        ACQUIRED_INTO_BUFFER,
        
        TRANSFERRED_FROM_BUFFER,
        
    }
    
    #[derive(Debug)]
    pub enum ExcitationDCorAC {
        [object]
        DC,
        [object]
        AC,
        
    }
    
    #[derive(Debug)]
    pub enum ExcitationIdleOutputBehavior {
        [object]
        ZERO_VOLTS_OR_AMPS,
        [object]
        MAINTAIN_EXISTING_VALUE,
        
    }
    
    #[derive(Debug)]
    pub enum ExcitationSource {
        [object]
        INTERNAL,
        [object]
        EXTERNAL,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum ExcitationVoltageOrCurrent {
        [object]
        VOLTAGE,
        [object]
        CURRENT,
        
    }
    
    #[derive(Debug)]
    pub enum ExportActions {
        
        PULSE,
        
        TOGGLE,
        
        LVL,
        
    }
    
    #[derive(Debug)]
    pub enum ExportActions2 {
        [object]
        PULSE,
        [object]
        TOGGLE,
        
    }
    
    #[derive(Debug)]
    pub enum ExportActions3 {
        [object]
        PULSE,
        [object]
        LVL,
        
    }
    
    #[derive(Debug)]
    pub enum ExportActions5 {
        [object]
        INTERLOCKED,
        [object]
        PULSE,
        
    }
    
    #[derive(Debug)]
    pub enum FilterResponse {
        [object]
        CONSTANT_GROUP_DELAY,
        [object]
        BUTTERWORTH,
        [object]
        ELLIPTICAL,
        [object]
        HARDWARE_DEFINED,
        
    }
    
    #[derive(Debug)]
    pub enum FilterResponse1 {
        [object]
        COMB,
        [object]
        BESSEL,
        [object]
        BRICKWALL,
        [object]
        BUTTERWORTH,
        
    }
    
    #[derive(Debug)]
    pub enum FilterType1 {
        
        HARDWARE_DEFINED,
        
    }
    
    #[derive(Debug)]
    pub enum FilterType2 {
        [object]
        LOWPASS,
        [object]
        HIGHPASS,
        [object]
        BANDPASS,
        [object]
        NOTCH,
        [object]
        CUSTOM,
        
    }
    
    #[derive(Debug)]
    pub enum ForceIEPESensorSensitivityUnits {
        [object]
        M_VOLTS_PER_NEWTON,
        [object]
        M_VOLTS_PER_POUND,
        
    }
    
    #[derive(Debug)]
    pub enum ForceIEPEUnits {
        
        NEWTONS,
        
        POUNDS,
        
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum ForceUnits {
        [object]
        NEWTONS,
        [object]
        POUNDS,
        [object]
        KILOGRAM_FORCE,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum FrequencyUnits {
        [object]
        HZ,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum FrequencyUnits2 {
        [object]
        HZ,
        
    }
    
    #[derive(Debug)]
    pub enum FrequencyUnits3 {
        [object]
        HZ,
        [object]
        TICKS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum FuncGenType {
        [object]
        SINE,
        [object]
        TRIANGLE,
        [object]
        SQUARE,
        [object]
        SAWTOOTH,
        
    }
    
    #[derive(Debug)]
    pub enum GpsSignalType1 {
        [object]
        IRIGB,
        [object]
        PPS,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum GroupBy {
        
        GROUP_BY_CHANNEL,
        
        GROUP_BY_SCAN_NUMBER,
        
    }
    
    #[derive(Debug)]
    pub enum HandshakeStartCondition {
        [object]
        IMMEDIATE,
        [object]
        WAIT_FOR_HANDSHAKE_TRIGGER_ASSERT,
        [object]
        WAIT_FOR_HANDSHAKE_TRIGGER_DEASSERT,
        
    }
    
    #[derive(Debug)]
    pub enum InputDataTransferCondition {
        [object]
        ON_BRD_MEM_MORE_THAN_HALF_FULL,
        [object]
        ON_BRD_MEM_NOT_EMPTY,
        [object]
        ONBRD_MEM_CUSTOM_THRESHOLD,
        [object]
        WHEN_ACQ_COMPLETE,
        
    }
    
    #[derive(Debug)]
    pub enum InputTermCfg {
        [object]
        RSE,
        [object]
        NRSE,
        [object]
        DIFF,
        [object]
        PSEUDO_DIFF,
        
    }
    
    #[derive(Debug)]
    pub enum InputTermCfg2 {
        [object]
        DIFF,
        [object]
        RSE,
        
    }
    
    #[derive(Debug)]
    pub enum InputTermCfgWithDefault {
        
        CFG_DEFAULT,
        
        RSE,
        
        NRSE,
        
        DIFF,
        
        PSEUDO_DIFF,
        
    }
    
    #[derive(Debug)]
    pub enum InvertPolarity {
        
        DO_NOT_INVERT_POLARITY,
        
        INVERT_POLARITY,
        
    }
    
    #[derive(Debug)]
    pub enum LVDTSensitivityUnits1 {
        [object]
        M_VOLTS_PER_VOLT_PER_MILLIMETER,
        [object]
        M_VOLTS_PER_VOLT_PER_MILLI_INCH,
        
    }
    
    #[derive(Debug)]
    pub enum LengthUnits2 {
        [object]
        METERS,
        [object]
        INCHES,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum LengthUnits3 {
        [object]
        METERS,
        [object]
        INCHES,
        [object]
        TICKS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum LengthUnits4 {
        
        METERS,
        
        FEET,
        
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum Level1 {
        [object]
        HIGH,
        [object]
        LOW,
        
    }
    
    #[derive(Debug)]
    pub enum LineGrouping {
        
        CHAN_PER_LINE,
        
        CHAN_FOR_ALL_LINES,
        
    }
    
    #[derive(Debug)]
    pub enum LoggingMode {
        [object]
        OFF,
        [object]
        LOG,
        [object]
        LOG_AND_READ,
        
    }
    
    #[derive(Debug)]
    pub enum LoggingOperation {
        [object]
        OPEN,
        [object]
        OPEN_OR_CREATE,
        [object]
        CREATE_OR_REPLACE,
        [object]
        CREATE,
        
    }
    
    #[derive(Debug)]
    pub enum LogicFamily {
        [object]
        2POINT_5_V,
        [object]
        3POINT_3_V,
        [object]
        5_V,
        
    }
    
    #[derive(Debug)]
    pub enum LogicLvlBehavior {
        [object]
        LOGIC_LEVEL_PULL_UP,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum MIOAIConvertTbSrc {
        [object]
        SAME_AS_SAMP_TIMEBASE,
        [object]
        SAME_AS_MASTER_TIMEBASE,
        [object]
        100_MHZ_TIMEBASE,
        [object]
        80_MHZ_TIMEBASE,
        [object]
        20_MHZ_TIMEBASE,
        [object]
        8_MHZ_TIMEBASE,
        
    }
    
    #[derive(Debug)]
    pub enum ModulationType {
        [object]
        AM,
        [object]
        FM,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum NavMeasurementType {
        
        ALTITUDE,
        
        LONGITUDE,
        
        LATITUDE,
        
        SPEED_OVER_GROUND,
        
        TRACK,
        
        TIMESTAMP,
        
        VERT_VELOCITY,
        
    }
    
    #[derive(Debug)]
    pub enum NavMode {
        
        MOBILE,
        
        STATIONARY_WITH_SURVEY,
        
        STATIONARY_WITH_PRESET_LOCATION,
        
    }
    
    #[derive(Debug)]
    pub enum OutputDataTransferCondition {
        [object]
        ON_BRD_MEM_EMPTY,
        [object]
        ON_BRD_MEM_HALF_FULL_OR_LESS,
        [object]
        ON_BRD_MEM_NOT_FULL,
        
    }
    
    #[derive(Debug)]
    pub enum OutputTermCfg {
        [object]
        RSE,
        [object]
        DIFF,
        [object]
        PSEUDO_DIFF,
        
    }
    
    #[derive(Debug)]
    pub enum OverflowBehavior {
        [object]
        STOP_TASK_AND_ERROR,
        [object]
        IGNORE_OVERRUNS,
        
    }
    
    #[derive(Debug)]
    pub enum OverwriteMode1 {
        [object]
        OVERWRITE_UNREAD_SAMPS,
        [object]
        DO_NOT_OVERWRITE_UNREAD_SAMPS,
        
    }
    
    #[derive(Debug)]
    pub enum Polarity2 {
        [object]
        ACTIVE_HIGH,
        [object]
        ACTIVE_LOW,
        
    }
    
    #[derive(Debug)]
    pub enum PowerUpChannelType {
        
        CHANNEL_VOLTAGE,
        
        CHANNEL_CURRENT,
        
        CHANNEL_HIGH_IMPEDANCE,
        
    }
    
    #[derive(Debug)]
    pub enum PowerUpStates {
        
        HIGH,
        
        LOW,
        
        TRISTATE,
        
    }
    
    #[derive(Debug)]
    pub enum PressureUnits {
        [object]
        PASCALS,
        [object]
        POUNDS_PER_SQUARE_INCH,
        [object]
        BAR,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum ProductCategory {
        [object]
        M_SERIES_DAQ,
        [object]
        X_SERIES_DAQ,
        [object]
        E_SERIES_DAQ,
        [object]
        S_SERIES_DAQ,
        [object]
        B_SERIES_DAQ,
        [object]
        SC_SERIES_DAQ,
        [object]
        USBDAQ,
        [object]
        AO_SERIES,
        [object]
        DIGITAL_IO,
        [object]
        TIO_SERIES,
        [object]
        DYNAMIC_SIGNAL_ACQUISITION,
        [object]
        SWITCHES,
        [object]
        COMPACT_DAQ_CHASSIS,
        [object]
        COMPACT_RIO_CHASSIS,
        [object]
        C_SERIES_MODULE,
        [object]
        SCXI_MODULE,
        [object]
        SCC_CONNECTOR_BLOCK,
        [object]
        SCC_MODULE,
        [object]
        NIELVIS,
        [object]
        NETWORK_DAQ,
        [object]
        SC_EXPRESS,
        [object]
        FIELD_DAQ,
        [object]
        UNKNOWN,
        
    }
    
    #[derive(Debug)]
    pub enum RTDType1 {
        [object]
        PT_3750,
        [object]
        PT_3851,
        [object]
        PT_3911,
        [object]
        PT_3916,
        [object]
        PT_3920,
        [object]
        PT_3928,
        [object]
        CUSTOM,
        
    }
    
    #[derive(Debug)]
    pub enum RVDTSensitivityUnits1 {
        [object]
        M_VOLTS_PER_VOLT_PER_DEGREE,
        [object]
        M_VOLTS_PER_VOLT_PER_RADIAN,
        
    }
    
    #[derive(Debug)]
    pub enum RawDataCompressionType {
        [object]
        NONE,
        [object]
        LOSSLESS_PACKING,
        [object]
        LOSSY_LSB_REMOVAL,
        
    }
    
    #[derive(Debug)]
    pub enum ReadRelativeTo {
        [object]
        FIRST_SAMPLE,
        [object]
        CURR_READ_POS,
        [object]
        REF_TRIG,
        [object]
        FIRST_PRETRIG_SAMP,
        [object]
        MOST_RECENT_SAMP,
        
    }
    
    #[derive(Debug)]
    pub enum RegenerationMode1 {
        [object]
        ALLOW_REGEN,
        [object]
        DO_NOT_ALLOW_REGEN,
        
    }
    
    #[derive(Debug)]
    pub enum ResistanceConfiguration {
        [object]
        2_WIRE,
        [object]
        3_WIRE,
        [object]
        4_WIRE,
        
    }
    
    #[derive(Debug)]
    pub enum ResistanceUnits1 {
        [object]
        OHMS,
        [object]
        FROM_CUSTOM_SCALE,
        [object]
        FROM_TEDS,
        
    }
    
    #[derive(Debug)]
    pub enum ResistanceUnits2 {
        
        OHMS,
        
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum ResistorState {
        
        PULL_UP,
        
        PULL_DOWN,
        
    }
    
    #[derive(Debug)]
    pub enum ResolutionType1 {
        [object]
        BITS,
        
    }
    
    #[derive(Debug)]
    pub enum SampClkOverrunBehavior {
        [object]
        REPEATED_DATA,
        [object]
        SENTINEL_VALUE,
        
    }
    
    #[derive(Debug)]
    pub enum SampleClockActiveOrInactiveEdgeSelection {
        [object]
        SAMP_CLK_ACTIVE_EDGE,
        [object]
        SAMP_CLK_INACTIVE_EDGE,
        
    }
    
    #[derive(Debug)]
    pub enum SampleInputDataWhen {
        [object]
        HANDSHAKE_TRIGGER_ASSERTS,
        [object]
        HANDSHAKE_TRIGGER_DEASSERTS,
        
    }
    
    #[derive(Debug)]
    pub enum SampleTimingType {
        [object]
        SAMP_CLK,
        [object]
        BURST_HANDSHAKE,
        [object]
        HANDSHAKE,
        [object]
        IMPLICIT,
        [object]
        ON_DEMAND,
        [object]
        CHANGE_DETECTION,
        [object]
        PIPELINED_SAMP_CLK,
        
    }
    
    #[derive(Debug)]
    pub enum SaveOptions {
        
        OVERWRITE,
        
        ALLOW_INTERACTIVE_EDITING,
        
        ALLOW_INTERACTIVE_DELETION,
        
    }
    
    #[derive(Debug)]
    pub enum ScaleType {
        [object]
        LINEAR,
        [object]
        MAP_RANGES,
        [object]
        POLYNOMIAL,
        [object]
        TABLE,
        
    }
    
    #[derive(Debug)]
    pub enum ScaleType2 {
        [object]
        POLYNOMIAL,
        [object]
        TABLE,
        
    }
    
    #[derive(Debug)]
    pub enum ScaleType3 {
        [object]
        POLYNOMIAL,
        [object]
        TABLE,
        
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum ScaleType4 {
        [object]
        NONE,
        [object]
        TWO_POINT_LINEAR,
        [object]
        TABLE,
        [object]
        POLYNOMIAL,
        
    }
    
    #[derive(Debug)]
    pub enum Sense {
        [object]
        LOCAL,
        [object]
        REMOTE,
        
    }
    
    #[derive(Debug)]
    pub enum SensorPowerCfg {
        [object]
        NO_CHANGE,
        [object]
        ENABLED,
        [object]
        DISABLED,
        
    }
    
    #[derive(Debug)]
    pub enum SensorPowerType {
        [object]
        DC,
        [object]
        AC,
        [object]
        BIPOLAR_DC,
        
    }
    
    #[derive(Debug)]
    pub enum ShuntCalSelect {
        [object]
        A,
        [object]
        B,
        [object]
        AAND_B,
        
    }
    
    #[derive(Debug)]
    pub enum Signal {
        
        AI_CONVERT_CLOCK,
        
        10_MHZ_REF_CLOCK,
        
        20_MHZ_TIMEBASE_CLOCK,
        
        SAMPLE_CLOCK,
        
        ADVANCE_TRIGGER,
        
        REFERENCE_TRIGGER,
        
        START_TRIGGER,
        
        ADV_CMPLT_EVENT,
        
        AI_HOLD_CMPLT_EVENT,
        
        COUNTER_OUTPUT_EVENT,
        
        CHANGE_DETECTION_EVENT,
        
        WDT_EXPIRED_EVENT,
        
    }
    
    #[derive(Debug)]
    pub enum Signal2 {
        [object]
        SAMPLE_COMPLETE_EVENT,
        [object]
        COUNTER_OUTPUT_EVENT,
        [object]
        CHANGE_DETECTION_EVENT,
        [object]
        SAMPLE_CLOCK,
        
    }
    
    #[derive(Debug)]
    pub enum Slope1 {
        [object]
        RISING_SLOPE,
        [object]
        FALLING_SLOPE,
        
    }
    
    #[derive(Debug)]
    pub enum SoundPressureUnits1 {
        [object]
        PASCALS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum SourceSelection {
        [object]
        INTERNAL,
        [object]
        EXTERNAL,
        
    }
    
    #[derive(Debug)]
    pub enum StrainGageBridgeType1 {
        [object]
        FULL_BRIDGE_I,
        [object]
        FULL_BRIDGE_II,
        [object]
        FULL_BRIDGE_III,
        [object]
        HALF_BRIDGE_I,
        [object]
        HALF_BRIDGE_II,
        [object]
        QUARTER_BRIDGE_I,
        [object]
        QUARTER_BRIDGE_II,
        
    }
    
    #[derive(Debug)]
    pub enum StrainGageRosetteMeasurementType {
        [object]
        PRINCIPAL_STRAIN_1,
        [object]
        PRINCIPAL_STRAIN_2,
        [object]
        PRINCIPAL_STRAIN_ANGLE,
        [object]
        CARTESIAN_STRAIN_X,
        [object]
        CARTESIAN_STRAIN_Y,
        [object]
        CARTESIAN_SHEAR_STRAIN_XY,
        [object]
        MAX_SHEAR_STRAIN,
        [object]
        MAX_SHEAR_STRAIN_ANGLE,
        
    }
    
    #[derive(Debug)]
    pub enum StrainGageRosetteType {
        [object]
        RECTANGULAR_ROSETTE,
        [object]
        DELTA_ROSETTE,
        [object]
        TEE_ROSETTE,
        
    }
    
    #[derive(Debug)]
    pub enum StrainUnits1 {
        [object]
        STRAIN,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum SyncPulseType {
        [object]
        ONBOARD,
        [object]
        DIG_EDGE,
        [object]
        TIME,
        
    }
    
    #[derive(Debug)]
    pub enum SyncType {
        [object]
        NONE,
        [object]
        MASTER,
        [object]
        SLAVE,
        
    }
    
    #[derive(Debug)]
    pub enum SyncUnlockBehavior {
        [object]
        STOP_TASK_AND_ERROR,
        [object]
        IGNORE_LOST_SYNC_LOCK,
        
    }
    
    #[derive(Debug)]
    pub enum TEDSUnits {
        [object]
        FROM_CUSTOM_SCALE,
        [object]
        FROM_TEDS,
        
    }
    
    #[derive(Debug)]
    pub enum TaskControlAction {
        
        TASK_START,
        
        TASK_STOP,
        
        TASK_VERIFY,
        
        TASK_COMMIT,
        
        TASK_RESERVE,
        
        TASK_UNRESERVE,
        
        TASK_ABORT,
        
    }
    
    #[derive(Debug)]
    pub enum TemperatureUnits {
        
        DEG_C,
        
        DEG_F,
        
        KELVINS,
        
        DEG_R,
        
    }
    
    #[derive(Debug)]
    pub enum TemperatureUnits1 {
        [object]
        DEG_C,
        [object]
        DEG_F,
        [object]
        KELVINS,
        [object]
        DEG_R,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum ThermocoupleType1 {
        [object]
        J_TYPE_TC,
        [object]
        K_TYPE_TC,
        [object]
        N_TYPE_TC,
        [object]
        R_TYPE_TC,
        [object]
        S_TYPE_TC,
        [object]
        T_TYPE_TC,
        [object]
        B_TYPE_TC,
        [object]
        E_TYPE_TC,
        
    }
    
    #[derive(Debug)]
    pub enum TimeUnits {
        [object]
        SECONDS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum TimeUnits2 {
        [object]
        SECONDS,
        
    }
    
    #[derive(Debug)]
    pub enum TimeUnits3 {
        [object]
        SECONDS,
        [object]
        TICKS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum Timescale {
        
        TAI,
        
        UTC,
        
    }
    
    #[derive(Debug)]
    pub enum Timescale2 {
        [object]
        HOST_TIME,
        [object]
        IO_DEVICE_TIME,
        
    }
    
    #[derive(Debug)]
    pub enum TimestampEvent {
        
        START_TRIGGER,
        
        REFERENCE_TRIGGER,
        
        ARM_START_TRIGGER,
        
        FIRST_SAMPLE_TIMESTAMP,
        
    }
    
    #[derive(Debug)]
    pub enum TorqueUnits {
        [object]
        NEWTON_METERS,
        [object]
        INCH_OUNCES,
        [object]
        INCH_POUNDS,
        [object]
        FOOT_POUNDS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerType10 {
        [object]
        ANLG_EDGE,
        [object]
        ANLG_MULTI_EDGE,
        [object]
        DIG_EDGE,
        [object]
        DIG_PATTERN,
        [object]
        ANLG_WIN,
        [object]
        TIME,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerType4 {
        [object]
        DIG_EDGE,
        [object]
        TIME,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerType5 {
        [object]
        DIG_EDGE,
        [object]
        SOFTWARE,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerType6 {
        [object]
        ANLG_LVL,
        [object]
        ANLG_WIN,
        [object]
        DIG_LVL,
        [object]
        DIG_PATTERN,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerType8 {
        [object]
        ANLG_EDGE,
        [object]
        ANLG_MULTI_EDGE,
        [object]
        DIG_EDGE,
        [object]
        DIG_PATTERN,
        [object]
        ANLG_WIN,
        [object]
        TIME,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerType9 {
        [object]
        INTERLOCKED,
        [object]
        NONE,
        
    }
    
    #[derive(Debug)]
    pub enum TriggerUsage {
        
        ADVANCE,
        
        PAUSE,
        
        REFERENCE,
        
        START,
        
        HANDSHAKE,
        
        ARM_START,
        
    }
    
    #[derive(Debug)]
    pub enum UnderflowBehavior {
        [object]
        HALT_OUTPUT_AND_ERROR,
        [object]
        PAUSE_UNTIL_DATA_AVAILABLE,
        
    }
    
    #[derive(Debug)]
    pub enum UnitsPreScaled {
        [object]
        VOLTS,
        [object]
        AMPS,
        [object]
        DEG_F,
        [object]
        DEG_C,
        [object]
        DEG_R,
        [object]
        KELVINS,
        [object]
        STRAIN,
        [object]
        OHMS,
        [object]
        HZ,
        [object]
        SECONDS,
        [object]
        METERS,
        [object]
        INCHES,
        [object]
        DEGREES,
        [object]
        RADIANS,
        [object]
        TICKS,
        [object]
        RPM,
        [object]
        RADIANS_PER_SECOND,
        [object]
        DEGREES_PER_SECOND,
        [object]
        G,
        [object]
        METERS_PER_SECOND_SQUARED,
        [object]
        INCHES_PER_SECOND_SQUARED,
        [object]
        METERS_PER_SECOND,
        [object]
        INCHES_PER_SECOND,
        [object]
        PASCALS,
        [object]
        NEWTONS,
        [object]
        POUNDS,
        [object]
        KILOGRAM_FORCE,
        [object]
        POUNDS_PER_SQUARE_INCH,
        [object]
        BAR,
        [object]
        NEWTON_METERS,
        [object]
        INCH_OUNCES,
        [object]
        INCH_POUNDS,
        [object]
        FOOT_POUNDS,
        [object]
        VOLTS_PER_VOLT,
        [object]
        M_VOLTS_PER_VOLT,
        [object]
        COULOMBS,
        [object]
        PICO_COULOMBS,
        [object]
        FROM_TEDS,
        
    }
    
    #[derive(Debug)]
    pub enum VelocityIEPESensorSensitivityUnits {
        [object]
        MILLIVOLTS_PER_MILLIMETER_PER_SECOND,
        [object]
        MILLI_VOLTS_PER_INCH_PER_SECOND,
        
    }
    
    #[derive(Debug)]
    pub enum VelocityUnits {
        [object]
        METERS_PER_SECOND,
        [object]
        INCHES_PER_SECOND,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum VelocityUnits2 {
        
        METERS_PER_SECOND,
        
        KILOMETERS_PER_HOUR,
        
        FEET_PER_SECOND,
        
        MILES_PER_HOUR,
        
        KNOTS,
        
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum VoltageUnits1 {
        [object]
        VOLTS,
        [object]
        FROM_CUSTOM_SCALE,
        [object]
        FROM_TEDS,
        
    }
    
    #[derive(Debug)]
    pub enum VoltageUnits2 {
        [object]
        VOLTS,
        [object]
        FROM_CUSTOM_SCALE,
        
    }
    
    #[derive(Debug)]
    pub enum WaitMode {
        [object]
        WAIT_FOR_INTERRUPT,
        [object]
        POLL,
        [object]
        YIELD,
        [object]
        SLEEP,
        
    }
    
    #[derive(Debug)]
    pub enum WaitMode2 {
        [object]
        POLL,
        [object]
        YIELD,
        [object]
        SLEEP,
        
    }
    
    #[derive(Debug)]
    pub enum WaitMode3 {
        [object]
        WAIT_FOR_INTERRUPT,
        [object]
        POLL,
        
    }
    
    #[derive(Debug)]
    pub enum WaitMode4 {
        [object]
        WAIT_FOR_INTERRUPT,
        [object]
        POLL,
        
    }
    
    #[derive(Debug)]
    pub enum WatchdogAOExpirState {
        [object]
        VOLTAGE,
        [object]
        CURRENT,
        [object]
        NO_CHANGE,
        
    }
    
    #[derive(Debug)]
    pub enum WatchdogAOOutputType {
        
        VOLTAGE,
        
        CURRENT,
        
        NO_CHANGE,
        
    }
    
    #[derive(Debug)]
    pub enum WatchdogCOExpirState {
        [object]
        LOW,
        [object]
        HIGH,
        [object]
        NO_CHANGE,
        
    }
    
    #[derive(Debug)]
    pub enum WatchdogControlAction {
        
        RESET_TIMER,
        
        CLEAR_EXPIRATION,
        
    }
    
    #[derive(Debug)]
    pub enum WindowTriggerCondition1 {
        [object]
        ENTERING_WIN,
        [object]
        LEAVING_WIN,
        
    }
    
    #[derive(Debug)]
    pub enum WindowTriggerCondition2 {
        [object]
        INSIDE_WIN,
        [object]
        OUTSIDE_WIN,
        
    }
    
    #[derive(Debug)]
    pub enum WriteBasicTEDSOptions {
        
        WRITE_TO_EEPROM,
        
        WRITE_TO_PROM,
        
        DO_NOT_WRITE,
        
    }
    
    #[derive(Debug)]
    pub enum WriteRelativeTo {
        [object]
        FIRST_SAMPLE,
        [object]
        CURR_WRITE_POS,
        
    }
    
}
