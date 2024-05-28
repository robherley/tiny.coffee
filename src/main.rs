use log::{info, warn};
use smol::{
    io::{self, AsyncWriteExt, BufReader},
    prelude::*,
    Async,
};
use std::{
    env,
    error::Error,
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

const INDEX: &str = include_str!("../static/index.html");
const COFFEE: [&[u8]; 4] = [
    include_bytes!("../frames/0.txt"),
    include_bytes!("../frames/1.txt"),
    include_bytes!("../frames/2.txt"),
    include_bytes!("../frames/3.txt"),
];
const ANSICOLORS: [&[u8]; 6] = [
    b"\x1b[31m",
    b"\x1b[32m",
    b"\x1b[33m",
    b"\x1b[34m",
    b"\x1b[35m",
    b"\x1b[36m",
];
const ANSIRESET: &[u8] = b"\x1b[0m";
const ANSICLEAR: &[u8] = b"\x1b[H\x1b[2J";

async fn index(mut stream: Async<TcpStream>) -> io::Result<()> {
    let response = format!(
        "Content-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        INDEX.len(),
        INDEX
    );
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn coffee(mut stream: Async<TcpStream>) -> io::Result<()> {
    stream.write_all(b"\r\n").await?;

    let mut counter: usize = 0;
    loop {
        let buffer = vec![
            ANSICLEAR,
            ANSICOLORS[counter % ANSICOLORS.len()],
            COFFEE[counter % COFFEE.len()],
            ANSIRESET,
        ]
        .concat();

        stream.write_all(&buffer).await?;
        smol::Timer::after(std::time::Duration::from_millis(100)).await;
        counter = counter.wrapping_add(1);
    }
}

async fn handler(mut stream: Async<TcpStream>, peer_addr: SocketAddr) -> io::Result<()> {
    let mut lines = BufReader::new(&stream).lines();

    let mut is_curl = false;
    while let Some(line) = lines.next().await {
        match line?.to_lowercase() {
            line if line.starts_with("user-agent:") => {
                is_curl = line.contains("curl");
                break;
            }
            line if line.is_empty() => break,
            _ => continue,
        }
    }

    info!(is_curl, peer_addr:?; "connected");
    stream.write_all(b"HTTP/1.1 200 OK\r\n").await?;

    let result = if is_curl {
        coffee(stream).await
    } else {
        index(stream).await
    };

    if let Err(err) = result {
        match err.kind() {
            io::ErrorKind::BrokenPipe | io::ErrorKind::ConnectionReset => {
                info!(is_curl, peer_addr:?; "disconnected");
            }
            _ => warn!("error: {}", err),
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .format_target(false)
        .init();

    let host: Ipv4Addr = match env::var("HOST") {
        Ok(host) => host.parse()?,
        Err(_) => Ipv4Addr::new(0, 0, 0, 0),
    };

    let port: u16 = match env::var("PORT") {
        Ok(port) => port.parse()?,
        Err(_) => 8000,
    };

    smol::block_on(async {
        let listener = Async::<TcpListener>::bind((host, port))?;
        info!(addr:? = listener.get_ref().local_addr()?; "listening");

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            smol::spawn(handler(stream, peer_addr)).detach();
        }
    })
}
