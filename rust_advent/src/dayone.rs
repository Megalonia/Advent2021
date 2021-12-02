use crate::*;
// Make  PartOne and PartTwo into one struct
pub struct PartOne {
    pub increases: u32,
    pub curr: u32,
}

impl PartOne {
     pub fn sonar_sweep(&mut self, inputs: VecDeque<&str>) -> u32  {
        for i in inputs {
            let next_up: u32 = FromStr::from_str(i).unwrap();
            if next_up > self.curr {
                self.increases += 1
            }
            self.curr = next_up
        } 
	self.increases
    }

    pub fn display_result(&self) {
	println!("{}",self.increases);
    }

    pub fn solve(&mut self, mut inputs: VecDeque<&str>) {
        inputs.pop_front();
        self.sonar_sweep(inputs);
        self.display_result();
    }
}

///////////////////////////////

pub struct PartTwo {
    pub increases: u32,
    pub current_group: VecDeque<u32>,
    pub prev: u32,
}

impl PartTwo {
    pub fn sonar_sweep(&mut self, inputs: VecDeque<&str>) -> u32 {
        for i in inputs {
            self.current_group.push_back(FromStr::from_str(i).unwrap());
            if self.current_group.len() == 3 {
                let sum: u32 = self.current_group.iter().map(|&i| i as u32).sum();
                if sum > self.prev { self.increases += 1; self.prev = sum; } else { self.prev = sum} 
                self.current_group.pop_front();
            }
	    
        }
        self.increases
    }

    pub fn display_result(&self) {
	println!("{}",self.increases);
    }

    pub fn solve(&mut self, inputs: VecDeque<&str>) {
        self.sonar_sweep(inputs);
        self.display_result();
    }
}
