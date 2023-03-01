use std::io;
use std::thread;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> io::Result<()> {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    socket.set_nonblocking(true).unwrap();
    loop {
        socket.send_to("BREED:ABORT".as_bytes(), "255.255.255.255:37540")?;
        println!("Send abort command to breed per 500ms.");
        let mut buf = [0; 14];
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                println!("Recv pong from breed.");  
                return Ok(());
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_secs_f32(0.5));
                continue;
            },
            Err(_) => {},
        }
    }
}
