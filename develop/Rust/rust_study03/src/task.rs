use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

mod schema {
    table! {
        tasks {
            id -> Nullable<Integer>,
            title -> VarChar,
            published -> Timestamp,
            body -> Text,
            completed -> Bool,
        }
    }
}

use self::schema::tasks;
use self::schema::tasks::dsl::{tasks as all_tasks, completed as task_completed};

#[table_name="blog"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Blog{
    pub id: Option<i32>,
    pub title: String,
    pub published: NaiveDateTime,
    //投稿時間
    pub body: String,
    pub completed: bool
}
//databaseのクエリ

#[derive(FromForm)]
pub struct Todo {
    pub title: String,
    pub published: NaiveDateTime,
    pub body: String,
}
//実際に投稿するメンバを記入

impl Blog{
    pub fn all(conn: &MysqlConnection) -> Vec<Blog> {
        all_tasks.order(tasks::id.desc()).load::<Blog>(conn).unwrap()
    }

    pub fn insert(todo: Todo, conn: &MysqlConnection) -> bool {
        let t = Blog { id: None, title: todo.title, published: todo.published, body: todo.body, completed: false};
        //メモ：completedをfalseにすると文字に横線が引かれる。
        diesel::insert_into(blog::table).values(&t).execute(conn).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &MysqlConnection) -> bool {
        let blog = all_tasks.find(id).get_result::<Blog>(conn);
        if blog.is_err() {
            return false;
        }

        let new_status = !task.unwrap().completed;
        let updated_task = diesel::update(all_tasks.find(id));
        updated_task.set(task_completed.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }
}
