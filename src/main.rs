use std::pin::Pin;
use std::task::{Context, Poll};

use clap::Parser;
use hyper::Uri;
use hyper::{
    client::connect::{Connected, Connection},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

struct VsockStream(tokio_vsock::VsockStream);

impl Connection for VsockStream {
    fn connected(&self) -> Connected {
        let connected = Connected::new();

        connected
    }
}

impl AsyncRead for VsockStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for VsockStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

async fn vsock_connector(dst: Uri) -> Result<VsockStream, std::io::Error> {
    let scheme = dst.scheme().ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "uri should have a scheme",
    ))?;

    if scheme != "vsock" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "only vsock uris supported",
        ));
    }

    let authority = dst.authority().ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "uri should have an authority",
    ))?;

    tokio_vsock::VsockStream::connect(
        authority.host().parse::<u32>().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "uri should have a u32 host",
            )
        })?,
        authority
            .port_u16()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "uri should have a u16 port",
            ))?
            .into(),
    )
    .await
    .map(VsockStream)
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// url to query
    #[clap(short, long, value_parser)]
    url: String,
}

fn main() {
    let cli = Cli::parse();

    println!("url: {}", cli.url);
}
