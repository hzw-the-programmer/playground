use std::io::Write;
use std::io::Read;
use std::net::TcpStream;

//const ECHO_SERVER_ADDRESS: &str = "tcpbin.com:4242";

//socat -v tcp-l:8080,fork exec:'/bin/cat'
const ECHO_SERVER_ADDRESS: &str = "localhost:8080";

fn main() {
    println!("connecting to echo server {}", ECHO_SERVER_ADDRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        println!("connected to echo server {}", ECHO_SERVER_ADDRESS);
        //println!("local address {}:{}", stream.local_addr().unwrap().ip(),
        //    stream.local_addr().unwrap().port());
        println!("local address {}", stream.local_addr().unwrap());
        let msg = "Hello World";
        stream.write(msg.as_bytes());
        stream.flush();
        println!("sent: {}", msg);

        let mut buf = [0;1024];
        stream.read(&mut buf);
        let msg = String::from_utf8_lossy(&buf);
        println!("received: {}", msg);
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
