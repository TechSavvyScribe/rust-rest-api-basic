use actix_web::{web, HttpResponse, Responder};

// Error response struct
#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    error: String,
}

// User struct
#[derive(Debug, serde::Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    // Add other fields as needed
}

pub async fn welcome_msg() -> impl Responder {

    HttpResponse::Created().body("Welcome to User RestApi ...")
}

// Create a new user
pub async fn create_user() -> impl Responder {
    // Implement the logic to create a new user
    let user = User {
        id: 1,
        name: "John Doe".to_owned(),
        email: "johndoe@example.com".to_owned(),
    };
    
    HttpResponse::Created().json(user)
}

// Get a user by ID
pub async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    // Implement the logic to retrieve a user by ID
    let user = match user_id {
        1 => Some(User {
            id: 1,
            name: "John Doe".to_owned(),
            email: "johndoe@example.com".to_owned(),
        }),
        _ => None,
    };
    
    if let Some(user) = user {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User ID {} not found", user_id),
        })
    }
}

// Update a user by ID
pub async fn update_user_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    // Implement the logic to update a user by ID
    let updated_user = match user_id {
        1 => Some(User {
            id: 1,
            name: "Updated John Doe".to_owned(),
            email: "updatedjohndoe@example.com".to_owned(),
        }),
        _ => None,
    };
    
    if let Some(updated_user) = updated_user {
        HttpResponse::Ok().json(updated_user)
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User ID {} not found", user_id),
        })
    }
}

// Delete a user by ID
pub async fn delete_user_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    // Implement the logic to delete a user by ID
    let result = match user_id {
        1 => true,
        _ => false,
    };
    
    if result {
        HttpResponse::Ok().body(format!("User ID {} deleted", user_id))
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User ID {} not found", user_id),
        })
    }
}
