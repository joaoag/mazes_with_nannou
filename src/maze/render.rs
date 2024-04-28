use nannou::color::{rgb8, Rgb8};
use nannou::Draw;
use nannou::geom::pt2;
use rand::random;
use crate::{Model };
use crate::maze::core::{Direction, MazeCell};

#[derive(Debug, Clone, Copy)]
pub struct Walls {
    pub(crate) width: f32,
    pub(crate) colours: WallColours,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WallColours {
    pub(crate) north: Rgb8,
    pub(crate) east: Rgb8,
    pub(crate) south: Rgb8,
    pub(crate) west: Rgb8,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum ColourType {
    Party,
    #[default]
    Default,
    Custom,
}
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl WallColours {
    pub fn party() -> Self {
        WallColours {
            north: rgb8(random::<u8>(), random::<u8>(), random::<u8>()),
            east: rgb8(random::<u8>(), random::<u8>(), random::<u8>()),
            south: rgb8(random::<u8>(), random::<u8>(), random::<u8>()),
            west: rgb8(random::<u8>(), random::<u8>(), random::<u8>()),
        }
    }
}
impl Default for WallColours {
    fn default() -> Self {
        WallColours {
            north: rgb8(255u8, 0u8, 0u8),
            east: rgb8(255u8, 255u8, 0u8),
            south: rgb8(0u8, 128u8, 0u8),
            west: rgb8(255u8, 165u8, 0u8),
        }
    }
}
impl Default for Walls {
    fn default() -> Self {
        Walls {
            width: 2.0,
            colours: WallColours::default(),
        }
    }
}
pub fn calculate_origin(columns: f32, rows: f32, cell_size: f32) -> Point {
    let x = -(columns / 2.0) * cell_size;
    let y = (rows / 2.0) * cell_size;
    Point { x, y }
}
pub fn draw_maze(model: &Model, draw: &Draw, colours: WallColours) {
    let is_solved = model.solved_maze.is_some();
    let line_weight = model.settings.walls.width;
    for row in &model.maze.cells {
        for cell in row.iter() {
            let cell = cell.borrow_mut();

            let x_index = cell.location.column;
            let x_origin = &model.origin.x;
            let cell_size = &model.cell_size;
            let current_x_origin = x_origin + (x_index as f32 * cell_size).floor();

            let y_index = cell.location.row;
            let y_origin = &model.origin.y;
            let size = &model.cell_size;
            let current_y_origin = y_origin - (y_index as f32 * size).floor();

            let north_west_point = pt2(current_x_origin, current_y_origin);
            let north_east_point = pt2((current_x_origin + cell_size).floor(), current_y_origin);
            let south_east_point = pt2(
                (current_x_origin + cell_size).floor(),
                (current_y_origin - cell_size).floor(),
            );
            let south_west_point = pt2(current_x_origin, (current_y_origin - cell_size).floor());

            let draw_north = cell.north.is_none();
            let draw_west = cell.west.is_none();
            let draw_east = !MazeCell::is_linked_to(&cell, Direction::East);
            let draw_south = !MazeCell::is_linked_to(&cell, Direction::South);
            let distance = cell.distance as f32;
            if is_solved {
                draw.quad()
                    .rgb8(1, 2, ((distance + 1.0) * 2.0) as u8)
                    .points(
                        north_west_point,
                        north_east_point,
                        south_east_point,
                        south_west_point,
                    );
            }

            if draw_north {
                draw.line()
                    .start(north_west_point)
                    .end(north_east_point)
                    .weight(line_weight)
                    .color(colours.north);
            }
            if draw_west {
                draw.line()
                    .start(north_west_point)
                    .end(south_west_point)
                    .weight(line_weight)
                    .color(colours.west);
            }
            if draw_east {
                draw.line()
                    .start(north_east_point)
                    .end(south_east_point)
                    .weight(line_weight)
                    .color(colours.east);
            }
            if draw_south {
                draw.line()
                    .start(south_west_point)
                    .end(south_east_point)
                    .weight(line_weight)
                    .color(colours.south);
            }
        }
    }
}