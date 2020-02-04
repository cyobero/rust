#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

use rocket::response::Redirect;
use rocket::Request;
use rocket_contrib::templates::{handlebars, Template};

use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/hello/Unknown")
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Hello",
            name: Some(name),
            items: vec!["una manzana", "dos bananas", "tres uvos"],
            parent: "layout",
        },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render(
        "about",
        &TemplateContext {
            title: "About",
            name: None,
            items: vec!["quatre", "cinq", "six"],
            parent: "layout",
        },
    )
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("<b></i>")?;
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, hello, about])
        .register(catchers![not_found])
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("wow", Box::new(wow_helper));
        }))
}

fn main() {
    rocket().launch();
}
