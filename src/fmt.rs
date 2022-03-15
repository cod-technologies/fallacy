//! Utilities for formatting and printing strings.

use crate::string::String;
use std::fmt;
use std::fmt::Write;

/// The `try_format` function takes an `Arguments` struct and returns the resulting
/// formatted string.
#[inline]
pub fn try_format(args: fmt::Arguments<'_>) -> Result<String, fmt::Error> {
    let capacity = args.estimated_capacity();
    let mut output = String::try_with_capacity(capacity).map_err(|_| fmt::Error)?;
    output.write_fmt(args)?;
    Ok(output)
}

/// Creates a `String` using interpolation of runtime expressions.
///
/// The first argument `try_format!` receives is a format string. This must be a string
/// literal. The power of the formatting string is in the `{}`s contained.
///
/// Additional parameters passed to `try_format!` replace the `{}`s within the
/// formatting string in the order given unless named or positional parameters
/// are used; see [`std::fmt`] for more information.
///
/// A common use for `try_format!` is concatenation and interpolation of strings.
#[macro_export]
macro_rules! try_format {
    ($($arg:tt)*) => {{
        let res = $crate::fmt::try_format(format_args!($($arg)*));
        res
    }}
}
