use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum Status {
    Available,
    Maintenance,
    Rented,
    Unavailable,
}

#[derive(Debug)]
struct Rental {
    status: Status,
    vehicle: Vehicle,
    vin: String,
}

#[derive(Debug)]
struct Corporate {
    rentals: Rc<RefCell<Vec<Rental>>>,
}

#[derive(Debug)]
struct StoreFront {
    rentals: Rc<RefCell<Vec<Rental>>>, 
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                status: Status::Available,
                vehicle: Vehicle::Car,
                vin: "123".to_owned(),
            },
            Rental {
                status: Status::Maintenance,
                vehicle: Vehicle::Truck,
                vin: "abc".to_owned(),
            },
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate {
            rentals: Rc::clone(&vehicles),
        };
        let storefront = StoreFront {
            rentals: Rc::clone(&vehicles),
        };

        // Rent out car from storefront.
        {
            let mut rentals = storefront.rentals.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Available);
                car.status = Status::Rented;
            }
        }

        // Corporate changes car back to available.
        {
            let mut rentals = corporate.rentals.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Rented);
                car.status = Status::Available;
            }
        }

        // Storefront status indicates available.
        let mut rentals = storefront.rentals.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, Status::Available);
        }
    }
}
