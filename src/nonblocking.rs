use crate::{
    error::Error,
    protocol::{SntpMessage, Timestamp},
};

pub trait NtpTransportAsync {
    type SendError;
    type RecvError;

    fn send(&self, buffer: &[u8]) -> impl Future<Output = Result<(), Self::SendError>> + Send;
    fn recv(
        &self,
        buffer: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::RecvError>> + Send;
}

async fn sntp_send_and_recv<T>(
    transport: &T,
) -> Result<SntpMessage, Error<T::SendError, T::RecvError>>
where
    T: NtpTransportAsync,
{
    let mut buf = [0; SntpMessage::BUFFER_SIZE];
    let mut msg = SntpMessage::new_v4();
    msg.write_to_buffer(&mut buf);
    transport
        .send(&buf)
        .await
        .map_err(Error::TransportSendError)?;
    transport
        .recv(&mut buf)
        .await
        .map_err(Error::TransportRecvError)?;
    msg.read_from_buffer(&buf);
    Ok(msg)
}

pub async fn sntp_get_transmit_timestamp<T>(
    transport: &T,
) -> Result<Timestamp, Error<T::SendError, T::RecvError>>
where
    T: NtpTransportAsync,
{
    let msg = sntp_send_and_recv(transport).await?;
    Ok(msg.transmit_timestamp)
}

#[cfg(feature = "std")]
impl NtpTransportAsync for std::net::UdpSocket {
    type SendError = std::io::Error;
    type RecvError = std::io::Error;

    async fn send(&self, mut buffer: &[u8]) -> Result<(), Self::SendError> {
        while !buffer.is_empty() {
            let sent = self.send(buffer)?;
            buffer = &buffer[sent..];
        }
        Ok(())
    }

    async fn recv(&self, buffer: &mut [u8]) -> Result<usize, Self::RecvError> {
        self.recv(buffer)
    }
}
