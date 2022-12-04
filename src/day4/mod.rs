pub fn day4() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}


fn part_one() -> i32 {
    let f: &str = include_str!("input.txt");
    let inn: Vec<&str> = f.lines().collect();
    let mut total = 0;

    for pair in inn {
        let section_pair: Vec<&str> = pair.split(",").collect();
        let (sections_a, sections_b): (Vec<&str>, Vec<&str>) = (section_pair[0].split("-").collect(), section_pair[1].split("-").collect());
        let (sections_a_start, sections_a_end, sections_b_start, sections_b_end) = (sections_a[0].parse::<i32>().unwrap(), sections_a[1].parse::<i32>().unwrap(), sections_b[0].parse::<i32>().unwrap(), sections_b[1].parse::<i32>().unwrap());
        
        if check_contains(sections_a_start, sections_a_end, sections_b_start, sections_b_end) {
            total += 1;
        }
    }
    total
}


fn part_two() -> i32 {
    let f: &str = include_str!("input.txt");
    let inn: Vec<&str> = f.lines().collect();
    let mut total = 0;

    for pair in inn {
        let section_pair: Vec<&str> = pair.split(",").collect();
        let (sections_a, sections_b): (Vec<&str>, Vec<&str>) = (section_pair[0].split("-").collect(), section_pair[1].split("-").collect());
        let (sections_a_start, sections_a_end, sections_b_start, sections_b_end) = (sections_a[0].parse::<i32>().unwrap(), sections_a[1].parse::<i32>().unwrap(), sections_b[0].parse::<i32>().unwrap(), sections_b[1].parse::<i32>().unwrap());
        
        if check_overlap(sections_a_start, sections_a_end, sections_b_start, sections_b_end) {
            total += 1;
        }
    }
    total
}


fn check_contains(a1: i32, a2: i32, b1: i32, b2: i32) -> bool { (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) }

fn check_overlap(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    let (a, b) = (a1..(a2 + 1), b1..(b2 + 1));
    a.contains(&b1) || a.contains(&b2) || b.contains(&a1) || b.contains(&a2)
}