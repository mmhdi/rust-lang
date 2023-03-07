use tokio;
use axum::{extract::Form, routing::get, http::StatusCode,response::{Response, IntoResponse},Router};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use mongodb::{bson::doc,Client};
use std::error::Error;

#[tokio::main]
async fn main() {
	axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(Router::new()
		.route("/", get(index))
		.fallback_service(ServeDir::new("static"))
		.route("/signin/", get(signin).post(signin_form))
		.route("/signup/", get(signup).post(signup_form))
		.route("/confirm/email/", get(confirm_email).post(confirm_email_form))
		.route("/confirm/email/verify/", get(confirm_email_verify).post(confirm_email_verify_form))
		.route("/confirm/phone/", get(confirm_phone).post(confirm_phone_form))
		.route("/confirm/phone/verify/", get(confirm_phone_verify).post(confirm_phone_verify_form))
		.route("/confirm/id/", get(confirm_id).post(confirm_id_form))
		.route("/confirm/id/verify/", get(confirm_id_verify).post(confirm_id_verify_form))
	.into_make_service()).await.unwrap();
}
async fn index()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("index", include_str!("layouts/index.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("index", &Context::new()).unwrap()).unwrap()
}

async fn signin()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signin", include_str!("layouts/signin.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("signin", &Context::new()).unwrap()).unwrap()
}
#[derive(Deserialize, Serialize)]
struct Login {
	r#fn: Option<String>,
    ln: Option<String>,
    un: Option<String>,
    em: Option<String>,
    pw: Option<String>,
    status: Option<String>,
    otpem: Option<String>,
    otpemurl: Option<String>,
    ac: Option<String>
}
async fn signin_form(Form(login): Form<Login>)-> Result<impl IntoResponse, StatusCode>{
	let db = Client::with_uri_str("mongodb+srv://mbra:mbra@cluster0.um0c2p7.mongodb.net/?retryWrites=true&w=majority").await.unwrap().database("braq").collection::<Login>("users");
	let deb: Login = db.find_one(doc!{"un":&login.ac},None).await.unwrap().unwrap();
	//let ggg= db.insert_one(doc!{"un":login.ac},None).await.unwrap();
	let mut tera = Tera::default();
	let mut context = Context::new();
	//match deb{
	//if &deb.get_str("un") == &login.un && &deb.get_str("pw") == &login.pw{
		context.insert("ac",&deb.em);
	//}else{
		//Err => context.insert("ac","none")
	//}
	tera.add_raw_templates(vec![("signin", include_str!("layouts/signin.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Ok(Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("signin", &context).unwrap()).unwrap())
}

async fn signup()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signup", include_str!("layouts/signup.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("signup", &Context::new()).unwrap()).unwrap()
}
async fn signup_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signup", include_str!("layouts/signup.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("signup", &Context::new()).unwrap()).unwrap()
}

async fn confirm_email()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email", include_str!("layouts/confirm_email.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_email", &Context::new()).unwrap()).unwrap()
}
async fn confirm_email_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email", include_str!("layouts/confirm_email.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_email", &Context::new()).unwrap()).unwrap()
}

async fn confirm_email_verify()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email_verify", include_str!("layouts/confirm_email_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_email_verify", &Context::new()).unwrap()).unwrap()
}
async fn confirm_email_verify_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email_verify", include_str!("layouts/confirm_email_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_email_verify", &Context::new()).unwrap()).unwrap()
}

async fn confirm_phone()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone", include_str!("layouts/confirm_phone.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_phone", &Context::new()).unwrap()).unwrap()
}
async fn confirm_phone_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone", include_str!("layouts/confirm_phone.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_phone", &Context::new()).unwrap()).unwrap()
}

async fn confirm_phone_verify()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone_verify", include_str!("layouts/confirm_phone_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_phone_verify", &Context::new()).unwrap()).unwrap()
}
async fn confirm_phone_verify_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone_verify", include_str!("layouts/confirm_phone_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_phone_verify", &Context::new()).unwrap()).unwrap()
}

async fn confirm_id()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id", include_str!("layouts/confirm_id.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_id", &Context::new()).unwrap()).unwrap()
}
async fn confirm_id_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id", include_str!("layouts/confirm_id.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_id", &Context::new()).unwrap()).unwrap()
}

async fn confirm_id_verify()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id_verify", include_str!("layouts/confirm_id_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_id_verify", &Context::new()).unwrap()).unwrap()
}
async fn confirm_id_verify_form()-> axum::response::Response<String> {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id_verify", include_str!("layouts/confirm_id_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(tera.render("confirm_id_verify", &Context::new()).unwrap()).unwrap()
}
