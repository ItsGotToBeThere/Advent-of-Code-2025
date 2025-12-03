use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").expect("could not read file");
    let lines: Vec<_> = lines.lines().collect();


    let result = func2(lines);
    println!("{result}");
}


fn func1(lines: Vec<&str>) -> u32{
    let mut runsum = 0;
    for line in lines{
        let mut maxnum = 0;
        let mut maxindex = 0;

        for (i,char) in line.chars().enumerate(){
            let charnum = char.to_digit(10).expect("not a digit");
            if i!=line.len()-1{
                if charnum > maxnum{
                    maxnum = charnum;
                    maxindex = i;
                }
            }
        }
        let mut secondmax = 0;
        for (j,char) in line.chars().enumerate(){
            if j <= maxindex{
                continue;
            } 
            let charnum = char.to_digit(10).expect("not a digit");
            if charnum > secondmax{
                secondmax = charnum
            }
        }
    println!("Index1: {maxindex} {maxnum}{secondmax}");
    runsum+=(10*maxnum)+secondmax;
    }
    runsum
}

fn func2(lines: Vec<&str>) -> u128 {
    let mut runsum = 0;
    for line in lines{
        let mut linesum = 0;
        let mut previndex = 0;
        for j in (1..=12).rev(){
            let endindex = line.len()+1-j;
            let (i,num) = loop_through(&line[previndex..endindex],previndex);
            previndex = i+1;
            linesum+=10_u128.pow((j as u32)-1) * num as u128;
        }
        println!("{linesum}");
        runsum+=linesum;
    }
    runsum
}

fn loop_through(line: &str, start: usize) -> (usize,u32){
    let mut maxnum: u32 = 0;
    let mut maxindex: usize = 0;
    for (i,char) in line.chars().enumerate() {
        let charnum = char.to_digit(10).expect("not a digit");
        
        if charnum > maxnum{
            maxnum = charnum;
            maxindex = i;
        }
    }
    (start+maxindex,maxnum)
}
