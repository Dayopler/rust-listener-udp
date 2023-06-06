use std::net::UdpSocket;

fn main() {
    loop {
        const IP_ADDR: &str = "127.0.0.1:0";
        let socket = UdpSocket::bind(IP_ADDR).expect("couldn't bind to address");
        let mut buf = vec![0; 60];

        let (_nb_bytes, _src_addr) = socket.recv_from(&mut buf).expect("Didn't reveive data");

        println!("The buffer : {:?}", buf)
    }
}
