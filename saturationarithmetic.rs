// saturationarithmetic.rs

// A library for performing saturation arithmetic in Rust

// Define a trait for performing saturation arithmetic
trait SaturationArithmetic {
    fn saturating_add(&self, rhs: Self) -> Self;
    fn saturating_sub(&self, rhs: Self) -> Self;
    fn saturating_mul(&self, rhs: Self) -> Self;
    fn saturating_div(&self, rhs: Self) -> Self;
}

// Implement the trait for i8
impl SaturationArithmetic for i8 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs > 0 {
            if *self <= i8::MAX - rhs {
                *self + rhs
            } else {
                i8::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i8::MIN - rhs {
                *self + rhs
            } else {
                i8::MIN
            }
        } else {
            *self + rhs
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs < 0 {
            if *self <= i8::MAX + rhs {
                *self - rhs
            } else {
                i8::MAX
            }
        } else if *self < 0 && rhs > 0 {
            if *self >= i8::MIN + rhs {
                *self - rhs
            } else {
                i8::MIN
            }
        } else {
            *self - rhs
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i8::MAX / rhs {
                *self * rhs
            } else {
                i8::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i8::MIN / rhs {
                *self * rhs
            } else {
                i8::MIN
            }
        } else {
            *self * rhs
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i8::MAX / rhs {
                *self / rhs
            } else {
                i8::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i8::MIN / rhs {
                *self / rhs
            } else {
                i8::MIN
            }
        } else {
            *self / rhs
        }
    }
}

// Implement the trait for i16
impl SaturationArithmetic for i16 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs > 0 {
            if *self <= i16::MAX - rhs {
                *self + rhs
            } else {
                i16::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i16::MIN - rhs {
                *self + rhs
            } else {
                i16::MIN
            }
        } else {
            *self + rhs
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs < 0 {
            if *self <= i16::MAX + rhs {
                *self - rhs
            } else {
                i16::MAX
            }
        } else if *self < 0 && rhs > 0 {
            if *self >= i16::MIN + rhs {
                *self - rhs
            } else {
                i16::MIN
            }
        } else {
            *self - rhs
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i16::MAX / rhs {
                *self * rhs
            } else {
                i16::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i16::MIN / rhs {
                *self * rhs
            } else {
                i16::MIN
            }
        } else {
            *self * rhs
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i16::MAX / rhs {
                *self / rhs
            } else {
                i16::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i16::MIN / rhs {
                *self / rhs
            } else {
                i16::MIN
            }
        } else {
            *self / rhs
        }
    }
}

// Implement the trait for i32
impl SaturationArithmetic for i32 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs > 0 {
            if *self <= i32::MAX - rhs {
                *self + rhs
            } else {
                i32::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i32::MIN - rhs {
                *self + rhs
            } else {
                i32::MIN
            }
        } else {
            *self + rhs
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs < 0 {
            if *self <= i32::MAX + rhs {
                *self - rhs
            } else {
                i32::MAX
            }
        } else if *self < 0 && rhs > 0 {
            if *self >= i32::MIN + rhs {
                *self - rhs
            } else {
                i32::MIN
            }
        } else {
            *self - rhs
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i32::MAX / rhs {
                *self * rhs
            } else {
                i32::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i32::MIN / rhs {
                *self * rhs
            } else {
                i32::MIN
            }
        } else {
            *self * rhs
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i32::MAX / rhs {
                *self / rhs
            } else {
                i32::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i32::MIN / rhs {
                *self / rhs
            } else {
                i32::MIN
            }
        } else {
            *self / rhs
        }
    }
}

// Implement the trait for i64
impl SaturationArithmetic for i64 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs > 0 {
            if *self <= i64::MAX - rhs {
                *self + rhs
            } else {
                i64::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i64::MIN - rhs {
                *self + rhs
            } else {
                i64::MIN
            }
        } else {
            *self + rhs
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs < 0 {
            if *self <= i64::MAX + rhs {
                *self - rhs
            } else {
                i64::MAX
            }
        } else if *self < 0 && rhs > 0 {
            if *self >= i64::MIN + rhs {
                *self - rhs
            } else {
                i64::MIN
            }
        } else {
            *self - rhs
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i64::MAX / rhs {
                *self * rhs
            } else {
                i64::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i64::MIN / rhs {
                *self * rhs
            } else {
                i64::MIN
            }
        } else {
            *self * rhs

        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i64::MAX / rhs {
                *self / rhs
            } else {
                i64::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i64::MIN / rhs {
                *self / rhs
            } else {
                i64::MIN
            }
        } else {
            *self / rhs
        }
    }
}

// Implement the trait for i128
impl SaturationArithmetic for i128 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs > 0 {
            if *self <= i128::MAX - rhs {
                *self + rhs
            } else {
                i128::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i128::MIN - rhs {
                *self + rhs
            } else {
                i128::MIN
            }
        } else {
            *self + rhs
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= 0 && rhs < 0 {
            if *self <= i128::MAX + rhs {
                *self - rhs
            } else {
                i128::MAX
            }
        } else if *self < 0 && rhs > 0 {
            if *self >= i128::MIN + rhs {
                *self - rhs
            } else {
                i128::MIN
            }
        } else {
            *self - rhs
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i128::MAX / rhs {
                *self * rhs
            } else {
                i128::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i128::MIN / rhs {
                *self * rhs
            } else {
                i128::MIN
            }
        } else {
            *self * rhs
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self > 0 && rhs > 0 {
            if *self <= i128::MAX / rhs {
                *self / rhs
            } else {
                i128::MAX
            }
        } else if *self < 0 && rhs < 0 {
            if *self >= i128::MIN / rhs {
                *self / rhs
            } else {
                i128::MIN
            }
        } else {
            *self / rhs
        }
    }
}

// Implement the trait for u8
impl SaturationArithmetic for u8 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self <= u8::MAX - rhs {
            *self + rhs
        } else {
            u8::MAX
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= rhs {
            *self - rhs
        } else {
            0
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u8::MAX / rhs {
            *self * rhs
        } else {
            u8::MAX
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u8::MAX / rhs {
            *self / rhs
        } else {
            u8::MAX
        }
    }
}

// Implement the trait for u16
impl SaturationArithmetic for u16 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self <= u16::MAX - rhs {
            *self + rhs
        } else {
            u16::MAX
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= rhs {
            *self - rhs
        } else {
            0
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u16::MAX / rhs {
            *self * rhs
        } else {
            u16::MAX
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u16::MAX / rhs {
            *self / rhs
        } else {
            u16::MAX
        }
    }
}

// Implement the trait for u32
impl SaturationArithmetic for u32 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self <= u32::MAX - rhs {
            *self + rhs
        } else {
            u32::MAX
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= rhs {
            *self - rhs
        } else {
            0
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u32::MAX / rhs {
            *self * rhs
        } else {
            u32::MAX
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u32::MAX / rhs {
            *self / rhs
        } else {
            u32::MAX
        }
    }
}

// Implement the trait for u64
impl SaturationArithmetic for u64 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self <= u64::MAX - rhs {
            *self + rhs
        } else {
            u64::MAX
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= rhs {
            *self - rhs
        } else {
            0
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u64::MAX / rhs {
            *self * rhs
        } else {
            u64::MAX
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u64::MAX / rhs {
            *self / rhs
        } else {
            u64::MAX
        }
    }
}

// Implement the trait for u128
impl SaturationArithmetic for u128 {
    fn saturating_add(&self, rhs: Self) -> Self {
        if *self <= u128::MAX - rhs {
            *self + rhs
        } else {
            u128::MAX
        }
    }

    fn saturating_sub(&self, rhs: Self) -> Self {
        if *self >= rhs {
            *self - rhs
        } else {
            0
        }
    }

    fn saturating_mul(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u128::MAX / rhs {
            *self * rhs
        } else {
            u128::MAX
        }
    }

    fn saturating_div(&self, rhs: Self) -> Self {
        if *self == 0 || rhs == 0 {
            0
        } else if *self <= u128::MAX / rhs {
            *self / rhs
        } else {
            u128::MAX
        }
    }
}


