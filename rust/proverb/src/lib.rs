pub fn build_proverb(list: &[&str]) -> String {
    let x = r"h\nola";
    println!("{}",x);
    let y :&[i32] = &[1,2,3];
    let z = &[1,2,3][..];
    println!("{},{:?},{:?}",x, y, z);

    match list.len() {
        0 => String::new(),
        _ => list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                list.first().unwrap()
            )))
            .fold(String::new(), |acc, verse| acc + &verse),
    }
}
