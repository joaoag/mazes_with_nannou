use std::cell::RefMut;
use rand::Rng;

use crate::maze::{Location, MazeCell, SmartGrid};
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
// pick a cell at random (walk_root)
// mark it as visited
// pick random neighbour (walk_neighbour)
// has this cell been visited?
// if yes:
//   make walk_neighbour walk_root
// if no:
//   mark it as visited && linked to walk_root
//   make walk_neighbour walk_root
// and process starts again
// continue until each cell is visited

    let mut unvisited_count = grid.columns * 2; // when this is 0, return grid

    // declare starting position
    let random_row = rand::thread_rng().gen_range(0..=grid.rows -1);
    let random_column = rand::thread_rng().gen_range(0..=grid.columns -1);


    while unvisited_count > 0 {
        let mut current_cell = grid.cells[random_row][random_column].borrow_mut();

        println!("current_cell initial assignment: {:?}", &current_cell.location);
        let neighbours = vec![current_cell.north, current_cell.east, current_cell.south, current_cell.west];
        let filtered_neighbours = neighbours.into_iter().filter(|n| n.is_some()).map(|i| i.unwrap()).collect::<Vec<_>>();
        let random_neighbour_location = filtered_neighbours.choose(&mut rand::thread_rng()).unwrap();

        let mut random_neighbour = &grid.cells[random_neighbour_location.row][random_neighbour_location.column];
        let unlinked = random_neighbour.clone().take().is_unlinked();

        if unlinked {
            SmartGrid::link_cells(&grid, &mut current_cell, *random_neighbour_location, BIDI);
            unvisited_count -= 1;
        }
        current_cell = random_neighbour.borrow_mut();
    }
    grid
}