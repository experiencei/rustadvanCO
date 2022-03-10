mod NoGenerics {
    #[derive(Copy, Clone)]
    struct LuggageId(usize);
    struct Luggage(LuggageId);

    struct CheckIn(LuggageId);
    struct OnLoad(LuggageId);
    struct Offload(LuggageId);
    struct AwaitingPickup(LuggageId);
    struct EndCustody(LuggageId);

    impl Luggage {
        fn new(id: LuggageId) -> Self {
            Luggage(id)
        }
        fn check_in(self) -> CheckIn {
            CheckIn(self.0)
        }
    }

    impl CheckIn {
        fn onload(self) -> OnLoad {
            OnLoad(self.0)
        }
    }

    impl OnLoad {
        fn offload(self) -> Offload {
            Offload(self.0)
        }
    }

    impl Offload {
        fn carousel(self) -> AwaitingPickup {
            AwaitingPickup(self.0)
        }
    }

    impl AwaitingPickup {
        fn pickup(self) -> (Luggage, EndCustody) {
            (Luggage(self.0), EndCustody(self.0))
        }
    }

    fn main() {
        let id = LuggageId(1);
        let luggage = Luggage::new(id);
        let luggage = luggage.check_in().onload().offload().carousel();
        luggage.pickup();
    }
}

mod Generics {

    #[derive(Copy, Clone)]

    struct LuggageId(usize);
    struct Luggage<State> {
        id: LuggageId,
        state: State,
    }

    impl<State> Luggage<State> {
        fn next<Next>(self, state: Next) -> Luggage<Next> {
            Luggage { id: self.id, state }
        }
    }

    struct BeginCustody;
    struct CheckIn;
    struct OnLoad;
    struct OffLoad;
    struct AwaitingPickup;
    struct EndCustody(LuggageId);

    impl Luggage<BeginCustody> {
        fn new(id: LuggageId) -> Self {
            Self {
                id,
                state: BeginCustody,
            }
        }
        fn check_in(self) -> Luggage<CheckIn> {
            self.next(CheckIn)
        }
    }

    impl Luggage<CheckIn> {
        fn onload(self) -> Luggage<OnLoad> {
            self.next(OnLoad)
        }
    }

    impl Luggage<OnLoad> {
        fn offload(self) -> Luggage<OffLoad> {
            self.next(OffLoad)
        }
    }

    impl Luggage<OffLoad> {
        fn carousel(self) -> Luggage<AwaitingPickup> {
            self.next(AwaitingPickup)
        }
    }

    impl Luggage<AwaitingPickup> {
        fn pickup(self) -> (Luggage<EndCustody>, EndCustody) {
            let id = self.id;
            (self.next(EndCustody(id)), EndCustody(id))
        }
    }

    fn main() {
        let id = LuggageId(1);
        let luggage = Luggage::new(id);
        let luggage = luggage.check_in().onload().offload().carousel();
        luggage.pickup();
    }
}

fn main() {}
