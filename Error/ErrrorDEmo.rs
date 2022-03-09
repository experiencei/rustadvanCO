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

fn use_pass(pass: &mut SubwayPass , cost: isize ) -> Result<() , PassError> {
    if Utc::now() > pass.expires {
        Err(PassError::PassExpired)
    } else {
        if pass.fund - cost < 0 {
            Err(PassError::InsufficientFunds(pass.funds))
        } else {
            pass.funds = pass.funds - cost;
            Ok(())
        }
    }
}