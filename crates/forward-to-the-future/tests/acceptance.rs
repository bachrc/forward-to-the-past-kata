use assertor::{assert_that, EqualityAssertion};
use forward_to_the_future::compute_price_for_movies;


// First example
#[test]
fn price_calculation_for_all_films_in_a_trilogy() {
    let films_bought = vec!["Back to the Future 1", "Back to the Future 2", "Back to the Future 3"];

    let price_for_films = compute_price_for_movies(&films_bought);

    assert_that!(price_for_films).is_equal_to(36.0)
}