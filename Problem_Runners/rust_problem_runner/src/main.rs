extern crate time;
extern crate problem_1;
extern crate problem_1_try_hard;
extern crate problem_2;
extern crate problem_3;


use time::precise_time_ns;

// TODO:
// * Parse command line args
// * Do better benchmarking calculations
fn main() {
    let problem_functions = vec![
        run_problem_one as fn() -> String,
        run_problem_one_try_hard as fn() -> String,
        run_problem_two as fn() -> String,
        run_problem_three as fn() -> String,
    ];

    let mut problem_numbers = Vec::new();
    for i in 0..problem_functions.len() {
        problem_numbers.push(i);
    }

    bench(problem_functions, &problem_numbers);
}


fn run<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        println!("{}", problems[*number]());
    }
}


fn bench<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        let mut times = Vec::new();
        let mut index = 100;
        while index > 0 {
            let start = precise_time_ns();
            problems[*number]();
            times.push(precise_time_ns() - start);

            index -= 1;
        }

        let times_sum: u64 = times.iter().sum();
        println!("Average time: {}", (times_sum as f32) / (times.len() as f32));
    }
}


fn run_problem_one() -> String {
    let upper = 1000;
    let multiples = &[3, 5];
    let sum = problem_1::sum_multiples(upper, multiples);
    format!("The sum of multiples of {:?} below {} is {}",
            multiples, upper, sum)
}


fn run_problem_one_try_hard() -> String {
    let upper = 1000;
    let sum = problem_1_try_hard::sum_multiples_of_3_and_5(upper);
    format!("The sum of multiples of 3 and 5 below {} is {}", upper, sum)
}


fn run_problem_two() -> String {
    let upper = 4000000;
    let sum = problem_2::sum_even_fibonacci(upper);
    format!("The sum of even Fibonacci numbers below {} is {}", upper, sum)
}


fn run_problem_three() -> String {
    let num = 600851475143;
    let prime = problem_3::largest_prime_factor(num);
    format!("The largest prime factor is {}", prime)
}

