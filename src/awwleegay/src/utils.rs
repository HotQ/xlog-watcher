use std::ffi::CString;
use std::os::raw::c_char;

#[macro_export]
macro_rules! trans_bytes_to_builtin {
    ($buffer:ident, $offset:expr, $len:expr, $ty:ty) => {{
        let mut source = [0u8; $len];
        source.copy_from_slice(
            $buffer
                .as_slice()
                .get($offset..$offset + $len)
                .unwrap_or_else(|| {
                    println!("shit happen");
                    &[0u8; $len]
                }),
        );

        let sink = unsafe { mem::transmute::<[u8; $len], $ty>(source) }; // FIXME: any better way?
        sink
    }};
}

#[macro_export]
macro_rules! unpack_from {
    // https://docs.python.org/2/library/struct.html
    (['B'], $buffer:ident, $offset:expr) => {
        trans_bytes_to_builtin!($buffer, $offset, 1, u8);
    };
    (['H'], $buffer:ident, $offset:expr) => {
        trans_bytes_to_builtin!($buffer, $offset, 2, u16);
    };
    (['I'], $buffer:ident, $offset:expr) => {
        trans_bytes_to_builtin!($buffer, $offset, 4, u32);
    };
}

pub(crate) fn stmp(info: &str) {
    let dt = chrono::prelude::Local::now();
    let time = dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    println!("{}\t\t{}", time, info);
}

pub(crate) fn gen_tmp_path(file: &str) -> std::io::Result<std::path::PathBuf> {
    use std::path::*;

    let xlog_file = Path::new(file);
    let xlog_file_stem = if let Some(s) = xlog_file.file_stem() {
        s.to_str().unwrap_or("")
    } else {
        ""
    };

    #[cfg(any(target_os = "macos"))]
    let tmp_path = "/tmp";

    let dt = chrono::prelude::Local::now();
    let date = dt.format("%Y-%m-%d").to_string();
    let time = dt.format("%H%M%S%3f").to_string();

    let log_file = format!("{}-{}.xlog.log", xlog_file_stem, time);

    let mut path: PathBuf = [tmp_path, "xlog_watcher", &date].iter().collect();

    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    };

    path.push(log_file);
    Ok(path)
}

pub(crate) fn string_to_c_str(log: &str) -> *mut c_char {
    let cstr: CString = CString::new(log).expect("CString::new failed");
    let c_char = cstr.into_raw() as *mut c_char;
    c_char
}
