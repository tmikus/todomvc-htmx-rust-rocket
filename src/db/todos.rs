use crate::models;
use rusqlite::{params, Result};

pub fn create_todo(title: &str) -> Result<models::Todo> {
    let conn = open_db()?;
    let mut stmt = conn.prepare("INSERT INTO todos (completed, title) VALUES (false, ?1)")?;
    stmt.execute([title])?;
    let id = conn.last_insert_rowid() as i32;
    Ok(models::Todo {
        id,
        completed: false,
        title: title.into(),
    })
}

pub fn delete_completed_todos() -> Result<()> {
    let conn = open_db()?;
    let mut stmt = conn.prepare("DELETE FROM todos WHERE completed = true")?;
    stmt.execute([])?;
    Ok(())
}

pub fn delete_todo_by_id(id: i32) -> Result<models::Todo> {
    let todo = get_todo_by_id(id)?;
    let conn = open_db()?;
    let mut stmt = conn.prepare("DELETE FROM todos WHERE id = ?1")?;
    stmt.execute([id])?;
    Ok(todo)
}

pub fn get_todo_by_id(id: i32) -> Result<models::Todo> {
    let conn = open_db()?;
    let mut stmt = conn.prepare("SELECT id, completed, title FROM todos WHERE id = ?1")?;
    let mut rows = stmt.query([id])?;
    let row = rows.next()?.unwrap();
    Ok(models::Todo {
        id: row.get(0)?,
        completed: row.get(1)?,
        title: row.get(2)?,
    })
}

pub fn list_todos(filter: models::Filter) -> Result<Vec<models::Todo>> {
    let conn = open_db()?;
    let select_query = "SELECT id, completed, title FROM todos";
    let query = match filter {
        models::Filter::All => select_query.into(),
        models::Filter::Active => format!("{} WHERE completed = false", select_query),
        models::Filter::Completed => format!("{} WHERE completed = true", select_query),
    };
    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| {
        Ok(models::Todo {
            id: row.get(0)?,
            completed: row.get(1)?,
            title: row.get(2)?,
        })
    })?;
    let mut todos = Vec::new();
    for note in rows {
        todos.push(note?);
    }
    Ok(todos)
}

fn open_db() -> Result<rusqlite::Connection> {
    let conn = rusqlite::Connection::open("todos.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            completed BOOLEAN NOT NULL,
            title TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn toggle_all_completed() -> Result<()> {
    let conn = open_db()?;
    let mut stmt = conn.prepare("UPDATE todos as t SET completed = NOT t.completed")?;
    stmt.execute([])?;
    Ok(())
}

pub fn toggle_completed(id: i32) -> Result<models::Todo> {
    let mut todo = get_todo_by_id(id)?;
    todo.completed = !todo.completed;
    let conn = open_db()?;
    let mut stmt = conn.prepare("UPDATE todos SET completed = ?1 WHERE id = ?2")?;
    stmt.execute(params![todo.completed, todo.id])?;
    Ok(todo)
}

pub fn update_todo(id: i32, title: &str) -> Result<models::Todo> {
    let mut todo = get_todo_by_id(id)?;
    todo.title = title.into();
    let conn = open_db()?;
    let mut stmt = conn.prepare("UPDATE todos SET title = ?1 WHERE id = ?2")?;
    stmt.execute(params![todo.title, todo.id])?;
    Ok(todo)
}
