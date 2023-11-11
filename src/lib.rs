pub trait IsOdd {
    fn is_odd(&self) -> bool;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl IsOdd for $t {
                fn is_odd(&self) -> bool {
                    self&1 != 0
                }
            }
        )*
    };
}

prim_impl!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);
