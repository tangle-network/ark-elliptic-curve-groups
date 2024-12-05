use ark_ec::CurveGroup;
use subtle::{Choice, ConstantTimeEq};

#[derive(Clone, Debug, Copy)]
pub struct ArkGroupWrapper<C: CurveGroup> {
    pub inner: C,
}

impl<C: CurveGroup> ArkGroupWrapper<C> {
    pub fn new(point: C) -> Self {
        Self { inner: point }
    }

    pub fn into_inner(self) -> C {
        self.inner
    }
}

impl<C: CurveGroup> ConstantTimeEq for ArkGroupWrapper<C> {
    fn ct_eq(&self, other: &Self) -> Choice {
        if self.inner.eq(&other.inner) {
            Choice::from(1)
        } else {
            Choice::from(0)
        }
    }
}

impl<C: CurveGroup> PartialEq for ArkGroupWrapper<C> {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl<C: CurveGroup> Eq for ArkGroupWrapper<C> {}

#[macro_export]
macro_rules! impl_group_for_curve {
    ($curve:ty) => {
        impl elliptic_curve::Group for ArkGroupWrapper<$curve> {
            type Scalar = ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>;

            fn random(mut rng: impl rand_core::RngCore) -> Self {
                Self::new(<$curve>::rand(&mut rng))
            }

            fn identity() -> Self {
                Self::new(<$curve>::zero())
            }

            fn generator() -> Self {
                Self::new(<$curve>::generator())
            }

            fn is_identity(&self) -> subtle::Choice {
                if self.inner.is_zero() {
                    subtle::Choice::from(1)
                } else {
                    subtle::Choice::from(0)
                }
            }

            fn double(&self) -> Self {
                Self::new(self.inner.double())
            }
        }

        // Implement scalar multiplication
        impl Mul<ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>>
            for ArkGroupWrapper<$curve>
        {
            type Output = Self;
            fn mul(
                self,
                scalar: ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) -> Self {
                Self::new(self.inner.mul(scalar.into_inner()))
            }
        }

        impl Mul<&ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>>
            for ArkGroupWrapper<$curve>
        {
            type Output = Self;
            fn mul(
                self,
                scalar: &ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) -> Self {
                Self::new(self.inner.mul(scalar.into_inner()))
            }
        }

        impl MulAssign<ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>>
            for ArkGroupWrapper<$curve>
        {
            fn mul_assign(
                &mut self,
                scalar: ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) {
                self.inner.mul_assign(scalar.into_inner())
            }
        }

        impl MulAssign<&ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>>
            for ArkGroupWrapper<$curve>
        {
            fn mul_assign(
                &mut self,
                scalar: &ArkScalarWrapper<<$curve as ark_ec::PrimeGroup>::ScalarField>,
            ) {
                self.inner.mul_assign(scalar.into_inner())
            }
        }

        // Value-based arithmetic implementations
        impl Add for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self::new(self.inner + other.inner)
            }
        }

        impl Sub for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self::new(self.inner - other.inner)
            }
        }

        impl Neg for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn neg(self) -> Self {
                Self::new(-self.inner)
            }
        }

        // Reference-based arithmetic implementations
        impl Add<&Self> for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn add(self, other: &Self) -> Self {
                Self::new(self.inner + other.inner)
            }
        }

        impl Sub<&Self> for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            type Output = Self;
            fn sub(self, other: &Self) -> Self {
                Self::new(self.inner - other.inner)
            }
        }

        impl AddAssign for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn add_assign(&mut self, other: Self) {
                self.inner.add_assign(other.inner);
            }
        }

        impl SubAssign for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn sub_assign(&mut self, other: Self) {
                self.inner.sub_assign(other.inner);
            }
        }

        impl AddAssign<&Self> for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn add_assign(&mut self, other: &Self) {
                self.inner.add_assign(other.inner);
            }
        }

        impl SubAssign<&Self> for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn sub_assign(&mut self, other: &Self) {
                self.inner.sub_assign(other.inner);
            }
        }

        // Implement Sum trait for iterator operations
        impl Sum for ArkGroupWrapper<$curve>
        where
            $curve: ark_ec::CurveGroup,
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                use elliptic_curve::Group;
                iter.fold(Self::identity(), |acc, x| acc + x)
            }
        }

        impl<'a> Sum<&'a Self> for ArkGroupWrapper<$curve> {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                use elliptic_curve::Group;
                iter.fold(Self::identity(), |acc, x| acc + x)
            }
        }
    };
}
