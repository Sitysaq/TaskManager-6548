```rust
// Importing libraries
use std::collections::HashMap;

// Struct for User data
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    gender: String,
    country: String,
}

// Function to create a new user
fn new_user(name: String, age: u8, gender: String, country: String) -> User {
    User {
        name,
        age,
        gender,
        country,
    }
}

// Function to process the user data
fn process_data(user_data: &HashMap<u32, User>) {
    let mut age_sum = 0;
    let mut count = 0;
    for user in user_data.values() {
        age_sum += user.age;
        count += 1;
    }
    let average_age = age_sum / count;
    println!("Average age of users is: {}", average_age);

    let mut country_count = HashMap::new();
    for user in user_data.values() {
        let count = country_count.entry(&user.country).or_insert(0);
        *count += 1;
    }
    println!("User count per country: {:?}", country_count);
}

fn main() {
    // Creating some users
    let user1 = new_user(String::from("John"), 25, String::from("Male"), String::from("USA"));
    let user2 = new_user(String::from("Jane"), 30, String::from("Female"), String::from("USA"));
    let user3 = new_user(String::from("Emma"), 22, String::from("Female"), String::from("Canada"));
    let user4 = new_user(String::from("Sam"), 35, String::from("Male"), String::from("Canada"));

    // Creating a HashMap to store User data
    let mut user_data: HashMap<u32, User> = HashMap::new();

    // Adding users to the HashMap
    user_data.insert(1, user1);
    user_data.insert(2, user2);
    user_data.insert(3, user3);
    user_data.insert(4, user4);

    // Printing all users
    println!("All users: {:?}", user_data);

    // Process the data
    process_data(&user_data);
}
```
Цей код на Rust створює структуру користувача, яка містить ім'я, вік, стать та країну користувача. Він також містить функцію для створення нового користувача та функцію обробки даних, яка визначає середній вік користувачів та кількість користувачів за країною.