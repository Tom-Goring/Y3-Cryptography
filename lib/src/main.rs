use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crypto::credit::CreditCardVerificationError;
use crypto::isbn::ISBNVerificationError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn isbn(req: HttpRequest) -> impl Responder {
    let x: String = req.match_info().get("isbn").unwrap().parse().unwrap();
    let valid = match crypto::isbn::verify_isbn(&x) {
        Ok(_) => String::from("ISBN is valid!"),
        Err(err) => match err {
            ISBNVerificationError::InvalidDigitCount => {
                String::from("ISBN has wrong number of digits.")
            }
            ISBNVerificationError::NonValidISBN => String::from("ISBN is invalid."),
            ISBNVerificationError::InvalidDigitsFound => {
                String::from("ISBN has invalid characters present.")
            }
        },
    };
    HttpResponse::Ok().body(valid)
}

async fn ccn(req: HttpRequest) -> impl Responder {
    let valid = match crypto::credit::verify_credit_card(req.match_info().get("ccn").unwrap()) {
        Ok(_) => String::from("Credit card number is valid!"),
        Err(err) => match err {
            CreditCardVerificationError::InvalidCreditCard => {
                String::from("Credit card number is not valid")
            }
            CreditCardVerificationError::InvalidDigitsFound => {
                String::from("Credit card number has invalid digits")
            }
            CreditCardVerificationError::InvalidLength => {
                String::from("Credit card number has invalid length")
            }
        },
    };
    HttpResponse::Ok().body(valid)
}

async fn hamming_check_digits(req: HttpRequest) -> impl Responder {
    let input = req.match_info().get("input").unwrap();
    HttpResponse::Ok().body(
        match crypto::hamming::calculate_hamming_check_digits(input) {
            Ok(check_digits) => format!(
                "Successfully generated check digits: {}{}",
                input, check_digits
            ),
            Err(error) => error.to_string(),
        },
    )
}

async fn hamming_syndrome_vector(req: HttpRequest) -> impl Responder {
    let input = req.match_info().get("input").unwrap();
    HttpResponse::Ok().body(match crypto::hamming::generate_syndromes(input) {
        Ok(syndrome_vector) => format!(
            "Successfully calculated syndrome vector: {}",
            syndrome_vector
        ),
        Err(error) => error.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(web::resource("/isbn/{isbn}").route(web::get().to(isbn)))
            .service(web::resource("/ccn/{ccn}").route(web::get().to(ccn)))
            .service(
                web::resource("/hamming/checkdigits/{input}")
                    .route(web::get().to(hamming_check_digits)),
            )
            .service(
                web::resource("/hamming/syndromes/{input}")
                    .route(web::get().to(hamming_syndrome_vector)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
