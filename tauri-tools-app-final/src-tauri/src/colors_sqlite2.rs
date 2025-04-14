use crate::color::{Color, NewColor};
use rusqlite::{params, Connection, Result};

pub struct ColorsSqlite2 {
    conn: Connection,
}

impl ColorsSqlite2 {
    pub fn new(db_path: &str) -> Result<ColorsSqlite2> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS colors (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                hex_code TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(ColorsSqlite2 { conn })
    }

    pub fn get_colors(&self) -> Result<Vec<Color>> {
        let mut stmt = self.conn.prepare("SELECT id, name, hex_code FROM colors")?;
        let color_iter = stmt.query_map([], |row| {
            Ok(Color {
                id: row.get(0)?,
                name: row.get(1)?,
                hexcode: row.get(2)?,
            })
        })?;

        let mut colors = Vec::new();
        for color in color_iter {
            colors.push(color?);
        }
        
        Ok(colors)
    }

    pub fn add_color(&self, new_color: NewColor) -> Result<()> {

        // Err(rusqlite::Error::InvalidQuery)

        self.conn.execute(
            "INSERT INTO colors (name, hex_code) VALUES (?1, ?2)",
            params![new_color.name, new_color.hexcode],
        )?;
        Ok(())
    }

    pub fn delete_color(&self, color_id_to_delete: usize) -> Result<()> {
        self.conn.execute(
            "DELETE FROM colors WHERE id = ?1",
            params![color_id_to_delete],
        )?;
        Ok(())
    }
}