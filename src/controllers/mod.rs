use rocket::Route;

mod home;
mod todos;

pub fn get_routes() -> Vec<Route> {
    routes![
        home::handle_index,
        todos::clear_completed,
        todos::create_todo,
        todos::delete_todo,
        todos::get_todo_editor,
        todos::toggle_all_completed,
        todos::toggle_completed,
        todos::update_todo,
    ]
}
