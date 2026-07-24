// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! Math function implementations (min, max, floor, ceil, round, sum).

use crate::error::ExpressionError;
use crate::function_library::EvalContext;
use crate::value::{float_fits_i64, ExprValue, Float64};

type R = Result<ExprValue, ExpressionError>;
type Ctx<'a> = &'a mut dyn EvalContext;

fn min_max_items(a: &[ExprValue], name: &str) -> Result<Vec<ExprValue>, ExpressionError> {
    if a.is_empty() {
        return Err(ExpressionError::new(format!(
            "{name}() requires at least 1 argument"
        )));
    }
    if a.len() == 1 {
        match &a[0] {
            val if val.is_list() => {
                let elements: Vec<ExprValue> =
                    val.list_iter().expect("guard ensures list").collect();
                if elements.is_empty() {
                    return Err(ExpressionError::new(format!(
                        "{name}() requires a non-empty list"
                    )));
                }
                Ok(elements)
            }
            ExprValue::RangeExpr(r) => {
                if r.is_empty() {
                    return Err(ExpressionError::new(format!(
                        "{name}() requires a non-empty list"
                    )));
                }
                if name == "min" {
                    return Ok(vec![ExprValue::Int(
                        r.iter().next().expect("guarded above: range is non-empty"),
                    )]);
                } else {
                    // get(-1) resolves against the exact logical length;
                    // deriving the last index from the saturating len()
                    // returns a mid-range element for ranges past the
                    // saturation point.
                    Ok(vec![ExprValue::Int(
                        r.get(-1).expect("guarded above: range is non-empty"),
                    )])
                }
            }
            _ => Ok(a.to_vec()),
        }
    } else {
        Ok(a.to_vec())
    }
}

pub fn min_fn(ctx: Ctx, a: &[ExprValue]) -> R {
    let items = min_max_items(a, "min")?;
    ctx.count_ops(items.len())?;
    let mut result = items[0].clone();
    for item in &items[1..] {
        if result.compare(item)?.is_gt() {
            result = item.clone();
        }
    }
    if items.iter().any(|i| matches!(i, ExprValue::Float(_))) {
        if let ExprValue::Int(i) = &result {
            return Ok(ExprValue::Float(Float64::new(*i as f64)?));
        }
    }
    Ok(result)
}

pub fn max_fn(ctx: Ctx, a: &[ExprValue]) -> R {
    let items = min_max_items(a, "max")?;
    ctx.count_ops(items.len())?;
    let mut result = items[0].clone();
    for item in &items[1..] {
        if result.compare(item)?.is_lt() {
            result = item.clone();
        }
    }
    if items.iter().any(|i| matches!(i, ExprValue::Float(_))) {
        if let ExprValue::Int(i) = &result {
            return Ok(ExprValue::Float(Float64::new(*i as f64)?));
        }
    }
    Ok(result)
}

fn round_half_even(x: f64) -> f64 {
    let rounded = x.round();
    if (x - rounded).abs() == 0.5 {
        if rounded as i64 % 2 != 0 {
            rounded - x.signum()
        } else {
            rounded
        }
    } else {
        rounded
    }
}

pub fn floor_float(_: Ctx, a: &[ExprValue]) -> R {
    match &a[0] {
        ExprValue::Float(f) => {
            let v = f.floor();
            if !float_fits_i64(v) {
                return Err(ExpressionError::integer_overflow());
            }
            Ok(ExprValue::Int(v as i64))
        }
        _ => Err(ExpressionError::type_error("type error")),
    }
}

pub fn floor_int(_: Ctx, a: &[ExprValue]) -> R {
    match &a[0] {
        ExprValue::Int(i) => Ok(ExprValue::Int(*i)),
        _ => Err(ExpressionError::type_error("type error")),
    }
}

pub fn ceil_float(_: Ctx, a: &[ExprValue]) -> R {
    match &a[0] {
        ExprValue::Float(f) => {
            let v = f.ceil();
            if !float_fits_i64(v) {
                return Err(ExpressionError::integer_overflow());
            }
            Ok(ExprValue::Int(v as i64))
        }
        _ => Err(ExpressionError::type_error("type error")),
    }
}

pub fn ceil_int(_: Ctx, a: &[ExprValue]) -> R {
    match &a[0] {
        ExprValue::Int(i) => Ok(ExprValue::Int(*i)),
        _ => Err(ExpressionError::type_error("type error")),
    }
}

/// Largest positive `ndigits` accepted by `round(x, ndigits)`. Python raises
/// "precision too big" once `ndigits` exceeds the platform C `int` range
/// (`i32::MAX`); we match that boundary exactly so the differential oracle
/// agrees on the extreme inputs the generator produces (e.g. `i64::MAX`).
const MAX_ROUND_NDIGITS: i64 = i32::MAX as i64;

