#![allow(dead_code)]

struct Solution;

struct Fancy {
    sequence: Vec<i32>,
    add_val: i64,
    mul_val: i64,
    modulo: i64,
    inv_muls: Vec<i64>, // Inverse of the current multiplier at each append
    adds: Vec<i64>,     // Add values at each append
}

impl Fancy {
    fn new() -> Self {
        Fancy {
            sequence: Vec::new(),
            add_val: 0,
            mul_val: 1,
            modulo: 1_000_000_007,
            inv_muls: Vec::new(),
            adds: Vec::new(),
        }
    }

    fn append(&mut self, val: i32) {
        self.sequence.push(val);
        self.inv_muls.push(self.mul_val);
        self.adds.push(self.add_val);
    }

    fn add_all(&mut self, inc: i32) {
        self.add_val = (self.add_val + inc as i64) % self.modulo;
    }

    fn mult_all(&mut self, m: i32) {
        self.mul_val = (self.mul_val * m as i64) % self.modulo;
        self.add_val = (self.add_val * m as i64) % self.modulo;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.sequence.len() {
            return -1;
        }

        let val = self.sequence[idx] as i64;
        let append_mul = self.inv_muls[idx];
        let append_add = self.adds[idx];

        // Calculate the transformation parameters
        let coefficient = self.mod_inverse(append_mul);
        let mult_factor = (coefficient * self.mul_val) % self.modulo;
        let add_factor =
            ((self.add_val - append_add * mult_factor % self.modulo) + self.modulo) % self.modulo;

        // Apply the transformation: val = (val * mult_factor + add_factor) % modulo
        let result = ((val * mult_factor) % self.modulo + add_factor) % self.modulo;

        result as i32
    }

    // Calculate modular inverse using Fermat's Little Theorem
    fn mod_inverse(&self, a: i64) -> i64 {
        self.mod_pow(a, self.modulo - 2)
    }

    // Fast modular exponentiation
    fn mod_pow(&self, mut base: i64, mut exp: i64) -> i64 {
        let mut result = 1;
        base %= self.modulo;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % self.modulo;
            }
            exp >>= 1;
            base = (base * base) % self.modulo;
        }

        result
    }
}

fn main() {}
