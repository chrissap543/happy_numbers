
fn main() {
    let nums = happy_numbers(200); 
    for x in nums {
        println!("{}", x);
    }
}

fn happy_numbers(i: i32) -> Vec<u32> {
    let mut num : u32 = 1; 
    let mut ans = Vec::new(); 

    while ans.len() != (i as usize) {
        let mut path = Vec::new(); 
        let mut next : u32 = get_squared_sum(get_digits(num)); 
        loop {
            if next == 1 {
                ans.push(num); 
                break; 
            }
            if path.contains(&next) {
                break; 
            }
            path.push(next); 
            next = get_squared_sum(get_digits(next)); 
        }
        num += 1; 
    }    

    ans
}

fn get_digits(mut num: u32) -> Vec<u32> {
    let mut ans = Vec::new(); 
    while num != 0 {
        ans.push((num % 10) as u32); 
        num /= 10; 
    }
    ans
}

fn get_squared_sum(digits: Vec<u32>) -> u32 {
    let mut sum : u32 = 0; 
    for x in digits {
        sum += x * x; 
    }
    sum
}