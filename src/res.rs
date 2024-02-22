#[derive(Debug, PartialEq)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}