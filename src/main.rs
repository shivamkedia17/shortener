use rocket::fs::{FileServer, NamedFile};
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[catch(404)]
async fn not_found() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./site/404.html").await
}

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/shorts")]
async fn shorts() -> Template {
    Template::render("shorts", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from("./site"))
        .register("/", catchers![not_found])
}
