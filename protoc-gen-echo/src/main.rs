use std::{
    cell::RefCell,
    io::{self, Read, Write},
    ops::Deref,
};

fn main() {
    let mut buf: Vec<u8> = Vec::default();
    io::stdin().read_to_end(&mut buf).unwrap();
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    handle.write_all(b"[echo start]\n").unwrap();
    handle.write_all(&buf).unwrap();
    handle.write_all(b"\n[echo end]").unwrap();
}
