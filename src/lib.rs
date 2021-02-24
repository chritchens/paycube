pub mod aba;
pub mod ach;
pub mod iso9326;
pub mod iso31661_alpha2;
pub mod error;
pub mod iban;
pub mod result;
pub mod sepa;
pub mod swift;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
