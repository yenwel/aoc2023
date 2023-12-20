use super::utils;

fn challenge_one (input: &str) ->i32{
    return input.split("\r\n")
    .map(|line| 
        {
            let digits : Vec<_> = (*line)
            .chars()
            .filter(
                |char| 
                char.is_digit(10)
            ).collect();
            let firstdigit = digits.first().map(|c| (c.to_string()).parse::<i32>() ).unwrap().unwrap();
            let lastdigit = digits.last().map(|c| (c.to_string()).parse::<i32>() ).unwrap().unwrap();
            println!("{:?}", (firstdigit*10) + lastdigit);
            return (firstdigit*10) + lastdigit
        }
    ).sum();
}

fn challenge_two (input: &str) ->i32{
    return challenge_one(
        &input
        .replace("one","1")
        .replace("two","2")
        .replace("three","3")
        .replace("four","4")
        .replace("five","5")
        .replace("six","6")
        .replace("seven","7")
        .replace("eight","8")
        .replace("nine","9")
    );
}

pub fn run_challenges() {
    let input_one = utils::read_day(1);
    match input_one {
        Ok(input_one) => {
            let result = challenge_one(&input_one);
            println!("{:?}",result)
            /*let result_two = challenge_two(&input_one);
            println!("{:?}", result_two);
            match result_two {
                Some((first, second, third)) => println!("{:?}",first*second*third),
                None => {}
            }*/
        },
        Err(e) => println!("{}",e)
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(1, None);
        match input {
            Ok(input) =>{
                let result = challenge_one(&input);
                assert_eq!(result,142)
            }
            Err(_) => assert!(false)
        }        
    }

    
    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(1, Some("strings"));
        match input {
            Ok(input) =>{
                let result = challenge_two(&input);
                assert_eq!(result,281)
            }
            Err(_) => assert!(false)
        }        
    }

}