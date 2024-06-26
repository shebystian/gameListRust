//  RESTful API para juegos:
//
// - `GET /games`: retorna un JSON con lista de juegos
// - `POST /games`: crear una nueva entrada de games
// - `PUT /games/:id`: actualiza un juego especifico
// - `DELETE /games/:id`: elimina un juego especifico

use warp::{Filter, Rejection, Reply};

use crate::custom_filters;
use crate::handlers;
use crate::schema::Db;

// Root, combinacion de todas las rutas
pub fn games_routes(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    games_list(db.clone())
        .or(games_create(db.clone()))
        .or(games_update(db.clone()))
        .or(games_delete(db))
}

// `GET /games?offset=3&limit=5`
pub fn games_list(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("games")
        .and(warp::get())
        .and(custom_filters::list_options())
        .and(custom_filters::with_db(db))
        .and_then(handlers::list_games)
}

// `POST /games`
pub fn games_create(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("games")
        .and(warp::post())
        .and(custom_filters::json_body())
        .and(custom_filters::with_db(db))
        .and_then(handlers::create_game)
}

// `PUT /games/:id`
pub fn games_update(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("games" / u64)
        .and(warp::put())
        .and(custom_filters::json_body())
        .and(custom_filters::with_db(db))
        .and_then(handlers::update_game)
}

// `DELETE /games/:id`
pub fn games_delete(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("games" / u64)
        .and(warp::delete())
        .and(custom_filters::with_db(db))
        .and_then(handlers::delete_game)
}