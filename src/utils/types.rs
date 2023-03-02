use std::marker::PhantomData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
struct u24([u8; 3]);

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
struct RGB {
    u: [u8; 3],
    _unit: PhantomData<()>,
}

impl std::ops::Add for u24 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_u32(self.to_u32() + rhs.to_u32())
    }
}

impl u24 {
    fn to_u32(self) -> u32 {
        let u24([a, b, c]) = self;
        u32::from_le_bytes([a, b, c, 0])
    }
    fn from_u32(n: u32) -> Self {
        let [a, b, c, d] = n.to_le_bytes();
        debug_assert!(d == 0);
        u24([a, b, c])
    }
}

mod test {
    #[test]
    fn test() {
        let n = 12345;
        let a = u24::from_u32(n);
        println!("{} {:?}", n, a);
    }
}
