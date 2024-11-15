Simple Rust web server
- Requires no external dependencies, just `cargo run` it
- Listens on `127.0.0.1:8080` 
- You can put any static files either to the project's `public` directory, or to the directory denoted by `PUBLIC_PATH`  
- Special `/hello` URL prints the query string, if available
- All HTTP verbs are acknowledged, but anything except `GET` throws 404