use ark_ff::{BigInteger, PrimeField as ArkPrimeField};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

#[derive(Clone, Debug, Default, Copy)]
pub struct ScalarRepr<F: ArkPrimeField> {
    inner: F::BigInt,
}

impl<F: ArkPrimeField> ScalarRepr<F> {
    pub const fn new(bigint: F::BigInt) -> Self {
        Self { inner: bigint }
    }
}

impl<F: ArkPrimeField> AsRef<[u8]> for ScalarRepr<F> {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.as_ref().as_ptr() as *const u8,
                std::mem::size_of::<F::BigInt>(),
            )
        }
    }
}

impl<F: ArkPrimeField> AsMut<[u8]> for ScalarRepr<F> {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.inner.as_mut().as_mut_ptr() as *mut u8,
                std::mem::size_of::<F::BigInt>(),
            )
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Copy)]
pub struct ArkScalarWrapper<F: ArkPrimeField> {
    pub inner: F,
}

impl<F: ArkPrimeField> ArkScalarWrapper<F> {
    pub const fn new(scalar: F) -> Self {
        Self { inner: scalar }
    }

    pub const fn into_inner(self) -> F {
        self.inner
    }
}

impl<F: ArkPrimeField> ConditionallySelectable for ArkScalarWrapper<F> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        if choice.into() { *b } else { *a }
    }
}

impl<F: ArkPrimeField> ConstantTimeEq for ArkScalarWrapper<F> {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.inner
            .into_bigint()
            .to_bytes_be()
            .ct_eq(&other.inner.into_bigint().to_bytes_be())
    }
}

impl<F: ArkPrimeField> From<u64> for ArkScalarWrapper<F> {
    fn from(value: u64) -> Self {
        Self::new(F::from(value))
    }
}

