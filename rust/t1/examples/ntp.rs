use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use std::io;
use std::net::UdpSocket;
use std::time::Duration;

const NTP_MESSAGE_LEN: usize = 12 * 4;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LEN],
}

impl NTPMessage {
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LEN],
        }
    }

    fn client() -> NTPMessage {
        const VERSION: u8 = 0b00_011_000;
        const MODE: u8 = 0b00_000_011;

        let mut msg = Self::new();

        msg.data[0] |= VERSION;
        msg.data[0] |= MODE;

        msg
    }

    fn rx_time(&self) -> Result<NTPTimeStamp, io::Error> {
        self.parse_timestamp(32)
    }

    fn tx_time(&self) -> Result<NTPTimeStamp, io::Error> {
        self.parse_timestamp(40)
    }

    fn parse_timestamp(&self, offset: usize) -> Result<NTPTimeStamp, io::Error> {
        let mut reader = &self.data[offset..offset + 8];

        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimeStamp { seconds, fraction })
    }
}

struct NTPTimeStamp {
    seconds: u32,
    fraction: u32,
}

impl From<NTPTimeStamp> for DateTime<Utc> {
    fn from(ntp: NTPTimeStamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);

        Utc.timestamp(secs, nanos as u32)
    }
}

struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, io::Error> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let request = NTPMessage::client();
    let mut response = NTPMessage::new();

    let udp = UdpSocket::bind(LOCAL_ADDR)?;

    udp.connect(&destination)?;

    let t1 = Utc::now();

    udp.send(&request.data)?;

    udp.set_read_timeout(Some(timeout))?;
    udp.recv(&mut response.data)?;

    let t4 = Utc::now();

    let t2 = response.rx_time().unwrap().into();
    let t3 = response.tx_time().unwrap().into();

    Ok(NTPResult { t1, t2, t3, t4 })
}

fn main() {
    ntp_roundtrip("time.nist.gov", 123).unwrap();
}
