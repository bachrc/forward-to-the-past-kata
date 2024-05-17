use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash)]
pub enum MovieType {
    BackToTheFuture,
    Other
}

pub struct Movie(String);

impl From<&str> for Movie {
    fn from(value: &str) -> Self {
        Movie(String::from(value))
    }

}

impl Movie {
    pub fn get_movie_type(&self) -> MovieType {
        if self.0.to_lowercase().starts_with("back to the future") {
            return MovieType::BackToTheFuture
        }

        return MovieType::Other
    }
}

pub struct Cart {
    pub movies: Vec<Movie>
}

impl From<&Vec<&str>> for Cart {
    fn from(value: &Vec<&str>) -> Self {
        let movies = value.iter().cloned().map(Movie::from).collect();

        Cart { movies }
    }
}

impl Cart {
    pub fn compute_cart_price(&self) -> u32 {
        let movie_types: HashMap<MovieType, usize> = self.movies.iter()
            .counts_by(Movie::get_movie_type);

        let number_of_bttf = movie_types.get(&MovieType::BackToTheFuture).cloned().unwrap_or(0);
        
        (number_of_bttf * 15) as u32
    }
}