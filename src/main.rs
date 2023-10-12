

static PUZZLE_SIZE: usize = 9;

struct SudokuPuzzle{
    puzzle: Vec<u8>,
    solved_puzzle: Vec<u8>,
    is_solved: bool,
}

impl SudokuPuzzle {
    fn new() -> Self {
        Self {
            puzzle: Vec::with_capacity(PUZZLE_SIZE * PUZZLE_SIZE),
            solved_puzzle: Vec::with_capacity(PUZZLE_SIZE * PUZZLE_SIZE),
            is_solved: false,
        }
    }
    fn from_string(spz:&String) -> Self {
        let mut pz = Self::new();
        for ltr in spz.chars(){
            let value: u8 = ltr.to_digit(10)
                .expect("Found illegal value in string puzzle")
                .try_into()
                .unwrap();
            pz.puzzle.push(value);
            pz.solved_puzzle.push(value);
        }
        pz
    }
    fn print(&self){
        for (i, &val) in self.puzzle.iter().enumerate() {
            if (i!=0) && (i%(PUZZLE_SIZE - 1) == 0) {
                println!();
            }
            print!("{} ",val);
        }
    }
}

fn main() {

    let str_puzzle = String::from("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let pz=SudokuPuzzle::from_string(&str_puzzle);

    pz.print();
    println!("Puzzle is solved: {}", pz.is_solved);

}

