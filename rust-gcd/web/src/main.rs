use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use gcd_core::gcd::compute_gcd;
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_gcd).service(post_gcd))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }
    let response = format!(
        "The GCD of the numbers {} and {} is <b>{}</b>!",
        form.n,
        form.m,
        compute_gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}

#[get("/")]
async fn get_gcd() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
                <form style="padding: 12px;" action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}
