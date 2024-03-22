extern crate nannou;
extern crate nannou_egui;
extern crate rand;

use nannou::prelude::pt2;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

use maze::SmartGrid;
use maze::{Direction, MazeCell};
use maze_makers::{binary_tree, sidewinder};

mod constants;
mod maze;
mod maze_makers;
mod sidewinder_hardcoded;

struct Settings {
    generate: bool,
    save: bool,
    algo: Algos,
    height: f64,
    width: f64,
    density: f32,
}

struct Point {
    pub x: f32,
    pub y: f32,
}
#[derive(PartialEq, Debug)]
enum Algos {
    BinaryTree,
    Sidewinder,
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

fn prepare_grid(columns: usize, rows: usize) -> SmartGrid {
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
    let settings = Settings {
        generate: false,
        save: false,
        algo: Algos::BinaryTree,
        height: 4.0,
        width: 4.0,
        density: cell_size,
    };

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

    let grid = prepare_grid(columns, rows);
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
    let Model {
        ref mut egui,
        ref mut settings,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Maze Maker").show(&ctx, |ui| {
        settings.generate = ui.button("Generate!").clicked();
        settings.save = ui.button("Save my maze!").clicked();

        ui.separator();

        ui.label("Height:");
        ui.add(egui::Slider::new(&mut settings.height, 2.0..=20.0));

        ui.label("Width:");
        ui.add(egui::Slider::new(&mut settings.width, 2.0..=20.0));

        ui.label("Corridor size");
        ui.add(egui::Slider::new(&mut settings.density, 0.1..=100.0));

        ui.horizontal(|ui| {
            ui.radio_value(&mut settings.algo, Algos::BinaryTree, "Binary tree");
            ui.radio_value(&mut settings.algo, Algos::Sidewinder, "Sidewinder");
        });
    });

    if settings.generate {
        model.cell_size = settings.density;
        let rows = settings.height as usize;
        let columns = settings.width as usize;
        let base_grid = prepare_grid(columns, rows);

        let x = -(columns as f32 / 2.0) * model.cell_size;
        let y = (rows as f32 / 2.0) * model.cell_size;
        model.origin = Point { x, y };

        model.maze = generate_maze(base_grid, &settings.algo)
    }
}
fn generate_maze(base_grid: SmartGrid, algorithm: &Algos) -> SmartGrid {
    let selected_algorithm = match algorithm {
        Algos::BinaryTree => binary_tree,
        Algos::Sidewinder => sidewinder,
    };
    selected_algorithm(base_grid)
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
    if model.settings.save {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
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

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
