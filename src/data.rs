use rand::Rng;

#[derive(Debug, Clone, Default)]
pub struct GameData {
    arr: [[u64; 4]; 4],
    score: u64,
}

impl GameData {
    pub fn move_arr<
        F1: Fn(usize, usize, usize) -> (usize, usize),
        F2: Fn(usize, usize, usize) -> (usize, usize),
    >(
        &mut self,
        get_old_idx: F1,
        get_new_idx: F2,
    ) -> &Self {
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
						if into_val == val {
                        	self.score += self.arr[new_idx.0][new_idx.1];
						}
                        self.arr[old_idx.0][old_idx.1] = 0;
                    } else {
                        break;
                    }
                }
            }
        }

        self.add_random()
    }

    pub fn up(&mut self) -> &Self {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (col - idx, row),
            |row: usize, col: usize, idx: usize| (col - idx - 1, row),
        )
    }

    pub fn down(&mut self) -> &Self {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (3 - (col - idx), row),
            |row: usize, col: usize, idx: usize| (3 - (col - idx) + 1, row),
        )
    }

    pub fn left(&mut self) -> &Self {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (row, col - idx),
            |row: usize, col: usize, idx: usize| (row, col - idx - 1),
        )
    }

    pub fn right(&mut self) -> &Self {
        self.move_arr(
            |row: usize, col: usize, idx: usize| (row, 3 - (col - idx)),
            |row: usize, col: usize, idx: usize| (row, 3 - (col - idx) + 1),
        )
    }

    fn add_random(&mut self) -> &Self {
        let mut rng = rand::thread_rng();
        let rnd = rng.gen_range(0..15);
        let row: usize = rnd / 4;
        let col = rnd % 4;

        if self.arr[row][col] != 0 {
            return self.add_random();
        }

        self.arr[row][col] = 2;

        self
    }

    pub fn new() -> GameData {
        let mut game = GameData::default();
        game.add_random();
        game.add_random();
        game
    }
}
