use tokio;
use axum::{extract::Form, routing::get, response::{Response, IntoResponse},Router};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use mongodb::{bson::doc,Client};

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
	.into_make_service()).await.unwrap()
}
async fn index()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("index", include_str!("layouts/index.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("index", &Context::new()).unwrap()).unwrap()
}

async fn signin()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signin", include_str!("layouts/signin.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("signin", &Context::new()).unwrap()).unwrap()
}



#[derive(Deserialize, Serialize)]
struct Sign {
	r#fn: Option<String>,
	ln: Option<String>,
	un: Option<String>,
	em: Option<String>,
	pw: Option<String>,
	rp: Option<String>,
	rpw: Option<String>,
	status: Option<String>,
	otpem: Option<String>,
	otpemurl: Option<String>,
	ac: Option<String>
}
async fn signin_form(Form(sign): Form<Sign>)-> impl IntoResponse {
	let db = Client::with_uri_str("mongodb+srv://mbra:mbra@cluster0.um0c2p7.mongodb.net/?retryWrites=true&w=majority").await.unwrap().database("braq").collection::<Sign>("users");
	let mut context = Context::new();
	match db.find_one(doc!{"un":&sign.ac},None).await.unwrap() {
		Some(u) => context.insert("ac",&u.em),
		None => context.insert("ac","signed not")
	}
	//db.insert_one(doc!{"un":login.ac},None).await.map_err(|_| "read file error")?;
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signin", include_str!("layouts/signin.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("signin", &context).unwrap()).unwrap()
}


async fn signup()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signup", include_str!("layouts/signup.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("signup", &Context::new()).unwrap()).unwrap()
}
#[derive(Deserialize, Serialize)]
struct Signup {
	r#fn: Option<String>,
	ln: Option<String>,
	un: Option<String>,
	em: Option<String>,
	pw: Option<String>,
	rp: Option<String>,
	rpw: Option<String>,
	status: Option<String>,
	otpem: Option<String>,
	otpemurl: Option<String>,
	ac: Option<String>
}
async fn signup_form(Form(signup): Form<Signup>)-> impl IntoResponse {
	let db = Client::with_uri_str("mongodb+srv://mbra:mbra@cluster0.um0c2p7.mongodb.net/?retryWrites=true&w=majority").await.unwrap().database("braq").collection::<Signup>::insert_one("users");
	let mut context = Context::new();
	if signup.r#fn.is_none(){
		context.insert("fn","يجب كتابة الإسم الأول")
	}
	if signup.ln.is_none(){
		context.insert("ln","يجب كتابة الإسم الأخير")
	}
	if signup.un.is_none(){
		context.insert("un","يجب كتابة إسم المستخدم")
	}
	if signup.em.is_none(){
		context.insert("em","يجب كتابة البريد الإلكتروني")
	}
	if signup.pw.is_none(){
		context.insert("pw","يجب كتابة كلمة المرور")
	}
	if signup.rp.is_none(){
		context.insert("rp","يجب إعادة كتابة كلمة المرور")
	}
	if signup.pw.is_none() != signup.rp.is_none() {
		context.insert("rpw","يجب كتابة كلمة المرور مرتين بشكل متطابق")
	}
	
	db.insert_one(doc!{"fn":signup.r#fn,"ln":signup.ln,"un":signup.un,"em":signup.em,"pw":signup.pw},None).await.unwrap();
	//match db.find_one(doc!{"un":&sign.ac},None).await.unwrap() {
		//Some(u) => context.insert("ac","signed it"),
		//None => context.insert("ac","signed not")
	//}
	
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("signup", include_str!("layouts/signup.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("signup", &Context::new()).unwrap()).unwrap()
}

async fn confirm_email()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email", include_str!("layouts/confirm_email.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_email", &Context::new()).unwrap()).unwrap()
}
async fn confirm_email_form()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email", include_str!("layouts/confirm_email.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_email", &Context::new()).unwrap()).unwrap()
}

async fn confirm_email_verify()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email_verify", include_str!("layouts/confirm_email_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_email_verify", &Context::new()).unwrap()).unwrap()
}
async fn confirm_email_verify_form()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_email_verify", include_str!("layouts/confirm_email_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_email_verify", &Context::new()).unwrap()).unwrap()
}

async fn confirm_phone()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone", include_str!("layouts/confirm_phone.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_phone", &Context::new()).unwrap()).unwrap()
}
async fn confirm_phone_form()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone", include_str!("layouts/confirm_phone.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_phone", &Context::new()).unwrap()).unwrap()
}

async fn confirm_phone_verify()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone_verify", include_str!("layouts/confirm_phone_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_phone_verify", &Context::new()).unwrap()).unwrap()
}
async fn confirm_phone_verify_form()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_phone_verify", include_str!("layouts/confirm_phone_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_phone_verify", &Context::new()).unwrap()).unwrap()
}

async fn confirm_id()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id", include_str!("layouts/confirm_id.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_id", &Context::new()).unwrap()).unwrap()
}
async fn confirm_id_form()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id", include_str!("layouts/confirm_id.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_id", &Context::new()).unwrap()).unwrap()
}

async fn confirm_id_verify()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id_verify", include_str!("layouts/confirm_id_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_id_verify", &Context::new()).unwrap()).unwrap()
}
async fn confirm_id_verify_form()-> impl IntoResponse {
	let mut tera = Tera::default();
	tera.add_raw_templates(vec![("confirm_id_verify", include_str!("layouts/confirm_id_verify.html")),("header", include_str!("layouts/partials/header.html")),("footer", include_str!("layouts/partials/footer.html"))]).unwrap();
	Response::builder().status(axum::http::StatusCode::OK)
		.header("Content-Type", "text/html; charset=utf-8")
		.body(tera.render("confirm_id_verify", &Context::new()).unwrap()).unwrap()
}
