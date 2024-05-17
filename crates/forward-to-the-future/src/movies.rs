use std::collections::HashMap;

use itertools::Itertools;
use regex::{Captures, Regex};

pub static BACK_TO_THE_FUTURE_PATTERN: &str = r"(?i)back\ to\ the\ future\ (\d)";

pub const BTTF_FILM_PRICE: usize = 15;
pub const OTHER_FILM_PRICE: usize = 20;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MovieType {
    BackToTheFuture {iteration: u8},
    Other
}

#[derive(Debug)]
pub struct Movie(String);

impl From<&str> for Movie {
    fn from(value: &str) -> Self {
        Movie(String::from(value))
    }

}

impl Movie {
    pub fn get_movie_type(&self) -> MovieType {
        let bttf_regex = Regex::new(BACK_TO_THE_FUTURE_PATTERN).expect("movie pattern to be valid");

        return match bttf_regex.captures(&self.0) {
            Some(captured_iteration) => match Self::parse_iteration_in_name(captured_iteration) {
                Some(iteration) => MovieType::BackToTheFuture { iteration },
                None => MovieType::Other              
            },
            None => MovieType::Other,
        }
    }

    fn parse_iteration_in_name(captured_iteration: Captures) -> Option<u8> {
        captured_iteration.get(1)
            .map(|capture| capture.as_str())
            .map(|capture| capture.parse::<u8>().ok())
            .flatten()
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
    pub fn compute_cart_price(&self) -> f32 {
        let price_of_other_films = self.compute_other_films_price();
        let price_for_bttf_films = self.compute_bttf_films_price();
        
        price_of_other_films + price_for_bttf_films
    }

    fn compute_other_films_price(&self) -> f32 {
        let number_of_other_films = self.movies.iter()
            .map(Movie::get_movie_type)
            .filter(|movie_type| movie_type == &MovieType::Other)
            .count();
            
        (number_of_other_films * OTHER_FILM_PRICE) as f32
    }
    
    fn compute_bttf_films_price(&self) -> f32 {
        let bttf_movies_count: HashMap<MovieType, usize> = self.movies.iter()
            .filter(|movie| movie.get_movie_type() != MovieType::Other)
            .counts_by(Movie::get_movie_type);

        let total_bttf_films_bought: usize = bttf_movies_count.values().cloned().sum();
        
        let number_of_different_bttf_iterations = bttf_movies_count.keys().count();
        let discount_for_bttf_movies = Self::discount_for_number_of_different_films(number_of_different_bttf_iterations);

        (total_bttf_films_bought * BTTF_FILM_PRICE) as f32 * discount_for_bttf_movies
    }

    fn discount_for_number_of_different_films(different_iterations: usize) -> f32 {
        match different_iterations {
            0 | 1 => 1.0,
            2 => 0.9,
            _ => 0.8
        }
    }
}