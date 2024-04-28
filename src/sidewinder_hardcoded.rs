use crate::maze::core::{Location, MazeCell, SmartGrid};
use std::cell::RefCell;
use std::rc::Rc;

pub fn static_sidewinder() -> SmartGrid {
    SmartGrid {
        rows: 4,
        columns: 4,
        cells: vec![
            vec![
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 0, column: 0 },
                    north: None,
                    east: Some(Location { row: 0, column: 1 }),
                    south: Some(Location { row: 1, column: 0 }),
                    west: None,
                    links: vec![Location { row: 0, column: 1 }],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 0, column: 1 },
                    north: None,
                    east: Some(Location { row: 0, column: 2 }),
                    south: Some(Location { row: 1, column: 1 }),
                    west: Some(Location { row: 0, column: 0 }),
                    links: vec![
                        Location { row: 0, column: 0 },
                        Location { row: 0, column: 2 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 0, column: 2 },
                    north: None,
                    east: Some(Location { row: 0, column: 3 }),
                    south: Some(Location { row: 1, column: 2 }),
                    west: Some(Location { row: 0, column: 1 }),
                    links: vec![
                        Location { row: 0, column: 1 },
                        Location { row: 0, column: 3 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 0, column: 3 },
                    north: None,
                    east: None,
                    south: Some(Location { row: 1, column: 3 }),
                    west: Some(Location { row: 0, column: 2 }),
                    links: vec![
                        Location { row: 0, column: 2 },
                        Location { row: 1, column: 3 },
                    ],
                    distance: 0,
                })),
            ],
            vec![
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 1, column: 0 },
                    north: Some(Location { row: 0, column: 0 }),
                    east: Some(Location { row: 1, column: 1 }),
                    south: Some(Location { row: 2, column: 0 }),
                    west: None,
                    links: vec![
                        Location { row: 1, column: 1 },
                        Location { row: 2, column: 0 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 1, column: 1 },
                    north: Some(Location { row: 0, column: 1 }),
                    east: Some(Location { row: 1, column: 2 }),
                    south: Some(Location { row: 2, column: 1 }),
                    west: Some(Location { row: 1, column: 0 }),
                    links: vec![
                        Location { row: 1, column: 0 },
                        Location { row: 1, column: 2 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 1, column: 2 },
                    north: Some(Location { row: 0, column: 2 }),
                    east: Some(Location { row: 1, column: 3 }),
                    south: Some(Location { row: 2, column: 2 }),
                    west: Some(Location { row: 1, column: 1 }),
                    links: vec![
                        Location { row: 1, column: 1 },
                        Location { row: 1, column: 3 },
                        Location { row: 2, column: 2 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 1, column: 3 },
                    north: Some(Location { row: 0, column: 3 }),
                    east: None,
                    south: Some(Location { row: 2, column: 3 }),
                    west: Some(Location { row: 1, column: 2 }),
                    links: vec![
                        Location { row: 1, column: 2 },
                        Location { row: 0, column: 3 },
                        Location { row: 2, column: 3 },
                    ],
                    distance: 0,
                })),
            ],
            vec![
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 2, column: 0 },
                    north: Some(Location { row: 1, column: 0 }),
                    east: Some(Location { row: 2, column: 1 }),
                    south: Some(Location { row: 3, column: 0 }),
                    west: None,
                    links: vec![
                        Location { row: 2, column: 1 },
                        Location { row: 1, column: 0 },
                        Location { row: 3, column: 0 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 2, column: 1 },
                    north: Some(Location { row: 1, column: 1 }),
                    east: Some(Location { row: 2, column: 2 }),
                    south: Some(Location { row: 3, column: 1 }),
                    west: Some(Location { row: 2, column: 0 }),
                    links: vec![
                        Location { row: 2, column: 0 },
                        Location { row: 3, column: 1 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 2, column: 2 },
                    north: Some(Location { row: 1, column: 2 }),
                    east: Some(Location { row: 2, column: 3 }),
                    south: Some(Location { row: 3, column: 2 }),
                    west: Some(Location { row: 2, column: 1 }),
                    links: vec![Location { row: 1, column: 2 }],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 2, column: 3 },
                    north: Some(Location { row: 1, column: 3 }),
                    east: None,
                    south: Some(Location { row: 3, column: 3 }),
                    west: Some(Location { row: 2, column: 2 }),
                    links: vec![Location { row: 1, column: 3 }],
                    distance: 0,
                })),
            ],
            vec![
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 3, column: 0 },
                    north: Some(Location { row: 2, column: 0 }),
                    east: Some(Location { row: 3, column: 1 }),
                    south: None,
                    west: None,
                    links: vec![Location { row: 2, column: 0 }],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 3, column: 1 },
                    north: Some(Location { row: 2, column: 1 }),
                    east: Some(Location { row: 3, column: 2 }),
                    south: None,
                    west: Some(Location { row: 3, column: 0 }),
                    links: vec![
                        Location { row: 3, column: 2 },
                        Location { row: 2, column: 1 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 3, column: 2 },
                    north: Some(Location { row: 2, column: 2 }),
                    east: Some(Location { row: 3, column: 3 }),
                    south: None,
                    west: Some(Location { row: 3, column: 1 }),
                    links: vec![
                        Location { row: 3, column: 1 },
                        Location { row: 3, column: 3 },
                    ],
                    distance: 0,
                })),
                Rc::new(RefCell::new(MazeCell {
                    location: Location { row: 3, column: 3 },
                    north: Some(Location { row: 2, column: 3 }),
                    east: None,
                    south: None,
                    west: Some(Location { row: 3, column: 2 }),
                    links: vec![Location { row: 3, column: 2 }],
                    distance: 0,
                })),
            ],
        ],
    }
}
