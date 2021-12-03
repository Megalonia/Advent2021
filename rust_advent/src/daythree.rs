use crate::*;
pub struct BinaryDiagnostic {
    pub gamma_rate  : u32,
    pub epsilon_rate: u32,
    pub common_bits: Vec<(u32,u32)>,
}


impl BinaryDiagnostic {
   pub fn power_consumption(&mut self, input: VecDeque<&str>)  {
       //O(n*m)?
       for line in input {
           let mut count = 0; 
           for bit in line.split("").collect::<Vec<&str>>() {
               match bit {
                   "0" => {
                              self.common_bits[count].0 += 1;
                              count += 1;
                          }
                   "1" => { 
                              self.common_bits[count].1 += 1;
                              count += 1;
                          }
                    _  => {},
               }
           }
       }

       for (i,(x,y)) in self.common_bits.iter().rev().enumerate() {
	   if x > y {
               self.epsilon_rate += 2_u32.pow(i as u32);
           }else {
               self.gamma_rate += 2_u32.pow(i as u32);
           }
       }

       println!("{}" ,self.gamma_rate * self.epsilon_rate);
   } 
}
