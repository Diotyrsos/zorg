
use super::schema::connections;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = connections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Connection {
    pub id: Option<i32>,
    pub name: String,
    pub username: String,
    pub hostname: String,
    pub port: Option<i32>,
    pub identity_file: Option<String>,
    pub note: Option<String>,
    pub created_at: i32,
    pub modified_at: Option<i32>,
    pub is_favorite: bool,
}

#[derive(Insertable)]
#[diesel(table_name = connections)]
pub struct NewConnection<'a> {
    pub name: &'a str,
    pub username: &'a str,
    pub hostname: &'a str,
    pub port: Option<i32>,
    pub identity_file: Option<&'a str>,
    pub note: Option<&'a str>,
    pub created_at: i32,
    pub modified_at: Option<i32>,
    pub is_favorite: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = connections)]
pub struct UpdateConnection<'a> {
    pub name: Option<&'a str>,
    pub username: Option<&'a str>,
    pub hostname: Option<&'a str>,
    pub port: Option<i32>,
    pub identity_file: Option<&'a str>,
    pub note: Option<&'a str>,
    pub modified_at: Option<i32>,
    pub is_favorite: Option<bool>,
}

impl<'a> NewConnection<'a> {
    pub fn create(
        db_connection: &mut SqliteConnection,
        name: &str,
        username: &str,
        hostname: &str,
        port: Option<i32>,
        identity_file: Option<&str>,
        note: Option<&str>,
    ) -> QueryResult<Connection> {
        let current_time = Utc::now().timestamp() as i32;

        let new_connection = NewConnection {
            name,
            username,
            hostname,
            port,
            identity_file,
            note,
            created_at: current_time,
            modified_at: None,
            is_favorite: false,
        };

        diesel::insert_into(connections::table)
            .values(&new_connection)
            .execute(db_connection)?;

        connections::table
            .order(connections::id.desc())
            .first(db_connection)
    }
}

impl<'a> UpdateConnection<'a> {
    pub fn update(
        db_connection: &mut SqliteConnection,
        conn_id: i32,
        name: &str,
        username: &str,
        hostname: &str,
        port: Option<i32>,
        identity_file: Option<&str>,
        note: Option<&str>,
    ) -> QueryResult<Connection> {
        let current_time = Utc::now().timestamp() as i32;

        let updated_connection = UpdateConnection {
            name: Some(name),
            username: Some(username),
            hostname: Some(hostname),
            port,
            identity_file,
            note,
            modified_at: Some(current_time),
            is_favorite: None,
        };

        diesel::update(connections::table.filter(connections::id.eq(conn_id)))
            .set(&updated_connection)
            .execute(db_connection)?;

        connections::table
            .filter(connections::id.eq(conn_id))
            .first(db_connection)
    }
}

impl Connection {
    pub fn get_all(db_connection: &mut SqliteConnection) -> QueryResult<Vec<Connection>> {
        connections::table
            .select(Connection::as_select())
            .load(db_connection)
    }

    pub fn toggle_favorite(db_connection: &mut SqliteConnection, conn_id: i32) -> QueryResult<Connection> {
        let current_conn: Connection = connections::table
            .filter(connections::id.eq(conn_id))
            .first(db_connection)?;
        
        let new_status = !current_conn.is_favorite;
        
        diesel::update(connections::table.filter(connections::id.eq(conn_id)))
            .set(connections::is_favorite.eq(new_status))
            .execute(db_connection)?;
            
        connections::table
            .filter(connections::id.eq(conn_id))
            .first(db_connection)
    }
}
