//! Provides `input!` macro.
//!
//! # Example
//!
//! ```no_run
//! #[macro_use]
//! extern crate input as _;
//!
//! fn main() {
//!     // https://atcoder.jp/contests/abc166/tasks/abc166_b
//!
//!     input! {
//!         n: usize,
//!         ass: [[{ input::usize1 }]],
//!     }
//!
//!     let _: usize = n;
//!     let _: Vec<Vec<usize>> = ass;
//! }
//! ```

use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self, BufRead, Read},
    rc::Rc,
    str::{FromStr, SplitWhitespace},
};

#[macro_export]
macro_rules! input {
    (from $scanner:ident; $($tt:tt)*) => {
        $crate::__input_inner!(@scanner($scanner), @tts($($tt)*))
    };
    ($($tt:tt)*) => {
        let __scanner = $crate::DEFAULT_SCANNER.with(|__scanner| __scanner.clone());
        let mut __scanner_ref = __scanner.borrow_mut();
        if let $crate::Scanner::Uninited = *__scanner_ref {
            *__scanner_ref = $crate::Scanner::stdin_auto().unwrap();
        }
        $crate::__input_inner!(@scanner(__scanner_ref), @tts($($tt)*));
        ::std::mem::drop(__scanner_ref);
        ::std::mem::drop(__scanner);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __input_inner {
    (@scanner($scanner:ident), @tts()) => {};
    (@scanner($scanner:ident), @tts(mut $single_tt_pat:tt : $readable:tt)) => {
        let mut $single_tt_pat = $crate::__read!(from $scanner { $readable });
    };
    (@scanner($scanner:ident), @tts($single_tt_pat:tt : $readable:tt)) => {
        let $single_tt_pat = $crate::__read!(from $scanner { $readable });
    };
    (@scanner($scanner:ident), @tts(mut $single_tt_pat:tt : $readable:tt, $($rest:tt)*)) => {
        $crate::__input_inner!(@scanner($scanner), @tts(mut $single_tt_pat: $readable));
        $crate::__input_inner!(@scanner($scanner), @tts($($rest)*));
    };
    (@scanner($scanner:ident), @tts($single_tt_pat:tt : $readable:tt, $($rest:tt)*)) => {
        $crate::__input_inner!(@scanner($scanner), @tts($single_tt_pat: $readable));
        $crate::__input_inner!(@scanner($scanner), @tts($($rest)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __read {
    (from $scanner:ident { [$tt:tt] }) => {
        $crate::__read!(from $scanner { [$tt; $crate::__read!(from $scanner { usize })] })
    };
    (from $scanner:ident  { [$tt:tt; $n:expr] }) => {
        (0..$n).map(|_| $crate::__read!(from $scanner { $tt })).collect::<Vec<_>>()
    };
    (from $scanner:ident { ($($tt:tt),+) }) => {
        ($($crate::__read!(from $scanner { $tt })),*)
    };
    (from $scanner:ident { { $f:expr } }) => {
        $crate::FnOnceExt::<_>::call_once_from_reader($f, &mut $scanner)
    };
    (from $scanner:ident { $ty:ty }) => {
        <$ty as $crate::Readable>::read(|| $scanner.next_unwrap())
    };
}

#[inline]
pub fn usize1(n: usize) -> usize {
    n - 1
}

#[inline]
pub fn bytes(s: String) -> Vec<u8> {
    s.into()
}

#[doc(hidden)]
pub trait FnOnceExt<A> {
    type Output;
    fn call_once_from_reader(this: Self, scanner: &mut Scanner) -> Self::Output;
}

impl<A, O, F> FnOnceExt<A> for F
where
    A: FromStr,
    A::Err: Debug,
    F: FnOnce(A) -> O,
{
    type Output = O;

    #[inline]
    fn call_once_from_reader(this: Self, scanner: &mut Scanner) -> O {
        this(scanner.next_unwrap().parse().unwrap())
    }
}

#[doc(hidden)]
pub enum Scanner {
    Uninited,
    Once {
        words: SplitWhitespace<'static>,
    },
    Lines {
        rdr: Box<dyn BufRead>,
        words: SplitWhitespace<'static>,
    },
}

impl Scanner {
    pub fn stdin_auto() -> io::Result<Self> {
        if cfg!(debug_assertions) {
            Ok(Self::lines(Box::leak(Box::new(io::stdin())).lock()))
        } else {
            Self::once(io::stdin())
        }
    }

    pub fn once<R: Read>(mut rdr: R) -> io::Result<Self> {
        let mut buf = String::with_capacity(1024);
        rdr.read_to_string(&mut buf)?;
        let words = Box::leak(buf.into_boxed_str()).split_whitespace();
        Ok(Self::Once { words })
    }

    pub fn lines<R: BufRead + 'static>(rdr: R) -> Self {
        Self::Lines {
            rdr: Box::new(rdr),
            words: "".split_whitespace(),
        }
    }

    pub fn next_unwrap(&mut self) -> &'static str {
        match self {
            Self::Uninited => None,
            Self::Once { words } => words.next(),
            Self::Lines { rdr, words } => words.next().or_else(|| {
                let mut line = "".to_owned();
                rdr.read_line(&mut line).unwrap();
                *words = Box::leak(line.into_boxed_str()).split_whitespace();
                words.next()
            }),
        }
        .expect("reached EOF")
    }
}

thread_local! {
    #[doc(hidden)]
    pub static DEFAULT_SCANNER: Rc<RefCell<Scanner>> = Rc::new(RefCell::new(Scanner::Uninited));
}

pub trait Readable {
    type Output;

    fn read<'a, F: FnMut() -> &'a str>(get: F) -> Self::Output;
}

impl<T: FromStr> Readable for T
where
    T::Err: Debug,
{
    type Output = Self;

    fn read<'a, F: FnMut() -> &'a str>(mut get: F) -> Self {
        get().parse().unwrap()
    }
}
