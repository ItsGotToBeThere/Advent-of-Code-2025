use std::fs;

fn part1(lines: Vec<&str>) {
    let mut zeros = 0;
    let mut current: i16 = 50;

    for line in lines{
        let direction = &line[..1];
        let num = &line[1..];
        let num: i16 = num.trim().parse().expect("not a num");
        if direction=="L"{
            current = (current - num) % 100; 
        }
        else{
            current = (current + num) % 100;
        }
        if current==0{
            zeros+=1;
        }
    }
    println!("{zeros}");
}

fn part2(lines: Vec<&str>){
    let mut zeros = 0;
    let mut current: i32 = 50;

    for line in lines{
        let direction = &line[..1];
        let num = &line[1..];
        let num: i32 = num.trim().parse().expect("not a num");
        if direction=="L"{
            let divis = num/100;
            zeros += divis; //adds full loops around 0 to zeros

            let not_zero = current!=0;
            let num = num % 100;
            current-= num;
            if num!=0{
                if not_zero{
                    if current<=0{
                        zeros+=1;
                        println!("Direction: {direction}, Previous: {}, After: {current}",current+num);
                    }
                }
            }
            current = current.rem_euclid(100); 
        }
        else{
            let divis = num/100;
            zeros += divis; //adds full loops around 0 to zeros

            let num = num % 100;
            current+=num;
            if num!=0{
                if current >= 100{
                    zeros+=1;
                    println!("Direction: {direction}, Previous: {}, After: {current}",current-num);
                }
            }
            current%=100
        }
    }
    println!("{zeros}");
}

fn main() {
    let lines = fs::read_to_string("input.txt").expect("could not read file");
    let lines: Vec<_> = lines.lines().collect();

    part2(lines);
}
