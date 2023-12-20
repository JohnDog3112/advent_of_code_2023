mod part1;
mod part2;
fn main() {
    let inp = get_input();
    let sol1 = part1::solution(inp.clone());
    println!("part1: {}", sol1);
    let sol2 = part2::solution(inp);
    println!("part2: {}", sol2);
}



pub fn get_input() -> Vec<String> {
    let inp = include_str!("input.txt");

    inp.split('\n').map(|st| st.to_string()).collect()
}
