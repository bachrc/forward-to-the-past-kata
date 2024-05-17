use assertor::{assert_that, EqualityAssertion};
use forward_to_the_future::movies::{Movie, MovieType};

#[test]
fn a_lambda_movie_name_is_recognized_as_an_other_movie() {
    let lambda_movie = Movie::from("Dimitri Danger et le secret du cassoulet");

    assert_that!(lambda_movie.get_movie_type()).is_equal_to(MovieType::Other)
}

#[test]
fn back_to_the_future_is_recognized_as_such() {
    let back_to_the_future = Movie::from("Back to the Future 1");

    assert_that!(back_to_the_future.get_movie_type())
        .is_equal_to(MovieType::BackToTheFuture { iteration: 1 })
}

#[test]
fn back_to_the_future_is_recognized_even_with_wrong_case() {
    let back_to_the_future = Movie::from("Back to the FUTURE 1");

    assert_that!(back_to_the_future.get_movie_type())
        .is_equal_to(MovieType::BackToTheFuture { iteration: 1 })
}

#[test]
fn back_to_the_future_is_recognized_with_different_iteration() {
    let back_to_the_future = Movie::from("Back to the Future 3");

    assert_that!(back_to_the_future.get_movie_type())
        .is_equal_to(MovieType::BackToTheFuture { iteration: 3 })
}