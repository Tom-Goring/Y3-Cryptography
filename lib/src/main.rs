use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crypto::credit::CreditCardVerificationError;
use crypto::isbn::ISBNVerificationError;

async fn isbn(req: HttpRequest) -> impl Responder {
    let x: String = req.match_info().get("isbn").unwrap().parse().unwrap();
    let valid = match crypto::isbn::verify_isbn(&x) {
        Ok(_) => String::from("ISBN is valid!"),
        Err(err) => match err {
            ISBNVerificationError::InvalidDigitCount => String::from("ISBN has wrong number of digits."),
            ISBNVerificationError::NonValidISBN => String::from("ISBN is invalid."),
            ISBNVerificationError::InvalidDigitsFound => String::from("ISBN has invalid characters present."),
        },
    };
    HttpResponse::Ok().body(valid)
}

async fn ccn(req: HttpRequest) -> impl Responder {
    let valid = match crypto::credit::verify_credit_card(req.match_info().get("ccn").unwrap()) {
        Ok(_) => String::from("Credit card number is valid!"),
        Err(err) => match err {
            CreditCardVerificationError::InvalidCreditCard => String::from("Credit card number is not valid"),
            CreditCardVerificationError::InvalidDigitsFound => String::from("Credit card number has invalid digits"),
            CreditCardVerificationError::InvalidLength => String::from("Credit card number has invalid length"),
        },
    };
    HttpResponse::Ok().body(valid)
}

async fn hamming_check_digits(req: HttpRequest) -> impl Responder {
    let input = req.match_info().get("input").unwrap();
    HttpResponse::Ok().body(match crypto::hamming::calculate_hamming_check_digits(input) {
        Ok(check_digits) => format!("Successfully generated check digits: {}{}", input, check_digits),
        Err(error) => error.to_string(),
    })
}

async fn hamming_syndrome_vector(req: HttpRequest) -> impl Responder {
    let input = req.match_info().get("input").unwrap();
    HttpResponse::Ok().body(match crypto::hamming::generate_syndromes(input) {
        Ok(syndrome_vector) => format!("Successfully calculated syndrome vector: {:?}", syndrome_vector),
        Err(error) => error.to_string(),
    })
}

async fn bch(req: HttpRequest) -> impl Responder {
    let input = req.match_info().get("bch").unwrap();
    HttpResponse::Ok().body(match crypto::bch::verify_bch_input(input) {
        Ok(_) => format!("{} is a valid BCH code with no errors!", input),
        Err(error) => error.to_string(),
    })
}

async fn sha(req: HttpRequest) -> impl Responder {
    let input = req.match_info().get("input").unwrap();
    HttpResponse::Ok().body(crypto::hash::sha1(input))
}

use futures_util::StreamExt;

const MAX_SIZE: usize = 262_144;
async fn crack_normal(mut payload: web::Payload) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        if (body.len() + chunk.len()) > MAX_SIZE {
            panic!("Couldn't process incoming json");
        }
        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<Vec<String>>(&body) {
        Ok(hashes) => HttpResponse::Ok().json(
            match crypto::gpu::crack(&hashes.iter().map(std::ops::Deref::deref).collect::<Vec<&str>>()) {
                None => Vec::new(),
                Some(passwords) => passwords,
            },
        ),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

async fn crack_bch(mut payload: web::Payload) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        if (body.len() + chunk.len()) > MAX_SIZE {
            panic!("Couldn't process incoming json");
        }
        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<Vec<String>>(&body) {
        Ok(hashes) => HttpResponse::Ok().json(
            match crypto::cpu::crack_bch(&hashes.iter().map(std::ops::Deref::deref).collect::<Vec<&str>>()) {
                None => Vec::new(),
                Some(passwords) => passwords,
            },
        ),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/isbn/{isbn}").route(web::get().to(isbn)))
            .service(web::resource("/ccn/{ccn}").route(web::get().to(ccn)))
            .service(web::resource("/hamming/checkdigits/{input}").route(web::get().to(hamming_check_digits)))
            .service(web::resource("/hamming/syndromes/{input}").route(web::get().to(hamming_syndrome_vector)))
            .service(web::resource("/bch/{bch}").route(web::get().to(bch)))
            .service(web::resource("/hash/{input}").route(web::get().to(sha)))
            .service(web::resource("/crack/").route(web::post().to(crack_normal)))
            .service(web::resource("/crackbch/").route(web::post().to(crack_bch)))
            .wrap(cors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
