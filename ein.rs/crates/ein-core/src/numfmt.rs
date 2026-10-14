//! Float formatting under a `[[fill]align][sign][0][width][.precision]f` spec.
//!
//! Several reported numbers are formatted floats: `--hyp-stats`'s `>5.1f`
//! percentages, `--timing`'s `9.2f` milliseconds (whose *values* a golden
//! cannot pin but whose *widths* it can), and `--stats`' `.1f` elapsed time.
//! The rules:
//!
//! - **digits** — `precision` digits after the point, six when none is given,
//!   rounded from the exact binary value with ties to even. Rust's
//!   `format!("{:.*}")` does exactly that for a finite `f64`, so the digits
//!   come from it.
//! - **NaN and infinity** — `nan` and `inf`, whatever the precision. NaN
//!   never carries a `-`, even with its sign bit set; `-inf` and `-0.0` do.
//!   (Rust spells the first `NaN`, which is why it is not left to `format!`.)
//! - **sign** — what goes in front of a non-negative number: nothing for `-`
//!   (the default), `+` for `+`, a space for ` `.
//! - **width, fill, alignment** — padded to `width` with `fill` (a space by
//!   default): `>` right (the default), `<` left, `^` centred with the odd
//!   character on the right, `=` between the sign and the digits. A `0`
//!   before the width means fill `0` with `=`, unless either was given.
//! - anything else — another presentation type, grouping, `#`, a missing
//!   `f` — is refused rather than guessed.

/// Where the padding goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    /// The default for numbers.
    Right,
    Center,
    /// `=` — padding between the sign and the digits, which is what `0` means.
    AfterSign,
}

/// What to print in front of a non-negative number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// `-`: nothing.
    Minus,
    /// `+`.
    Plus,
    /// ` `.
    Space,
}

/// A parsed `[[fill]align][sign][0][width][.precision]f` spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spec {
    pub fill: char,
    pub align: Align,
    pub sign: Sign,
    pub width: usize,
    pub precision: usize,
}

impl Default for Spec {
    fn default() -> Self {
        // The defaults for an `f` spec: right-aligned, space-filled, six
        // digits.
        Spec {
            fill: ' ',
            align: Align::Right,
            sign: Sign::Minus,
            width: 0,
            precision: 6,
        }
    }
}

impl Spec {
    /// Parse a spec of the shape above.
    ///
    /// `None` for anything outside it (a presentation type other than `f`,
    /// grouping, `#`) — a caller that meets one has found a site this module
    /// does not cover yet, and should say so rather than format it wrong.
    pub fn parse(spec: &str) -> Option<Spec> {
        let chars: Vec<char> = spec.chars().collect();
        let mut i = 0;
        let mut out = Spec::default();
        let mut explicit_fill = false;
        let mut explicit_align = false;

        let align_of = |c: char| match c {
            '<' => Some(Align::Left),
            '>' => Some(Align::Right),
            '^' => Some(Align::Center),
            '=' => Some(Align::AfterSign),
            _ => None,
        };
        // `[[fill]align]` — a fill character is only a fill if an alignment
        // follows it, which is why this looks two characters ahead first.
        if chars.len() >= 2
            && let Some(a) = align_of(chars[1])
        {
            out.fill = chars[0];
            out.align = a;
            explicit_fill = true;
            explicit_align = true;
            i = 2;
        } else if !chars.is_empty()
            && let Some(a) = align_of(chars[0])
        {
            out.align = a;
            explicit_align = true;
            i = 1;
        }
        if let Some(&c) = chars.get(i) {
            match c {
                '+' => {
                    out.sign = Sign::Plus;
                    i += 1;
                }
                '-' => {
                    out.sign = Sign::Minus;
                    i += 1;
                }
                ' ' => {
                    out.sign = Sign::Space;
                    i += 1;
                }
                _ => {}
            }
        }
        if chars.get(i) == Some(&'#') {
            return None; // alt form: not a shape any site uses
        }
        // `0` is fill `'0'` with `=` alignment, unless either was given.
        if chars.get(i) == Some(&'0') {
            if !explicit_fill {
                out.fill = '0';
            }
            if !explicit_align {
                out.align = Align::AfterSign;
            }
            i += 1;
        }
        let start = i;
        while chars.get(i).is_some_and(|c| c.is_ascii_digit()) {
            i += 1;
        }
        if i > start {
            out.width = chars[start..i].iter().collect::<String>().parse().ok()?;
        }
        if matches!(chars.get(i), Some(',') | Some('_')) {
            return None; // grouping: not a shape any site uses
        }
        if chars.get(i) == Some(&'.') {
            i += 1;
            let start = i;
            while chars.get(i).is_some_and(|c| c.is_ascii_digit()) {
                i += 1;
            }
            if i == start {
                return None;
            }
            out.precision = chars[start..i].iter().collect::<String>().parse().ok()?;
        }
        // The `f` is **required**. An empty spec does not mean `.6f`: the
        // reading it would otherwise invite is the shortest round-tripping
        // text, a different algorithm entirely — `1.5` where `f` gives
        // `1.500000`.
        match chars.get(i) {
            Some('f') => i += 1,
            _ => return None,
        }
        (i == chars.len()).then_some(out)
    }
}

