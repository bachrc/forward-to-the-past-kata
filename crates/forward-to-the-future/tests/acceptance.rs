use assertor::{assert_that, EqualityAssertion};
use forward_to_the_future::compute_price_for_movies;


// Example 1
#[test]
fn price_calculation_for_all_films_in_a_trilogy() {
    let films_bought = vec!["Back to the Future 1", "Back to the Future 2", "Back to the Future 3"];

    let price_for_films = compute_price_for_movies(&films_bought);

    assert_that!(price_for_films).is_equal_to(36.0)
}

// Example 2
#[test]
fn price_calculation_for_two_films_in_the_trilogy() {
    let films_bought = vec!["Back to the Future 1", "Back to the Future 3"];

    let price_for_films = compute_price_for_movies(&films_bought);

    assert_that!(price_for_films).is_equal_to(27.0)
}

// Example 3
#[test]
fn price_calculation_for_only_one_film_in_the_trilogy() {
    let films_bought = vec!["Back to the Future 1"];

    let price_for_films = compute_price_for_movies(&films_bought);

    assert_that!(price_for_films).is_equal_to(15.0)
}

// Example 4
#[test]
fn price_calculation_for_full_trilogy_with_a_duplicate() {
    let films_bought = vec!["Back to the Future 1", "Back to the Future 2", "Back to the Future 3", "Back to the Future 2"];

    let price_for_films = compute_price_for_movies(&films_bought);

    assert_that!(price_for_films).is_equal_to(48.0)
}

// Example 5
#[test]
fn trilogy_price_calculation_with_another_movie() {
    let films_bought = vec!["Back to the Future 1", "Back to the Future 2", "Back to the Future 3", "La tartiflette de Michel: documentaire exclusif"];

    let price_for_films = compute_price_for_movies(&films_bought);

    assert_that!(price_for_films).is_equal_to(56.0)
}