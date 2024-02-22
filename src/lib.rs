#![no_std]

mod opt;
mod res;
    

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
