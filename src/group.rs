use ark_ec::CurveGroup;
use subtle::{Choice, ConstantTimeEq};

#[derive(Clone, Copy)]
pub struct PointRepr<const N: usize>([u8; N]);

impl<const N: usize> Default for PointRepr<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> AsRef<[u8]> for PointRepr<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for PointRepr<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ArkGroupWrapper<G>(pub(crate) G);

impl<C: CurveGroup> ArkGroupWrapper<C> {
    pub fn new(point: C) -> Self {
        Self(point)
    }

    pub fn into_inner(self) -> C {
        self.0
    }
}

impl<C: CurveGroup> ConstantTimeEq for ArkGroupWrapper<C> {
    fn ct_eq(&self, other: &Self) -> Choice {
        if self.0.eq(&other.0) {
            Choice::from(1)
        } else {
            Choice::from(0)
        }
    }
}

#[macro_export]
macro_rules! impl_group_for_curve {
    ($curve:ty, $constants:ident) => {
        impl elliptic_curve::Group for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup + 'static,
        {
            type Scalar = $crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>;

            fn random(mut rng: impl rand_core::RngCore) -> Self {
                use ark_ff::UniformRand;
                Self::new(<$curve>::rand(&mut rng))
            }

            fn identity() -> Self {
                use ark_ff::Zero;
                Self::new(<$curve>::zero())
            }

            fn generator() -> Self {
                use ark_ec::PrimeGroup;
                Self::new(<$curve>::generator())
            }

            fn is_identity(&self) -> subtle::Choice {
                use ark_ff::Zero;
                if self.0.is_zero() {
                    subtle::Choice::from(1)
                } else {
                    subtle::Choice::from(0)
                }
            }

            fn double(&self) -> Self {
                use ark_ec::AdditiveGroup;
                Self::new(self.0.double())
            }
        }

        // Implement scalar multiplication
        impl core::ops::Mul<crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>>
            for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup + 'static,
        {
            type Output = Self;
            fn mul(
                self,
                scalar: crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) -> Self {
                Self::new(self.0.mul(scalar.into_inner()))
            }
        }

        impl core::ops::Mul<&crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>>
            for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup + 'static,
        {
            type Output = Self;
            fn mul(
                self,
                scalar: &crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) -> Self {
                Self::new(self.0.mul(scalar.into_inner()))
            }
        }

        impl
            core::ops::MulAssign<
                crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            > for $crate::ArkGroupWrapper<$curve>
        {
            fn mul_assign(
                &mut self,
                scalar: crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) {
                self.0.mul_assign(scalar.into_inner())
            }
        }

        impl
            core::ops::MulAssign<
                &crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            > for $crate::ArkGroupWrapper<$curve>
        {
            fn mul_assign(
                &mut self,
                scalar: &crate::ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) {
                self.0.mul_assign(scalar.into_inner())
            }
        }

        // Value-based arithmetic implementations
        impl core::ops::Add for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self::new(self.0 + other.0)
            }
        }

        impl core::ops::Sub for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self::new(self.0 - other.0)
            }
        }

        impl core::ops::Neg for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn neg(self) -> Self {
                Self::new(-self.0)
            }
        }

        // Reference-based arithmetic implementations
        impl core::ops::Add<&Self> for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn add(self, other: &Self) -> Self {
                Self::new(self.0 + other.0)
            }
        }

        impl core::ops::Sub<&Self> for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn sub(self, other: &Self) -> Self {
                Self::new(self.0 - other.0)
            }
        }

        impl core::ops::AddAssign for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn add_assign(&mut self, other: Self) {
                self.0.add_assign(other.0);
            }
        }

        impl core::ops::SubAssign for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn sub_assign(&mut self, other: Self) {
                self.0.sub_assign(other.0);
            }
        }

        impl core::ops::AddAssign<&Self> for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn add_assign(&mut self, other: &Self) {
                self.0.add_assign(other.0);
            }
        }

        impl core::ops::SubAssign<&Self> for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn sub_assign(&mut self, other: &Self) {
                self.0.sub_assign(other.0);
            }
        }

        // Implement Sum trait for iterator operations
        impl core::iter::Sum for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                use elliptic_curve::Group;
                iter.fold(Self::identity(), |acc, x| acc + x)
            }
        }

        impl<'a> core::iter::Sum<&'a Self> for $crate::ArkGroupWrapper<$curve> {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                use elliptic_curve::Group;
                iter.fold(Self::identity(), |acc, x| acc + x)
            }
        }

        impl elliptic_curve::group::GroupEncoding for $crate::ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Repr = $crate::group::PointRepr<{ $constants::COMPRESSED_POINT_SIZE }>;

            fn from_bytes(bytes: &Self::Repr) -> subtle::CtOption<Self> {
                use ark_ff::Zero;
                use ark_serialize::CanonicalDeserialize;
                // Note: This is not constant-time, but arkworks doesn't provide constant-time deserialization
                match <$curve>::deserialize_compressed(bytes.as_ref()) {
                    Ok(point) => subtle::CtOption::new(Self::new(point), 1u8.into()),
                    Err(_) => subtle::CtOption::new(Self::new(<$curve>::zero()), 0u8.into()),
                }
            }

            fn from_bytes_unchecked(bytes: &Self::Repr) -> subtle::CtOption<Self> {
                // For arkworks curves, unchecked is the same as checked since validation
                // is part of the deserialization process
                Self::from_bytes(bytes)
            }

            fn to_bytes(&self) -> Self::Repr {
                use ark_serialize::CanonicalSerialize;
                let mut buf = Vec::with_capacity($constants::COMPRESSED_POINT_SIZE);
                // Note: This unwrap is safe because serialization to a Vec cannot fail
                self.0.serialize_compressed(&mut buf).unwrap();
                // Convert Vec<u8> to fixed-size array
                let mut repr = Self::Repr::default();
                repr.as_mut().copy_from_slice(&buf);
                repr
            }
        }
    };
}
