use std::io;

fn compareString(x: &str, y: &str) -> bool
{
    let mut xx = x.chars();
    let mut yy = y.chars();
    loop{
        match(xx.next(), yy.next()){
            (Some(_char1), Some(_char2) )=>{
                if _char1 > _char2 { return true; }
                else if _char1 < _char2 { return false; }
            },
            (Some(_char1), None) => { return true; },
            (None, Some(_char2)) => { return false; },
            (None, None) => { return false; } 
        }
    }
}

fn main()
{
    let mut x = String::new();
    let mut y = String::new();
    println!("input string x: ");
    let _ = io::stdin().read_line(&mut x);
    println!("input string y: ");
    let _ = io::stdin().read_line(&mut y);
    println!("-----------------------------");
    println!("The result of x>y is {}.", compareString(&x, &y));
}