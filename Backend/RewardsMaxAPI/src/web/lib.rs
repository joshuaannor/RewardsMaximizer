use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Function to establish a connection to the database using an environment variable for the database URL.
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok(); // Load environment variables from .env file

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish the connection to the SQLite database
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database at {}", database_url))
}
