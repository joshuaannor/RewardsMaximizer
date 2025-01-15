# RewardsMaximizer

| Command           | Builds                   | Date/time      | 
| :---------------- | :------:                 | :------:       |
| cargo run         |   :heavy_check_mark:     | 30OCT24 @ 2015 | 

## API section
### Running backend locally through cargo and not Docker
0. Change this line in app.rs: let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(); to let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
1. cargo run
2. Visit [`localhost:8080`](http://localhost:8080)

## When deploying backend to back4app
0. Make sure this line in app.rs has quad 0's: let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(); 

## JSON Examples to POST to Backend to write to DB
### There are a few examples of JSON's we can POST to the desired routes, they follow DB format
### They are located in: RewardsMaxAPI/JSON Examples/

### Diesel
1. cargo install diesel_cli
#### To run migration: diesel migration run

### Generate database diagram
1.  pip install eralchemy
2.  pip install graphviz
3.  brew install graphviz (if on MacOS) || sudo apt-get install graphviz (Ubuntu)
4.  eralchemy -i sqlite:///database_NAME.db -o output_diagram_NAME.pdf


### Resources:
1. https://doc.rust-lang.org/std/
2. https://doc.rust-lang.org/reference/
3. https://tourofrust.com/TOC_en.html 
4. https://diesel.rs/guides/getting-started