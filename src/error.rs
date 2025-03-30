#[derive(Debug)]
#[non_exhaustive]
pub enum Error<S, R> {
    TransportSend(S),
    TransportRecv(R),
    SntpProtocol(SntpProtocolError),
}

impl<S, R> core::fmt::Display for Error<S, R>
where
    S: core::fmt::Display,
    R: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TransportSend(_) => write!(f, "transport send error"),
            Error::TransportRecv(_) => write!(f, "transport recv error"),
            Error::SntpProtocol(_) => write!(f, "SNTP protocol error"),
        }
    }
}

impl<S, R> core::error::Error for Error<S, R>
where
    S: 'static + core::error::Error,
    R: 'static + core::error::Error,
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::TransportSend(e) => Some(e),
            Error::TransportRecv(e) => Some(e),
            Error::SntpProtocol(e) => Some(e),
        }
    }
}

impl<S, R> From<SntpProtocolError> for Error<S, R> {
    fn from(e: SntpProtocolError) -> Self {
        Error::SntpProtocol(e)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SntpProtocolError {
    SntpBufferTooSmall { size: usize, expected: usize },
    InvalidSntpMode(u8),
    InvalidSntpVersion(u8),
    InvalidSntpLeadIndicator(u8),
}

impl core::fmt::Display for SntpProtocolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SntpProtocolError::SntpBufferTooSmall { size, expected } => {
                write!(
                    f,
                    "SNTP buffer is too small for message: size={size}, expected={expected}"
                )
            }
            SntpProtocolError::InvalidSntpMode(mode) => write!(f, "invalid SNTP mode: 0x{mode:x}"),
            SntpProtocolError::InvalidSntpVersion(version) => {
                write!(f, "invalid SNTP version: 0x{version:x}")
            }
            SntpProtocolError::InvalidSntpLeadIndicator(lead_indicator) => {
                write!(f, "invalid SNTP lead indicator: 0x{lead_indicator:x}")
            }
        }
    }
}

impl core::error::Error for SntpProtocolError {}
