use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::services::dal_service; // Import from services/price.rs
use crate::services::oracle_service;  // Import from services/task.rs

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ExecuteTaskPayload {
    pub taskDefinitionId: Option<i32>, // optional in case it's not included in the request body
}

#[derive(Serialize)]
#[allow(dead_code)]
// remove the lime above if using a CustomResponse
struct CustomResponse {
    status: String,
    data: HashMap<String, serde_json::Value>,
}

pub async fn execute_task(payload: web::Json<ExecuteTaskPayload>) -> impl Responder {
    println!("Executing Task");

    // Default taskDefinitionId to 0 if not provided
    let task_definition_id = payload.taskDefinitionId.unwrap_or(0);
    println!("task_definition_id: {}", task_definition_id);

    match oracle_service::get_price("ETHUSDT").await {
        Ok(price_data) => {
            let proof_of_task = price_data.price;
            // Send the task
            let _ = dal_service::send_task(proof_of_task, task_definition_id).await;
            HttpResponse::Ok().json("Task executed successfully")
        }
        Err(err) => {
            eprintln!("Error fetching price: {}", err);
            HttpResponse::ServiceUnavailable().json("Network error occurred")
            
        }
    }
}
