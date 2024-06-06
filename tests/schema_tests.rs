

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::prelude::*;
    use serde_json::error::Error;
    use serde_test::{assert_tokens, Token};
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

    #[test]
    fn game_serialize_correctly() {
        let game = Game {
            id: 1,
            title: String::from("Test"),
            rating: 90,
            genre: Genre::Shooter,
            description: None,
            release_date: String::from("2019-11-12"),
        };

        assert_tokens(
            &game,
            &[
                Token::Struct { name: "Game", len: 6 },
                Token::String("id"),
                Token::U64(1),
                Token::String("title"),
                Token::String("Test"),
                Token::String("rating"),
                Token::U8(90),
                Token::String("genre"),
                Token::UnitVariant {
                    name: "Genre",
                    variant: "SHOOTER",
                },
                Token::String("description"),
                Token::None,
                Token::String("releaseDate"),
                Token::String("2019-11-12"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn game_deserialize_correctly() {
        let data = r#"{"id":3,"title":"Another game","rating":65,"genre":"STRATEGY","description":null,"releaseDate":"2016-03-11"}"#;
        let game: Game = serde_json::from_str(data).unwrap();
        let expected_game = Game {
            id: 3,
            title: String::from("Another game"),
            rating: 65,
            genre: Genre::Strategy,
            description: None,
            release_date: String::from("2016-03-11"),
        };

        assert_eq!(game, expected_game);
    }

    #[test]
    fn game_error_when_wrong_rating_passed() {
        let data = r#"{"id":3,"title":"Another game","rating":120,"genre":"STRATEGY","description":null,"releaseDate":"2016-03-11"}"#;
        let err: Error = serde_json::from_str::<Game>(data).unwrap_err();

        assert_eq!(err.is_data(), true);
    }

    #[test]
    fn genre_serialize_correctly() {
        let genre = Genre::Shooter;
        assert_tokens(
            &genre,
            &[Token::UnitVariant {
                name: "Genre",
                variant: "SHOOTER",
            }],
        );

        let genre = Genre::RolePlaying;
        assert_tokens(
            &genre,
            &[Token::UnitVariant {
                name: "Genre",
                variant: "ROLE_PLAYING",
            }],
        );

        let genre = Genre::Strategy;
        assert_tokens(
            &genre,
            &[Token::UnitVariant {
                name: "Genre",
                variant: "STRATEGY",
            }],
        );
    }

    #[test]
    fn genre_deserialize_correctly() {
        let data = r#""SHOOTER""#;
        let genre: Genre = serde_json::from_str(data).unwrap();
        let expected_genre = Genre::Shooter;

        assert_eq!(genre, expected_genre);

        let data = r#""ROLE_PLAYING""#;
        let genre: Genre = serde_json::from_str(data).unwrap();
        let expected_genre = Genre::RolePlaying;

        assert_eq!(genre, expected_genre);
    }

    #[test]
    fn genre_error_when_wrong_rating_passed() {
        let data = r#""SPORT""#;
        let err: Error = serde_json::from_str::<Genre>(data).unwrap_err();

        assert_eq!(err.is_data(), true);
    }
}
