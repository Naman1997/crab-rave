fn main() {

    #[derive(Debug)]
    struct User {
        fname: String,
        lname: String,
        age: u16,
        height: f32,
        email: String
    }

    let mut me: User = User {
        fname: String::from("Naman"),
        lname: String::from("Arora"),
        age: 27,
        height: 182.5,
        email: String::from("myme@linux.com"),
    };

    me.age += 1;

    println!("{}'s email is {}", me.fname, me.email);

    fn build_user(email: String, fname: String) -> User {
        User {
            fname: fname,
            lname: String::from(""),
            age: 0,
            height: 0.0,
            email:email
        }
    }

    let test_user = build_user(String::from("test@mail"), String::from("test"));
    println!("{:?}", test_user);

}
