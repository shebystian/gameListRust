// Common types used across API

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::validators;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u64,
    pub title: String,
    #[serde(with = "validators::validate_game_rating")]
    pub rating: u8,
    pub genre: Genre,
    pub description: Option<String>,
    pub release_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Genre {
    RolePlaying,
    Strategy,
    Shooter,
    Fight,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// For presentation purposes keep mocked data in in-memory structure
// In real life scenario connection with regular database would be established

pub type Db = Arc<Mutex<Vec<Game>>>;

pub fn example_db() -> Db {
    Arc::new(Mutex::new(
        vec![
        Game {
            id: 1,
            title: String::from("Dark Souls"),
            rating: 91,
            genre: Genre::RolePlaying,
            description: Some(String::from("Tiene lugar en el reino ficticio de Lordran, donde los jugadores asumen el papel de un personaje no-muerto maldito que comienza una peregrinación para descubrir el destino de los de su especie.")),
            release_date: String::from("2011-09-22"),
        },
        Game {
            id: 2,
            title: String::from("Street Fighter II"),
            rating: 87,
            genre: Genre::Fight,
            description: Some(String::from("juego de lucha en 2D desarrollado por Capcom y lanzado originalmente para salas de juegos en 1991.")),
            release_date: String::from("1991-03-07"),
        },
        Game {
            id: 3,
            title: String::from("Dark Souls 3"),
            rating: 89,
            genre: Genre::RolePlaying,
            description: Some(String::from("El último capítulo de la serie con su característico combate con espadas y hechicería y un gratificante juego de rol de acción.")),
            release_date: String::from("2016-3-24"),
        },
    ]
    ))
}
