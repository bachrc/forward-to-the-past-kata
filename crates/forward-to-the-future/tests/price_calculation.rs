use assertor::{assert_that, EqualityAssertion};
use forward_to_the_future::compute_price_for_movies;

#[test]
fn a_film_from_the_trilogy_costs_15_euros() {
    let films_bought = vec!["Back to the Future 1"];

    let total_price = compute_price_for_movies(&films_bought);

    assert_that!(total_price).is_equal_to(15.0)
}

#[test]
fn two_different_films_from_trilogy_have_a_discount_of_10_percents() {
    let films_bought = vec!["Back to the Future 1", "Back to the Future 2"];

    let total_price = compute_price_for_movies(&films_bought);

    assert_that!(total_price).is_equal_to(27.0)
}