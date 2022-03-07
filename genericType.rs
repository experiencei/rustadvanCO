#[derive(Clone , Copy)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32)
}

impl Seat for ConcertSeat {
    fn show(&self) {
        ..........................................
    }
}


#[derive(Clone , Copy)]
enum Airline {
   BusinessClass,
   Economy,
   FirstClass
}

impl Seat for Airline {
    fn show(&self) {
        ..........................................
    }
}

// usage with generic type
trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T
}

fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show();
}

let airline = Ticket { location: AirlineSeat::FirstClass };
let concert = Ticket { location: ConcertSeat::FrontRow };

ticket_info(airline);
ticket_info(concert)


// usage with single type
trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T
}

fn ticket_info(ticket: Ticket<AirlineSeat>) {
    ticket.location.show();
}

let airline = Ticket { location: AirlineSeat::FirstClass };
let concert = Ticket { location: ConcertSeat::FrontRow };

ticket_info(airline);
ticket_info(concert)
