use super::{Secret, GuessError};

#[test]
fn has_a_new_constructor() {
    Secret::new(0);
}

#[test]
fn has_a_secret_number() {
    let secret = Secret::new(0);
    assert_eq!(secret.secret(), 0);
    let secret = Secret::new(500);
    assert_eq!(secret.secret(), 500);
}

#[test]
fn guess_not_a_number() {
    let mut secret = Secret::new(0);
    assert_eq!(secret.guess("wat"), Err(GuessError::NotANumber));
}

#[test]
fn guess_too_high() {
    let mut secret = Secret::new(0);
    assert_eq!(secret.guess("1"), Err(GuessError::TooHigh(1)));
}

#[test]
fn guess_too_low() {
    let mut secret = Secret::new(1);
    assert_eq!(secret.guess("0"), Err(GuessError::TooLow(0)));
}

#[test]
fn guess_correct_first_try() {
    let mut secret = Secret::new(0);
    assert_eq!(secret.guess("0"), Ok(1));
}

#[test]
fn guess_correct_second_try() {
    let mut secret = Secret::new(1);
    let _ = secret.guess("0");
    assert_eq!(secret.guess("1"), Ok(2));
}

#[test]
fn guess_correct_third_try() {
    let mut secret = Secret::new(0);
    let _ = secret.guess("2");
    let _ = secret.guess("nope");
    assert_eq!(secret.guess("0"), Ok(3));
}

#[test]
fn guess_with_whitespace() {
    let mut secret = Secret::new(0);
    assert_eq!(secret.guess("\t0  \n"), Ok(1));
}

#[test]
fn display_error() {
    assert_eq!(&GuessError::NotANumber.to_string(), "That's not a number.");
    assert_eq!(&GuessError::TooHigh(10).to_string(), "10 is too high.");
    assert_eq!(&GuessError::TooLow(17).to_string(), "17 is too low.");
}

#[test]
fn has_a_random_constructor() {
    Secret::random();
}

