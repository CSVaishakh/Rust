use std::io::stdin;
fn collatz(mut n: u32)->u32{
    let mut l:u32=0;
    if n==1{
        l+=1;
        return l;
    }else if n>1{
        loop{
            if n%2==1{
                n=n*3+1;
                l+=1;
                print!(" {}",n);
                if n==1{
                    break;
                }
            }
            if n%2==0{
                n=n/2;
                l+=1;
                print!(" {}",n);
                if n==1{
                    break;
                }
            }
        }
    }
    return l;
}
fn main(){
    println!("Enter the value of n");   
    let mut n=String::new();
    let _ = stdin()
        .read_line(&mut n);
    let mut num:u32=0;
    match n.as_str().trim().parse::<u32>() {
        Ok(val) => {num=val},
        Err(error) => {println!("{:?}",error)}
    }
    let ans:u32=collatz(num);
    println!();
    println!("{}",ans);
}