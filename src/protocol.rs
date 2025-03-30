/// SNTP message format
///
/// ```text
///                     1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |Mode| VN  |LI |     Stratum    |     Poll      |   Precision   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          Root Delay                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                       Root Dispersion                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                     Reference Identifier                      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                   Reference Timestamp (64)                    |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                   Originate Timestamp (64)                    |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                    Receive Timestamp (64)                     |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                    Transmit Timestamp (64)                    |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                 Key Identifier (optional) (32)                |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                                                               |
/// |                 Message Digest (optional) (128)               |
/// |                                                               |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug)]
pub struct SntpMessage {
    pub leap_indicator: LeapIndicator,
    pub version: Version,
    pub mode: Mode,
    pub stratum: u8,
    pub poll: u8,
    pub precision: u8,
    pub root_delay: u32,
    pub root_dispersion: u32,
    pub reference_identifier: u32,
    pub reference_timestamp: Timestamp,
    pub originate_timestamp: Timestamp,
    pub receive_timestamp: Timestamp,
    pub transmit_timestamp: Timestamp,
}

impl SntpMessage {
    pub const BUFFER_SIZE: usize = 48;

    pub fn new_v4() -> Self {
        Self {
            leap_indicator: LeapIndicator::NoWarning,
            version: Version::V4,
            mode: Mode::Client,
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            reference_identifier: 0,
            reference_timestamp: Timestamp(0),
            originate_timestamp: Timestamp(0),
            receive_timestamp: Timestamp(0),
            transmit_timestamp: Timestamp(0),
        }
    }

    pub fn write_to_buffer(&self, buffer: &mut [u8]) {
        let leap_indicator = match self.leap_indicator {
            LeapIndicator::NoWarning => 0,
            LeapIndicator::LastMinuteHas61Seconds => 1,
            LeapIndicator::LastMinuteHas59Seconds => 2,
            LeapIndicator::AlarmCondition => 3,
        };

        let version = match self.version {
            Version::V4 => 4,
            Version::V3 => 3,
        };

        let mode = match self.mode {
            Mode::Reserved => 0,
            Mode::SymmetricActive => 1,
            Mode::SymmetricPassive => 2,
            Mode::Client => 3,
            Mode::Server => 4,
            Mode::Broadcast => 5,
            Mode::Reserved6 => 6,
            Mode::Reserved7 => 7,
        };

        buffer[0] = mode | (version << 3) | (leap_indicator << 6);
        buffer[1] = self.stratum;
        buffer[2] = self.poll;
        buffer[3] = self.precision;
        buffer[4..8].copy_from_slice(&self.root_delay.to_be_bytes());
        buffer[8..12].copy_from_slice(&self.root_dispersion.to_be_bytes());
        buffer[12..16].copy_from_slice(&self.reference_identifier.to_be_bytes());
        buffer[16..24].copy_from_slice(&self.reference_timestamp.to_be_bytes());
        buffer[24..32].copy_from_slice(&self.originate_timestamp.to_be_bytes());
        buffer[32..40].copy_from_slice(&self.receive_timestamp.to_be_bytes());
        buffer[40..48].copy_from_slice(&self.transmit_timestamp.to_be_bytes());
    }

