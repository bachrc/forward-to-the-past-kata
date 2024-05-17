use movies::Cart;

mod movies;

pub fn compute_price_for_movies(movies: &Vec<&str>) -> u32 {
    let cart = Cart::from(movies);

    cart.compute_cart_price()
}
