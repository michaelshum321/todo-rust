use rusqlite::{params, Connection, Result, NO_PARAMS, named_params};
use crate::List;
use crate::Todo;

pub struct Db {
    conn: Connection
}

impl Db {
    pub fn new() -> Db {
        let conn = Connection::open(&"xd.db").unwrap();
        Db{
            conn
        }
    }

    pub fn make_new_table(&self) -> Result<()>{
        let conn = &self.conn;
        conn.execute("
            CREATE TABLE IF NOT EXISTS lists (
                id      INTEGER PRIMARY KEY
            )
        ", params![])?;

        conn.execute("
            CREATE TABLE IF NOT EXISTS todos (
                id          INTEGER PRIMARY KEY,
                list_id     INTEGER,
                title       TEXT,
                description TEXT,
                is_done     BOOLEAN,
                FOREIGN KEY(list_id) REFERENCES lists(id)
            )
        ", NO_PARAMS)?;

        Ok(())
    }

    pub fn set_list(&self, list: &List) -> Result<i32>{
        let conn = &self.conn;
        conn.execute("INSERT INTO lists (id) VALUES (NULL)", NO_PARAMS)?;
        // TODO: watch out for overflow!!1
        let id = conn.last_insert_rowid() as i32;

        let mut statement = conn.prepare(
            "INSERT INTO todos (
                list_id, 
                title, 
                description, 
                is_done
            ) VALUES (
                :list_id, 
                :title, 
                :desc, 
                :is_done
            )"
        )?;

        for todo in &list.todos {
            statement.execute_named(
                named_params!{
                    ":list_id": id,
                    ":title": todo.title,
                    ":desc": todo.description,
                    ":is_done": todo.is_done
                }
            )?;
        }

        Ok(id)
    }

    pub fn get_list(&self, list_id: i32) -> List {
        let mut statement = self.conn.prepare("SELECT * FROM todos WHERE list_id=?1").unwrap();

        let todos = statement.query_map(
            params![list_id],
            |row| {
                Ok(Todo {
                    is_done: row.get(4)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                })
            }).unwrap();

        let mut list = List::new();
        for todo in todos {
            list.add_todo(todo.unwrap());
        }
        return list;
    }
}