    pub fn read_from_buffer(&mut self, buffer: &[u8]) {
        self.mode = match buffer[0] & 0x7 {
            0 => Mode::Reserved,
            1 => Mode::SymmetricActive,
            2 => Mode::SymmetricPassive,
            3 => Mode::Client,
            4 => Mode::Server,
            5 => Mode::Broadcast,
            6 => Mode::Reserved6,
            7 => Mode::Reserved7,
            _ => unreachable!(),
        };

        self.version = match (buffer[0] >> 3) & 0x7 {
            4 => Version::V4,
            3 => Version::V3,
            v => unreachable!("version is {}", v),
        };

        self.leap_indicator = match (buffer[0] >> 6) & 0x3 {
            0 => LeapIndicator::NoWarning,
            1 => LeapIndicator::LastMinuteHas61Seconds,
            2 => LeapIndicator::LastMinuteHas59Seconds,
            3 => LeapIndicator::AlarmCondition,
            _ => unreachable!(),
        };

        self.stratum = buffer[1];
        self.poll = buffer[2];
        self.precision = buffer[3];
        self.root_delay = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        self.root_dispersion = u32::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
        self.reference_identifier =
            u32::from_be_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]);
        self.reference_timestamp = Timestamp::from_be_bytes([
            buffer[16], buffer[17], buffer[18], buffer[19], buffer[20], buffer[21], buffer[22],
            buffer[23],
        ]);
        self.originate_timestamp = Timestamp::from_be_bytes([
            buffer[24], buffer[25], buffer[26], buffer[27], buffer[28], buffer[29], buffer[30],
            buffer[31],
        ]);
        self.receive_timestamp = Timestamp::from_be_bytes([
            buffer[32], buffer[33], buffer[34], buffer[35], buffer[36], buffer[37], buffer[38],
            buffer[39],
        ]);
        self.transmit_timestamp = Timestamp::from_be_bytes([
            buffer[40], buffer[41], buffer[42], buffer[43], buffer[44], buffer[45], buffer[46],
            buffer[47],
        ]);
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum LeapIndicator {
    NoWarning = 0,
    LastMinuteHas61Seconds = 1,
    LastMinuteHas59Seconds = 2,
    /// Clock not synchronized
    AlarmCondition = 3,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Mode {
    Reserved = 0,
    SymmetricActive = 1,
    SymmetricPassive = 2,
    Client = 3,
    Server = 4,
    Broadcast = 5,
    /// reserved for NTP control message
    Reserved6 = 6,
    // reserved for private use
    Reserved7 = 7,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Version {
    V4 = 4,
    V3 = 3,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Timestamp(pub(crate) u64);

impl Timestamp {
    pub(crate) fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub(crate) fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }

    /// Returns true if the most significant bit is set.
    ///
    /// Relevant documentation from RFC 2030:
    ///
    /// ```text
    /// Note that, since some time in 1968 (second 2,147,483,648) the most
    /// significant bit (bit 0 of the integer part) has been set and that the
    /// 64-bit field will overflow some time in 2036 (second 4,294,967,296).
    /// Should NTP or SNTP be in use in 2036, some external means will be
    /// necessary to qualify time relative to 1900 and time relative to 2036
    /// (and other multiples of 136 years). There will exist a 200-picosecond
    /// interval, henceforth ignored, every 136 years when the 64-bit field
    /// will be 0, which by convention is interpreted as an invalid or
    /// unavailable timestamp.
    ///    As the NTP timestamp format has been in use for the last 17 years,
    ///    it remains a possibility that it will be in use 40 years from now
    ///    when the seconds field overflows. As it is probably inappropriate
    ///    to archive NTP timestamps before bit 0 was set in 1968, a
    ///    convenient way to extend the useful life of NTP timestamps is the
    ///    following convention: If bit 0 is set, the UTC time is in the
    ///    range 1968-2036 and UTC time is reckoned from 0h 0m 0s UTC on 1
    ///    January 1900. If bit 0 is not set, the time is in the range 2036-
    ///    2104 and UTC time is reckoned from 6h 28m 16s UTC on 7 February
    ///    2036. Note that when calculating the correspondence, 2000 is not a
    ///    leap year. Note also that leap seconds are not counted in the
    ///    reckoning.
    ///```
    pub fn msb_set(&self) -> bool {
        self.0 & (1 << 63) != 0
    }

    pub fn seconds(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn seconds_fraction(&self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "chrono")]
impl From<Timestamp> for chrono::NaiveDateTime {
    fn from(timestamp: Timestamp) -> Self {
        let epoch = if timestamp.msb_set() {
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            )
        } else {
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2036, 2, 7).unwrap(),
                chrono::NaiveTime::from_hms_opt(6, 28, 16).unwrap(),
            )
        };

        let micros_since_epoch = (timestamp.seconds() as i64) * 1_000_000
            + (((timestamp.seconds_fraction() as i64) * 1_000_000) / 0x100000000);
        let duration_since_epoc = chrono::Duration::microseconds(micros_since_epoch);

        epoch + duration_since_epoc
    }
}

#[cfg(feature = "chrono")]
impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(timestamp: Timestamp) -> Self {
        let epoch = if timestamp.msb_set() {
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            )
        } else {
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2036, 2, 7).unwrap(),
                chrono::NaiveTime::from_hms_opt(6, 28, 16).unwrap(),
            )
        }
        .and_utc();

        let micros_since_epoch = (timestamp.seconds() as i64) * 1_000_000
            + (((timestamp.seconds_fraction() as i64) * 1_000_000) / 0x100000000);
        let duration_since_epoc = chrono::Duration::microseconds(micros_since_epoch);

        epoch + duration_since_epoc
    }
}
