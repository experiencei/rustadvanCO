async fn life() -> u32 {
    42
}

#[tokio::main]
pub async fn main() {
    let future = life();
    let vcalue = future.await;

    let value: u32 = life().await;
}

// example of Future
async fn connect() -> Result<Connection, ConnectionError> {
    Ok(Connection)
}

async fn get_credentials(conn: &Connection) 
-> Result<Credentials, CredentialError> {
    Ok(Credentials)
}

async fn generate_session(conn: &Connection, creds: &Credentials) 
-> Result<Session , SessionError> {
    Ok(Session)
}