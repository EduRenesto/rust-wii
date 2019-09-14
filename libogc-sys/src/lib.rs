#![no_std]

pub mod gx;
pub mod video;
pub mod consol;
pub mod system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
