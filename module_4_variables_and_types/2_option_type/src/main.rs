fn find_index(target: &str, names: Vec<&str>) -> Option<usize> {
    for i in 0..names.len() {
        if target == names[i] {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    let names = vec!["Kinan", "Matt", "Taishan", "Zach", "Kesar", "Lingie", "Emir"];
    let grades = vec![ 0,      100,    95,        88,     99,      98,       97];

    let target = "tom";
    let i = find_index(target, names);
    if i.is_none() {
        println!("Not found");
    } else {
        let value = i.unwrap();
        let grade = grades[value];
        println!("{grade}");
    }
}