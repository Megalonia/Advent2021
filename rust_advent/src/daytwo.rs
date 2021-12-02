use crate::*;
//#[derive(Debug)]
pub struct DayTwo {
    pub horizontal_position: i32,
    pub depth: i32,
    pub aim: i32
}

impl DayTwo {
    pub fn pilot_part_one(&mut self, input: VecDeque<&str>) -> i32 {
        for i in input {
            let split_i = i.split(" ").collect::<Vec<&str>>();

	    match split_i[0] { 
                "forward" => self.horizontal_position += i32::from_str(split_i[1]).unwrap(),
                "down"    => self.depth               += i32::from_str(split_i[1]).unwrap(),
                "up"      => self.depth               -= i32::from_str(split_i[1]).unwrap(),
		 _        => {}
	    }
        }        
        self.horizontal_position * self.depth
    } 

    pub fn display(&self) { 
        println!("{} {}",self.horizontal_position,self.depth);
        println!("{}", self.horizontal_position * self.depth);
    }

    pub fn pilot_part_two(&mut self, input: VecDeque<&str>) {
        for i in input {
            let split_i = i.split(" ").collect::<Vec<&str>>();

            match split_i[0] {
                "forward" => {
                                 self.horizontal_position += i32::from_str(split_i[1]).unwrap();
                                 self.depth += self.aim * i32::from_str(split_i[1]).unwrap();
                             },
                "down"    => self.aim                 += i32::from_str(split_i[1]).unwrap(),
                "up"      => self.aim                 -= i32::from_str(split_i[1]).unwrap(),
		 _        => {}
               
            }
        }
    }
}
