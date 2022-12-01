use std::str::FromStr;


pub fn part_one() {
    let f: &str = include_str!("input.txt");
    let elf_invs: Vec<&str> = f.split("\n\n")
    .collect();

    let cals: Vec<i32> = elf_invs.iter()
    .map(|e| e.lines() //split on newline
    .map(|e| i32::from_str(e).unwrap()) //convert to int
    .fold(0, |acc, x| acc + x)) //sum the calories for each elf
    .collect();

    println!("{:?}", cals.iter().max().unwrap());
    
}
