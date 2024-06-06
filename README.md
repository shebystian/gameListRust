### Inicio

Como ejecutar el proyecto local:

1. **Iniciar la API**  `cargo run`
4. correr tests `cargo test`
5. lint code with `cargo clippy` and format it with `cargo fmt`
6. correr `cargo build --release` command to generate single optimized binary

### Dependencias
Dependency | Description
--- | ---
[warp](https://crates.io/crates/warp) | Composable web server framework with powerful *filters* system
[serde](https://crates.io/crates/serde) | Library for *serializing* and *deserializing* data structures
[chrono](https://crates.io/crates/chrono) | Date and time utilities
[log](https://crates.io/crates/log) + [pretty_env_logger](https://crates.io/crates/pretty_env_logger) | Simple logger (by default enabled in *debug* mode)

### endpoints

Lista de rutas de API:

* http://localhost:8080/games - GET, POST
* http://localhost:8080/games/:id - PUT, DELETE

### ejecutar Dockerfile
docker build -t ms_bpd .  
docker run -it -p 8080:80 gamelistrust_web_1  