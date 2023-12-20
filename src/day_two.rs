use super::utils;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Game {
  pub gameId: u8,
  pub grabs: Vec<Grab>
}


#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Grab {
  pub red:     Option<u8>,
  pub green:   Option<u8>,
  pub blue:    Option<u8>,
}

fn game_parser(line: &str) -> nom::IResult<&str,Game>{
    match nom::sequence::tuple(
        gameheader_parser
    )
}

fn challenge_one (input: &str) ->i32{
    return input.lines()
    .map(|game| 
        {
            
        }
    ).sum();
}

pub fn run_challenges() {
    let input_one = utils::read_day(1);
    match input_one {
        Ok(input_one) => {
            let result = challenge_one(&input_one);
            println!("{:?}",result)
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