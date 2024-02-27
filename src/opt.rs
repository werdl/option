#[derive(Debug, PartialEq)]
pub enum Option<T> {
    Some(T),
    None,
}

#[cfg(feature = "no_std")]
pub trait OptionFns<T> {
    fn map<U, F: Fn(T) -> U>(self, f: F) -> Option<U>;
    fn and_then<U, F: Fn(T) -> Option<U>>(self, f: F) -> Option<U>;
    fn or_else<F: Fn() -> Option<T>>(self, f: F) -> Option<T>;
    fn unwrap(self) -> T;
    fn unwrap_or(self, default: T) -> T;
    fn unwrap_or_else<F: Fn() -> T>(self, f: F) -> T;
    fn expect<U: core::fmt::Display>(self, msg: U) -> T;
    fn filter<F: Fn(&T) -> bool>(self, f: F) -> Option<T>;
    fn is_some(&self) -> bool;
    fn is_none(&self) -> bool;
}

#[cfg(feature = "check")]
pub trait OptionCheck<T> 
where T: PartialEq {
    fn check(&self, other: T) -> bool;
}

#[cfg(feature = "no_std")]
impl<T> OptionFns<T> for Option<T> {
    fn map<U, F: Fn(T) -> U>(self, f: F) -> Option<U> {
        match self {
            Option::Some(x) => Option::Some(f(x)),
            Option::None => Option::None,
        }
    }

    fn and_then<U, F: Fn(T) -> Option<U>>(self, f: F) -> Option<U> {
        match self {
            Option::Some(x) => f(x),
            Option::None => Option::None,
        }
    }

    fn or_else<F: Fn() -> Option<T>>(self, f: F) -> Option<T> {
        match self {
            Option::Some(x) => Option::Some(x),
            Option::None => f(),
        }
    }

    fn unwrap(self) -> T {
        match self {
            Option::Some(x) => x,
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }

    fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(x) => x,
            Option::None => default,
        }
    }

    fn unwrap_or_else<F: Fn() -> T>(self, f: F) -> T {
        match self {
            Option::Some(x) => x,
            Option::None => f(),
        }
    }

    fn filter<F: Fn(&T) -> bool>(self, f: F) -> Option<T> {
        match self {
            Option::Some(x) => {
                if f(&x) {
                    Option::Some(x)
                } else {
                    Option::None
                }
            }
            Option::None => Option::None,
        }
    }

    fn is_some(&self) -> bool {
        match self {
            Option::Some(_) => true,
            Option::None => false,
        }
    }

    fn is_none(&self) -> bool {
        match self {
            Option::Some(_) => false,
            Option::None => true,
        }
    }

    fn expect<U: core::fmt::Display>(self, msg: U) -> T {
        match self {
            Option::Some(x) => x,
            Option::None => panic!("{}", msg),
        }
    }
}

#[cfg(feature = "check")]
impl<T> OptionCheck<T> for Option<T> 
where T: PartialEq {
    fn check(&self, other: T) -> bool {
        match self {
            Option::Some(x) => x == &other,
            Option::None => false,
        }
    }
}

#[cfg(feature = "std")]
impl<T> OptionCheck<T> for std::option::Option<T> 
where T: PartialEq {
    fn check(&self, other: T) -> bool {
        match self {
            std::option::Option::Some(x) => x == &other,
            std::option::Option::None => false,
        }
    }
}