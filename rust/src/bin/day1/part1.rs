pub fn solution(inp: Vec<String>) -> u32{
    let digits = inp.into_iter().map(|str| 
        str.chars().filter_map(|ch| ch.to_digit(10)).collect::<Vec<u32>>()
    );
    let numbers = digits.map(|digits| {
        let mut digits = digits.iter();
        match (digits.next(), digits.last()) {
            (None, None) => 0,
            (Some(first), None) => first*11,
            (Some(first), Some(last)) => first * 10 + last,
            (None, Some(_)) => unreachable!(),
        }
    });

    numbers.sum::<u32>()
    
}