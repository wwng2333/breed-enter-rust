use std::time;
use std::thread;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");
    socket.set_broadcast(true)?;
    loop {
        socket
            .send_to("BREED:ABORT".as_bytes(), "255.255.255.255:37540")
            .expect("Couldn't send data");
        println!("Send abort per 500ms.");
        let mut buf = [0; 32];
        let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Didn't receive data");
        if number_of_bytes > 0 {
            return Ok(());
        } else {
            thread::sleep(time::Duration::from_micros(500));
        }
    }
}
