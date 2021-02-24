pub mod aba;
pub mod ach;
pub mod bic;
pub mod country_codes;
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
