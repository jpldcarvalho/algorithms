// Bloom Filter Implementation
// n - number of items in the filter = ceil(m / (-k / log(1 - exp(log(p) / k))))
// m - number of bits in the filter = ceil((n * log(p)) / log(1 / pow(2, log(2))));
// k - number of hash functions = round((m / n) * log(2));
// p - probability of false positives = pow(1 - exp(-k / (m / n)), k)

pub struct BloomFilter {
    n: i32,
    m: i32,
    k: i32,
    bits: Vec<bool>,
}

fn hash_fn(i: i32, m: i32, data: &Vec<u8>) -> usize {
    let mut sum: i32 = 0;
    for c in data {
        sum = i32::from(c - b'0');
    }

    ((sum * i) % m) as usize
}

impl BloomFilter {
    pub fn new(m: i32, k: i32) -> BloomFilter {
        let init_vec = vec![false; m as usize];        

        BloomFilter {
            n: 0,
            m,
            k,
            bits: init_vec,
        }
    }

    pub fn add(&mut self, data: &Vec<u8>) {
        for i in 0..self.k {
            self.bits[hash_fn(i, self.m, data)] = true;
        }

        self.n += 1;
    }

    pub fn contains(self, data: &Vec<u8>) -> bool {
        for i in 0..self.k {
            if !self.bits[hash_fn(i, self.m, data)] {
                return false;
            }
        }

        true
    }

    pub fn size(self) -> i32 { self.n }

    pub fn false_positives_rate(self) -> f64 {
        (1_f64 - ((-(self.k * self.n) / self.m) as f64).exp()).powf(self.k as f64)
    }

    pub fn reset(&mut self) {
        self.bits = vec![false; self.m as usize];
    }

    pub fn print_stats(self) {
        println!("Number of items: {}", self.n);
        println!("Number of bits: {}", self.m);
        println!("Number of hash fns: {}", self.k);
        println!("Number of prob of false positives: {}", self.false_positives_rate());
    }
}
