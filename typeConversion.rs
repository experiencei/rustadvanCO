// From/Into 

let owned = String::from("slice");
let owned : String = "slice".into();


fn to_owned(slice: &str) -> String {
    slice.into()
}

// implementing from
enum Status {
    Broken(u8),
    Working
}

impl From<u8> for Status {
    fn from(code: u8) -> Self {
        match code {
           0 => Status::Working,
           c => Status::Broken(code),
        }
    }
}

// using from/Into implementation

fn legacy_interface() -> u8 {
    5
}

let status: Status = legacy_interface().into();
let status = Status::from(legacy_interface());


// Question mark operator

struct Job;

enum Job;

enum JobError {
  Expired,
  Missing,
  Other(u8),
}

impl From<u8> for JobError {
    fn from(code: u8) -> Self {
        match code {
            1 => Self::Expired,
            2 => Self::Missing,
            c => Self::Other(c),
        }
    }
}

fn execute_job(job: Job) -> Result<(), JobError> {
    Err(2)
}