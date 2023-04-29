use core::time;
use std::{io, str};
use std::{io::Read, net::TcpStream};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
};

fn main() -> io::Result<()> {
    thread::spawn(|| server());

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..100 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read!");
        stream.write(input.as_bytes()).expect("failed to write!");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("failed to read into buffer!");

        println!("sever return message: {}", str::from_utf8(&buffer).unwrap());
    }
    Ok(())
}

fn server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("stream failed!");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });
        thread_vec.push(handle)
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
