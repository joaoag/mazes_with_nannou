extern crate nannou;
extern crate rand;

use std::env;
use nannou::prelude::pt2;
use nannou::prelude::*;

use crate::maze::SmartGrid;
use maze::{Direction, MazeCell};
use maze_makers::{binary_tree, sidewinder};

mod maze;
mod maze_makers;

struct Point {
    pub x: f32,
    pub y: f32,
}
struct Model {
    pub grid: SmartGrid,
    pub origin: Point,
    pub cell_size: f32,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn get_maze_algorithm(algorithm_arg: Vec<String>) -> fn(SmartGrid) -> SmartGrid {
    let algorithm = if algorithm_arg.len() > 1 { &algorithm_arg[1] } else { "binarytree" };

    match algorithm {
        "binarytree" => binary_tree,
        "sidewinder" => sidewinder,
        _ => panic!("Unrecognised algorithm"),
    }
}

fn model(_app: &App) -> Model {
    let cell_size: f32 = 30.0;
    let columns = 20;
    let rows = 20;
    let mut grid = SmartGrid {
        rows,
        columns,
        cells: Vec::new(),
    };

    let args: Vec<String> = env::args().collect();
    let validated_algorithm = get_maze_algorithm(args);

    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    grid = validated_algorithm(grid);

    let x = -(columns as f32 / 2.0) * cell_size;
    let y = (rows as f32 / 2.0) * cell_size;
    let origin = Point { x, y };

    Model {
        grid,
        origin,
        cell_size,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();

    for row in &_model.grid.cells {
        for cell in row.iter() {
            let cell = cell.borrow_mut();

            let x_index = cell.location.column;
            let x_origin = &_model.origin.x;
            let cell_size = &_model.cell_size;
            let current_x_origin = x_origin + (x_index as f32 * cell_size).floor();

            let y_index = cell.location.row;
            let y_origin = &_model.origin.y;
            let size = &_model.cell_size;
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
            let draw_east = !MazeCell::is_linked(&cell, Direction::East);
            let draw_south = !MazeCell::is_linked(&cell, Direction::South);

            if draw_north {
                draw.line()
                    .start(north_west_point)
                    .end(north_east_point)
                    .weight(2.0)
                    .color(RED);
            }
            if draw_west {
                draw.line()
                    .start(north_west_point)
                    .end(south_west_point)
                    .weight(2.0)
                    .color(ORANGE);
            }
            if draw_east {
                draw.line()
                    .start(north_east_point)
                    .end(south_east_point)
                    .weight(2.0)
                    .color(YELLOW);
            }
            if draw_south {
                draw.line()
                    .start(south_west_point)
                    .end(south_east_point)
                    .weight(2.0)
                    .color(GREEN);
            }
        }
    }

    draw.background().color(BLACK);
    draw.to_frame(_app, &_frame).unwrap();
}
