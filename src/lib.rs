#![cfg_attr(not(feature = "std"), no_std)]
pub extern crate alloc;
pub mod card;
pub mod crypto;
pub mod test_vector;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
