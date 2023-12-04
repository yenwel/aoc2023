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
        let input = utils::read_test(1, None);;
        match input {
            Ok(input) =>{
                let result = challenge_one(&input);
                assert_eq!(result,142)
            }
            Err(_) => assert!(false)
        }        
    }

}