use rand::Rng;

use crate::maze::{Location, SmartGrid};
use rand::seq::SliceRandom;

const BIDI: bool = true;

fn binary_tree_random_neighbour(eastern: Location, northern: Location) -> Location {
    // Do I need to do this in two steps?
    let mut neighbours: Vec<Location> = vec![];
    neighbours.extend([eastern, northern]);

    let linked_location = rand::thread_rng().gen_range(0..=1);

    neighbours[linked_location]
}

pub fn binary_tree(grid: SmartGrid) -> SmartGrid {
    for row in &grid.cells {
        for cell in row {
            let mut cell = cell.borrow_mut();
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let is_north_eastern_cell = is_northmost_cell & is_eastmost_cell;

            if is_north_eastern_cell {
                break;
            } else if is_northmost_cell {
                let eastern_location = cell.east.unwrap();
                SmartGrid::link_cells(&grid, &mut cell, eastern_location, BIDI);
            } else if is_eastmost_cell {
                let northern_location = cell.north.unwrap();
                SmartGrid::link_cells(&grid, &mut cell, northern_location, BIDI);
            } else {
                let linked_neighbour =
                    binary_tree_random_neighbour(cell.east.unwrap(), cell.north.unwrap());
                SmartGrid::link_cells(&grid, &mut cell, linked_neighbour, BIDI);
            }
        }
    }
    grid
}

pub fn sidewinder(grid: SmartGrid) -> SmartGrid {
    for row in &grid.cells {
        let mut run: Vec<Location> = Vec::new();

        for cell in row {
            let mut cell = cell.borrow_mut();
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let zero_or_one = rand::thread_rng().gen_range(0..=1);
            let should_close_run = is_eastmost_cell || (!is_northmost_cell & (zero_or_one == 0));

            run.push(cell.location);

            if should_close_run {
                let member_location = run.choose(&mut rand::thread_rng()).unwrap();

                let mut member_cell = if member_location == &cell.location {
                    cell
                } else {
                    grid.cells[member_location.row][member_location.column].borrow_mut()
                };

                if !is_northmost_cell {
                    let northern_location = member_cell.north.unwrap();
                    SmartGrid::link_cells(&grid, &mut member_cell, northern_location, BIDI);
                    run.clear();
                }
            } else {
                let eastern_location = cell.east.unwrap();
                SmartGrid::link_cells(&grid, &mut cell, eastern_location, BIDI);
            }
        }
    }
    grid
}

pub fn aldous_broder(grid: SmartGrid) -> SmartGrid {
    // no idea count needs the -1...but it does
    let mut unvisited_count = grid.columns * grid.columns - 1;
    let random_row = rand::thread_rng().gen_range(0..=grid.rows - 1);
    let random_column = rand::thread_rng().gen_range(0..=grid.columns - 1);
    let mut current_cell = &grid.cells[random_row][random_column];
    while unvisited_count > 0 {
        let neighbours = current_cell.borrow().get_neighbours();
        let random_neighbour_location = neighbours.choose(&mut rand::thread_rng()).unwrap();
        let random_neighbour =
            &grid.cells[random_neighbour_location.row][random_neighbour_location.column];

        if random_neighbour.borrow().is_unlinked() {
            SmartGrid::link_cells(
                &grid,
                &mut current_cell.borrow_mut(),
                *random_neighbour_location,
                BIDI,
            );
            unvisited_count -= 1;
        }
        current_cell = random_neighbour;
    }
    grid
}