#[macro_export]
macro_rules! impl_wrapped_field {
    ($field:ty, $constants:ident) => {
        impl ff::Field for $crate::ArkScalarWrapper<$field> {
            const ZERO: Self = unsafe {
                // SAFETY: This is safe because we're creating a known-good zero value
                let bytes: [u8; std::mem::size_of::<Self>()] = $crate::$constants::ZERO;
                std::mem::transmute(bytes)
            };
            const ONE: Self = unsafe {
                // SAFETY: This is safe because we're creating a known-good one value
                let bytes: [u8; std::mem::size_of::<Self>()] = crate::$constants::ONE;
                std::mem::transmute(bytes)
            };

            fn random(mut rng: impl rand_core::RngCore) -> Self {
                use ark_ff::UniformRand;
                Self::new(<$field>::rand(&mut rng))
            }

            fn is_zero(&self) -> subtle::Choice {
                use ark_ff::Zero;
                subtle::Choice::from(self.inner.is_zero() as u8)
            }

            fn square(&self) -> Self {
                use ark_ff::Field;
                Self::new(self.inner.square())
            }

            fn double(&self) -> Self {
                use ark_ff::AdditiveGroup;
                Self::new(self.inner.double())
            }

            fn invert(&self) -> subtle::CtOption<Self> {
                use ark_ff::Field;
                self.inner.inverse().map_or(
                    subtle::CtOption::new(Self::ZERO, subtle::Choice::from(0)),
                    |inv| subtle::CtOption::new(Self::new(inv), subtle::Choice::from(1)),
                )
            }

            fn sqrt_ratio(num: &Self, div: &Self) -> (subtle::Choice, Self) {
                use ark_ff::Field;
                let num_is_square = num.inner.sqrt().is_some();
                let div_is_square = div.inner.sqrt().is_some();
                let is_ratio_square = num_is_square && div_is_square;

                let value = if is_ratio_square {
                    let num_sqrt = num.inner.sqrt().unwrap();
                    let div_sqrt = div.inner.sqrt().unwrap();
                    let ratio = num_sqrt * div_sqrt.inverse().unwrap();
                    Self::new(ratio)
                } else {
                    Self::ZERO
                };

                (subtle::Choice::from(is_ratio_square as u8), value)
            }
        }

        // Implement arithmetic operations
        impl core::ops::Add for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self::new(self.inner + other.inner)
            }
        }

        impl<'a> core::ops::Add<&'a Self> for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            type Output = Self;
            fn add(self, other: &Self) -> Self {
                Self::new(self.inner + other.inner)
            }
        }

        impl core::ops::AddAssign for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            fn add_assign(&mut self, other: Self) {
                self.inner += other.inner;
            }
        }

        impl<'a> core::ops::AddAssign<&'a Self> for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            fn add_assign(&mut self, other: &Self) {
                self.inner += other.inner;
            }
        }

        impl core::ops::Sub for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self::new(self.inner - other.inner)
            }
        }

        impl<'a> core::ops::Sub<&'a Self> for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            type Output = Self;
            fn sub(self, other: &Self) -> Self {
                Self::new(self.inner - other.inner)
            }
        }

        impl core::ops::SubAssign for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            fn sub_assign(&mut self, other: Self) {
                self.inner -= other.inner;
            }
        }

        impl<'a> core::ops::SubAssign<&'a Self> for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            fn sub_assign(&mut self, other: &Self) {
                self.inner -= other.inner;
            }
        }

        impl core::ops::Mul for crate::ArkScalarWrapper<$field>
        where
            $field: ark_ff::Field,
        {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                Self::new(self.inner * other.inner)
            }
        }

        impl<'a> core::ops::Mul<&'a Self> for crate::ArkScalarWrapper<$field> {
            type Output = Self;
            fn mul(self, other: &Self) -> Self {
                Self::new(self.inner * other.inner)
            }
        }

        impl core::ops::MulAssign for crate::ArkScalarWrapper<$field> {
            fn mul_assign(&mut self, other: Self) {
                self.inner *= other.inner;
            }
        }

        impl<'a> core::ops::MulAssign<&'a Self> for crate::ArkScalarWrapper<$field> {
            fn mul_assign(&mut self, other: &Self) {
                self.inner *= other.inner;
            }
        }

        impl core::ops::Neg for crate::ArkScalarWrapper<$field> {
            type Output = Self;
            fn neg(self) -> Self {
                Self::new(-self.inner)
            }
        }

        impl core::iter::Sum for crate::ArkScalarWrapper<$field> {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                use ff::Field;
                iter.fold(Self::ZERO, |acc, x| acc + x)
            }
        }

        impl<'a> core::iter::Sum<&'a Self> for crate::ArkScalarWrapper<$field> {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                use ff::Field;
                iter.fold(Self::ZERO, |acc, x| acc + x)
            }
        }

        impl core::iter::Product for crate::ArkScalarWrapper<$field> {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                use ff::Field;
                iter.fold(Self::ONE, |acc, x| acc * x)
            }
        }

        impl<'a> core::iter::Product<&'a Self> for crate::ArkScalarWrapper<$field> {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                use ff::Field;
                iter.fold(Self::ONE, |acc, x| acc * x)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_prime_field {
    ($field:ty, $constants:ident) => {
        impl elliptic_curve::PrimeField for $crate::ArkScalarWrapper<$field> {
            type Repr = $crate::scalar::ScalarRepr<$field>;

            const MODULUS: &'static str = crate::$constants::MODULUS;
            const NUM_BITS: u32 = crate::$constants::NUM_BITS;
            const CAPACITY: u32 = crate::$constants::CAPACITY;

            const TWO_INV: Self = unsafe {
                let bytes: [u8; std::mem::size_of::<Self>()] = crate::$constants::TWO_INV;
                std::mem::transmute(bytes)
            };
            const MULTIPLICATIVE_GENERATOR: Self = unsafe {
                let bytes: [u8; std::mem::size_of::<Self>()] = crate::$constants::GENERATOR;
                std::mem::transmute(bytes)
            };
            const S: u32 = crate::$constants::TWO_ADICITY;
            const ROOT_OF_UNITY: Self = unsafe {
                let bytes: [u8; std::mem::size_of::<Self>()] = crate::$constants::ROOT_OF_UNITY;
                std::mem::transmute(bytes)
            };
            const ROOT_OF_UNITY_INV: Self = unsafe {
                let bytes: [u8; std::mem::size_of::<Self>()] = crate::$constants::ROOT_OF_UNITY_INV;
                std::mem::transmute(bytes)
            };
            const DELTA: Self = unsafe {
                let bytes: [u8; std::mem::size_of::<Self>()] = crate::$constants::DELTA;
                std::mem::transmute(bytes)
            };

            fn from_repr(repr: Self::Repr) -> subtle::CtOption<Self> {
                use ark_ff::PrimeField;
                let field_element = <$field>::from_be_bytes_mod_order(repr.as_ref());
                subtle::CtOption::new(Self::new(field_element), subtle::Choice::from(1))
            }

            fn to_repr(&self) -> Self::Repr {
                use ark_ff::PrimeField;
                crate::scalar::ScalarRepr::new(self.inner.into_bigint())
            }

            fn is_odd(&self) -> subtle::Choice {
                subtle::Choice::from(self.to_repr().as_ref()[0] & 1)
            }
        }
    };
}
