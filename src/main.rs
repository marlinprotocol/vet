use clap::Parser;
use hyper::Uri;
use tokio_vsock::VsockStream;

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
