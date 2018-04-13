#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate glob;
extern crate gtmpl;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

mod json_value;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use glob::glob;

use gtmpl::{Context, Template};

use rocket_contrib::Json;

use rocket::State;
use rocket::response::content;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;

use json_value::{read_config, extend};

struct Ctx {
    pub ctx: Mutex<HashMap<String, gtmpl::Value>>
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

#[get("/")]
fn index(state: State<(Template)>) -> content::Html<String> {
    let static_config = read_config(Path::new("static.json")).unwrap();
    let index_config = read_config(Path::new("index.json")).unwrap();
    let config = extend(&static_config, &index_config);
    println!("{}", config);
    content::Html(state.render(&Context::from(config).unwrap()).unwrap())
}

#[post("/", format = "application/json", data = "<msg>")]
fn update(msg: Json, state: State<(Ctx)>) -> rocket_contrib::Json<rocket_contrib::Value> {
    state.ctx.lock().unwrap().insert("foo".to_owned(), json_value::json_to_value(msg.into_inner()));
    rocket_contrib::Json(json!("foo"))

}
#[get("/<page>")]
fn preview(page: String, state: State<Template>, ctx: State<Ctx>) -> content::Html<String> {
    let cfg = ctx.ctx.lock().unwrap().get(&page).unwrap().clone();
    let index_config = read_config(Path::new("index.json")).unwrap();
    let config = extend(&cfg, &index_config);
    println!("{}", config);
    content::Html(state.render(&Context::from(config).unwrap()).unwrap())
}

#[get("/<file..>")]
fn edit(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("edit/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}


fn main() {
    let mut tmpl = Template::default();
    load_components(&mut tmpl).expect("DOOM");

    let ctx = Ctx { ctx: Mutex::new(HashMap::new()) };

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static/", routes![files])
        .mount("/edit/", routes![edit])
        .mount("/preview/static", routes![files])
        .mount("/preview/", routes![preview])
        .mount("/update/", routes![update])
        .manage(tmpl)
        .manage(ctx)
        .launch();
}

fn add_template<T: Into<String>>(
    tmpl: &mut Template,
    file: &Path,
    name: Option<T>,
) -> Result<(), String> {
    let name = name.map(|s| s.into())
        .unwrap_or_else(|| file.file_name().unwrap().to_str().unwrap().to_owned());
    let mut f = File::open(file).map_err(|e| format!("file not found: {}", e))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    tmpl.add_template(name, contents)?;
    Ok(())
}

fn load_components(tmpl: &mut Template) -> Result<(), String> {
    add_template(tmpl, &Path::new("index.html"), Some(""))?;
    for component in glob("templates/*.html").unwrap() {
        let path = component.unwrap();
        add_template(tmpl, &path, None::<String>)?;
    }
    Ok(())
}
