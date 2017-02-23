

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! trait_zero {
    ($t:ident) => (
        impl Zero for $t {
            #[inline(always)]
            fn zero() -> Self { 0 }
            #[inline(always)]
            fn is_zero(&self) -> bool { *self == 0 }
        }
    );
}

macro_rules! trait_zero_float {
    ($t:ident) => (
        impl Zero for $t {
            #[inline(always)]
            fn zero() -> Self { 0.0 }
            #[inline(always)]
            fn is_zero(&self) -> bool { *self == 0.0 }
        }
    );
}

trait_zero!(usize);
trait_zero!(u8);
trait_zero!(u16);
trait_zero!(u32);
trait_zero!(u64);

trait_zero!(isize);
trait_zero!(i8);
trait_zero!(i16);
trait_zero!(i32);
trait_zero!(i64);

trait_zero_float!(f32);
trait_zero_float!(f64);


impl Zero for bool {
    #[inline(always)]
    fn zero() -> bool { false }
    #[inline(always)]
    fn is_zero(&self) -> bool { *self }
}
