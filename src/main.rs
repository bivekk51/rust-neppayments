use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rustpayment::{
    generate_transaction_uuid, pay_with_esewa, validate_esewa_response, EsewaPaymentRequest,
};
use serde::Deserialize;

const SECRET_KEY: &str = "8gBm/:&EnhH.1/q";

#[derive(Deserialize)]
struct SuccessQuery {
    data: String,
}

#[get("/")]
async fn index() -> impl Responder {
    // Create payment request
    let request = EsewaPaymentRequest {
        amount: "100".to_string(),
        tax_amount: "10".to_string(),
        total_amount: "110".to_string(),
        transaction_uuid: generate_transaction_uuid(),
        product_code: "EPAYTEST".to_string(),
        product_service_charge: "0".to_string(),
        product_delivery_charge: "0".to_string(),
        success_url: "http://127.0.0.1:8080/success".to_string(),
        failure_url: "http://127.0.0.1:8080/failure".to_string(),
        signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
    };

    // Initiate payment with eSewa
    match pay_with_esewa(request, SECRET_KEY).await {
        Ok(payment_url) => HttpResponse::Found()
            .append_header(("Location", payment_url))
            .finish(),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body(format!("Payment error: {}", e)),
    }
}

#[get("/success")]
async fn success(query: web::Query<SuccessQuery>) -> impl Responder {
    // Validate the response from eSewa
    match validate_esewa_response(&query.data, SECRET_KEY) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("Validation error: {}", e)),
    }
}

#[get("/failure")]
async fn failure() -> impl Responder {
    HttpResponse::Ok().body("Payment failed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    println!("Visit http://127.0.0.1:8080 to initiate a test payment");
    
    HttpServer::new(|| App::new().service(index).service(success).service(failure))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
