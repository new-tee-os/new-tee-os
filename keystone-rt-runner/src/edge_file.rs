use std::{fs::File, mem::ManuallyDrop};

use failure::ResultExt;
use hal::edge::EdgeMemory;

pub fn dispatch_api_call(edge_mem: &mut EdgeMemory) {
    use hal::edge::EdgeCallInfo::*;

    let result = match edge_mem.read_info() {
        FileOpen => edge_open(edge_mem).map(|file_obj| {
            edge_mem.write_info(U64Info(file_obj));
        }),
        FileGetSize { file_obj } => edge_get_size(file_obj).map(|size| {
            edge_mem.write_info(U64Info(size));
        }),
        FileSeek { file_obj, pos } => edge_seek(file_obj, pos),
        FileRead { file_obj, len } => edge_read(file_obj, &mut edge_mem.buffer[0..len as usize])
            .map(|bytes_read| edge_mem.buf_len = bytes_read),
        FileClose { file_obj } => Ok(edge_close(file_obj)),
        _ => panic!("invalid edge file API call"),
    };

    match result {
        Ok(_) => {
            edge_mem.req = 0;
        }
        Err(err) => {
            eprintln!("Warning: {:?}", err);
            edge_mem.req = 1;
        }
    }
}

pub fn edge_open(edge_mem: &mut EdgeMemory) -> Result<u64, failure::Error> {
    let path = std::str::from_utf8(edge_mem.read_buffer())
        .context("enclave passed an invalid UTF-8 string")?;
    let boxed_file = Box::new(File::open(path).context("failed to open edge file")?);
    let boxed_file_ptr = Box::into_raw(boxed_file);
    Ok(boxed_file_ptr as u64)
}

fn edge_get_file(file_obj: u64) -> ManuallyDrop<Box<File>> {
    ManuallyDrop::new(unsafe { Box::from_raw(file_obj as *mut File) })
}

pub fn edge_get_size(file_obj: u64) -> Result<u64, failure::Error> {
    let boxed_file = edge_get_file(file_obj);
    let file_len = boxed_file
        .metadata()
        .context("failed to stat edge file")?
        .len();
    Ok(file_len)
}

pub fn edge_seek(file_obj: u64, pos: u64) -> Result<(), failure::Error> {
    use std::io::{Seek, SeekFrom};
    let mut boxed_file = edge_get_file(file_obj);
    boxed_file
        .seek(SeekFrom::Start(pos))
        .map(|_| ()) // TODO: do something with the return value
        .context("failed to seek edge file")?;
    Ok(())
}

pub fn edge_read(file_obj: u64, buf: &mut [u8]) -> Result<u32, failure::Error> {
    use std::io::Read;
    let mut boxed_file = edge_get_file(file_obj);
    let bytes_read = boxed_file.read(buf).context("failed to read edge file")?;
    Ok(bytes_read as u32)
}

pub fn edge_close(file_obj: u64) {
    unsafe { ManuallyDrop::drop(&mut edge_get_file(file_obj)) };
}
