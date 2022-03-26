//concrete implementation synthax

trait Game {
    fn name(&self) -> String;
}



enum BoardGame {
    Chess,
    Monopoly
}


enum VideoGame {
    PlayStation,
    Xbox,
}

impl Game for BoardGame {

}

impl Game for VideoGame {

}

struct PlayRoom<T: Game> {
    game: T,
}

// concrete impl usage
impl PlayRoom<BoardGame> {
    pub fn cleanup(&self) {

    }
}



let video_room = PlayRoom {
    game: VideoGame::Xbox,
}

let board_room = PlayRoom {
    game : BoardGame::Monopoly
}

board_room.cleanup();
video_room.cleanup();    // we will get an error
 



// generic implementation synthax

struct Name<T: Trait1 + Trait2 , U: Trait3> {
    field1: T,
    field2: U,
}

impl<T: Trait1 + Trait2, U: Trait3> Name<T, U> {
    fn func(&self, arg1: T , arg2: U) {}
}

// generic fn using the where keywords

impl <T, U> Name<T, U>
where 
   T: Trait1 + Trait2, 
   U: Trait3,
   {
     fn func(&self, arg1: T , arg2: U) {}
   }

//generic Example

trait Game {
    fn name(&self) -> String;
}

struct PlayRoom<T: Game> {
   game : T,
}

impl<T:Game> PlayRoom<T> {
    pub fn game_info(&self) {
        println!("{}", self.game.name());
    }
}

let video_room = PlayRoom {
    game: VideoGame::Xbox,
}

let board_room = PlayRoom {
    game : BoardGame::Monopoly
}

board_room.game_info();
video_room.game_info();   