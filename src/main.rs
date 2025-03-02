use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressRequest {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
    #[serde(rename = "alamat")] // menambahkan field attribute
    address: AddressRequest,
}

// Struct untuk materi array dan vector
#[derive(Debug, Serialize, Deserialize)]

// menambahkan container attribute
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE", 
    deserialize = "SCREAMING_SNAKE_CASE"
))]
struct User {
    first_name: String,
    username: String,
    email: String,
    hobbies: Vec<String>,
    phone: Option<String>,
    gender: Gender, // menambahkan gender dari enum Gender
    payment: Payment, // menambahkan payment dari nemum Payment
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Payment {
    CreditCard {
        card_number: String,
        expiration: String,
    },
    BankAccount {
        accaunt_number: String,
        bank_name: String,
    }
}


// Chrono
#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_ad: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_ad: DateTime<Utc>,
}


#[test]
fn test_create_json_for_user_login_request() {
    let login_request = UserLoginRequest {
        username: "testuser".to_string(),
        password: "rahasia".to_string()
    };

    let json = serde_json::to_string(&login_request).unwrap();
    println!("{}", json);

    let login_result: UserLoginRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result)
}


#[test]
fn test_create_json_for_user_create_user_request() {
    let request = CreateUserRequest {
        username: "testuser".to_string(),
        password: "rahasia".to_string(),
        email: "test03@gmail.com".to_string(),
        address: AddressRequest {
            street: "Jalan Kayu".to_string(),
            city: "Malang".to_string(),
            state: "Konoha".to_string(),
            zip: "1313131".to_string()
        }
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", result)
}

// Array
#[test]
fn test_create_json_from_array() {
    let numbers = [10, 20, 15, 25, 30];
    let json = serde_json::to_string(&numbers).unwrap();

    println!("{}", json)
}

// Vector
#[test]
fn test_vector() {
    let request = User {
        first_name: "Suharjin".to_string(),
        username: "suharjin".to_string(),
        email: "suharjin01@gmail.com".to_string(),
        hobbies: vec!["Reading".to_string(), "swimming".to_string(), "browsing".to_string()],

        // vector with option
        //phone: None,
        phone: Some("083138198579".to_string()),
        gender: Gender::Female, // tambah enum di fn test vector

        payment: Payment::BankAccount { // tambah enum Payment di fn test_vector
            accaunt_number: "1234474374".to_string(), 
            bank_name: "Bank Muamalat".to_string() 
        }
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    // konversi balik ke data vector
    let result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", result)
}

// Map
#[test]
fn test_map() {
    let mut values: HashMap<String, i32> = HashMap::new();
    values.insert("one".to_string(), 1);
    values.insert("two".to_string(), 2);
    values.insert("three".to_string(), 3);

    let json = serde_json::to_string(&values).unwrap();
    println!("{}", json);

    let result: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
    println!("{:?}", result)
}


// Enum
#[test]
fn test_enum() {
    let request = User {
        first_name: "Suharjin".to_string(),
        username: "suharjin".to_string(),
        email: "suharjin01@gmail.com".to_string(),
        hobbies: vec!["Reading".to_string(), "swimming".to_string(), "browsing".to_string()],
        phone: None,
        gender: Gender::Male,

        payment: Payment::CreditCard { 
            card_number: "093434374".to_string(), 
            expiration: "01-07-29".to_string() 
        }
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    // konversi balik ke data vector
    let result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", result)
}


// Chrono
#[test]
fn test_chrono() {
    let category = Category {
        id: "12345".to_string(),
        name: "Gadget".to_string(),
        created_ad: Utc::now(),
        updated_ad: Utc::now()
    };

    let json = serde_json::to_string(&category).unwrap();
    println!("{}", json);

    // konversi balik ke data vector
    let result: Category = serde_json::from_str(&json).unwrap();
    println!("{:?}", result)
}
