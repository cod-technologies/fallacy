//! Unicode string slices.

use crate::alloc::AllocError;
use crate::sealed::Sealed;
use crate::string::String;
use crate::vec::Vec;
use core::unicode::conversions;

/// String slice extension.
pub trait StrExt: Sealed {
    /// Returns the uppercase equivalent of this string slice, as a new [`String`].
    fn try_to_uppercase(&self) -> Result<String, AllocError>;
    /// Returns the lowercase equivalent of this string slice, as a new [`String`].
    fn try_to_lowercase(&self) -> Result<String, AllocError>;
    /// Returns a copy of this string where each character is mapped to its
    /// ASCII upper case equivalent.
    fn try_to_ascii_uppercase(&self) -> Result<String, AllocError>;
    /// Returns a copy of this string where each character is mapped to its
    /// ASCII lower case equivalent.
    fn try_to_ascii_lowercase(&self) -> Result<String, AllocError>;
}

impl Sealed for str {}

impl StrExt for str {
    #[inline]
    fn try_to_uppercase(&self) -> Result<String, AllocError> {
        let mut s = String::try_with_capacity(self.len())?;
        for c in self[..].chars() {
            match conversions::to_upper(c) {
                [a, '\0', _] => s.push(a),
                [a, b, '\0'] => {
                    s.push(a);
                    s.push(b);
                }
                [a, b, c] => {
                    s.push(a);
                    s.push(b);
                    s.push(c);
                }
            }
        }
        Ok(s)
    }

    #[inline]
    fn try_to_lowercase(&self) -> Result<String, AllocError> {
        let mut s = String::try_with_capacity(self.len())?;
        for (i, c) in self[..].char_indices() {
            if c == 'Σ' {
                // Σ maps to σ, except at the end of a word where it maps to ς.
                // This is the only conditional (contextual) but language-independent mapping
                // in `SpecialCasing.txt`,
                // so hard-code it rather than have a generic "condition" mechanism.
                // See https://github.com/rust-lang/rust/issues/26035
                map_uppercase_sigma(self, i, &mut s)?;
            } else {
                match conversions::to_lower(c) {
                    [a, '\0', _] => s.push(a),
                    [a, b, '\0'] => {
                        s.push(a);
                        s.push(b);
                    }
                    [a, b, c] => {
                        s.push(a);
                        s.push(b);
                        s.push(c);
                    }
                }
            }
        }
        return Ok(s);

        fn map_uppercase_sigma(from: &str, i: usize, to: &mut String) -> Result<(), AllocError> {
            // See https://www.unicode.org/versions/Unicode7.0.0/ch03.pdf#G33992
            // for the definition of `Final_Sigma`.
            debug_assert!('Σ'.len_utf8() == 2);
            let is_word_final = case_ignoreable_then_cased(from[..i].chars().rev())
                && !case_ignoreable_then_cased(from[i + 2..].chars());
            to.try_push_str(if is_word_final { "ς" } else { "σ" })
        }

        #[allow(clippy::skip_while_next)]
        fn case_ignoreable_then_cased<I: Iterator<Item = char>>(iter: I) -> bool {
            use core::unicode::{Case_Ignorable, Cased};
            match iter.skip_while(|&c| Case_Ignorable(c)).next() {
                Some(c) => Cased(c),
                None => false,
            }
        }
    }

    #[inline]
    fn try_to_ascii_uppercase(&self) -> Result<String, AllocError> {
        let mut bytes = Vec::new();
        bytes.try_copy_from_slice(self.as_bytes())?;
        bytes.make_ascii_uppercase();
        // make_ascii_uppercase() preserves the UTF-8 invariant.
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
    }

    #[inline]
    fn try_to_ascii_lowercase(&self) -> Result<String, AllocError> {
        let mut bytes = Vec::new();
        bytes.try_copy_from_slice(self.as_bytes())?;
        bytes.make_ascii_lowercase();
        // make_ascii_uppercase() preserves the UTF-8 invariant.
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
    }
}
