mod data_structures;

use clap::Parser;
use rand::RngCore;
use bs58;

use data_structures::vectors::{
    create_vector,
    double_vector,
    get_prime_numbers,
    sum_vector,
};

use data_structures::hashmaps::{
    setup_student_grades,
    Student
};



/// Generate a random base58-encoded 32-byte key (Solana style)
fn generate_base58_key() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

/// Rust Vectors Explorer
#[derive(Parser)]
#[command(author, version, about, long_about = "A tool to explore Rust vector operations. Pass the vector size using -v or --vector-size. Add students with --student 'NAME,GRADE,LICENSE'")]
struct Cli {
    /// Size of the vector (e.g. -v 20 or --vector-size 20)
    #[arg(short = 'v', long = "vector-size", default_value_t = 10, help = "Set the size of the vector")]
    vector_size: i16,

    /// Add a student in the format NAME,GRADE,LICENSE (can be used multiple times)
    #[arg(long = "student", value_parser = parse_student, num_args = 0..)]
    students: Vec<Student>,
}

fn parse_student(s: &str) -> Result<Student, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("Student must be in format NAME,GRADE,LICENSE".to_string());
    }
    let grade = parts[1].parse::<u8>().map_err(|_| "Grade must be a number".to_string())?;
    Ok(Student {
        name: parts[0].to_string(),
        grade,
        drivers_license: parts[2].to_string(),
    })
}

fn main() {
    let cli = Cli::parse();

    println!("Welcome to the Rust Vectors Explorer!");

    let vec = create_vector(cli.vector_size);
    println!("Created vector: {:?}", vec);

    let sum = sum_vector(&vec);
    println!("Sum of vector elements: {}", sum);

    let doubled = double_vector(&vec);
    println!("Doubled vector: {:?}", doubled);

    let primes = get_prime_numbers(&vec);
    println!("Prime numbers in vector: {:?}", primes);

    // Check if the primes vector contains the number 23
    let contains_23 = primes.contains(&23);
    if contains_23 {
        println!("The vector contains the number 23.");
    } else {
        println!("The vector does not contain the number 23.");
    }

    // Setup students HashMap with base58 keys
    let mut grades = setup_student_grades();

    for student in cli.students {
        let key = generate_base58_key();
        grades.insert(key, student);
    }

    println!("Student grades (base58 keys):");
    for (key, student) in &grades {
        println!("Key: {}, Student: {:?}", key, student);
    }
}
