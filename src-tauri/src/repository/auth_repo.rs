use rusqlite::{params, Connection};

use crate::models::{AuthProvider, AuthSession};

pub struct AuthRepository;

impl AuthRepository {
    pub fn get_session(conn: &Connection) -> Result<Option<AuthSession>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT user_id, access_token, refresh_token, expires_at, provider FROM auth_session WHERE id = 1",
        )?;

        let mut rows = stmt.query_map([], |row| {
            let provider_str: String = row.get(4)?;
            Ok(AuthSession {
                user_id: row.get(0)?,
                access_token: row.get(1)?,
                refresh_token: row.get(2)?,
                expires_at: row.get(3)?,
                provider: AuthProvider::from_str(&provider_str),
            })
        })?;

        if let Some(session) = rows.next() {
            Ok(Some(session?))
        } else {
            Ok(None)
        }
    }

    pub fn save_session(conn: &Connection, session: &AuthSession) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT OR REPLACE INTO auth_session (id, user_id, access_token, refresh_token, expires_at, provider) VALUES (1, ?1, ?2, ?3, ?4, ?5)",
            params![
                session.user_id,
                session.access_token,
                session.refresh_token,
                session.expires_at,
                session.provider.to_str()
            ],
        )?;
        Ok(())
    }

    pub fn delete_session(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM auth_session WHERE id = 1", [])?;
        Ok(())
    }

}
