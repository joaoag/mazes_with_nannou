extern crate nannou;
extern crate nannou_egui;
extern crate rand;

use nannou::color::named::BLACK;
use nannou::prelude::pt2;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

use maze::SmartGrid;
use maze::{Direction, MazeCell};
use maze_makers::{aldous_broder, binary_tree, hunt_and_kill, sidewinder};
use maze_solvers::dijkstra_simplified_solver;

mod maze;
mod maze_makers;
mod maze_solvers;
mod sidewinder_hardcoded;

#[derive(Debug, Clone, Copy)]
struct Settings {
    generate: bool,
    saving: bool,
    solve: bool,
    colour_type: ColourType,
    walls: Walls,
    algo: Algos,
    height: f64,
    width: f64,
    corridor_size: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            generate: false,
            saving: false,
            algo: Algos::default(),
            height: 15.0,
            width: 15.0,
            corridor_size: 30.0,
            solve: false,
            colour_type: ColourType::default(),
            walls: Walls::default(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Walls {
    width: f32,
    colours: WallColours,
}
#[derive(PartialEq, Debug, Clone, Copy)]
struct WallColours {
    north: Rgb8,
    east: Rgb8,
    south: Rgb8,
    west: Rgb8,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum ColourType {
    Party,
    #[default]
    Default,
    Custom,
}
struct Point {
    pub x: f32,
    pub y: f32,
}
type SolvedMaze = Option<SmartGrid>;
#[derive(PartialEq, Debug, Copy, Clone, Default)]
enum Algos {
    #[default]
    BinaryTree,
    Sidewinder,
    AldousBroder,
    HuntAndKill,
}
struct Model {
    pub settings: Settings,
    pub egui: Egui,
    pub maze: SmartGrid,
    pub solved_maze: SolvedMaze,
    pub origin: Point,
    pub cell_size: f32,
}
struct MazeAndMetaData {
    maze: SmartGrid,
    origin: Point,
    cell_size: f32,
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
fn calculate_origin(columns: f32, rows: f32, cell_size: f32) -> Point {
    let x = -(columns / 2.0) * cell_size;
    let y = (rows / 2.0) * cell_size;
    Point { x, y }
}
fn initial_maze() -> MazeAndMetaData {
    let cell_size: f32 = 30.0;
    let columns = 15;
    let rows = 15;
    let origin = calculate_origin(columns as f32, rows as f32, cell_size);
    let grid = prepare_grid(columns, rows);
    let maze = binary_tree(grid);
    MazeAndMetaData {
        maze,
        origin,
        cell_size,
    }
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings::default();
    let MazeAndMetaData {
        maze,
        origin,
        cell_size,
    } = initial_maze();

    Model {
        settings,
        egui,
        maze,
        solved_maze: None,
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

    egui::Window::new("Maze Maker")
        .default_width(100.0)
        .show(&ctx, |ui| {
            settings.generate = ui.button("Generate new maze").clicked();
            settings.saving = ui.button("Save my maze").clicked();
            settings.solve = ui.button("Solve!").clicked();

            ui.separator();
            ui.label("Height:");
            ui.add(egui::Slider::new(&mut settings.height, 2.0..=50.0));

            ui.label("Width:");
            ui.add(egui::Slider::new(&mut settings.width, 2.0..=50.0));

            ui.label("Corridor size");
            ui.add(egui::Slider::new(&mut settings.corridor_size, 0.1..=100.0));

            ui.label("Wall thickness");
            ui.add(egui::Slider::new(&mut settings.walls.width, 0.1..=100.0));

            ui.separator();
            ui.label("Colours");

            ui.vertical(|ui| {
                ui.radio_value(&mut settings.colour_type, ColourType::Default, "Default");
                ui.radio_value(&mut settings.colour_type, ColourType::Party, "Party (strobe warning)");
                ui.radio_value(&mut settings.colour_type, ColourType::Custom, "Custom");
            });
            if let ColourType::Custom = settings.colour_type {
                ui.label("North");
                edit_rgb(ui, &mut settings.walls.colours.north);
                ui.label("East");
                edit_rgb(ui, &mut settings.walls.colours.east);
                ui.label("South");
                edit_rgb(ui, &mut settings.walls.colours.south);
                ui.label("West");
                edit_rgb(ui, &mut settings.walls.colours.west);
            }

            ui.separator();
            ui.vertical(|ui| {
                ui.radio_value(&mut settings.algo, Algos::BinaryTree, "Binary tree");
                ui.radio_value(&mut settings.algo, Algos::Sidewinder, "Sidewinder");
                ui.radio_value(&mut settings.algo, Algos::AldousBroder, "Aldous-Broder");
                ui.radio_value(&mut settings.algo, Algos::HuntAndKill, "Hunt-and-kill");
            });
        });

    if settings.generate {
        model.cell_size = settings.corridor_size;
        let rows = settings.height as usize;
        let columns = settings.width as usize;
        let base_grid = prepare_grid(columns, rows);
        model.origin = calculate_origin(columns as f32, rows as f32, model.cell_size);
        model.maze = generate_maze(base_grid, &settings.algo)
    }
    if settings.solve {
        model.solved_maze = Some(dijkstra_simplified_solver(model.maze.clone()))
    }
}
fn edit_rgb(ui: &mut egui::Ui, colour: &mut Rgb8) {
    let mut egui_rgb = [colour.red, colour.green, colour.blue];

    if egui::color_picker::color_edit_button_srgb(ui, &mut egui_rgb).changed() {
        *colour = rgb8(egui_rgb[0], egui_rgb[1], egui_rgb[2]);
    }
}
fn generate_maze(base_grid: SmartGrid, algorithm: &Algos) -> SmartGrid {
    let selected_algorithm = match algorithm {
        Algos::BinaryTree => binary_tree,
        Algos::Sidewinder => sidewinder,
        Algos::AldousBroder => aldous_broder,
        Algos::HuntAndKill => hunt_and_kill,
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

    let colours = get_wall_colours(&model.settings);
    draw_maze(model, &draw, colours);

    draw.to_frame(app, &frame).unwrap();

    if model.settings.saving {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    } else {
        model.egui.draw_to_frame(&frame).unwrap();
    }
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

fn draw_maze(model: &Model, draw: &Draw, colours: WallColours) {
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

fn get_wall_colours(settings: &Settings) -> WallColours {
    let colour_type = settings.colour_type;
    let custom_colours = settings.walls.colours;

    match colour_type {
        ColourType::Party => WallColours::party(),
        ColourType::Default => WallColours::default(),
        ColourType::Custom => custom_colours,
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
