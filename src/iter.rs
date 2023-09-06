fn plus1(items: Vec<char>) -> Vec<char>
{
    items.iter().map(|&x|(x as u8 + 1) as char).collect()
}

fn main()
{
    let mut v = vec!['a', 'b', 'c', 'd', 'e'];
    v = plus1(v);
    println!("{:?}", v);
}