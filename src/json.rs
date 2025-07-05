use std::{io, marker::PhantomData};

use futures::{AsyncRead, AsyncWrite};
use libp2p_request_response::Codec;
use serde::{de::DeserializeOwned, Serialize};

use crate::{read_length_prefixed, write_length_prefixed};

const MAX_SIZE: usize = 1024 * 1024;

/// Length-prefix CBOR codec.
pub struct LpJson<Req, Resp, Protocol> {
    phantom: PhantomData<(Req, Resp, Protocol)>,
}

impl<Req, Resp, Protocol> Default for LpJson<Req, Resp, Protocol> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Req, Resp, Protocol> Clone for LpJson<Req, Resp, Protocol> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl<Req, Resp, Protocol> Codec for LpJson<Req, Resp, Protocol>
where
    Req: Send + Serialize + DeserializeOwned,
    Resp: Send + Serialize + DeserializeOwned,
    Protocol: AsRef<str> + Send + Clone,
{
    type Protocol = Protocol;
    type Request = Req;
    type Response = Resp;

    async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, MAX_SIZE).await?;
        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        serde_json::from_slice(vec.as_slice()).map_err(serde_json_error_into_io_error)
    }

    /// Reads a response from the given I/O stream according to the
    /// negotiated protocol.
    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, MAX_SIZE).await?;
        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        serde_json::from_slice(vec.as_slice()).map_err(serde_json_error_into_io_error)
    }

    /// Writes a request to the given I/O stream according to the
    /// negotiated protocol.
    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data: Vec<u8> = serde_json::to_vec(&req).map_err(serde_json_error_into_io_error)?;
        write_length_prefixed(io, data).await
    }

    /// Writes a response to the given I/O stream according to the
    /// negotiated protocol.
    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data: Vec<u8> = serde_json::to_vec(&res).map_err(serde_json_error_into_io_error)?;
        write_length_prefixed(io, data).await
    }
}

fn serde_json_error_into_io_error(err: serde_json::Error) -> io::Error {
    match err.classify() {
        serde_json::error::Category::Io => io::Error::new(
            err.io_error_kind().unwrap_or(io::ErrorKind::Other),
            err.to_string(),
        ),
        serde_json::error::Category::Data | serde_json::error::Category::Syntax => {
            io::Error::new(io::ErrorKind::Other, err.to_string())
        }
        serde_json::error::Category::Eof => {
            io::Error::new(io::ErrorKind::UnexpectedEof, err.to_string())
        }
    }
}
