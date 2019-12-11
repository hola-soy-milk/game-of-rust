struct Cell {
    alive: bool
}

impl Cell {
    fn display(&self) -> &str {
        match &self.alive {
            true => "O",
            false => " ",
            _ => " ",
        }
    }

    //fn tick(&self, game: Vec<Vec<Cell>>, row: usize, column: usize) -> Cell {
        //Cell {alive: false}
    //}
}

fn dead_cell() -> Cell {
    Cell {alive: false}
}

fn live_cell() -> Cell {
    Cell {alive: true}
}

fn main() {
        let mut game = vec![
            vec![dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), live_cell(), live_cell(), dead_cell(), live_cell(), live_cell(), live_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), live_cell(), dead_cell(), dead_cell(), live_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![live_cell(), live_cell(), live_cell(), live_cell(), dead_cell(), live_cell(), live_cell(), live_cell(), live_cell(), live_cell()],
            vec![live_cell(), live_cell(), live_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), live_cell(), live_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
            vec![dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell(), dead_cell()],
        ];

    while(true) {
        let mut new_game: Vec<Vec<Cell>> = Vec::new();
        for row in 0..game.len() {
            let mut new_row: Vec<Cell> = Vec::new();
            for column in 0..game[row].len() {
                let cell = &game[row][column];
                print!("{}", cell.display());
                let mut living_neighbor_count = 0;
                if column > 1 && row > 1 && game[row-1][column-1].alive {
                    living_neighbor_count += 1;
                }
                if row > 1 && game[row-1][column].alive {
                    living_neighbor_count += 1;
                }
                if column < game[row].len() -1 && row > 1 && game[row-1][column+1].alive {
                    living_neighbor_count += 1;
                }
                if column > 1 && game[row][column-1].alive {
                    living_neighbor_count += 1;
                }
                if column < game[row].len() -1 && game[row][column+1].alive {
                    living_neighbor_count += 1;
                }
                if row < game.len() -1 && column > 1 && game[row+1][column-1].alive {
                    living_neighbor_count += 1;
                }
                if row < game.len() -1 && game[row+1][column].alive {
                    living_neighbor_count += 1;
                }
                if row < game.len() -1 && column < game[row].len() -1 && game[row+1][column+1].alive {
                    living_neighbor_count += 1;
                }
                if cell.alive {
                    if living_neighbor_count >=4 || living_neighbor_count <= 1 {
                        new_row.push(dead_cell())
                    } else {
                        new_row.push(live_cell())
                    }
                } else {
                    if living_neighbor_count == 3 {
                        new_row.push(live_cell())
                    } else {
                        new_row.push(dead_cell())
                    }
                }
            }
            new_game.push(new_row);
            println!("");
        }
        game = new_game
    }

}
