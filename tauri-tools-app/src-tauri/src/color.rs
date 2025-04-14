use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub id: usize,
    pub name: String,
    pub hexcode: String,
}

impl Color {
    pub fn new(id: usize, name: &str, hex_code: &str) -> Color {
        Color {
            id,
            name: name.to_string(),
            hexcode: hex_code.to_string(),
        }
    }
}

impl From<(NewColor, usize)> for Color {
    fn from((new_color, id): (NewColor, usize)) -> Self {
        Color::new(id, &new_color.name, &new_color.hexcode)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewColor {
    pub name: String,
    pub hexcode: String,
}
