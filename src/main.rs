use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use termion::{color, style};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

#[derive(Debug)]
struct Position(usize, usize);

#[derive(Debug, Clone)]
enum Cell {
    DeadCell,
    AliveCell,
}

#[derive(Debug, Clone)]
struct World {
    cells: Vec<Vec<Cell>>,
}

impl World {
    fn new() -> Self {
        World {
            // Note: Added 2 cells of padding to deal with boundary cells
            cells: vec![vec![Cell::DeadCell; WIDTH + 2]; HEIGHT + 2],
        }
    }

    fn render(&self) {
        let mut stdout = stdout().lock();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            style::Bold,
        )
        .unwrap();

        // Take into consideration the padding
        for line in self.cells[1..HEIGHT + 1].iter() {
            for cell in line[1..WIDTH + 1].iter() {
                match cell {
                    Cell::AliveCell => {
                        write!(
                            stdout,
                            " {}O{}",
                            color::Fg(color::Red),
                            color::Fg(color::Reset)
                        )
                        .unwrap();
                    }
                    Cell::DeadCell => {
                        write!(
                            stdout,
                            " {}O{}",
                            color::Fg(color::Blue),
                            color::Fg(color::Reset)
                        )
                        .unwrap();
                    }
                }
            }
            write!(stdout, "\n").unwrap();
        }

        stdout.flush().unwrap();
    }

    fn step(&mut self) {
        // Rules :
        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Any live cell with two or three live neighbours lives on to the next generation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        let mut switch_cells = Vec::new();
        let (x_from, x_to, y_from, y_to) = (1, WIDTH, 1, HEIGHT);

        for (y, line) in self.cells.iter().enumerate().take(y_to).skip(y_from) {
            for (x, cell) in line.iter().enumerate().take(x_to).skip(x_from) {
                let neighbors = self.live_neighbors(Position(x, y));
                match cell {
                    Cell::AliveCell => {
                        if neighbors < 2 || neighbors > 3 {
                            switch_cells.push(Position(x, y))
                        }
                    }
                    Cell::DeadCell => {
                        if neighbors == 3 {
                            switch_cells.push(Position(x, y))
                        }
                    }
                }
            }
        }
        self.update(switch_cells);
    }

    fn update(&mut self, positions: Vec<Position>) {
        for Position(x, y) in positions {
            match self.cells[y][x] {
                Cell::AliveCell => self.cells[y][x] = Cell::DeadCell,
                Cell::DeadCell => self.cells[y][x] = Cell::AliveCell,
            }
        }
    }

    fn live_neighbors(&self, pos: Position) -> usize {
        assert!(pos.0 >= 1 && pos.0 < WIDTH);
        assert!(pos.1 >= 1 && pos.1 < HEIGHT);
        let mut neighbors = Vec::with_capacity(8);

        neighbors.push(&self.cells[pos.1 - 1][pos.0]);
        neighbors.push(&self.cells[pos.1 + 1][pos.0]);
        neighbors.push(&self.cells[pos.1][pos.0 + 1]);
        neighbors.push(&self.cells[pos.1][pos.0 - 1]);
        neighbors.push(&self.cells[pos.1 - 1][pos.0 + 1]);
        neighbors.push(&self.cells[pos.1 + 1][pos.0 + 1]);
        neighbors.push(&self.cells[pos.1 - 1][pos.0 - 1]);
        neighbors.push(&self.cells[pos.1 + 1][pos.0 - 1]);
        neighbors
            .iter()
            .map(|c| match c {
                Cell::AliveCell => 1,
                Cell::DeadCell => 0,
            })
            .sum()
    }
}

fn main() {
    // 1. we need to define a world of cells
    let mut world = World::new();

    //  todo : Read this from a file
    world.cells[5][6] = Cell::AliveCell;
    world.cells[4][6] = Cell::AliveCell;
    world.cells[5][7] = Cell::AliveCell;
    world.cells[5][8] = Cell::AliveCell;
    world.cells[1][2] = Cell::AliveCell;
    world.cells[3][4] = Cell::AliveCell;
    world.cells[2][4] = Cell::AliveCell;
    world.cells[3][4] = Cell::AliveCell;

    loop {
        world.render();
        world.step();
        sleep(Duration::from_millis(200))
    }
    // 2. we need to define a way to visualize the the world of cells

    // 3. we need to define a way to update the world each step using rules
}
