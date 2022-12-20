use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day20_sample.txt", true));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day20.txt", true));
}

fn run(path: &str, part_2: bool) -> Result<(), ErrorMsg> {
    let numbers = utils::read_lines(path)?.map(|s| Ok(
        s?.parse::<i64>()? * (if part_2 {811589153} else {1})
    )).collect::<Result<Vec<i64>, ErrorMsg>>()?;
    let mut index_for_num = (0..numbers.len()).collect::<Vec<_>>();
    let mut num_for_index = (0..numbers.len()).collect::<Vec<_>>();
    for outer_i in 0..(if part_2 {10} else {1})
    {for (i, num) in numbers.iter().enumerate() {
        let prev_index = index_for_num[i];
        let mut new_index_oob = prev_index as i64 + num;
        let max_i = numbers.len() as i64 - 1;
        if new_index_oob < 0 { new_index_oob = (new_index_oob % max_i) + max_i }
        else if new_index_oob >= numbers.len() as i64 { new_index_oob = new_index_oob % max_i }
        let new_index = new_index_oob as usize;
        let should_be_i = num_for_index.remove(prev_index);
        assert_eq!(should_be_i, i);
        num_for_index.insert(new_index, i);
        if new_index > prev_index {
            for ii in prev_index..new_index {
                index_for_num[num_for_index[ii]] = ii;
            }
        } else if new_index < prev_index {
            for ii in (new_index+1)..=prev_index {
                index_for_num[num_for_index[ii]] = ii;
            }
        }
        index_for_num[i] = new_index;
        // println!("{outer_i}:{i}: Nums: {}", num_for_index.iter().map(|i|numbers[*i].to_string() + " ").collect::<String>());
        // println!("{outer_i}:{i}: {}", num_for_index.iter().map(|i|i.to_string() + " ").collect::<String>());
        // println!("{outer_i}:{i}: {}", index_for_num.iter().map(|i|i.to_string() + " ").collect::<String>())
    }
    }
    let index_of_zero = index_for_num[numbers.iter().position(|&n| n == 0).unwrap()];
    let a = numbers[num_for_index[(index_of_zero + 1000) % numbers.len()]];
    let b = numbers[num_for_index[(index_of_zero + 2000) % numbers.len()]];
    let c = numbers[num_for_index[(index_of_zero + 3000) % numbers.len()]];
    Ok(println!("index_of_zero: {index_of_zero}, a: {a}, b: {b}, c: {c}, sum: {}", a+b+c))
}