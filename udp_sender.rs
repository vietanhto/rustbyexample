use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::io;

fn snd() -> Result<(), io::Error> {
    // Define the local connection (to send the data from)
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let connection = SocketAddrV4::new(ip, 9992);

    // Bind the socket
    let socket = try!(UdpSocket::bind(connection));

    // Define the remote connection (to send the data to)
    let connection2 = SocketAddrV4::new(ip, 9991);

    // Send data via the socket
    let buf = &vec![0; 9_012];
    try!(socket.send_to(buf, connection2));
    println!("{:?}", buf);

    Ok(())
}

fn main() {
    match snd() {
        Ok(()) => println!("All snd-ing went well"),
        Err(err) => println!("Error: {:?}", err),
    }
}
