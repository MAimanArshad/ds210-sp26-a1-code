fn find_index(target: &str, names: Vec<&str>) -> usize {
    for i in 0..names.len() {
        if target == names[i] {
            return i;
        }
    }
    return names.len();
}

fn main() {
    let names = vec!["Kinan", "Matt", "Taishan", "Zach", "Kesar", "Lingie", "Emir"];
    let grades = vec![ 0,      100,    95,        88,     99,      98,       97];

    let target = "tom";
    let index = find_index(target, names);
    if index < grades.len() {
        let grade = grades[index];
        println!("{grade}");
    } else {
        println!("Not found");
    }
}
