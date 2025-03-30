#[derive(Debug)]
pub enum Error<S, R> {
    TransportSendError(S),
    TransportRecvError(R),
}

impl<S, R> core::fmt::Display for Error<S, R>
where
    S: core::fmt::Display,
    R: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TransportSendError(_) => write!(f, "transport send error"),
            Error::TransportRecvError(_) => write!(f, "transport recv error"),
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
            Error::TransportSendError(e) => Some(e),
            Error::TransportRecvError(e) => Some(e),
        }
    }
}
