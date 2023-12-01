use crate::view_renderer::VIEW_RENDERER;
use crate::{db, models};
use rocket::form::Form;
use rocket::response::content;

#[derive(FromForm)]
pub struct CreateOrUpdateTodo {
    title: String,
}

#[post("/todos/clear-completed?<filter>")]
pub fn clear_completed(filter: Option<models::Filter>) -> content::RawHtml<String> {
    _ = db::delete_completed_todos().unwrap();
    let todos = db::list_todos(filter.unwrap_or_default()).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("todos", &todos);
    VIEW_RENDERER.render_html("todo_list.tera", ctx).unwrap()
}

#[post("/todos", data = "<todo>")]
pub fn create_todo(todo: Form<CreateOrUpdateTodo>) -> content::RawHtml<String> {
    let todo = db::create_todo(&todo.title).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("todo", &todo);
    VIEW_RENDERER.render_html("todo_item.tera", ctx).unwrap()
}

#[delete("/todos/<id>")]
pub fn delete_todo(id: i32) -> content::RawHtml<String> {
    _ = db::delete_todo_by_id(id).unwrap();
    content::RawHtml(String::from(""))
}

#[get("/todos/edit/<id>")]
pub fn get_todo_editor(id: i32) -> content::RawHtml<String> {
    let todo = db::get_todo_by_id(id).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("todo", &todo);
    VIEW_RENDERER
        .render_html("edit_todo_item.tera", ctx)
        .unwrap()
}

#[post("/todos/toggle-all?<filter>")]
pub fn toggle_all_completed(filter: Option<models::Filter>) -> content::RawHtml<String> {
    _ = db::toggle_all_completed().unwrap();
    let todos = db::list_todos(filter.unwrap_or_default()).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("todos", &todos);
    VIEW_RENDERER.render_html("todo_list.tera", ctx).unwrap()
}

#[patch("/todos/<id>")]
pub fn toggle_completed(id: i32) -> content::RawHtml<String> {
    let todo = db::toggle_completed(id).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("todo", &todo);
    VIEW_RENDERER.render_html("todo_item.tera", ctx).unwrap()
}

#[post("/todos/update/<id>", data = "<todo>")]
pub fn update_todo(id: i32, todo: Form<CreateOrUpdateTodo>) -> content::RawHtml<String> {
    let todo = db::update_todo(id, &todo.title).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("todo", &todo);
    VIEW_RENDERER.render_html("todo_item.tera", ctx).unwrap()
}
