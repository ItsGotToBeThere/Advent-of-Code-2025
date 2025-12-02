use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").expect("can't read file");

    let ranges: Vec<_> = lines.split(',').collect();
    let r: Vec<_> = vec!["40403-40405"];

    func1(ranges.clone());
    func2(ranges);
    
}

//This function simply runs through the numbers and compares the first and second half in string
//form
fn func1(ranges: Vec<&str>){
    let mut runsum: u128 = 0;
    for range in ranges{
        let nums: Vec<&str> = range.split('-').collect();
        let start: u128 = nums[0].trim().parse().expect("cant parse num");
        let end: u128 = nums[1].trim().parse().expect("cant parse num");
        'inner: for i in start..=end{
            let numstr = i.to_string();
            let len = numstr.len();
            if len%2 == 1{
                continue 'inner;
            }
            if numstr[..len/2] == numstr[len/2..]{
                runsum+=i;
            }
        }
    }
    println!("{runsum}");
}

//This function loops through all possible loop sizes, seeing if any produce a correct loop
fn func2(ranges: Vec<&str>){
    let mut runsum: u128 = 0;
    for range in ranges{
        let nums: Vec<&str> = range.split('-').collect();
        let start: u128 = nums[0].trim().parse().expect("cant parse num");
        let end: u128 = nums[1].trim().parse().expect("cant parse num");
        'numloop: for num in start..=end{
            let numstr = num.to_string();
            let len = numstr.len();
            let b = numstr.as_bytes();
            let mut increment = 1;
            'inner: while increment <= len/2 {

            //println!("{increment}");
            let mut i = 0;
            let mut j = increment;
            increment+=1;

            //println!("{},{}",b[i],b[j]);
            while b[i]==b[j]{
                i+=1;
                j+=1;
                if j>=len{
                        println!("Close to Repeat: {num}");
                        if i % (increment-1) == 0 {
                        println!("Actual Repeat: {num}");
                        runsum+=num;
                        continue 'numloop;
                        }
                        continue 'inner;
                    }
                }
            }
        }
    }
    println!("{runsum}");
}
