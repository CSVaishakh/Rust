
struct UserData {

    name: String,
    email: String,
    age: u8,    
}


trait Operations {
    fn add_user(&mut self);
    fn remove_user(&mut self)->Option<UserData>;
    fn modify_user(&mut self);
    fn display_users(&self);
}

//trait Display {}

// impl Display for Vec<UserData> {}


impl Operations for Vec<UserData> {
    fn add_user(&mut self) {
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
            self.push(user);
        }

    fn remove_user(&mut self)->Option<UserData> {
        println!("Enter the name");   
        let mut name=String::new();
        let _ = stdin()
            .read_line(&mut name);
        if let Some(index) = self.iter().position(|u| u.name==name){
            Some(self.remove(index))
            }else{
                None
            }
    }

    fn modify_user(&mut self){
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
        if let Some(user) = self.iter_mut().find(|u| u.name==name){
            *user = new_user;
        }else{
            println!("user not found")
        }
    }
    fn display_users(&self) {
        for user in self{
        println!(" {} {} {} ", user.name, user.email, user.age);
        }
    }
}


impl Clone for UserData{
    fn clone(&self)->Self{
        UserData{
            name:self.name.clone(),
            email:self.email.clone(),
            age:self.age.clone(),
        }
    }
}


use std::io::stdin;

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

    loop {
        println!("\nUser Management System");
        println!("1. Add user");
        println!("2. Remove user");
        println!("3. Modify user");
        println!("4. Display all user info");
        println!("5. Exit");
        println!("Enter your choice:");
        
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");
    
        match choice.trim() {
            "1" => users.add_user(),
            "2" => {match users.remove_user(){
                    Some(_) => {println!("user removed");},
                    None => 
                    {println!("username not found");},
                }
            },
            "3" => users.modify_user(),
            "4" => users.display_users(),
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
        let mut users:Vec<UserData> = Vec::new();
        users.add_user();
        assert_eq!(users.len(), 1);
    }
    #[test]
    fn test_remove_user() {
        let mut users:Vec<UserData> = Vec::new();
        let user=UserData{
            name:String::from("csv"),
            email:String::from("cas@gmail"),
            age:21,
        };
        users.push(user.clone());
        let remove_data=users.remove_user().unwrap();
        assert_eq!(remove_data.name, user.name);
        assert_eq!(remove_data.email, user.email);
        assert_eq!(remove_data.age, user.age);
    }


    #[test]
    fn test_modify_user() {
        let mut users:Vec<UserData> = Vec::new();
        users.modify_user();
    }

    #[test]
    fn test_display_user() {
        let users:Vec<UserData> = Vec::new();
         users.display_users();
    }
    // Add more tests for remove_user, modify_user, etc.
}
