// Sudoku solver program
// @Gabriel Dumitrescu 2023

//Remove after complete impementation
#![allow(dead_code)]

use std::collections::HashSet;


//The puzzle is a square of size:
static PUZZLE_SIZE: usize = 9;
static REGION_SIZE: usize = 3;


//Object to hold the puzzle
struct SudokuPuzzle{
    puzzle: Vec<u8>,//initial puzzle
}

struct SudokuSolver{
    solved_puzzle: Vec<u8>,//after solving 
    pos_values: Vec<HashSet<u8>>,
    is_solved: bool,
}

impl SudokuPuzzle {
    fn new() -> Self {
        Self {
            puzzle: Vec::with_capacity(PUZZLE_SIZE * PUZZLE_SIZE),
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
        }
        pz
    }
    fn print(&self){
        print_vec_puzzle(&self.puzzle);
    }

}

impl SudokuSolver{
    fn new(pz: &SudokuPuzzle) -> Self {
        Self {
            solved_puzzle: pz.puzzle.to_vec(), 
            pos_values: Vec::new(),
            is_solved: false,
        }
    }
    fn print_solution(&self){
        print_vec_puzzle(&self.solved_puzzle);
    }

    fn get_row_col(idx: usize) -> (usize,usize){
        let col: usize=idx % PUZZLE_SIZE;
        let row: usize=idx / PUZZLE_SIZE;
        (row,col)
    }

    fn get_region_start_idx(row:usize, column: usize) -> usize{
        let start_row:usize=(row/REGION_SIZE) * REGION_SIZE;
        let start_column:usize=(column/REGION_SIZE) * REGION_SIZE;
        let idx=start_row * PUZZLE_SIZE + start_column;
        idx
    }

    fn get_pos_values(&self,idx: usize)-> HashSet<u8>{
        let cell_value=self.solved_puzzle[idx];
        if cell_value != 0 {
            return HashSet::from([cell_value]);
        }
        let mut poss:HashSet<u8>=HashSet::from([1,2,3,4,5,6,7,8,9]);
        let (r,c)=SudokuSolver::get_row_col(idx);
        let r_start=r*PUZZLE_SIZE;
        let mut i=0;
        println!("idx {} start row analyzing", idx);
        while i<PUZZLE_SIZE{
            let c_idx=r_start+i;
            if c_idx != idx {
                poss.remove(&self.solved_puzzle[c_idx]);
            }
            i=i+1;
        }
        i=0;
        println!("idx {} start column analyzing", idx);
        while i<PUZZLE_SIZE{
            let c_idx=i*PUZZLE_SIZE+c;
            if c_idx != idx {
                poss.remove(&self.solved_puzzle[c_idx]);
            }
            i=i+1;
        }
        let start_idx=SudokuSolver::get_region_start_idx(r,c);
        i=0;
        println!("idx {} start region analyzing", idx);
        while i<REGION_SIZE {
            let mut j=0;
            while j<REGION_SIZE{
                let c_idx=start_idx+i*PUZZLE_SIZE+j;
                if c_idx != idx {
                    poss.remove(&self.solved_puzzle[c_idx]);
                }
                j=j+1;
            }
            i=i+1;
        }

        println!("idx {} end analyzing", idx);
        poss
    }

    fn place_single_values(&mut self) -> u8{
        let mut placed:u8 = 0;
        for (i,h) in self.pos_values.iter().enumerate(){
            if self.solved_puzzle[i] == 0{
                if h.len() == 1{
                    for posval in h{
                        self.solved_puzzle[i]=*posval;
                    }
                    placed+=1;
                }
            }
        }
        placed
    }

    fn solve(&mut self) -> bool {
        //let mut pos_values:Vec<HashSet<u8>> = Vec::new();
        let mut cr:usize=0;
        if self.pos_values.len()==0 {
            while cr<PUZZLE_SIZE*PUZZLE_SIZE {
                self.pos_values.push(HashSet::new());
                cr=cr+1;
            }
        }
        loop{
            cr=0;
            while cr<PUZZLE_SIZE*PUZZLE_SIZE {
                println!("At index : {}", cr);
                let cr_set=&self.pos_values[cr];
                println!("{} possible values",cr_set.len());
                if cr_set.len() != 1 { 
                    let cr_pos=self.get_pos_values(cr);
                    //let (r,c)=SudokuSolver::get_row_col(cr);
                    //print!("Posible values at {},{}:",r,c);
                    //for value in &cr_pos{
                    //    print!("{},",value);
                    //}
                    //println!("");
                    self.pos_values[cr]=cr_pos;
                }
                cr=cr+1;
            }
            let num_placed=self.place_single_values();
            println!("Placed {} values", num_placed);
            if num_placed == 0 { break;}
        }
        false
    }
}

fn print_vec_puzzle(puzzle:&Vec<u8>){
    println!("__________________");
    for (i, &val) in puzzle.iter().enumerate() {
        if (i!=0) && (i % PUZZLE_SIZE == 0) {
            println!("|");
        }
        if val == 0  {
            print!("_ ")
        }
        else {
            print!("{} ",val);
        }
    }
    println!("|");
    println!("------------------");

}


fn main() {

    let str_puzzle = String::from("001700509573024106800501002700295018009400305652800007465080071000159004908007053");
    let pz=SudokuPuzzle::from_string(&str_puzzle);
    let mut sol=SudokuSolver::new(&pz);
    sol.print_solution();
    sol.solve();

    sol.print_solution();
    //h=h.difference(&h1).copied().collect();
    //for v in &h{
    //    println!("{}",v);
    //}
    //println!("h-h1 contains 4 :{}",h.contains(&7));

}