/// Threshold above which an `f64` has no fractional bits — rounding it to any
/// number of decimal places is a no-op.
const NO_FRACTION_ABOVE: f64 = 4_503_599_627_370_496.0; // 2^52

/// Rust's format-precision panics past `u16::MAX`, and an f64's exact decimal
/// expansion has at most 1074 fractional digits, so we format at most that many
/// and zero-fill the rest.
const MAX_F64_FRACTION_DIGITS: usize = 1074;

/// Round `value` to the nearest multiple of `10^magnitude` (i.e. `round(value,
/// -magnitude)`), robust to astronomically large `magnitude`.
///
/// `magnitude` can be as large as `i64::MIN.unsigned_abs()` when a caller passes
/// `round(x, i64::MIN)`. Computing `10f64.powi(magnitude)` there overflows to
/// infinity, and the naive `round(value / factor) * factor` then yields `NaN`.
/// Python returns `0` in that regime (any finite value is nearer to `0` than to
/// the first nonzero multiple of an enormous power of ten), so special-case a
/// zero scaled result to `0.0` instead of `0 * inf`.
fn round_to_neg_power(value: f64, magnitude: u64) -> f64 {
    let factor = if magnitude > 308 {
        f64::INFINITY
    } else {
        10f64.powi(magnitude as i32)
    };
    let scaled = round_half_even(value / factor);
    if scaled == 0.0 {
        0.0
    } else {
        scaled * factor
    }
}

/// Convert a whole-valued `f64` to an `ExprValue::Int`, erroring on out-of-range
/// input. Uses `float_fits_i64` for the exact 2^63 boundary check.
fn float_to_int_checked(v: f64) -> R {
    if !float_fits_i64(v) {
        return Err(ExpressionError::integer_overflow());
    }
    Ok(ExprValue::Int(v as i64))
}

pub fn round_fn(ctx: Ctx, a: &[ExprValue]) -> R {
    // Extract the optional `ndigits`. `None` (single-arg `round`) is distinct
    // from `Some(0)` at the type level: both yield an int result here, but
    // keeping the distinction mirrors the Python overloads.
    let ndigits = match a.get(1) {
        Some(ExprValue::Int(n)) => Some(*n),
        Some(_) => return Err(ExpressionError::new("round() ndigits must be int")),
        None => None,
    };
    match &a[0] {
        ExprValue::Float(f) => match ndigits {
            // No ndigits, or ndigits <= 0: Python returns an *int*.
            None => float_to_int_checked(round_half_even(f.value())),
            Some(n) if n <= 0 => {
                // A float with |v| >= 2^52 is already integral, so rounding it
                // to a negative power smaller than its magnitude is a no-op.
                // Returning it directly also dodges the overshoot in
                // `round_to_neg_power`'s `v/factor*factor` round-trip, which
                // for values near 2^63 lands just past the i64 range and would
                // spuriously overflow. (Rare 1-ulp double-rounding
                // disagreements with Python remain at this magnitude — see
                // the allowlist.) When 10^|n| exceeds the magnitude, the value
                // is small enough that `round_to_neg_power` handles it and
                // correctly yields 0.
                let v = f.value();
                if v.abs() >= NO_FRACTION_ABOVE {
                    float_to_int_checked(v)
                } else {
                    float_to_int_checked(round_to_neg_power(v, n.unsigned_abs()))
                }
            }
            // ndigits > 0: Python returns a *float*, formatted to that many
            // decimal places.
            Some(n) => {
                if n > MAX_ROUND_NDIGITS {
                    return Err(ExpressionError::new("round() precision too big"));
                }
                // Charge the string budget up front so a huge precision (e.g.
                // `round(1.5, 10**18)`) trips the operation limit rather than
                // attempting a matching allocation.
                let requested = n as usize;
                ctx.count_string_ops(requested)?;
                let n32 = n as i32;
                // Any f64 with |v| >= 2^52 already has no fractional bits, so
                // rounding it to n > 0 decimal places is a no-op — and the
                // naive `v * 10^n / 10^n` round-trip would only inject error
                // (e.g. -9.2e19 came back as ...16384 instead of ...00000).
                // Likewise n >= 17 exceeds f64's decimal precision, and
                // `v * 10^n` may overflow to infinity for large v.
                let v = f.value();
                let rounded = if n32 >= 17 || v.abs() >= NO_FRACTION_ABOVE {
                    v
                } else {
                    let scaled = v * 10f64.powi(n32);
                    if scaled.is_finite() {
                        round_half_even(scaled) / 10f64.powi(n32)
                    } else {
                        v
                    }
                };
                // Rust's format-precision panics past u16::MAX; an f64 has at
                // most 1074 exact fractional digits, so format up to that and
                // zero-pad the rest (digits past 1074 are always zero, so the
                // output is unchanged). Memory-check the padded length.
                let prec = requested.min(MAX_F64_FRACTION_DIGITS);
                let mut text = format!("{:.prec$}", rounded, prec = prec);
                let pad = requested - prec;
                if pad > 0 {
                    ctx.check_memory(text.len() + pad)?;
                    text.reserve_exact(pad);
                    for _ in 0..pad {
                        text.push('0');
                    }
                }
                Ok(ExprValue::Float(Float64::with_str(rounded, text)?))
            }
        },
        ExprValue::Int(i) => match ndigits {
            // Non-negative ndigits on an int is a no-op (Python returns the int
            // unchanged); a single-arg `round(int)` likewise returns it as-is.
            None => Ok(ExprValue::Int(*i)),
            Some(n) if n >= 0 => Ok(ExprValue::Int(*i)),
            // Negative ndigits rounds an int to a multiple of 10^k. Done in
            // *integer* arithmetic (not via f64) so large magnitudes keep full
            // precision — `round(4611686018427387904, -5)` must yield
            // `...400000`, not the `...400192` an f64 round-trip produces.
            Some(n) => round_int_neg(*i, n.unsigned_abs()),
        },
        _ => Err(ExpressionError::new("round() requires numeric argument")),
    }
}

