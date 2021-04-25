use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

pub fn read_file_concurrent(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let path = std::env::current_dir()
        .unwrap()
        .join(std::path::Path::new(filename));

    let file = File::open(path);

    match file {
        Ok(file) => {
            let (send, recv): (Sender<u8>, Receiver<u8>) = mpsc::channel();
            let mut reader = BufReader::new(file);
            std::thread::spawn(move || drain_reader(&mut reader, send));
            let mut data_vec = Vec::new();
            while let Ok(byte) = recv.recv() {
                data_vec.push(byte);
            }
            drop(recv);
            Ok(data_vec)
        }
        Err(err) => Err(err),
    }
}

fn drain_reader(reader: &mut BufReader<File>, sender: Sender<u8>) {
    for byte in reader.bytes() {
        let byte = byte.unwrap();
        sender.send(byte).unwrap();
    }
    drop(sender);
}
