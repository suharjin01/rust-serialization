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
    address: AddressRequest,
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
