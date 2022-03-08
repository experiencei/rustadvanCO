use chrono::{DateTime , Duration , Utc};
use thiserror::Error;

struct SubwayPass {
    id : usize,
    funds: isize,
    expires: DateTime<Utc>,
}

enum PassError {
    PassExpired,
    InsufficientFunds(isize),
    ReadError(String)
}

fn swipe_card() -> Result<SubwayPass, PassError> {
    OK(SubwayPass {
        id : 0,
        funds: 200,
        expires: Utc::now() + Duration::weeks(52),
    })
}

fn use