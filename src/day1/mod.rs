use std::str::FromStr;


pub fn day1() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}


fn common() -> Vec<i32> {
    let f: &str = include_str!("input.txt");
    let elf_invs: Vec<&str> = f.split("\n\n")
    .collect();

    let mut cals: Vec<i32> = elf_invs.iter()
    .map(|e| e.lines() //split on newline
    .map(|e| i32::from_str(e).unwrap()) //convert to int
    .fold(0, |acc, x| acc + x)) //sum the calories for each elf
    .collect();

    cals.sort();
    cals.reverse();

    return cals;
}


fn part_one() -> i32{
    let cals: Vec<i32> = common();
    return cals[0];
}


fn part_two() -> i32 {
    let cals: Vec<i32> = common();
    let total_cals : i32 = cals[..3].iter().sum(); //get first 3 elems and sum them
    return total_cals;

}
