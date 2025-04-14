use crate::color::{Color, NewColor};

pub struct ColorsInMemory {
    colors: Vec<Color>,
}

impl ColorsInMemory {
    pub fn from_vec(colors: Vec<Color>) -> ColorsInMemory {
        ColorsInMemory { colors }
    }

    pub fn get_colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    pub fn add_color(&mut self, new_color: NewColor) {
        let new_color_id = self.colors.iter().map(|color| color.id).max().unwrap_or(0) + 1;
        let color: Color = (new_color, new_color_id).into();
        self.colors.push(color);
    }

    pub fn delete_color(&mut self, color_id_to_delete: usize) {
        let mut color_index: Option<usize> = None;
        for (i, color) in self.colors.iter().enumerate() {
            if (*color).id == color_id_to_delete {
                color_index = Some(i);
            }
        }

        if let Some(color_index) = color_index {
            self.colors.remove(color_index);
        }
    }
}
