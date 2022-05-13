use std::io::{Read, Write};

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("-d") => {
            let mut stdin = std::io::stdin();
            let mut buf = Vec::new();
            stdin.read_to_end(&mut buf).unwrap();
            let buf = String::from_utf8(buf).unwrap();
            let parsed = buf.trim().parse::<u64>().unwrap();
            let decompresed = decompress(parsed);
            std::io::stdout().write_all(&decompresed).unwrap();
            println!();
        }
        Some("-e") => {
            let mut stdin = std::io::stdin();
            let mut buf = Vec::new();
            stdin.read_to_end(&mut buf).unwrap();
            let buf = String::from_utf8(buf).unwrap();
            let compressed = compress(buf.trim().as_bytes().to_vec());
            println!("Compressed: {}", compressed);
        }
        _ => {
            println!("Usage: {} [-d] [-e]", std::env::args().next().unwrap());
        }
    }
}

// "Compress" the data
fn compress(data: Vec<u8>) -> u64 {
    let (send, receive) = std::sync::mpsc::channel::<u64>();
    for i in 0..rayon::current_num_threads() {
        let sender = send.clone();
        let my_data = data.clone();
        rayon::spawn(move || {
            let mut iteration = i as u64;
            loop {
                let mut hasher = blake3::Hasher::new();
                hasher.update(&iteration.to_le_bytes());
                let hash = hasher.finalize();
                if hash.as_bytes().starts_with(&my_data) {
                    break sender.send(iteration).unwrap();
                }
                iteration += rayon::current_num_threads() as u64;
            }
        });
    }

    receive.recv().unwrap()
}

// "Decompress" the data
fn decompress(data: u64) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&data.to_le_bytes());
    let hash = hasher.finalize();
    hash.as_bytes().to_vec()
}