/// Round integer `i` to the nearest multiple of `10^k`, ties to even (matching
/// Python's `round(int, -k)`). Exact — no float round-trip. Returns an overflow
/// error if the result leaves the `i64` range.
fn round_int_neg(i: i64, k: u64) -> R {
    // 10^k overflows i64 past k == 18; at that scale any i64 rounds to 0
    // (its magnitude is below half of 10^19).
    if k >= 19 {
        return Ok(ExprValue::Int(0));
    }
    let pow = 10i64.pow(k as u32);
    let rem = i.rem_euclid(pow); // in [0, pow); floored remainder
    // Largest multiple of `pow` that is <= i (works for negatives). `i - rem`
    // can underflow for i near i64::MIN, so use checked subtraction; on
    // underflow the floor multiple is unrepresentable but the rounded-up value
    // may still be, so fall through to the overflow-checked add below.
    let down = i.checked_sub(rem);
    let half = pow / 2;
    // Decide whether to round up to `down + pow`, ties to even.
    let round_up = match rem.cmp(&half) {
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Less => false,
        // Exact tie: round to the even multiple. `(i - rem) / pow` is the
        // multiple index; round up iff it is odd. Derive it as `i / pow`
        // rounded toward negative infinity, avoiding the unrepresentable
        // `down` near i64::MIN.
        std::cmp::Ordering::Equal => i.div_euclid(pow) % 2 != 0,
    };
    let result = if round_up {
        // down + pow == i - rem + pow; compute without materializing `down`.
        i.checked_sub(rem)
            .and_then(|d| d.checked_add(pow))
            .or_else(|| {
                // `down` underflowed but `down + pow` may be fine: it equals
                // `i + (pow - rem)`, and `pow - rem` is in (0, pow].
                i.checked_add(pow - rem)
            })
            .ok_or_else(ExpressionError::integer_overflow)?
    } else {
        down.ok_or_else(ExpressionError::integer_overflow)?
    };
    Ok(ExprValue::Int(result))
}


pub fn sum_list(ctx: Ctx, a: &[ExprValue]) -> R {
    if let Some(iter) = a[0].list_iter() {
        let mut int_sum: i64 = 0;
        let mut is_float = false;
        let mut float_sum: f64 = 0.0;
        for e in iter {
            ctx.count_op()?;
            match e {
                ExprValue::Int(i) => {
                    int_sum = int_sum
                        .checked_add(i)
                        .ok_or_else(ExpressionError::integer_overflow)?;
                    float_sum += i as f64;
                }
                ExprValue::Float(f) => {
                    is_float = true;
                    float_sum += f.value();
                }
                _ => return Err(ExpressionError::new("sum() elements must be numeric")),
            }
        }
        if is_float {
            Ok(ExprValue::Float(Float64::new(float_sum)?))
        } else {
            Ok(ExprValue::Int(int_sum))
        }
    } else if let ExprValue::RangeExpr(r) = &a[0] {
        for _ in r.iter() {
            ctx.count_op()?;
        }
        Ok(ExprValue::Int(r.iter().sum()))
    } else {
        Err(ExpressionError::new("sum() requires list or range_expr"))
    }
}
