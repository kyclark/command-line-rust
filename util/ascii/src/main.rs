use transpose::transpose;

fn main() {
    let range: Vec<u32> = (33..=127).collect();
    let mut nums = vec![0; 95];
    transpose(&range, &mut nums, 19, 5);
    let vals: Vec<String> = nums
        .iter()
        .map(|i| {
            format!(
                "{:3}: {}",
                i,
                if *i == 127 {
                    "DEL".to_string()
                } else {
                    std::char::from_u32(*i).unwrap().to_string()
                }
            )
        })
        .collect();
    let rows: Vec<&[String]> = vals.chunks(5).collect();
    for row in rows {
        println!("{}", row.join("\t"));
    }
}
