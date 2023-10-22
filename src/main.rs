// Sudoku solver program
// @Gabriel Dumitrescu 2023

//Remove after complete impementation
#![allow(dead_code)]

use std::collections::HashSet;


//The puzzle is a square of size:
static PUZZLE_SIZE: usize = 9;
static REGION_SIZE: usize = 3;
static NUM_CELLS:usize = PUZZLE_SIZE*PUZZLE_SIZE;


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
        let mut poss:HashSet<u8>=HashSet::from([1,2,3,4,5,6,7,8,9]);
        let (r,c)=SudokuSolver::get_row_col(idx);
        let r_start=r*PUZZLE_SIZE;
        //verify row
        let mut i=0;
        while i<PUZZLE_SIZE{
            let c_idx=r_start+i;
            if c_idx != idx {
                poss.remove(&self.solved_puzzle[c_idx]);
            }
            i=i+1;
        }
        //verify column
        i=0;
        while i<PUZZLE_SIZE{
            let c_idx=i*PUZZLE_SIZE+c;
            if c_idx != idx {
                poss.remove(&self.solved_puzzle[c_idx]);
            }
            i=i+1;
        }
        let start_idx=SudokuSolver::get_region_start_idx(r,c);
        //verify region
        i=0;
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

    fn test_solved(&mut self)->bool{
        if self.is_solved {
            return true;
        }
        let mut result: bool=true;
        for c in &self.solved_puzzle{
            if *c==0{
                result = false;
                break;
            }
        }
        self.is_solved = result;
        result
    }


    fn apply_basic_rules(&mut self) -> u8 {
        for i in 0..NUM_CELLS {
            let cr_set=&self.pos_values[i];
            if cr_set.len() != 1 { 
                let cr_pos=self.get_pos_values(i);
                //let (r,c)=SudokuSolver::get_row_col(cr);
                //print!("Posible values at {},{}:",r,c);
                //for value in &cr_pos{
                //    print!("{},",value);
                //}
                //println!("");
                self.pos_values[i]=cr_pos;
            }
        }
        self.place_single_values()
    }

    fn apply_advanced_rule(&mut self) -> u8{
        let mut placed: u8 = 0;
        for i in 0..NUM_CELLS{
            if self.solved_puzzle[i] > 0 { continue; }
            let mut cr_set=self.pos_values[i].clone();
            let (r,c) = SudokuSolver::get_row_col(i);
            for j in 0..PUZZLE_SIZE {
                if j==c { continue; }
                let ri=r*PUZZLE_SIZE + j;
                cr_set = cr_set.difference(&self.pos_values[ri]).copied().collect();
            }
            if cr_set.len() == 1 {
                for v in &cr_set { 
                    self.solved_puzzle[i]=*v;
                }
                placed+=1;
            }



        }
        placed
    }

    fn solve(&mut self) -> bool {
        if self.pos_values.len()==0 {
            for i in  0..NUM_CELLS {
                let mut cr_set=HashSet::new();
                if self.solved_puzzle[i]>0 {
                    cr_set.insert(self.solved_puzzle[i]);
                }
                self.pos_values.push(cr_set);
            }
        }
        loop{
            if self.test_solved(){ break;}
            let mut num_placed=self.apply_basic_rules();
            println!("Placed {} values", num_placed);
            if num_placed == 0 {
                self.print_solution();
                println!("Trying advanced rule");
                num_placed=self.apply_advanced_rule();
                println!("Placed {} values", num_placed);
                if num_placed==0 {break;}
            }
        }
        self.is_solved
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

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn main() {
    let puzzles=read_lines("./puzzles1.txt");
     let pz=SudokuPuzzle::from_string(&puzzles[2727]);
     let mut sol=SudokuSolver::new(&pz);
     sol.print_solution();
     sol.solve();
     sol.print_solution()

//    let mut solved: usize=0;
//    for (i,p) in puzzles.iter().enumerate(){
//        let pz=SudokuPuzzle::from_string(p);
//        let mut sol=SudokuSolver::new(&pz);
//        //sol.print_solution();
//        sol.solve();
//        if sol.is_solved {
//            println!("Solved puzzle no {}.",i);
//            solved+=1;
//        }
//    }
//
//    println!("Solved {} from {} puzzles.", solved, puzzles.len());
        //sol.print_solution();
    //h=h.difference(&h1).copied().collect();
    //for v in &h{
    //    println!("{}",v);
    //}
    //println!("h-h1 contains 4 :{}",h.contains(&7));

}