/// `value`, formatted under `spec`.
pub fn format_float(value: f64, spec: &Spec) -> String {
    // NaN never carries a sign, not even with its sign bit set; infinity
    // does. Neither is affected by the precision.
    let (negative, body) = if value.is_nan() {
        (false, "nan".to_string())
    } else if value.is_infinite() {
        (value < 0.0, "inf".to_string())
    } else {
        // `abs()` first so `-0.0` still reports its sign through `negative`.
        (
            value.is_sign_negative(),
            format!("{:.*}", spec.precision, value.abs()),
        )
    };
    let sign = if negative {
        "-"
    } else {
        match spec.sign {
            Sign::Minus => "",
            Sign::Plus => "+",
            Sign::Space => " ",
        }
    };
    let len = sign.chars().count() + body.chars().count();
    if len >= spec.width {
        return format!("{sign}{body}");
    }
    let pad = spec.width - len;
    let fill: String = std::iter::repeat_n(spec.fill, pad).collect();
    match spec.align {
        Align::Right => format!("{fill}{sign}{body}"),
        Align::Left => format!("{sign}{body}{fill}"),
        Align::AfterSign => format!("{sign}{fill}{body}"),
        Align::Center => {
            let left: String = std::iter::repeat_n(spec.fill, pad / 2).collect();
            let right: String = std::iter::repeat_n(spec.fill, pad - pad / 2).collect();
            format!("{left}{sign}{body}{right}")
        }
    }
}

/// `value`, formatted under `spec` written out — the shape the call sites
/// use, so the spec reads inline where the number is printed.
///
/// Panics on a spec outside the supported subset; that is a programming error
/// in ein.rs, not a runtime condition.
pub fn format_spec(value: f64, spec: &str) -> String {
    let parsed = Spec::parse(spec).unwrap_or_else(|| panic!("unsupported float format {spec:?}"));
    format_float(value, &parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shapes_the_cli_actually_uses() {
        assert_eq!(format_spec(1.25, ".1f"), "1.2"); // round-half-even
        assert_eq!(format_spec(1.35, ".1f"), "1.4"); // …on the exact binary value
        assert_eq!(format_spec(12.345, "9.2f"), "    12.35");
        assert_eq!(format_spec(100.0 * 1.0 / 3.0, ">5.1f"), " 33.3");
    }

    #[test]
    fn nan_loses_its_sign_and_infinity_keeps_it() {
        assert_eq!(format_spec(f64::NAN, ".1f"), "nan");
        assert_eq!(format_spec(-f64::NAN, ".1f"), "nan");
        assert_eq!(format_spec(f64::NAN, "+.1f"), "+nan");
        assert_eq!(format_spec(f64::INFINITY, "9.2f"), "      inf");
        assert_eq!(format_spec(f64::NEG_INFINITY, "9.2f"), "     -inf");
        assert_eq!(format_spec(f64::NEG_INFINITY, "08.2f"), "-0000inf");
    }

    #[test]
    fn negative_zero_keeps_its_sign() {
        assert_eq!(format_spec(-0.0, ".1f"), "-0.0");
        assert_eq!(format_spec(-0.0, ".0f"), "-0");
        assert_eq!(format_spec(-0.0, "08.2f"), "-0000.00");
    }

    #[test]
    fn zero_pad_puts_the_sign_first() {
        assert_eq!(format_spec(-1.5, "08.2f"), "-0001.50");
        assert_eq!(format_spec(1.5, "08.2f"), "00001.50");
        assert_eq!(format_spec(1.5, "^9.2f"), "  1.50   ");
        assert_eq!(format_spec(1.5, "<9.2f"), "1.50     ");
        assert_eq!(format_spec(1.5, "_>9.2f"), "_____1.50");
    }

    #[test]
    fn specs_outside_the_subset_are_rejected_rather_than_guessed() {
        for spec in [
            "#.2f", ",.2f", "9_.2f", ".2e", ".2g", "9.2s", ".", "", "9.2",
        ] {
            assert!(Spec::parse(spec).is_none(), "{spec}");
        }
        // `f` with no precision is six digits.
        assert_eq!(format_spec(1.5, "f"), "1.500000");
    }
}
