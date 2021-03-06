#![feature(plugin, decl_macro, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate chrono;

mod static_files;
mod task;
mod db;


use rocket::Rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use task::{Blog, Todo};

#[derive(Debug, Serialize)]
//Serialize: データベースに配列データをそのまま保存したい時などに使う
//アトリビュートについて：　アトリビュートはstructだけに適用されるものではない。
// 例えば、ある関数の上に#[test]というアトリビュートが付くと、その関数はテストを実行した際に自動的に実行されるテスト関数になる。
struct Context<'a, 'b>{
    msg: Option<(&'a str, &'b str)>,
    tasks: Vec<Blog>
}
//'aでライフタイムを無視する。ざっくり言うとどこからでも引数が受け取れる


impl<'a, 'b> Context<'a, 'b> {
    //struct Contextに対するimpl
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), tasks: Blog::all(conn)}
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, tasks: Blog::all(conn)}
    }
}

#[post("/", data = "<todo_form>")]
fn new(todo_form: Form<Todo>, conn: db::Conn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.body.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
        //何も書かずにpostした時のエラー文
    } else if Blog::insert(todo, &conn) {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}


#[put("/<id>")]
fn toggle(id: i32, conn: db::Conn) -> Result<Redirect, Template> {
    if Blog::toggle_with_id(id, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("index.tera", &Context::err(&conn, "Couldn't toggle task.")))
    }
}


#[delete("/<id>")]
fn delete(id: i32, conn: db::Conn) -> Result<Flash<Redirect>, Template> {
    if Blog::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Todo was deleted."))
    } else {
        Err(Template::render("index.tera", &Context::err(&conn, "Couldn't delete task.")))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
}



fn rocket() -> (Rocket, Option<db::Conn>) {
    let pool = db::init_pool();
    let conn = if cfg!(test) {
        Some(db::Conn(pool.get().expect("database connection for testing")))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, static_files::all])
        //.mount("/", routes![index.tera])
        .mount("/todo/", routes![new, toggle, delete])
        .attach(Template::fairing());

    (rocket, conn)
    //返り値のタプル！
}

fn main() {
    rocket().0.launch();
}
