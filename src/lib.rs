#![feature(box_into_raw_non_null)]

pub mod bit;
pub mod factorial;
pub mod fibonacci;
pub mod gcd;
pub mod is_power_of_two;
pub mod lcm;
pub mod pascal_triangle;
pub mod prime;
pub mod queue;
pub mod sieve_of_eratosthenes;
pub mod singly_linked_list;
pub mod stack;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
