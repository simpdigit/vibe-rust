use std::collections::HashMap;
use rand::RngCore;
use bs58;

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub grade: u8,
    pub drivers_license: String,
}

fn generate_base58_key() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

pub fn setup_student_grades() -> HashMap<String, Student> {
    let mut students = HashMap::new();

    students.insert(
        generate_base58_key(),
        Student {
            name: "Alice".to_string(),
            grade: 85,
            drivers_license: "DL12345".to_string(),
        },
    );
    students.insert(
        generate_base58_key(),
        Student {
            name: "Bob".to_string(),
            grade: 92,
            drivers_license: "DL67890".to_string(),
        },
    );
    students.insert(
        generate_base58_key(),
        Student {
            name: "Charlie".to_string(),
            grade: 78,
            drivers_license: "DL54321".to_string(),
        },
    );

    students
}