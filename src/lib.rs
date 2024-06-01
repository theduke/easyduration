//! Provides the [`EasyDuration`] extension trait to simplify the construction
//! of [`std::time::Duration`] from numeric values with extension methods.
//!
//! # Example
//!
//! ```rust
//! use easyduration::EasyDuration;
//!
//! fn easy() {
//!     let _v = 1.seconds();
//!     let _v = 12.minutes();
//!     let _v = 5.hours();
//!     let _v = 7.days();
//!     let _v = 2.years();
//! }
//! ````

use std::time::Duration;

/// Extension trait that simplifies construction [`std::time::Duration`] from
/// numeric values.
///
/// The trait is implemented for the regular integer types:
/// * [`u8`], [`u16`], [`u32`], [`u64`]
/// * [`i8`], [`i16`], [`i32`], [`i64`]
///
/// Note that for signed integers, the absolute value will be used.
///
/// # Example
///
/// ```rust
/// use easyduration::EasyDuration;
///
/// fn easy() {
///     let _v = 1.seconds();
///     let _v = 12.minutes();
///     let _v = 5.hours();
///     let _v = 7.days();
///     let _v = 2.years();
/// }
pub trait EasyDuration: Sized {
    /// Creates a [`std::time::Duration`] by converting `self` to seconds.
    fn seconds(self) -> Duration;

    /// Creates a [`std::time::Duration`] by converting `self` to minutes.
    fn minutes(self) -> Duration {
        self.seconds() * 60
    }

    /// Creates a [`std::time::Duration`] by converting `self` to hours.
    fn hours(self) -> Duration {
        self.seconds() * 60 * 60
    }

    /// Creates a [`std::time::Duration`] by converting `self` to days.
    ///
    /// A day is defined as 24 hours.
    fn days(self) -> Duration {
        self.seconds() * 60 * 60 * 24
    }

    /// Creates a [`std::time::Duration`] by converting `self` to years.
    ///
    /// A year is defined as 365 days.
    fn years(self) -> Duration {
        self.days() * 365
    }
}

impl EasyDuration for u8 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        Duration::from_secs(self.into())
    }
}

impl EasyDuration for u16 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        Duration::from_secs(self.into())
    }
}

impl EasyDuration for u32 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        Duration::from_secs(self.into())
    }
}

impl EasyDuration for u64 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        Duration::from_secs(self)
    }
}

impl EasyDuration for i8 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        Duration::from_secs(self.unsigned_abs() as u64)
    }
}

impl EasyDuration for i16 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        Duration::from_secs(self.unsigned_abs() as u64)
    }
}

impl EasyDuration for i32 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        let secs = self.abs().try_into().unwrap();
        Duration::from_secs(secs)
    }
}

impl EasyDuration for i64 {
    /// See [`EasyDuration::seconds`].
    fn seconds(self) -> Duration {
        let secs = self.abs().try_into().unwrap();
        Duration::from_secs(secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy_duration() {
        // Seconds.
        assert_eq!(1i8.seconds(), Duration::from_secs(1));
        assert_eq!(1i16.seconds(), Duration::from_secs(1));
        assert_eq!(1i32.seconds(), Duration::from_secs(1));
        assert_eq!(1i64.seconds(), Duration::from_secs(1));
        assert_eq!(1u8.seconds(), Duration::from_secs(1));
        assert_eq!(1u16.seconds(), Duration::from_secs(1));
        assert_eq!(1u32.seconds(), Duration::from_secs(1));
        assert_eq!(1u64.seconds(), Duration::from_secs(1));

        // Minutes
        assert_eq!(1i8.minutes(), Duration::from_secs(60));
        assert_eq!(1i16.minutes(), Duration::from_secs(60));
        assert_eq!(1i32.minutes(), Duration::from_secs(60));
        assert_eq!(1i64.minutes(), Duration::from_secs(60));
        assert_eq!(1u8.minutes(), Duration::from_secs(60));
        assert_eq!(1u16.minutes(), Duration::from_secs(60));
        assert_eq!(1u32.minutes(), Duration::from_secs(60));
        assert_eq!(1u64.minutes(), Duration::from_secs(60));

        // Hours.
        assert_eq!(1i8.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1i16.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1i32.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1i64.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1u8.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1u16.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1u32.hours(), Duration::from_secs(60 * 60));
        assert_eq!(1u64.hours(), Duration::from_secs(60 * 60));

        // Days.
        assert_eq!(1i8.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1i16.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1i32.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1i64.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1u8.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1u16.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1u32.days(), Duration::from_secs(60 * 60 * 24));
        assert_eq!(1u64.days(), Duration::from_secs(60 * 60 * 24));

        // Years.
        assert_eq!(1i8.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1i16.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1i32.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1i64.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1u8.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1u16.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1u32.years(), Duration::from_secs(60 * 60 * 24 * 365));
        assert_eq!(1u64.years(), Duration::from_secs(60 * 60 * 24 * 365));
    }
}
