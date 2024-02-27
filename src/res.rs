#[derive(Debug, PartialEq)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[cfg(feature = "no_std")]
pub trait ResultFns<T, E> {
    fn map<U, F: Fn(T) -> U>(self, f: F) -> Result<U, E>;
    fn and_then<U, F: Fn(T) -> Result<U, E>>(self, f: F) -> Result<U, E>;
    fn or_else<F: Fn(E) -> Result<T, E>>(self, f: F) -> Result<T, E>;
    fn unwrap(self) -> T;
    fn unwrap_or(self, default: T) -> T;
    fn unwrap_or_else<F: Fn(E) -> T>(self, f: F) -> T;
    fn expect<U: core::fmt::Display>(self, msg: U) -> T;
    fn is_ok(&self) -> bool;
    fn is_err(&self) -> bool;
}

#[cfg(feature = "check")]
pub trait ResultCheck<T, E> 
where T: PartialEq,
      E: PartialEq {
    fn check(&self, other: T) -> bool;
    fn check_err(&self, other: E) -> bool;
}

#[cfg(feature = "no_std")]
impl<T, E> ResultFns<T, E> for Result<T, E> {
    fn map<U, F: Fn(T) -> U>(self, f: F) -> Result<U, E> {
        match self {
            Result::Ok(x) => Result::Ok(f(x)),
            Result::Err(e) => Result::Err(e),
        }
    }

    fn and_then<U, F: Fn(T) -> Result<U, E>>(self, f: F) -> Result<U, E> {
        match self {
            Result::Ok(x) => f(x),
            Result::Err(e) => Result::Err(e),
        }
    }

    fn or_else<F: Fn(E) -> Result<T, E>>(self, f: F) -> Result<T, E> {
        match self {
            Result::Ok(x) => Result::Ok(x),
            Result::Err(e) => f(e),
        }
    }

    fn unwrap(self) -> T {
        match self {
            Result::Ok(x) => x,
            Result::Err(e) => panic!("called `Result::unwrap()` on an `Err` value"),
        }
    }

    fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(x) => x,
            Result::Err(_) => default,
        }
    }

    fn unwrap_or_else<F: Fn(E) -> T>(self, f: F) -> T {
        match self {
            Result::Ok(x) => x,
            Result::Err(e) => f(e),
        }
    }

    fn expect<U: core::fmt::Display>(self, msg: U) -> T {
        match self {
            Result::Ok(x) => x,
            Result::Err(e) => panic!("{}", msg),
        }
    }

    fn is_ok(&self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    fn is_err(&self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

#[cfg(feature = "check")]
impl<T, E> ResultCheck<T, E> for Result<T, E> 
where T: PartialEq,
      E: PartialEq {
    fn check(&self, other: T) -> bool {
        match self {
            Result::Ok(x) => x == &other,
            Result::Err(_) => false,
        }
    }

    fn check_err(&self, other: E) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(e) => e == &other,
        }
    }
}

#[cfg(feature = "std")]
impl<T, E> ResultCheck<T, E> for std::result::Result<T, E> 
where T: PartialEq,
      E: PartialEq {
    fn check(&self, other: T) -> bool {
        match self {
            std::result::Result::Ok(x) => x == &other,
            std::result::Result::Err(_) => false,
        }
    }

    fn check_err(&self, other: E) -> bool {
        match self {
            std::result::Result::Ok(_) => false,
            std::result::Result::Err(e) => e == &other,
        }
    }
}