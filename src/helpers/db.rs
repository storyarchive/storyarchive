use mongodb::{Client, Error, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};

pub fn init_db(uri: &str, db_uri: &str) -> Result<(Client, Database), Error> {
    let client = Client::with_uri(uri)?;
    let db = client.db(db_uri);

    Ok((client, db))
}