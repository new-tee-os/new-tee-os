use std::{fs::File, mem::ManuallyDrop};

use bytes::{Buf, BufMut};
use failure::ResultExt;
use keystone_hal::edge::EdgeMemory;

pub fn on_error(err: &failure::Error, edge_mem: &mut EdgeMemory) {
    eprintln!("Warning: {}", err);
    edge_mem.req = 1;
}

pub fn edge_open(edge_mem: &mut EdgeMemory) -> Result<(), failure::Error> {
    let path = std::str::from_utf8(edge_mem.read_buffer())
        .context("enclave passed an invalid UTF-8 string")?;
    let boxed_file = Box::new(File::open(path).context("failed to open edge file")?);
    let boxed_file_ptr = Box::into_raw(boxed_file);
    (&mut edge_mem.buffer[0..8]).put_u64(boxed_file_ptr as u64);
    edge_mem.req = 0;
    Ok(())
}

fn edge_get_file(edge_mem: &mut EdgeMemory) -> ManuallyDrop<Box<File>> {
    let boxed_file_ptr = (&edge_mem.buffer[0..8]).get_u64();
    let boxed_file = ManuallyDrop::new(unsafe { Box::from_raw(boxed_file_ptr as *mut File) });
    boxed_file
}

pub fn edge_get_size(edge_mem: &mut EdgeMemory) -> Result<(), failure::Error> {
    let boxed_file = edge_get_file(edge_mem);
    let file_len = boxed_file
        .metadata()
        .context("failed to stat edge file")?
        .len();
    (&mut edge_mem.buffer[0..8]).put_u64(file_len);
    edge_mem.req = 0;
    Ok(())
}

pub fn edge_read(edge_mem: &mut EdgeMemory) -> Result<(), failure::Error> {
    use std::io::Read;
    let mut boxed_file = edge_get_file(edge_mem);
    let bytes_read = boxed_file
        .read(&mut edge_mem.buffer[0..edge_mem.len as usize])
        .context("failed to read edge file")?;
    edge_mem.len = bytes_read as u32;
    edge_mem.req = 0;
    Ok(())
}

pub fn edge_close(edge_mem: &mut EdgeMemory) {
    unsafe { ManuallyDrop::drop(&mut edge_get_file(edge_mem)) };
}
