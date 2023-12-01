use crate::view_renderer::VIEW_RENDERER;
use crate::{db, models};
use rocket::response::content;
use tera::Context;

#[get("/?<filter>")]
pub fn handle_index(filter: Option<models::Filter>) -> content::RawHtml<String> {
    let todos = db::list_todos(filter.unwrap_or_default()).unwrap();
    let mut ctx = Context::new();
    ctx.insert("allCompleted", &todos.iter().all(|todo| todo.completed));
    ctx.insert("filter", &filter.unwrap_or_default());
    ctx.insert("todos", &todos);
    VIEW_RENDERER.render_html("index.tera", ctx).unwrap()
}
