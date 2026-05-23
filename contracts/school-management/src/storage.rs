use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub struct StudentDetails {
    pub student_id: u64,
    pub name: String,
    pub wallet_address: Address,
    pub class_name: Class,
    pub total_paid: i128,
    pub is_registered: bool,
    pub is_active: bool,
    pub attendance_count: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct Payment {
    pub student_id: u64,
    pub amount: i128,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Student(u64),
    StudentPayments(u64),
    StudentCount,
}

#[derive(Clone)]
#[contracttype]
pub enum Class {
    Grade,
    HighSchool,
    College,
}
