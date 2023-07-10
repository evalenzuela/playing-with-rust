fn main() {
    let car: Car = Car {
        brand: "Honda".to_string(),
        model: "Civic".to_string(),
        year: 2018,
    };

    println!("I bought a {} {} {} ", car.brand, car.model, car.year);
}

struct Car {
    model: String,
    brand: String,
    year: u32,
}
