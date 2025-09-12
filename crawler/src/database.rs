use postgres::{Client, NoTls};

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Self {
        let newClient = Client::connect("host=localhost user=prushton password=password dbname=maindb", NoTls).unwrap();

        let db: Self = Self{
            client: newClient,
        };
        
        return db;
    }

    pub fn get(self: &mut Self) -> String {
        let _ = self.client.batch_execute("
            CREATE TABLE person (
                id      SERIAL PRIMARY KEY,
                name    TEXT NOT NULL,
                data    BYTEA
            )
        ");

        return "A".to_string()
    }
}