// Structs - used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct new person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run(){
    // let mut color = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // color.red = 200;

    // println!("Color: {:?}", (color.red, color.green, color.blue));
    
    // let mut color = Color(255, 0, 0);
    
    // println!("Color: {:?}", (color.0, color.1, color.2));

    let mut person = Person::new("Mary", "Doe");

    person.set_last_name("Williams");

    println!("Person {}", person.get_full_name());

    println!("Person as tuple: {:?}", person.to_tuple());
}