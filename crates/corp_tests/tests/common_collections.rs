use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

#[test]
fn vector_collcection() {
    let _ = [1, 2, 3];
    let _: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3, 4, 5];

    match v.get(2) {
        Some(third) => println!("The third element is {}", &third),
        None => println!("No third element"),
    }
}

#[test]
fn string_slices() {
    let mut foo = String::from("foo");

    {
        foo.push_str("bar");
    }
    foo.push('!');

    println!("{}", foo);

    for c in foo.chars() {
        println!("{}", c);
    }

    for g in "здраво".graphemes(true) {
        print!("{}", g);
        print!("-");
    }
    println!();
}

#[test]
fn hash_maps() {
    let police = String::from("police");
    let military = String::from("military");

    let mut scores = HashMap::new();

    scores.insert(police, 10);
    scores.insert(military, 50);

    let fraction_name = String::from("military");
    let score = scores.get(&fraction_name);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    assert_eq!(score, Some(&50));
}

#[test]
fn hash_maps_updating() {
    let mut scores = HashMap::new();

    scores.insert(String::from("military"), 10);
    scores.insert(String::from("military"), 50);

    scores.entry(String::from("police")).or_insert(30);
    scores.entry(String::from("police")).or_insert(40);

    let expected = scores.get("police");

    assert_eq!(expected, Some(&30));
}

#[test]
fn count_fraction_members() {
    let members = "military corp police corp";

    let mut map = HashMap::new();

    for member in members.split_whitespace() {
        let count = map.entry(member).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
