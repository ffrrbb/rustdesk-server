use rusqlite::{params, Connection, Result};
use std::env;

/// Obtiene la URL de la base de datos desde una variable de entorno y abre una conexión.
///
/// # Returns
///
/// `Result<Connection>` - Un `Result` que contiene la conexión o un error.
///
/// # Example
///
/// ```
/// let conn = get_db_connection()?;
/// ```
fn get_db_connection() -> Result<Connection> {
    // Lee la URL de la base de datos desde la variable de entorno DB_URL
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

    log::info!("DB_URL={}", db_url);

    // Abre y retorna la conexión a la base de datos
    Connection::open(db_url)
}

/// Actualiza el estado del cliente en la base de datos.
///
/// # Arguments
///
/// * `id` - El ID del cliente cuyo estado debe ser actualizado.
/// * `status` - El nuevo estado del cliente, representado como `Option<i64>`.
///
/// # Returns
///
/// `Result<()>` - Un `Result` que indica el éxito o el fallo de la operación.
///
/// # Example
///
/// ```
/// let conn = get_db_connection()?;
/// update_cliente_status(&conn, 1, Some(42))?;
/// ```
pub fn update_cliente_status(id: i64, status: Option<i64>) -> Result<()> {
    // Obtiene la conexión a la base de datos
    let conn = get_db_connection()?;

    // Consulta SQL para actualizar el campo status
    let sql = "UPDATE clientes SET status = ?1 WHERE id = ?2";
    
    // Ejecuta la consulta SQL con los parámetros proporcionados
    conn.execute(sql, params![status, id])?;
    
    Ok(())
}
