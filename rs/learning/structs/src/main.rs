struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

let mut user1 = build_user(  
        String::from("iliamftw2013@gmail.com"),
        String::from("liano"));     
 
}

fn build_user(email: String, username: String) -> User {
    User{
        email: email, 
        username: username,
        sign_in_count: 1,
        active: true,
        }
}
