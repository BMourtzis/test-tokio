use bytes::{BytesMut, Buf};
use std::io::{
    Cursor,
    BufWriter
};
use mini_redis::{
    Frame,
    Result,
    frame::Error
};
use tokio::{
    net::TcpStream,
    io::AsyncReadExt
};

struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut
}

impl Connection {
    pub fn new(add: &str) -> Self {
        Connection {
            stream: BufWriter::new(TcpListener::Bind(&str).unwrap()),
            buffer: BytesMut::with_capaicty(4096)
        }
    }

    pub fn new_with_stream(stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capaicty(4096)
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Fame>> {
        loop {
            if let Some(frame) = self.parse_frame() ? {
                return Ok(Some(frame));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Error(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            },
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            },
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Bulk(val) => unimplemented!()
        }

        self.stream.flush().await;

        Ok(())
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);

                Ok(Some(frame))
            },
            Err(Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into())
        }
    }
}