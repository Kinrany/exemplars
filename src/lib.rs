#![no_std]

/// A trait for providing examples of a type.
pub trait Exemplars: Sized {
    /// Iterate over all available examples.
    ///
    /// Must return at least one value.
    fn exemplars() -> impl IntoIterator<Item = Self>;
    /// Returns the primary example value.
    ///
    /// The value is the same as the first value returned from `exemplars()`.
    fn exemplar() -> Self {
        Self::exemplars()
            .into_iter()
            .next()
            .expect("invalid impl Exemplars: must provide at least one example value")
    }
}

impl Exemplars for () {
    fn exemplars() -> impl IntoIterator<Item = Self> {
        [()]
    }
}

/// Implement `Exemplars` for number types with 1 as the primary example.
///
/// Generates impls like this one:
///
/// ```ignore
/// impl Exemplars for usize {
///     fn exemplars() -> impl IntoIterator<Item = Self> {
///         1..=Self::MAX
///     }
/// }
/// ```
macro_rules! impl_for_number_types {
    ($($t:ident),+) => {$(
        impl Exemplars for $t {
            fn exemplars() -> impl IntoIterator<Item = Self> {
                1..=Self::MAX
            }
        }
    )+}
}
impl_for_number_types!(usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl<T: Exemplars> Exemplars for Option<T> {
    fn exemplars() -> impl IntoIterator<Item = Self> {
        T::exemplars().into_iter().map(Some).chain([None])
    }
}

#[cfg(feature = "alloc")]
mod alloc {
    extern crate alloc;

    use crate::Exemplars;

    impl Exemplars for alloc::string::String {
        fn exemplars() -> impl IntoIterator<Item = Self> {
            ["example".into()]
        }
    }

    impl<T: Exemplars> Exemplars for alloc::vec::Vec<T> {
        fn exemplars() -> impl IntoIterator<Item = Self> {
            T::exemplars().into_iter().map(|x| alloc::vec![x])
        }
    }
}

#[cfg(feature = "std")]
mod std {
    extern crate std;
}

#[cfg(feature = "bigdecimal_03")]
impl Exemplars for ::bigdecimal_03::BigDecimal {
    fn exemplars() -> impl IntoIterator<Item = Self> {
        [<Self as ::bigdecimal_03::One>::one()]
    }
}

#[cfg(feature = "bigdecimal_04")]
impl Exemplars for ::bigdecimal_04::BigDecimal {
    fn exemplars() -> impl IntoIterator<Item = Self> {
        [<Self as ::bigdecimal_04::One>::one()]
    }
}

#[cfg(feature = "rust_decimal")]
impl Exemplars for ::rust_decimal::Decimal {
    fn exemplars() -> impl IntoIterator<Item = Self> {
        [Self::ONE]
    }
}

#[cfg(feature = "uuid")]
impl Exemplars for ::uuid::Uuid {
    fn exemplars() -> impl IntoIterator<Item = Self> {
        [Self::max()]
    }
}
