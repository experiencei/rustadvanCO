use chrono::prelude::*;

let utc = Utc::now();
let local = Local::now();


// can be West or East depending where u are located;
let est = FixedOffset::west(5 * 3600)
   .ymd(2022, 05 , 16)
   .and_hms(13, 45 , 12);

println!("{:?}", est);

// Converting Between TimeZone and UTC

let est = FixedOffset::west(5 * 3600)
        .ymd(2022, 05 , 16)
        .and_hms(13, 45 , 12);
let utc = est.with_timezone(&Utc);
let utc: DateTime<Utc> = est.into();


let est_tz = FixedOffset::east(5 * 3600);
let est = utc.with_timezone(&est_tz);

// Duration
use chrono::Duration;
let now = Utc::now();

let an_hour = Duration::hours(1);
let when = now + an_hour;
println!("now {:?}", an_hour);
println!("in an hour {:?}", when);

let duration = when - now;
println!("duration {:?}", duration);

// Parsing time
let date_str = "2020-08-23T10:00:00Z";
let dt = date_str.parse::<DateTime<Utc>>();

let date_str = "2020-08-23T10:00:00 +03:00";
let fmt_str = "%Y-%m-%d %H:%M:%S %z";
let dt =DateTime::parse_from_str(date_str , fmt_str);

let date_str = "2020-08-23 10:00:00";
let fmt_str = "%Y-%m-%d %H:%M:%S";
let dt =Utc.datetime_from_str(date_str , fmt_str);


//Formatting
let date = Local::now();
let fmt_str = "%Y-%m-%d %H:%M:%S";
println!("{:?}" , date.format(fmt_str));