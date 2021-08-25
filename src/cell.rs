use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    fill: Color,
}

impl Cell {
    pub(crate) fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Cell::new_with_fill(x, y, width, height, (255, 255, 255))
    }

    pub(crate) fn new_with_fill(x: u32, y: u32, width: u32, height: u32, rgb: (u8, u8, u8)) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill: Color::new(rgb.0, rgb.1, rgb.2),
        }
    }

    pub(crate) fn get_fill(&self) -> (u8, u8, u8) {
        self.fill.get_rgb()
    }

    pub(crate) fn set_fill(&mut self, rgb: (u8, u8, u8)) {
        self.fill.set_rgb(rgb);
    }

    pub(crate) fn split_horizontally(&self, split_y: u32) -> Option<(Cell, Cell)> {
        if split_y < self.y || split_y > self.y + self.height {
            return None;
        }
        let top = Cell::new(self.x, self.y, self.width, split_y - self.y);
        let bottom = Cell::new(self.x, split_y, self.width, self.height - top.height);
        Some((top, bottom))
    }

    pub(crate) fn split_vertically(&self, split_x: u32) -> Option<(Cell, Cell)> {
        if split_x < self.x || split_x > self.x + self.width {
            return None;
        }
        let left = Cell::new(self.x, self.y, split_x - self.x, self.height);
        let right = Cell::new(split_x, self.y, self.width - left.width, self.height);
        Some((left, right))
    }

    pub(crate) fn split_four_ways(&self, split_x: u32, split_y: u32) -> Option<((Cell, Cell), (Cell, Cell))> {
        if let Some((top, bottom)) = self.split_horizontally(split_y) {
            return top.split_vertically(split_x)
                .and_then(|(tl, tr)| {
                    bottom.split_vertically(split_x).and_then(|(bl, br)| {
                        Some(((tl, tr), (bl, br)))
                    })
                });
        }
        None
    }
}
