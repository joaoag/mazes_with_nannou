use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Link {
    pub source: Location,
    pub target: Location,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Hash)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct MazeCell {
    pub location: Location,
    pub north: Option<Location>,
    pub east: Option<Location>,
    pub south: Option<Location>,
    pub west: Option<Location>,
    pub links: Vec<Location>,
    pub distance: usize,
}

impl MazeCell {
    pub fn empty(row: usize, column: usize) -> Self {
        MazeCell {
            location: Location { row, column },
            ..Default::default()
        }
    }
    pub fn is_linked(&self, direction: Direction) -> bool {
        if self.links.is_empty() {
            return false;
        }
        match direction {
            Direction::North if self.north.is_some() => self.links.contains(&self.north.unwrap()),
            Direction::East if self.east.is_some() => self.links.contains(&self.east.unwrap()),
            Direction::South if self.south.is_some() => self.links.contains(&self.south.unwrap()),
            Direction::West if self.west.is_some() => self.links.contains(&self.west.unwrap()),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SmartGrid {
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<Vec<Rc<RefCell<MazeCell>>>>,
}

impl SmartGrid {
    pub fn prepare_grid(&mut self) -> Vec<Vec<Rc<RefCell<MazeCell>>>> {
        // TODO use new consistently for initialising empty Vec
        let mut cells = Vec::new();
        // can I wrap the Rc and RefCell in a type, say SmartCell
        // no leave this out, it's idiomatic and expressive to Rust people, wrapping in a type might obscure this

        for r in 0..self.rows {
            let mut row: Vec<Rc<RefCell<MazeCell>>> = Vec::new();

            for c in 0..self.columns {
                row.push(Rc::new(RefCell::new(MazeCell::empty(r, c))));
            }

            cells.push(row)
        }
        cells
    }

    pub fn get_neighbour(
        rows: &i32,
        columns: &i32,
        current_location: &Location,
        direction: Direction,
    ) -> Option<Location> {
        let row_range = 0..*rows;
        let col_range = 0..*columns;
        let current_row = current_location.row as i32;
        let current_column = current_location.column as i32;

        match direction {
            Direction::North => {
                if row_range.contains(&(current_row - 1)) {
                    Some(Location {
                        row: current_location.row - 1,
                        column: current_location.column,
                    })
                } else {
                    None
                }
            }
            Direction::East => {
                if col_range.contains(&(current_column + 1)) {
                    Some(Location {
                        row: current_location.row,
                        column: current_location.column + 1,
                    })
                } else {
                    None
                }
            }
            Direction::South => {
                if row_range.contains(&(current_row + 1)) {
                    Some(Location {
                        row: current_location.row + 1,
                        column: current_location.column,
                    })
                } else {
                    None
                }
            }
            Direction::West => {
                if row_range.contains(&(current_column - 1)) {
                    Some(Location {
                        row: current_location.row,
                        column: current_location.column - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
    pub fn link_cells(
        &self,
        mut source: RefMut<MazeCell>,
        target: Location,
        is_bidirectional: bool,
    ) {
        if is_bidirectional {
            source.links.push(target);
            let mut target_cell = self.cells[target.row][target.column].borrow_mut();
            target_cell.links.push(source.location);
        } else {
            source.links.push(target);
        }
    }

    pub fn configure_cells(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                let rows = self.rows as i32;
                let columns = self.columns as i32;
                let mut cell = cell.borrow_mut();

                cell.north =
                    SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::North);
                cell.east =
                    SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::East);
                cell.south =
                    SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::South);
                cell.west =
                    SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::West);
            }
        }
    }
}

pub fn cli_display(grid: &SmartGrid) {
    let start = String::from("+");
    let middle = "---+".repeat(grid.columns);
    let end = String::from("\n");
    let mut output = format!("{}{}{}", start, middle, end);

    // TODO work out if there's a cleaner and/or faster way to handle the string concatenation
    for row in grid.cells.iter() {
        let mut top = String::from("|");
        let mut bottom = String::from("+");

        for cell in row.iter() {
            // let distance = cell.borrow().distance;
            // let body = format!(" {} ", distance);
            let body = "   ";
            let east_boundary = if MazeCell::is_linked(&cell.borrow(), Direction::East) {
                " "
            } else {
                "|"
            };

            top.push_str((body.to_owned() + east_boundary).as_str());

            let south_boundary = if MazeCell::is_linked(&cell.borrow(), Direction::South) {
                "   "
            } else {
                "---"
            };
            let corner = "+";
            bottom.push_str((south_boundary.to_owned() + corner).as_str());
        }
        output.push_str((top.to_owned() + "\n").as_str());
        output.push_str((bottom.to_owned() + "\n").as_str());
    }

    println!("{}", output);
}
