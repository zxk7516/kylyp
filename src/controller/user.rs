use diesel;
use diesel::prelude::*;
use rocket_contrib::Template;
use rocket::request::{self,Form, FlashMessage,FromRequest,Request};
use rocket::response::{Redirect,Flash};
use model::db::establish_connection;
use model::user::{User, NewUser};
use rocket::http::{Cookie, Cookies};
use rocket::http::RawStr;
// use std::fmt::Debug;
use std::collections::HashMap;
use rocket::outcome::IntoOutcome;


#[derive(Debug)]
pub struct UserOr(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for UserOr {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserOr, ()> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserOr(id))
            .or_forward(())
    }
}

#[derive(FromForm)]
struct UserRegister {
    username: String,
    password: String,
    password2: String,
}

#[derive(FromForm,Debug)]
struct UserLogin {
    username: String,
    password: String,
}

#[get("/<name>",rank = 3)]
pub fn user_page(name: &RawStr,flash: Option<FlashMessage>) -> Template {
   let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg().to_string());
    }
    // println!("=====name login =========",);
    Template::render("login", &context)
}

#[get("/<name>")]
pub fn user_page_login(name: &RawStr,user: UserOr) -> Template {
    if name == &user.0 {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    context.insert("username", user.0);
    // println!("=====login   user=========",);
    Template::render("user", &context)
    }else{
        let mut context = HashMap::new();
        context.insert("title", "Forum".to_string());
        Template::render("login", &context)
    }
}

#[get("/register", rank = 2)]
pub fn register(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg().to_string());
    }
    // println!("=====register =========",);
    Template::render("register", &context)
}

#[get("/register")]
pub fn login_register(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    context.insert("username", user.0);
    // println!("=====login   register=========",);
    Template::render("index", &context)
}

#[get("/login", rank = 2)]
pub fn login(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg().to_string());
    }
    // println!("=====login =========",);
    Template::render("login", &context)
}

#[get("/login")]
pub fn login_user(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    context.insert("username", user.0);
    // println!("=====login   user=========",);
    Template::render("index", &context)
}

#[post("/register",data = "<user_form>")]
fn register_post(user_form: Form< UserRegister>) -> Result<Redirect, String> {
    let post_user = user_form.get();
    use utils::schema::users;
    if &post_user.password == &post_user.password2 {
        if true {
            let connection = establish_connection();
            let new_user = NewUser {
                username: &post_user.username,
                password: &post_user.password,
            };
            diesel::insert(&new_user).into(users::table).execute(&connection).expect("Error saving new user");
            Ok(Redirect::to("/user/login"))
        } else {
                Err("Something Wrong!".to_string())
        }
    }else {
        Err("password != password2".to_string())
    }
}

#[post("/login", data = "<user_form>")]
fn login_post(mut cookies: Cookies, user_form: Form<UserLogin>) -> Flash<Redirect> {
    use utils::schema::users::dsl::*;
    let post_user = user_form.get();
    let connection = establish_connection();
    let user_result =  users.filter(&username.eq(&post_user.username)).load::<User>(&connection);
    let login_user = match user_result {
        Ok(user_s) => match user_s.first() {
            Some(a_user) => Some(a_user.clone()),
            None => None
        },
        Err(_) => None
    };
    match login_user {
        Some(login_user) => {
            if &post_user.password == &login_user.password {
                cookies.add_private(Cookie::new("username",post_user.username.to_string() ));
                Flash::success(Redirect::to("/"), "Successfully logged in")
            }else {
                Flash::error(Redirect::to("/user/login"), "Incorrect Password")
            }
        },
        None => Flash::error(Redirect::to("/user/login"), "Incorrect Username"),
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("username"));
    Flash::success(Redirect::to("/user/login"), "Successfully logged out.")
}