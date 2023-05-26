use hmath::{Ratio, BigInt, UBigInt, ConversionError};

pub trait IntoRatio {
    fn into_ratio(&self) -> Ratio;
}

impl IntoRatio for f32 {
    fn into_ratio(&self) -> Ratio {
        match Ratio::from_ieee754_f32(*self) {
            Ok(n) => n,
            Err(ConversionError::NotANumber) => Ratio::zero(),
            Err(ConversionError::Infinity) => Ratio::from_ieee754_f32(f32::MAX).unwrap(),
            Err(ConversionError::NegInfinity) => Ratio::from_ieee754_f32(f32::MIN).unwrap(),
            _ => unreachable!()
        }
    }
}

impl IntoRatio for f64 {
    fn into_ratio(&self) -> Ratio {
        match Ratio::from_ieee754_f64(*self) {
            Ok(n) => n,
            Err(ConversionError::NotANumber) => Ratio::zero(),
            Err(ConversionError::Infinity) => Ratio::from_ieee754_f64(f64::MAX).unwrap(),
            Err(ConversionError::NegInfinity) => Ratio::from_ieee754_f64(f64::MIN).unwrap(),
            _ => unreachable!()
        }
    }
}

impl IntoRatio for i8 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i32(*self as i32)
    }
}

impl IntoRatio for i16 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i32(*self as i32)
    }
}

impl IntoRatio for i32 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i32(*self)
    }
}

impl IntoRatio for i64 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i64(*self)
    }
}

impl IntoRatio for i128 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i128(*self)
    }
}

impl IntoRatio for isize {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i64(*self as i64)
    }
}

impl IntoRatio for u8 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i32(*self as i32)
    }
}

impl IntoRatio for u16 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i32(*self as i32)
    }
}

impl IntoRatio for u32 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_i64(*self as i64)
    }
}

impl IntoRatio for u64 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_bi(BigInt::from_ubi(UBigInt::from_u64(*self), false))
    }
}

impl IntoRatio for u128 {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_bi(BigInt::from_ubi(UBigInt::from_u128(*self), false))
    }
}

impl IntoRatio for usize {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_bi(BigInt::from_ubi(UBigInt::from_u64(*self as u64), false))
    }
}

impl IntoRatio for Ratio {
    fn into_ratio(&self) -> Ratio {
        self.clone()
    }
}

impl IntoRatio for String {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_string(self).unwrap_or(Ratio::zero())
    }
}

impl IntoRatio for &str {
    fn into_ratio(&self) -> Ratio {
        Ratio::from_string(self).unwrap_or(Ratio::zero())
    }
}