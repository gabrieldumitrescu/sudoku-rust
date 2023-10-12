// Sudoku solver program
// @Gabriel Dumitrescu 2023

//The puzzle is a square of size:
static PUZZLE_SIZE: usize = 9;

//Object to hold the puzzle
struct SudokuPuzzle{
    puzzle: Vec<u8>,//initial puzzle
    solved_puzzle: Vec<u8>,//after solving 
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
        assert_eq!(spz.len(), PUZZLE_SIZE * PUZZLE_SIZE);
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
            if (i!=0) && (i % PUZZLE_SIZE == 0) {
                println!();
            }
            if val == 0  {
                print!("_ ")
            }
            else {
                print!("{} ",val);
            }
        }
        println!();
    }
}

fn main() {

    let str_puzzle = String::from("001700509573024106800501002700295018009400305652800007465080071000159004908007053");
    let pz=SudokuPuzzle::from_string(&str_puzzle);

    pz.print();
   println!("Puzzle is solved: {}", pz.is_solved);

}

