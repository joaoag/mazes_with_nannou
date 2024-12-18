extern crate nannou;
extern crate nannou_egui;
extern crate rand;

use nannou::color::named::BLACK;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

use crate::maze::core::SmartGrid;
use crate::maze::make::{aldous_broder, binary_tree, hunt_and_kill, sidewinder};
use crate::maze::render::{calculate_origin, draw_maze, ColourType, Point, WallColours, Walls};
use crate::maze::solve::dijkstra_simplified_solver;

mod maze;
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
    pub is_solved: bool,
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
        max_distance: 0,
    };
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    grid
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
        is_solved: false,
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
            ui.add(egui::Slider::new(&mut settings.height, 2.0..=100.0));

            ui.label("Width:");
            ui.add(egui::Slider::new(&mut settings.width, 2.0..=100.0));

            ui.label("Corridor size");
            ui.add(egui::Slider::new(&mut settings.corridor_size, 0.1..=100.0));

            ui.label("Wall thickness");
            ui.add(egui::Slider::new(&mut settings.walls.width, 0.1..=100.0));

            ui.separator();
            ui.label("Colours");

            ui.vertical(|ui| {
                ui.radio_value(&mut settings.colour_type, ColourType::Default, "Default");
                ui.radio_value(
                    &mut settings.colour_type,
                    ColourType::Party,
                    "Party (strobe warning)",
                );
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
        model.is_solved = true;
        model.maze = dijkstra_simplified_solver(model.maze.clone())
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
