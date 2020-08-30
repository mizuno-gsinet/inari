use std::{
    arch::x86_64::*, cmp::Ordering, convert::TryFrom, error::Error, fmt, mem::transmute, result,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalErrorKind {
    IntvlPartOfNaI,
    PossiblyUndefinedOperation,
    UndefinedOperation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalError<T: fmt::Debug> {
    pub(crate) kind: IntervalErrorKind,
    pub(crate) value: T,
}

impl<T: fmt::Debug> IntervalError<T> {
    pub fn kind(&self) -> IntervalErrorKind {
        self.kind
    }

    pub fn value(self) -> T {
        self.value
    }
}

impl<T: fmt::Debug> fmt::Display for IntervalError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            IntervalErrorKind::IntvlPartOfNaI => write!(f, "interval part of nai"),
            IntervalErrorKind::PossiblyUndefinedOperation => {
                write!(f, "possibly undefined operation")
            }
            IntervalErrorKind::UndefinedOperation => write!(f, "undefined operation"),
        }
    }
}

impl<T: fmt::Debug> Error for IntervalError<T> {}

pub type Result<T> = result::Result<T, IntervalError<T>>;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Interval {
    // The neginf-sup-nan form is used to represent an interval.
    // Zero can be stored as either 0.0 or -0.0.
    // NaN can be stored in any representation.
    //
    // An interval [a, b] is stored as [b; -a]:
    //   +----------------+----------------+
    //   |        b       |       -a       |
    //   +----------------+----------------+
    //   |127           64|63             0|
    //
    // The value of `rep` is Formatted as `__m128d(-a, b)` in Debug formatting.
    pub(crate) rep: __m128d,
}

impl Interval {
    pub(crate) fn inf_raw(self) -> f64 {
        unsafe { -transmute::<_, [f64; 2]>(self)[0] }
    }

    pub(crate) fn sup_raw(self) -> f64 {
        unsafe { transmute::<_, [f64; 2]>(self)[1] }
    }

    pub(crate) fn with_infsup_raw(a: f64, b: f64) -> Self {
        // More optimized code is generated than by `transmute([-a, b])`.
        Self {
            rep: unsafe { _mm_set_pd(b, -a) },
        }
    }

    pub(crate) fn zero() -> Self {
        unsafe { transmute([0.0, 0.0]) }
    }
}

impl PartialEq for Interval {
    fn eq(&self, rhs: &Self) -> bool {
        self.is_empty() && rhs.is_empty()
            || unsafe { _mm_movemask_pd(_mm_cmpeq_pd(self.rep, rhs.rep)) == 3 }
    }
}

impl Eq for Interval {}

impl TryFrom<(f64, f64)> for Interval {
    type Error = IntervalError<Self>;

