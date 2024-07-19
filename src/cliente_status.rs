use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Error as SqlxError, SqliteConnection};
use std::env;
use async_trait::async_trait;

/// Obtiene la URL de la base de datos desde una variable de entorno y abre una conexión.
pub async fn get_db_connection() -> Result<SqliteConnection, SqlxError> {
    let db_url = env::var("DB_URL").unwrap_or_else(|_| {
        let default_db = "db_v2.sqlite3".to_owned();
        #[cfg(all(windows, not(debug_assertions)))]
        {
            if let Some(path) = hbb_common::config::Config::icon_path().parent() {
                return format!("{}\\{}", path.to_str().unwrap_or("."), default_db);
            }
        }
        #[cfg(not(windows))]
        {
            return format!("./{}", default_db);
        }
        default_db
    });

    // Eliminado el uso del logging
    // info!("DB_URL={}", db_url);

    let options = SqliteConnectOptions::new().filename(db_url);
    SqliteConnection::connect_with(&options).await
}

/// Actualiza el estado del cliente en la base de datos.
pub async fn update_cliente_status(id: i64, status: Option<i64>) -> Result<(), SqlxError> {
    let mut conn = get_db_connection().await?;
    let sql = "UPDATE clientes SET status = ?1 WHERE id = ?2";
    
    sqlx::query(sql)
        .bind(status)
        .bind(id)
        .execute(&mut conn)
        .await?;
    
    Ok(())
}
