pub fn _solution(inp: Vec<String>) -> u32 {
    let spelled_digits = ["one","two","three","four","five","six","seven","eight","nine"];
    let number_digits = ['1','2','3','4','5','6','7','8','9'];

    let first_stage = inp.into_iter().map(|st| {
        let mut digits = vec![];
        let chars = st.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            let slice = &st[i..];
            for (j, ch) in number_digits.iter().enumerate() {
                if slice.starts_with(spelled_digits[j]) || chars[i] == *ch {
                    digits.push(j+1);
                }
            }
        }
        
        let out = match (digits.first(), digits.last()) {
            (None, None) => 0,
            (Some(first), None) => first*11,
            (Some(first), Some(second)) => first*10 + second,
            (None, Some(_)) => unreachable!(),
        };
        out as u32
    }).collect::<Vec<u32>>();


    first_stage.iter().sum()
}

pub fn solution(inp: Vec<String>) -> u32 {
    let first = find(&inp, &|a: &str, b: &str| a.find(b).map(|a| a as i32));
    let second = find(&inp, &|a: &str, b: &str| a.rfind(b).map(|a| -(a as i32)));

    first.iter().zip(second.iter()).map(|(&first, &second)| first*10 + second).sum::<usize>() as u32
    
}
const SPELLED_DIGITS: [&str; 9] = ["one","two","three","four","five","six","seven","eight","nine"];
const NUMBER_DIGITS: [&str; 9] = ["1","2", "3", "4", "5", "6", "7", "8", "9"];

pub fn find(inp: &[String], find: &dyn Fn(&str, &str) -> Option<i32>) -> Vec<usize> {
    inp.iter().map(|st| {
        SPELLED_DIGITS.iter().enumerate().zip(NUMBER_DIGITS.iter()).filter_map(|((i, &sp_digit), &digit)| {
            Some((match (find(st, sp_digit), find(st, digit)) {
                (None, None) => return None,
                (None, Some(b)) => b,
                (Some(a), None) => a,
                (Some(a), Some(b)) => a.min(b),
            }, i+1))
        }).min_by(|a, b| a.0.cmp(&b.0)).unwrap_or((0,0)).1
    }).collect()
}