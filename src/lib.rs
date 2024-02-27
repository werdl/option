#![cfg_attr(all(feature = "no-std"), no_std)]

#[cfg(all(feature = "no-std", feature = "std"))] 
compile_error!("features `no-std` and `std` are mutually exclusive");

mod opt;
mod res;

pub mod prelude {
    pub use crate::opt::*;
    pub use crate::res::*;
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let x = Option::Some(5);
        let y = x.map(|x| x + 1);
        assert_eq!(y, Option::Some(6));
    }
}
