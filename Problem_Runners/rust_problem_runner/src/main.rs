#[macro_use]
extern crate serde_derive;
extern crate docopt;

extern crate time;
extern crate problem_1;
extern crate problem_2;
extern crate problem_3;
extern crate problem_4;

use std::process::exit;
use time::precise_time_ns;
use docopt::Docopt;

const USAGE: &'static str = "
Usage:
    rust_problem_runner (--help)
    rust_problem_runner [--run PROBLEMS | --bench PROBLEMS]

Options:
    -h --help            Shows this screen
    -r --run PROBLEMS    Runs the specified problem's solution
    -b --bench PROBLEMS  Benchmarks the specified problem's solution

Arguments:
    PROBLEMS  Problems can be specified with 3 forms where # is a number
                  all: all is special and specifies all solutions
                  #,#: will run the first number, then the second and can be
                       chained like #,#,#,#
                  #:#: will run all the soltuions from the first number to the
                       second number, can be chained with commas like #:#,#:#

Examples:
    rust_problem_runner -r 1
        Runs the first problem's solution
    rust_problem_runner -b 3,1,2
        Benchmarks problems 3, 1, then 2
";
#[derive(Debug, Deserialize)]
struct Args {
    flag_run: String,
    flag_bench: String,
}


fn main() {
    let problem_functions = vec![
        problem_one as fn() -> String,
        problem_two as fn() -> String,
        problem_three as fn() -> String,
        problem_four as fn() -> String,
    ];

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    let problems = if args.flag_bench != "" {
        &args.flag_bench
    } else {
        &args.flag_run
    };

    let numbers = parse_problems(problems, problem_functions.len());

    if args.flag_bench != "" {
        bench(problem_functions, &numbers);
    } else {
        run(problem_functions, &numbers);
    }
}


fn parse_problems(raw: &str, high_bound: usize) -> Vec<usize> {
    let mut numbers = Vec::new();
    if raw == "all" {
        numbers = (1..=high_bound).collect();
    } else {
        for section in raw.split(',') {
            if section.contains(':') {
                let limits: Vec<&str> = section.split(':').collect();
                if limits.len() != 2 {
                    println!("Error: ranges can not be chained and must be in \
                             the form of number:number like 1:3");
                    exit(1);
                }

                let start = parse_with_high_limit(limits[0], high_bound);
                let end = parse_with_high_limit(limits[1], high_bound);

                // look into matching for Ordering for here
                if start < end {
                    numbers.append(&mut (start..=end).collect());
                } else {
                    numbers.append(&mut (end..=start).rev().collect());
                }
            } else {
                numbers.push(parse_with_high_limit(section, high_bound));
            }
        }
    }
    numbers
}


fn parse_with_high_limit(limit: &str, high_bound: usize) -> usize {
    match limit.parse() {
        Err(e) => {
            println!("Error Parsing Number: {}", e);
            exit(1);
        },
        Ok(num) => {
            if num < 1 || num > high_bound {
                println!("Error Parsing Number: {} should be \
                         within 1 and {}", limit,
                         high_bound);
                exit(1);
            }

            num
        }
    }
}

fn run<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        println!("===================================================");
        println!("[Running Problem {}]", *number);
        println!("{}", problems[*number - 1]());
    }
    println!("===================================================");
}


fn bench<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        println!("===================================================");
        println!("[Benchmarking Problem {}]", *number);

        let mut times = Vec::new();
        // Eventually dig in to automatically determining sample size but for
        // now this will do
        let mut index = 10000;
        while index > 0 {
            let start = precise_time_ns();
            problems[*number - 1]();
            times.push(precise_time_ns() - start);

            index -= 1;
        }
        let (mean, deviation) = standard_deviation(&times);
        println!("mean ± σ [µs]: {:.2} ± {:.2}",
                 mean / 1000.0, deviation / 1000.0);
    }
    println!("===================================================");
}


fn standard_deviation(samples: &Vec<u64>) -> (f64, f64) {
    let sum: u64 = samples.iter().sum();
    let mean = (sum as f64) / (samples.len() as f64);
    let mut sum_diff_sqrd = 0.0;
    for sample in samples {
        sum_diff_sqrd += ((*sample as f64) - mean).powi(2);
    }
    let deviation = (sum_diff_sqrd / (samples.len() as f64)).sqrt();

    (mean, deviation)
}


fn problem_one() -> String {
    let upper = 1000;
    let sum = problem_1::sum_multiples_of_3_and_5(upper);
    format!("The sum of multiples of 3 and 5 below {} is {}", upper, sum)
}


fn problem_two() -> String {
    let upper = 4000000;
    let sum = problem_2::sum_even_fibonacci(upper);
    format!("The sum of even Fibonacci numbers below {} is {}", upper, sum)
}


fn problem_three() -> String {
    let num = 600851475143;
    let prime = problem_3::largest_prime_factor(num);
    format!("The largest prime factor is {}", prime)
}


fn problem_four() -> String {
    let upper = 1000;
    let (num1, num2) = problem_4::largest_palindrome_product(upper);
    let product = num1 * num2;
    format!("The largest palindrome product is {} = {} * {}",
            product, num1, num2)
}

