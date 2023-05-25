use criterion::{black_box, criterion_group, criterion_main, Criterion};

use meval::max_array;
use meval::min_array;
use meval::Context;
use meval::ContextProvider;
use meval::Expr;
use meval::FuncEvalError;
use std::f64::consts;

const EXPR: &str = "abs(sin(x + 1) * (x^2 + x + 1))";

fn parsing(c: &mut Criterion) {
    let expr = black_box(EXPR);
    c.bench_function("parsing", |b| {
        b.iter(|| {
            expr.parse::<Expr>().unwrap();
        })
    });
}

fn evaluation_matchcontext(c: &mut Criterion) {
    let expr: Expr = black_box(EXPR.parse().unwrap());
    let func = expr.bind_with_context(MatchBuiltins, "x").unwrap();
    c.bench_function("evaluation_matchcontext", |b| {
        b.iter(|| {
            func(1.);
        })
    });
}

fn evaluation_hashcontext(c: &mut Criterion) {
    let expr: Expr = black_box(EXPR.parse().unwrap());
    let func = expr.bind_with_context(Context::new(), "x").unwrap();
    c.bench_function("evaluation_hashcontext", |b| {
        b.iter(|| {
            func(1.);
        })
    });
}

fn default_context(c: &mut Criterion) {
    let expr: Expr = black_box("1 + 2 * 3".parse().unwrap());
    c.bench_function("default_context", |b| b.iter(|| expr.eval()));
}

macro_rules! one_arg {
    ($args:expr, $func:ident) => {
        if $args.len() == 1 {
            Ok($args[0].$func())
        } else {
            Err(FuncEvalError::NumberArgs(1))
        }
    };
}

macro_rules! two_args {
    ($args:expr, $func:ident) => {
        if $args.len() == 2 {
            Ok($args[0].$func($args[1]))
        } else {
            Err(FuncEvalError::NumberArgs(2))
        }
    };
}

macro_rules! one_or_more_arg {
    ($args:expr, $func:ident) => {
        if $args.len() >= 1 {
            Ok($func($args))
        } else {
            Err(FuncEvalError::TooFewArguments)
        }
    };
}

/// Built-in functions and constants.
///
/// See the library documentation for the list of built-ins.
#[doc(hidden)]
pub struct MatchBuiltins;

impl ContextProvider for MatchBuiltins {
    fn get_var(&self, name: &str) -> Option<f64> {
        match name {
            "pi" => Some(consts::PI),
            "e" => Some(consts::E),
            _ => None,
        }
    }
    fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
        match name {
            "sqrt" => one_arg!(args, sqrt),
            "exp" => one_arg!(args, exp),
            "ln" => one_arg!(args, ln),
            "abs" => one_arg!(args, abs),
            "sin" => one_arg!(args, sin),
            "cos" => one_arg!(args, cos),
            "tan" => one_arg!(args, tan),
            "asin" => one_arg!(args, asin),
            "acos" => one_arg!(args, acos),
            "atan" => one_arg!(args, atan),
            "sinh" => one_arg!(args, sinh),
            "cosh" => one_arg!(args, cosh),
            "tanh" => one_arg!(args, tanh),
            "asinh" => one_arg!(args, asinh),
            "acosh" => one_arg!(args, acosh),
            "atanh" => one_arg!(args, atanh),
            "floor" => one_arg!(args, floor),
            "ceil" => one_arg!(args, ceil),
            "round" => one_arg!(args, round),
            "signum" => one_arg!(args, signum),
            "atan2" => two_args!(args, atan2),
            "max" => one_or_more_arg!(args, max_array),
            "min" => one_or_more_arg!(args, min_array),
            _ => Err(FuncEvalError::UnknownFunction),
        }
    }
}

criterion_group!(
    benches,
    parsing,
    evaluation_matchcontext,
    evaluation_hashcontext,
    default_context
);
criterion_main!(benches);
