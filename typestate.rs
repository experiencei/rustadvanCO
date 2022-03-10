struct BusTicket;
struct BoarderdBusTicket;

impl BusTicket {
    fn board(self) -> BoarderdBusTicket {
        BoarderdBusTicket
    }
}

let ticket = BusTicket;
let boarded = ticket.board();

// compile error 
ticket.board();

// because the reference to the self isn't burrowed i.e &self;



struct File<'a>(&'a str);
impl<'a> File<'a> {
    fn read_bytes(&self) -> Vec<u8> {

    }

    fn delete(self) {

    }
}

let file = File("data.txt");
let data = file.read_bytes();
file.delete();

//compile error
let read_again = file.read_bytes();
