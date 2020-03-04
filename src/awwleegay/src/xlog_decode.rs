use crate::utils::*;
use flate2::bufread::DeflateDecoder;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::mem;

const MAGIC_COMPRESS_NO_CRYPT_START: u8 = 0x09;
const MAGIC_END: u8 = 0x00;
const HEAD: usize = 1 + 2 + 1 + 1 + 4;
const CRYPY_KEY_LEN: usize = 64;
const CODE_OFFSET: usize = HEAD + CRYPY_KEY_LEN;

#[derive(Debug)]
struct LogBufferMeta {
    offset: usize,
    seq: u16,
    begin_hour: u8,
    end_hour: u8,
    length: usize,
}

#[derive(Debug)]
pub enum DetectError {
    WrongStart(u8),
    WrongEnd(u8),
    HeadOutOfRange,
    LengthOutOfEnd,
    OverFlow(usize, usize),
    OffsetOutOfRange,
}

fn is_good_log_buffer(buffer: &mut Vec<u8>, offset: usize) -> Result<LogBufferMeta, DetectError> {
    fn is_start(byte: u8) -> bool {
        byte == MAGIC_COMPRESS_NO_CRYPT_START
    }

    if offset >= buffer.len() {
        return Err(DetectError::OffsetOutOfRange);
    }
    if !is_start(buffer[offset]) {
        return Err(DetectError::WrongStart(buffer[offset]));
    }

    if (offset + CODE_OFFSET) as u64 > buffer.len() as u64 {
        return Err(DetectError::HeadOutOfRange);
    }

    let length = unpack_from!(['I'], buffer, offset + 5) as usize;

    if offset as u64 + CODE_OFFSET as u64 + length as u64 > 0xFFFFFFFF {
        return Err(DetectError::OverFlow(offset, length));
    }

    if offset + CODE_OFFSET + length > buffer.len() {
        return Err(DetectError::LengthOutOfEnd);
    }

    if buffer[offset + CODE_OFFSET + length] != MAGIC_END {
        return Err(DetectError::WrongEnd(buffer[offset + CODE_OFFSET + length]));
    }

    let seq = unpack_from!(['H'], buffer, offset + 1);
    let begin_hour = unpack_from!(['B'], buffer, offset + 3);
    let end_hour = unpack_from!(['B'], buffer, offset + 4);

    let meta = LogBufferMeta {
        offset,
        seq,
        begin_hour,
        end_hour,
        length,
    };

    Ok(meta)
}

fn search_log_buffer(buffer: &mut Vec<u8>, mut offset: usize) -> Option<LogBufferMeta> {
    loop {
        match is_good_log_buffer(buffer, offset) {
            Ok(log_buffer_meta) => return Some(log_buffer_meta),
            Err(e) => match e {
                DetectError::OffsetOutOfRange => return None,
                _ => offset += 1,
            },
        }
    }
}

fn decode_log_buffer(buffer: &mut Vec<u8>, meta: &LogBufferMeta) -> std::io::Result<String> {
    let code_begin = meta.offset + CODE_OFFSET;
    let code_end = code_begin + meta.length as usize;

    let code = &buffer[code_begin..code_end];
    let mut bytes = Vec::new();

    let mut decompressor = DeflateDecoder::new(code);
    decompressor.read_to_end(&mut bytes)?;
    let s = String::from_utf8_lossy(&bytes).into_owned();
    Ok(s)
}

fn parse_file(src_file: &str) -> std::io::Result<Vec<(LogBufferMeta, String)>> {
    let mut buffer = Vec::new();
    let mut src_file = File::open(src_file)?;
    let mut decode_result = Vec::new();

    src_file.read_to_end(&mut buffer)?;
    let mut search_start = 0;
    loop {
        match search_log_buffer(&mut buffer, search_start) {
            Some(meta) => {
                let s = decode_log_buffer(&mut buffer, &meta)?;
                search_start = meta.offset + CODE_OFFSET + meta.length as usize + 1;
                decode_result.push((meta, s));
            }
            None => break,
        }
    }
    Ok(decode_result)
}

pub(crate) fn parse_to_file(src_file: &str, dst_file: &str) -> std::io::Result<()> {
    let decode_result = parse_file(src_file)?;
    stmp("Done parse_file");

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dst_file)?;
    let mut final_log = String::new();
    for (_meta, log) in decode_result {
        final_log.push_str(&log);
    }
    file.write(final_log.as_bytes())?;

    stmp("Done parse_to_file");

    Ok(())
}

pub(crate) fn parse_to_file_tmp(src_file: &str) -> std::io::Result<std::path::PathBuf> {
    let dst_file = crate::utils::gen_tmp_path(src_file)?;
    parse_to_file(src_file, &dst_file.display().to_string())?;
    Ok(dst_file)
}

pub(crate) fn parse_to_string(src_file: &str) -> std::io::Result<String> {
    let decode_result = parse_file(src_file)?;
    let mut ret_log = String::new();
    for (_, log) in decode_result {
        ret_log.push_str(&log);
    }
    Ok(ret_log)
}
