#[derive(Debug, Clone, Default)]
pub struct GameData {
    pub arr: [[u8; 4]; 4],
}

impl GameData {
    pub fn move_arr<
        F1: Fn(usize, usize, usize) -> (usize, usize),
        F2: Fn(usize, usize, usize) -> (usize, usize),
    >(
        &mut self,
        get_old_idx: F1,
        get_new_idx: F2,
    ) {
        for row in 0..4 {
            for col in 1..4 {
                for idx in 0..col {
                    let old_idx = get_old_idx(row, col, idx);
                    let new_idx = get_new_idx(row, col, idx);

                    let val = self.arr[old_idx.0][old_idx.1];
                    let into_val = self.arr[new_idx.0][new_idx.1];

                    if val == 0 {
                        break;
                    }

                    if into_val == 0 || into_val == val {
                        self.arr[new_idx.0][new_idx.1] += val;
                        self.arr[old_idx.0][old_idx.1] = 0;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn up(&mut self) {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (col - idx, row),
            |row: usize, col: usize, idx: usize| (col - idx - 1, row),
        )
    }

	pub fn down(&mut self) {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (3 - (col - idx), row),
            |row: usize, col: usize, idx: usize| (3 - (col - idx) + 1, row),
        )
    }

    pub fn left(&mut self) {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (row, col - idx),
            |row: usize, col: usize, idx: usize| (row, col - idx - 1),
        )
    }

    pub fn right(&mut self) {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (row, 3 - (col - idx)),
            |row: usize, col: usize, idx: usize| (row, 3 - (col - idx) + 1),
        )
    }
}