    fn try_from((a, b): (f64, f64)) -> Result<Self> {
        if a <= b && a != f64::INFINITY && b != f64::NEG_INFINITY {
            Ok(Self::with_infsup_raw(a, b))
        } else {
            Err(Self::Error {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Self::EMPTY,
            })
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Decoration {
    Ill = 0,
    Trv = 4,
    Def = 8,
    Dac = 12,
    Com = 16,
}

impl Ord for Decoration {
    fn cmp(&self, rhs: &Self) -> Ordering {
        (*self as u8).cmp(&(*rhs as u8))
    }
}

impl PartialOrd for Decoration {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DecoratedInterval {
    pub(crate) x: Interval,
    pub(crate) d: Decoration,
}

impl DecoratedInterval {
    pub fn new(x: Interval) -> Self {
        use Decoration::*;

        let d = if x.is_empty() {
            Trv
        } else if !x.is_common_interval() {
            Dac
        } else {
            Com
        };

        Self::new_unchecked(x, d)
    }

    pub(crate) const fn new_unchecked(x: Interval, d: Decoration) -> Self {
        Self { x, d }
    }

    pub fn decoration_part(self) -> Decoration {
        self.d
    }

    pub fn interval_part(self) -> Result<Interval> {
        if self.is_nai() {
            return Err(IntervalError {
                kind: IntervalErrorKind::IntvlPartOfNaI,
                value: Interval::EMPTY,
            });
        }

        Ok(self.x)
    }

    pub fn set_dec(x: Interval, d: Decoration) -> Self {
        use Decoration::*;

        if d == Ill {
            return Self::NAI;
        }
        if x.is_empty() {
            return Self::EMPTY;
        }
        if d == Com && !x.is_common_interval() {
            return Self::new_unchecked(x, Dac);
        }

        Self::new_unchecked(x, d)
    }
}

impl PartialEq for DecoratedInterval {
    fn eq(&self, rhs: &Self) -> bool {
        if self.is_nai() || rhs.is_nai() {
            return false;
        }

        self.x == rhs.x
    }
}

// `DecoratedInterval` is not a model of `Eq` as NaI ≠ NaI.

impl TryFrom<(f64, f64)> for DecoratedInterval {
    type Error = IntervalError<Self>;

    fn try_from(x: (f64, f64)) -> Result<Self> {
        match Interval::try_from(x) {
            Ok(x) => Ok(Self::new(x)),
            _ => Err(Self::Error {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Self::NAI,
            }),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _interval {
    ($a:expr, $b:expr) => {{
        use ::std::{convert::TryFrom, primitive::*};
        fn is_f64(_: f64) {}
        is_f64($a);
        is_f64($b);
        $crate::Interval::try_from(($a, $b))
    }};
}

#[cfg(not(feature = "gmp"))]
#[macro_export]
macro_rules! interval {
    ($a:expr, $b:expr) => {
        $crate::_interval!($a, $b)
    };
}

#[cfg(feature = "gmp")]
#[macro_export]
macro_rules! interval {
    ($text:expr) => {{
        use ::std::primitive::*;
        fn is_str(_: &str) {}
        is_str($text);
        $text.parse::<$crate::Interval>()
    }};

    ($text:expr, exact) => {{
        use ::std::primitive::*;
        fn is_str(_: &str) {}
        is_str($text);
        $crate::Interval::_try_from_str_exact($text)
    }};

    ($a:expr, $b:expr) => {
        $crate::_interval!($a, $b)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _dec_interval {
    ($a:expr, $b:expr) => {{
        use ::std::{convert::TryFrom, primitive::*};
        fn is_f64(_: f64) {}
        is_f64($a);
        is_f64($b);
        $crate::DecoratedInterval::try_from(($a, $b))
    }};
}

#[cfg(not(feature = "gmp"))]
#[macro_export]
macro_rules! dec_interval {
    ($a:expr, $b:expr) => {
        $crate::_dec_interval!($a, $b)
    };
}

#[cfg(feature = "gmp")]
#[macro_export]
macro_rules! dec_interval {
    ($text:expr) => {{
        use ::std::primitive::*;
        fn is_str(_: &str) {}
        is_str($text);
        $text.parse::<$crate::DecoratedInterval>()
    }};

    ($a:expr, $b:expr) => {
        $crate::_dec_interval!($a, $b)
    };
}

#[macro_export]
macro_rules! const_interval {
    ($a:expr, $b:expr) => {{
        use ::std::{mem::transmute, primitive::*};

        static_assertions::const_assert!(
            $a <= $b && $a != f64::INFINITY && $b != f64::NEG_INFINITY
        );

        #[allow(unused_unsafe)]
        unsafe {
            // Parentheses are used to avoid `clippy::double_neg`.
            transmute([-($a), $b])
        }
    }};
}

#[macro_export]
macro_rules! const_dec_interval {
    ($a:expr, $b:expr) => {{
        use ::std::{mem::transmute, primitive::*};

        #[derive(Clone, Copy)]
        #[repr(C)]
        struct _DecoratedInterval {
            x: $crate::Interval,
            d: $crate::Decoration,
        }

        #[allow(unused_unsafe)]
        unsafe {
            transmute(_DecoratedInterval {
                x: const_interval!($a, $b),
                d: if $a == f64::NEG_INFINITY || $b == f64::INFINITY {
                    $crate::Decoration::Dac
                } else {
                    $crate::Decoration::Com
                },
            })
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoration_order() {
        use Decoration::*;
        assert!(Ill < Trv);
        assert!(Trv < Def);
        assert!(Def < Dac);
        assert!(Dac < Com);
    }

    #[test]
    fn macros() {
        // Check that these macros are usable in const contexts.
        const _I: Interval = const_interval!(1.0, 2.0);
        const _DI: DecoratedInterval = const_dec_interval!(1.0, 2.0);

        assert_eq!(interval!(1.0, 1.0).unwrap(), const_interval!(1.0, 1.0));
        assert_eq!(interval!(1.0, 2.0).unwrap(), const_interval!(1.0, 2.0));
        assert_eq!(
            interval!(f64::NEG_INFINITY, 1.0).unwrap(),
            const_interval!(f64::NEG_INFINITY, 1.0)
        );
        assert_eq!(
            interval!(1.0, f64::INFINITY).unwrap(),
            const_interval!(1.0, f64::INFINITY)
        );

        assert_eq!(
            dec_interval!(1.0, 1.0).unwrap(),
            const_dec_interval!(1.0, 1.0)
        );
        assert_eq!(
            dec_interval!(1.0, 2.0).unwrap(),
            const_dec_interval!(1.0, 2.0)
        );
        assert_eq!(
            dec_interval!(f64::NEG_INFINITY, 1.0).unwrap(),
            const_dec_interval!(f64::NEG_INFINITY, 1.0)
        );
        assert_eq!(
            dec_interval!(1.0, f64::INFINITY).unwrap(),
            const_dec_interval!(1.0, f64::INFINITY)
        );
    }
}
