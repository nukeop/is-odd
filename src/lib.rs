pub trait IsOdd {
    fn is_odd(&self) -> bool;
}

impl IsOdd for i8 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for u8 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for i16 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for u16 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for i32 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for u32 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for i64 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}

impl IsOdd for u64 {
    fn is_odd(&self) -> bool {
        self&1 != 0
    }
}
