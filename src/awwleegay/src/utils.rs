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
