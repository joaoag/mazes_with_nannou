#[cfg(test)]
mod tests {
    use crate::maze::core::{Location, MazeCell, SmartGrid};
    use crate::maze::make::{aldous_broder, binary_tree, hunt_and_kill, sidewinder};
    use crate::maze::solve::dijkstra_simplified_solver;
    use std::collections::HashSet;

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

    fn all_cells_reachable(grid: &SmartGrid) -> bool {
        let start = Location { row: 0, column: 0 };
        let mut visited: HashSet<Location> = HashSet::new();
        let mut frontier: Vec<Location> = vec![start];

        while let Some(loc) = frontier.pop() {
            if visited.contains(&loc) {
                continue;
            }
            visited.insert(loc);

            let cell = grid.cells[loc.row][loc.column].borrow();
            for linked_loc in &cell.links {
                if !visited.contains(linked_loc) {
                    frontier.push(*linked_loc);
                }
            }
        }

        visited.len() == grid.rows * grid.columns
    }

    // Property 1: All cells reachable from origin for each algorithm

    #[test]
    fn binary_tree_all_cells_reachable() {
        for _ in 0..10 {
            let grid = prepare_grid(10, 10);
            let maze = binary_tree(grid);
            assert!(all_cells_reachable(&maze), "Not all cells reachable in binary_tree maze");
        }
    }

    #[test]
    fn binary_tree_all_cells_reachable_non_square() {
        let grid = prepare_grid(15, 8);
        let maze = binary_tree(grid);
        assert!(all_cells_reachable(&maze), "Not all cells reachable in non-square binary_tree maze");
    }

    #[test]
    fn sidewinder_all_cells_reachable() {
        for _ in 0..10 {
            let grid = prepare_grid(10, 10);
            let maze = sidewinder(grid);
            assert!(all_cells_reachable(&maze), "Not all cells reachable in sidewinder maze");
        }
    }

    #[test]
    fn sidewinder_all_cells_reachable_non_square() {
        let grid = prepare_grid(15, 8);
        let maze = sidewinder(grid);
        assert!(all_cells_reachable(&maze), "Not all cells reachable in non-square sidewinder maze");
    }

    #[test]
    fn aldous_broder_all_cells_reachable() {
        for _ in 0..10 {
            let grid = prepare_grid(10, 10);
            let maze = aldous_broder(grid);
            assert!(all_cells_reachable(&maze), "Not all cells reachable in aldous_broder maze");
        }
    }

    #[test]
    fn aldous_broder_all_cells_reachable_non_square() {
        let grid = prepare_grid(15, 8);
        let maze = aldous_broder(grid);
        assert!(all_cells_reachable(&maze), "Not all cells reachable in non-square aldous_broder maze");
    }

    #[test]
    fn hunt_and_kill_all_cells_reachable() {
        for _ in 0..10 {
            let grid = prepare_grid(10, 10);
            let maze = hunt_and_kill(grid);
            assert!(all_cells_reachable(&maze), "Not all cells reachable in hunt_and_kill maze");
        }
    }

    #[test]
    fn hunt_and_kill_all_cells_reachable_non_square() {
        let grid = prepare_grid(15, 8);
        let maze = hunt_and_kill(grid);
        assert!(all_cells_reachable(&maze), "Not all cells reachable in non-square hunt_and_kill maze");
    }

    // Property 2: Maze dimensions match requested dimensions

    #[test]
    fn maze_dimensions_match_square() {
        let grid = prepare_grid(12, 12);
        let maze = binary_tree(grid);
        assert_eq!(maze.rows, 12);
        assert_eq!(maze.columns, 12);
        assert_eq!(maze.cells.len(), 12);
        assert!(maze.cells.iter().all(|row| row.len() == 12));
    }

    #[test]
    fn maze_dimensions_match_non_square() {
        let grid = prepare_grid(20, 5);
        let maze = sidewinder(grid);
        assert_eq!(maze.rows, 5);
        assert_eq!(maze.columns, 20);
        assert_eq!(maze.cells.len(), 5);
        assert!(maze.cells.iter().all(|row| row.len() == 20));
    }

    // Property 3: Solver assigns distance 0 to start cell, increasing distances outward

    #[test]
    fn solver_start_cell_has_distance_zero() {
        let grid = prepare_grid(10, 10);
        let maze = binary_tree(grid);
        let solved = dijkstra_simplified_solver(maze);

        let start_cell = solved.cells[0][0].borrow();
        assert_eq!(start_cell.distance, 0, "Start cell should have distance 0");
    }

    #[test]
    fn solver_distances_increase_along_path() {
        let grid = prepare_grid(10, 10);
        let maze = binary_tree(grid);
        let solved = dijkstra_simplified_solver(maze);

        // For every cell, all linked neighbors should have distance differing by exactly 1
        for row in &solved.cells {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                for linked_loc in &cell.links {
                    let linked_cell = solved.cells[linked_loc.row][linked_loc.column].borrow();
                    let diff = (cell.distance as i32 - linked_cell.distance as i32).abs();
                    assert_eq!(
                        diff, 1,
                        "Linked cells at {:?} (dist={}) and {:?} (dist={}) should differ by 1",
                        cell.location, cell.distance, linked_loc, linked_cell.distance
                    );
                }
            }
        }
    }

    #[test]
    fn solver_max_distance_is_set() {
        let grid = prepare_grid(10, 10);
        let maze = binary_tree(grid);
        let solved = dijkstra_simplified_solver(maze);

        assert!(solved.max_distance > 0, "max_distance should be greater than 0");

        // max_distance should equal the highest distance found in any cell + 1
        // (because the solver increments distance after the last frontier)
        let max_cell_distance = solved.cells.iter()
            .flat_map(|row| row.iter())
            .map(|cell| cell.borrow().distance)
            .max()
            .unwrap();

        assert_eq!(
            solved.max_distance,
            max_cell_distance + 1,
            "max_distance should be one more than highest cell distance"
        );
    }

    // Property 4: No cell links to itself or to non-adjacent cells

    fn is_adjacent(a: &Location, b: &Location) -> bool {
        let row_diff = (a.row as i32 - b.row as i32).abs();
        let col_diff = (a.column as i32 - b.column as i32).abs();
        // Adjacent means exactly one step in cardinal direction (not diagonal)
        (row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1)
    }

    #[test]
    fn no_self_links() {
        let grid = prepare_grid(10, 10);
        let maze = binary_tree(grid);

        for row in &maze.cells {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                assert!(
                    !cell.links.contains(&cell.location),
                    "Cell at {:?} links to itself",
                    cell.location
                );
            }
        }
    }

    #[test]
    fn all_links_are_adjacent() {
        let grid = prepare_grid(10, 10);
        let maze = hunt_and_kill(grid);

        for row in &maze.cells {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                for linked_loc in &cell.links {
                    assert!(
                        is_adjacent(&cell.location, linked_loc),
                        "Cell at {:?} links to non-adjacent cell {:?}",
                        cell.location,
                        linked_loc
                    );
                }
            }
        }
    }

    #[test]
    fn links_are_bidirectional() {
        let grid = prepare_grid(10, 10);
        let maze = aldous_broder(grid);

        for row in &maze.cells {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                for linked_loc in &cell.links {
                    let linked_cell = maze.cells[linked_loc.row][linked_loc.column].borrow();
                    assert!(
                        linked_cell.links.contains(&cell.location),
                        "Cell at {:?} links to {:?}, but reverse link missing",
                        cell.location,
                        linked_loc
                    );
                }
            }
        }
    }
}
