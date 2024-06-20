use ffi::Event;
use poll::Poll;
use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::vec;

mod ffi;
mod poll;

fn main() -> Result<(), io::Error> {
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams = vec![];

    let socket_addresses = "delay_server:8080".to_socket_addrs()?;
    let socket_address = socket_addresses
        .into_iter()
        .next()
        .ok_or("Unable to resolve DNS")
        .unwrap();

    println!("Connecting to: {socket_address}");

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let req = get_req(&url_path, &socket_address.to_string());

        let mut stream = std::net::TcpStream::connect(socket_address)?;

        stream.set_nonblocking(true)?;

        stream.write_all(req.as_bytes())?;
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;

        streams.push(stream);
    }

    println!("All streams connected");

    let mut handled_events = 0;

    let mut handled_ids = HashSet::new();

    while handled_events < n_events {
        let mut events = Vec::<ffi::Event>::with_capacity(10);

        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("Timeout or spurious event");
            continue;
        }

        handled_events += handle_events(&mut events, &mut streams, &mut handled_ids).unwrap();
    }

    println!("All events handled");

    Ok(())
}

fn get_req(path: &str, host: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: {host}\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

fn handle_events(
    events: &[Event],
    streams: &mut [TcpStream],
    handled: &mut HashSet<usize>,
) -> Result<usize, io::Error> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    if !handled.insert(index) {
                        break;
                    }
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    if txt.starts_with("HTTP") {
                        println!("RECEIVED: {:?}", event);
                        println!("{txt}\n------\n");
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}
