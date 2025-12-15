use rand::Rng;

#[derive(Clone, Copy)]
pub enum ProblemType {
    Gcd,
    Lcm,
    Divisors,
    Primes,
}

pub struct MathProblem {
    pub problem_type: ProblemType,
    pub numbers: Vec<u32>,
    pub correct_answers: Vec<u32>,
    pub description: String,
}

impl MathProblem {
    pub fn new(problem_type: ProblemType) -> Self {
        match problem_type {
            ProblemType::Gcd => Self::generate_gcd_problem(),
            ProblemType::Lcm => Self::generate_lcm_problem(),
            ProblemType::Divisors => Self::generate_divisors_problem(),
            ProblemType::Primes => Self::generate_primes_problem(),
        }
    }

    fn generate_gcd_problem() -> Self {
        let mut rng = rand::thread_rng();
        
        // Generate two numbers with a known GCD
        let gcd = rng.gen_range(2..12);
        let a_mult = rng.gen_range(2..8);
        let b_mult = rng.gen_range(2..8);
        
        let a = gcd * a_mult;
        let b = gcd * b_mult;
        
        let description = format!("Catch the GCD of {} and {}", a, b);
        
        MathProblem {
            problem_type: ProblemType::Gcd,
            numbers: vec![a, b],
            correct_answers: vec![gcd],
            description,
        }
    }

    fn generate_lcm_problem() -> Self {
        let mut rng = rand::thread_rng();
        
        let a = rng.gen_range(3..12);
        let b = rng.gen_range(3..12);
        
        let lcm = Self::calculate_lcm(a, b);
        let description = format!("Catch the LCM of {} and {}", a, b);
        
        MathProblem {
            problem_type: ProblemType::Lcm,
            numbers: vec![a, b],
            correct_answers: vec![lcm],
            description,
        }
    }

    fn generate_divisors_problem() -> Self {
        let mut rng = rand::thread_rng();
        
        let number = rng.gen_range(12..30);
        let divisors = Self::get_divisors(number);
        
        let description = format!("Catch all divisors of {}", number);
        
        MathProblem {
            problem_type: ProblemType::Divisors,
            numbers: vec![number],
            correct_answers: divisors,
            description,
        }
    }

    fn generate_primes_problem() -> Self {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let description = "Catch all prime numbers!".to_string();
        
        MathProblem {
            problem_type: ProblemType::Primes,
            numbers: vec![],
            correct_answers: primes.iter().filter(|&&p| p < 50).cloned().collect(),
            description,
        }
    }

    pub fn is_correct_answer(&self, value: u32) -> bool {
        self.correct_answers.contains(&value)
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    // Helper functions
    fn calculate_gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn calculate_lcm(a: u32, b: u32) -> u32 {
        (a * b) / Self::calculate_gcd(a, b)
    }

    fn get_divisors(n: u32) -> Vec<u32> {
        let mut divisors = Vec::new();
        for i in 1..=n {
            if n % i == 0 {
                divisors.push(i);
            }
        }
        divisors
    }

    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let sqrt_n = (n as f64).sqrt() as u32;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}
