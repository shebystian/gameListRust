

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::prelude::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;

    use crate::schema::{Game, Genre};

    // Mocked dataset for each test

    fn mocked_db() -> Db {
        Arc::new(Mutex::new(vec![
            Game {
                id: 1,
                title: String::from("Crappy title"),
                rating: 35,
                genre: Genre::RolePlaying,
                description: Some(String::from("Test description...")),
                release_date: NaiveDate::from_ymd(2011, 9, 22).and_hms(0, 0, 0).to_string(),
            },
            Game {
                id: 2,
                title: String::from("Decent game"),
                rating: 84,
                genre: Genre::Strategy,
                description: None,
                release_date: NaiveDate::from_ymd(2014, 3, 11).and_hms(0, 0, 0).to_string(),
            },
        ]))
    }

    fn mocked_game() -> Game {
        Game {
            id: 3,
            title: String::from("Another game"),
            rating: 65,
            description: None,
            genre: Genre::Strategy,
            release_date: NaiveDate::from_ymd(2016, 3, 11).and_hms(0, 0, 0).to_string(),
        }
    }

    #[tokio::test]
    async fn get_list_of_games_200() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request().method("GET").path("/games").reply(&filter).await;

        assert_eq!(res.status(), StatusCode::OK);

        let expected_json_body = r#"[{"id":1,"title":"Crappy title","rating":35,"genre":"ROLE_PLAYING","description":"Test description...","releaseDate":"2011-09-22"},{"id":2,"title":"Decent game","rating":84,"genre":"STRATEGY","description":null,"releaseDate":"2014-03-11"}]"#;
        assert_eq!(res.body(), expected_json_body);
    }

    #[tokio::test]
    async fn get_list_of_games_with_options_200() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/games?offset=1&limit=5")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let expected_json_body = r#"[{"id":2,"title":"Decent game","rating":84,"genre":"STRATEGY","description":null,"releaseDate":"2014-03-11"}]"#;
        assert_eq!(res.body(), expected_json_body);
    }

    #[tokio::test]
    async fn get_empty_list_with_offset_overshot_200() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/games?offset=5&limit=5")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let expected_json_body = r#"[]"#;
        assert_eq!(res.body(), expected_json_body);
    }

    #[tokio::test]
    async fn get_incorrect_options_400() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/games?offset=a&limit=b")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn get_wrong_path_405() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/games/42")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn post_json_201() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .json(&mocked_game())
            .path("/games")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(db.lock().await.len(), 3);
    }

    #[tokio::test]
    async fn post_too_long_content_413() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("POST")
            .header("content-length", 1024 * 36)
            .path("/games")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn post_wrong_payload_400() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("POST")
            .body(&r#"{"id":4}"#)
            .path("/games")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn post_wrong_path_405() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("POST")
            .path("/games/42")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn put_json_200() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request()
            .method("PUT")
            .json(&mocked_game())
            .path("/games/2")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let db = db.lock().await;
        let ref title = db[1].title;
        assert_eq!(title, "Another game");
    }

    #[tokio::test]
    async fn put_wrong_id_404() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("PUT")
            .json(&mocked_game())
            .path("/games/42")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn put_wrong_payload_400() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("PUT")
            .header("content-length", 1024 * 16)
            .body(&r#"{"id":2"#)
            .path("/games/2")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn put_too_long_content_413() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("PUT")
            .header("content-length", 1024 * 36)
            .path("/games/2")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn delete_wrong_id_404() {
        let db = mocked_db();
        let filter = games_routes(db);

        let res = warp::test::request()
            .method("DELETE")
            .path("/games/42")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_game_204() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request()
            .method("DELETE")
            .path("/games/1")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        assert_eq!(db.lock().await.len(), 1);
    }
}
