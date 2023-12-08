extern crate nannou;
extern crate rand;

use std::env;
use nannou::prelude::pt2;
use nannou::prelude::*;

use maze::{SmartGrid, cli_display};
use maze::{Direction, MazeCell};
use maze_makers::{binary_tree, sidewinder};
use constants::*;
use sidewinder_hardcoded::static_sidewinder;

mod maze;
mod maze_makers;
mod constants;
mod sidewinder_hardcoded;

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

    let args = parse_cli_args(env::args().collect());

    // TODO use a cli args library because this implementation is horrible
    if args[1] == NANNOU {
        nannou::app(model).event(event).simple_window(view).run();
    } else if args[1] == ASCII {
        cli_display(&static_sidewinder());
    }
}

fn parse_cli_args(args: Vec<String>) -> Vec<String> {
    let default_algo = &String::from(BINARY_TREE);
    let default_mode = &String::from(NANNOU);
    let algo = args.get(1).unwrap_or(default_algo);
    let mode = args.get(2).unwrap_or(default_mode);

    vec![algo.to_string(), mode.to_string()]
}


fn get_maze_algorithm(algorithm_arg: &str) -> fn(SmartGrid) -> SmartGrid {
    match algorithm_arg {
        BINARY_TREE => binary_tree ,
        SIDEWINDER => sidewinder,
        _ => panic!("Unrecognised algorithm"),
    }
}

fn prepare_grid(columns: usize, rows: usize)-> SmartGrid {

    let mut grid = SmartGrid {
        rows,
        columns,
        cells: Vec::new(),
    };
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    grid
}

fn model(_app: &App) -> Model {
    let cell_size: f32 = 50.0;
    let columns = 4;
    let rows = 4;
    let args = parse_cli_args(env::args().collect());
    // let (first, second, third) = if let [first, second, third] = elements[0..2] { (first, second, third) } else { todo!() };
    let mut grid;

    if args[0] == STATIC_SIDEWINDER {
        grid = static_sidewinder();
    } else {
        let validated_algorithm = get_maze_algorithm(&args[0]);
        grid= prepare_grid(columns, rows);
        grid = validated_algorithm(grid);
    }

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

            // println!(
            //     "For cell {:?},\nCell x y origin [{}, {}] \n",
            //     &cell.location,
            //     current_x_origin,
            //     current_y_origin
            // );

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
