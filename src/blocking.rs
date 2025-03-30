use crate::{
    error::Error,
    protocol::{SntpMessage, Timestamp},
};

pub trait NtpTransport {
    type SendError;
    type RecvError;

    fn send(&self, buffer: &[u8]) -> Result<(), Self::SendError>;
    fn recv(&self, buffer: &mut [u8]) -> Result<usize, Self::RecvError>;
}

fn sntp_send_and_recv<T>(transport: &T) -> Result<SntpMessage, Error<T::SendError, T::RecvError>>
where
    T: NtpTransport,
{
    let mut buf = [0; SntpMessage::BUFFER_SIZE];
    let mut msg = SntpMessage::new_v4();
    msg.write_to_buffer(&mut buf)?;
    transport.send(&buf).map_err(Error::TransportSend)?;
    transport.recv(&mut buf).map_err(Error::TransportRecv)?;
    msg.read_from_buffer(&buf)?;
    Ok(msg)
}

pub fn sntp_get_transmit_timestamp<T>(
    transport: &T,
) -> Result<Timestamp, Error<T::SendError, T::RecvError>>
where
    T: NtpTransport,
{
    let msg = sntp_send_and_recv(transport)?;
    Ok(msg.transmit_timestamp)
}

#[cfg(feature = "std")]
impl NtpTransport for std::net::UdpSocket {
    type SendError = std::io::Error;
    type RecvError = std::io::Error;

    fn send(&self, mut buffer: &[u8]) -> Result<(), Self::SendError> {
        while !buffer.is_empty() {
            let sent = self.send(buffer)?;
            buffer = &buffer[sent..];
        }
        Ok(())
    }

    fn recv(&self, buffer: &mut [u8]) -> Result<usize, Self::RecvError> {
        self.recv(buffer)
    }
}
