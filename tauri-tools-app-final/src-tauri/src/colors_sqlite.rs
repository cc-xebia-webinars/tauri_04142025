use crate::color::{Color, NewColor};
use rusqlite::{params, Connection};

pub struct ColorsSqlite {
    conn: Connection,
}

impl ColorsSqlite {
    pub fn new(db_path: &str) -> ColorsSqlite {
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS colors (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                hex_code TEXT NOT NULL
            )",
            [],
        ).unwrap();
        
        ColorsSqlite { conn }
    }

    pub fn get_colors(&self) -> Vec<Color> {
        let mut stmt = self.conn.prepare("SELECT id, name, hex_code FROM colors").unwrap();
        let color_iter = stmt.query_map([], |row| {
            Ok(Color {
                id: row.get(0)?,
                name: row.get(1)?,
                hexcode: row.get(2)?,
            })
        }).unwrap();

        let mut colors = Vec::new();
        for color in color_iter {
            colors.push(color.unwrap());
        }
        
        colors
    }

    pub fn add_color(&self, new_color: NewColor) {
        self.conn.execute(
            "INSERT INTO colors (name, hex_code) VALUES (?1, ?2)",
            params![new_color.name, new_color.hexcode],
        ).unwrap();
    }

    pub fn delete_color(&self, color_id_to_delete: usize) {
        self.conn.execute(
            "DELETE FROM colors WHERE id = ?1",
            params![color_id_to_delete],
        ).unwrap();
    }
}