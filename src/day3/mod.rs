pub fn day3() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}


fn part_one() -> i32 {
    let f: &str = include_str!("input.txt");
    
    let inn: Vec<&str> = f.lines().collect();
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut sum : i32 = 0;
    for ele in inn {
        let first_compartment = &ele[..(ele.len() / 2)];
        let second_compartment = &ele[((ele.len() / 2))..];

        let mut seen : Vec<char> = vec![];
        for item in first_compartment.chars() { //find what item is in both compartaments
            if second_compartment.contains(item) && !seen.contains(&item) {
                seen.push(item);
                let index = letters.find(item).unwrap();
                sum += index as i32 + 1;
            }
        }
    }

   return sum;
}


fn part_two() -> i32  {
    let f: &str = include_str!("input.txt");

    let inn: Vec<&str> = f.lines().collect();
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut groups = vec![];
    let mut total = 0;

    for ele in inn.chunks(3) {
        groups.push(ele);
    }

    for group in groups {
        let elf1 = group[0];
        let elf2 = group[1];
        let elf3 = group[2];
        
        let mut seen: Vec<char> = vec![];

        for c in elf1.chars() {
            if elf2.contains(c) && elf3.contains(c) && !seen.contains(&c) {
                seen.push(c);
                total += letters.find(c).unwrap() as i32 + 1;
            }
        }

    }
    return total;

}