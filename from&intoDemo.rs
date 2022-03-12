// Demo 1
struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}



impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

fn main() {
    let upper = Uppercase::from("lowercase");
    let upper: Uppercase = "lowercase".into();
}

// demo 2
enum KeyPress {
    Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

enum InputEvent {
    key(u16, KeyPress),
    Mouse,
}

impl From<KeyEvent> for InputEvent {
    fn from(ev: KeyEvent) -> Self {
        InputEvent::Key(ev.keycode, ev.state)
    }
}

fn main() {
    let key_ev = KeyEvent {
        keycode: 8,
        state: KeyPress::Down,
    };

    let input_ev = InputEvent::from(key_ev);
    let input_ev:InputEvent =key_ev.into();
    
}

// Demo 3

use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    TimeOut,
}

#[derive(Debug, Error)]
enum DatabaseErrors {
    #[error("error querying database")]
   QueryFailure,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] NetworkError),
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
}

//WE CAN DELETE ALL THIS IMPL TO REPLACE WITH #[FROM]
impl From<NetworkError> for ApiError {
    fn from(err: NetworkError) -> Self {
        Self::Network(err)
    }
}
//WE CAN DELETE ALL THIS IMPL TO REPLACE WITH #[FROM]
impl From<DatabaseError> for ApiError {
    fn from(err: DatabaseError) -> Self {
        Self::Database(err)
    }
}

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::TimeOut)?
}