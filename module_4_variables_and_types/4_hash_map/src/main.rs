use std::collections::HashMap;

fn main() {
    let names = vec!["Kinan", "Matt", "Taishan", "Zach", "Kesar", "Lingie", "Emir"];
    let grades = vec![ 0,      100,    95,        88,     99,      98,       97];

    // Let us turn the names and grades into a hashmap!
    let mut map = HashMap::new();
    for i in 0..names.len() {
        map.insert(names[i], grades[i]);
    }
    
    // Now, map[<name>] gives us the grade without having to search for the name!
    let target = "tom";
    let grade = map.get(target);
    match grade {
        Some(value) => println!("{value}"),
        None=> println!("not found")
    }
    
    let target = "Kinan";
    let grade = map.get(target);
    match grade {
        Some(value) => println!("{value}"),
        None=> println!("not found")
    }
    
    let target = "Kesar";
    let grade = map.get(target);
    match grade {
        Some(value) => println!("{value}"),
        None=> println!("not found")
    }
}
