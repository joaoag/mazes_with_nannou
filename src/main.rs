extern crate nannou;
extern crate nannou_egui;
extern crate rand;

use nannou::prelude::pt2;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

use maze::{SmartGrid};
use maze::{Direction, MazeCell};
use maze_makers::{binary_tree, sidewinder};

mod maze;
mod maze_makers;
mod constants;
mod sidewinder_hardcoded;

struct Settings {
    generate: bool
}

struct Point {
    pub x: f32,
    pub y: f32,
}
struct Model {
    pub settings: Settings,
    pub egui: Egui,
    pub maze: SmartGrid,
    pub origin: Point,
    pub cell_size: f32,
}


fn main() {
     nannou::app(model).update(update).run();
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

fn model(app: &App) -> Model {
    let cell_size: f32 = 50.0;
    let columns = 4;
    let rows = 4;
    let settings = Settings {generate: false};

    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    let x = -(columns as f32 / 2.0) * cell_size;
    let y = (rows as f32 / 2.0) * cell_size;
    let origin = Point { x, y };

    let grid= prepare_grid(columns, rows);
    let maze = binary_tree(grid);
    
    Model {
        settings,
        egui,
        maze,
        origin,
        cell_size,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    settings.generate = false;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Maze Maker").show(&ctx, | ui| {

        let generate_selected = ui.button("Make me a maze!").clicked();
        if generate_selected {
            settings.generate = true;
        }
    });
    if settings.generate {
        let grid= prepare_grid(model.maze.columns, model.maze.rows);
        model.maze = binary_tree(grid);

    }
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    draw_maze(&model, &draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_maze(model: &&Model, draw: &Draw) {
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
}


