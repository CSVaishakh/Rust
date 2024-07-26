struct UserData {
    name: String,
    email: String,
    age: u8,    
}

trait Operations {
    fn add_user(&mut self,user:UserData);
    fn remove_user(&mut self,name:String)->Option<UserData>;
    fn modify_user(&mut self,name:String,new_user: UserData)->bool;
    fn display_info(&self);
}

trait Display {
    
}

impl Display for Vec<UserData> {}


impl Operations for Vec<UserData> {
    fn add_user(&mut self,user:UserData) {
        self.push(user);
    }

    fn remove_user(&mut self,name:String) ->Option<UserData> {
        if let Some(index) = self.iter().position(|u| u.name==name){
            Some(self.remove(index))
        }else{
            None
        }     
    }
    fn modify_user(&mut self,name:String,new_user: UserData)->bool {
        if let Some(user) = self.iter_mut().find(|u| u.name==name){
            *user = new_user;
            true
        }else{
        false
        }
    }
    fn display_info(&self) {
        for user in self{
        println!("{} , {} , {} ",user.name,user.email,user.age);
        }
    }
}



use std::io::stdin;


fn add_user(mut users:Vec<UserData>){
    println!("Enter the name");   
    let mut name=String::new();
    let _ = stdin()
        .read_line(&mut name);


    println!("Enter the email");   
    let mut email=String::new();
    let _ = stdin()
        .read_line(&mut email);


    println!("Enter the age");   
    let mut n=String::new();
    let _ = stdin()
        .read_line(&mut n);
    let mut age:u8=0;
    match n.as_str().trim().parse::<u8>() {
        Ok(val) => {age=val},
        Err(error) => {println!("{:?}",error)}
    }
    let user = UserData {
            name:name,
            email:email,
            age:age,
    };

    users.add_user(user);
}


fn remove_user(mut users: Vec<UserData>){
        println!("Enter the name");   
        let mut name=String::new();
        let _ = stdin()
            .read_line(&mut name);
        users.remove_user(name);
}

fn modify_user(mut users:Vec<UserData>){
    println!("Enter the name");   
    let mut name=String::new();
    let _ = stdin()
        .read_line(&mut name);

    println!("Enter the new name");   
    let mut newname=String::new();
    let _ = stdin()
        .read_line(&mut newname);


    println!("Enter the email");   
    let mut email=String::new();
    let _ = stdin()
        .read_line(&mut email);


    println!("Enter the age");   
    let mut n=String::new();
    let _ = stdin()
        .read_line(&mut n);
    let mut age:u8=0;
    match n.as_str().trim().parse::<u8>() {
        Ok(val) => {age=val},
        Err(error) => {println!("{:?}",error)}
    }
    let new_user = UserData {
            name:newname,
            email:email,
            age:age,
    };

    users.modify_user(name, new_user);
}
fn display_users(users: &Vec<UserData>) { {
        users.display_info()
    }
}



    // Program capabilities
    //
    // 1. Receive required information regrding user through commandline input
    // 2. Create and append the new user data to the Vec<UserData>
    // 3. Remove specific user data from the Vec<UserData>
    // 4. Modify specific user data in the Vec<UserData>
    // 5. Display the user data in the Vec<UserData>
    //
    // Description:
    // The program must ran in a menu driven manner, should have capabliliries
    // mentioned ablove. There should be a test module to verify the edge cases
    //

fn main() {
    let mut users: Vec<UserData> = Vec::new();

    let mut choice = String::new();
    loop {
        println!("\nUser Management System");
        println!("1. Add user");
        println!("2. Remove user");
        println!("3. Modify user");
        println!("4. Display all userinfo");
        println!("5. Exit");
        println!("Enter your choice:");
        stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => add_user(users),
            "2" => remove_user(users),
            "3" => modify_user(users),
            "4" => display_users(&users),
            "5" => break,
             _ => println!("Invalid choice, please try again."),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut users = Vec::new();
        let user = UserData {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            age: 30,
        };
        users.add_user(user);
        assert_eq!(users.len(), 1);
        users.display_info();
    }
    #[test]
    fn test_remove_user() {
        let mut users = Vec::new();
        let name:String = String::from("Test User");
        users.remove_user(name);
    }


    #[test]
    fn test_modify_user() {
        let mut users = Vec::new();
        let user:String = String::from("Test User");
        let new_user = UserData {
            name: "Test User2".to_string(),
            email: "test2@example.com".to_string(),
            age: 23,
        };
        users.modify_user(user,new_user);
    }

    #[test]
    fn test_display_user() {
        let  users:Vec<UserData> = Vec::new();
        users.display_info();
    }
    // Add more tests for remove_user, modify_user, etc.
}
