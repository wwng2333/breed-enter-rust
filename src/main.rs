fn main() -> std::io::Result<()> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:37540")?;
    socket.set_broadcast(true)?;
    socket.set_nonblocking(true).unwrap();
    loop {
        socket.send_to("BREED:ABORT".as_bytes(), "255.255.255.255:37541")?;
        println!("Send abort command to breed per 500ms.");
        let mut buf = [0; 14];
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                println!("Recv pong from breed.");
                open::that("http://192.168.1.1").unwrap();
                return Ok(());
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
                continue;
            }
            Err(_) => {}
        }
    }
}
