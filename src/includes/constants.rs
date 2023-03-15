use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPECIAL: String = String::from("abcdefghijklmnoqprstuvwxyzABCDEFGHIJKLMNOQPRSTUVWXYZ");
    pub static ref FNAME_LENGHT: String = String::from("Your first name must be between 2 and 25 characters!");
    pub static ref LNAME_LENGHT: String = String::from("Your last name must be between 2 and 25 characters!");
    pub static ref UNAME_LENGHT: String = String::from("Your username must be between 2 and 25 characters!");
    pub static ref SPECIAL_FNAME: String = String::from("First name must not contain special characters!");
    pub static ref SPECIAL_LNAME: String = String::from("Last name must not contain special characters!");
    pub static ref SPECIAL_UNAME: String = String::from("Username must not contain special characters!");
    pub static ref EMAIL_INVALID: String = String::from("Invalid email!");
    pub static ref EMAIL_USED: String = String::from("Email already in use!");
    pub static ref UNAME_USED: String = String::from("Username already in use!");
    pub static ref EMAIL_DONT_MATCH: String = String::from("Email not match!");
    pub static ref PASS_LENGHT: String = String::from("Your password must be between 6 and 25 characters!");
    pub static ref PASS_DONT_MATCH: String = String::from("Password not match!");
    pub static ref EMAILPASS_DONT_MATCH: String = String::from("Email or password was incorrect!");
    pub static ref PASS_NOT_MATCH: String = String::from("Password incorrect!");
    
}