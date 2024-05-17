use movies::Cart;

pub mod movies;

pub fn compute_price_for_movies(movies: &Vec<&str>) -> f32 {
    let cart = Cart::from(movies);

    cart.compute_cart_price()
}
