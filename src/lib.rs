pub fn soln1() -> i32 {
    let input = [139301,84565,124180,133902,138726,62665,142967,95598,118044,73234,76476,51634,71582,63619,148430,134733,101537,101140,144543,102233,62048,128633,130113,92531,73820,54964,103485,96364,104119,121954,79215,99235,120179,69237,145584,79193,50684,146481,67783,112741,85024,62298,54083,137704,116561,76862,81410,96341,89992,132926,97955,74751,147553,121496,113303,119671,120871,114278,125628,144275,78826,87092,65883,87517,93974,55358,100922,113304,115728,144556,91728,86367,55283,101841,55454,140703,70706,98173,106920,126984,148960,77909,128304,140036,81044,141419,126770,52787,115783,128647,125986,124506,113935,142203,106404,78433,146573,68575,63563,115616];

    input.iter().map(|num| fule_adder(num / 3 - 2)).sum()
}

fn fule_adder(num : i32) -> i32 {
    let next_fule = num / 3 - 2;
    if next_fule <= 0 {
        num
    } else {
        num + fule_adder(next_fule)
    }
}

pub fn soln2(vec_inp: Option<Vec<usize>>, boot_param: Option<(usize, usize)>) -> usize {
    let mut input: Vec<usize> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,1,6,23,27,2,27,9,31,2,6,31,35,1,5,35,39,1,10,39,43,1,43,13,47,1,47,9,51,1,51,9,55,1,55,9,59,2,9,59,63,2,9,63,67,1,5,67,71,2,13,71,75,1,6,75,79,1,10,79,83,2,6,83,87,1,87,5,91,1,91,9,95,1,95,10,99,2,9,99,103,1,5,103,107,1,5,107,111,2,111,10,115,1,6,115,119,2,10,119,123,1,6,123,127,1,127,5,131,2,9,131,135,1,5,135,139,1,139,10,143,1,143,2,147,1,147,5,0,99,2,0,14,0];
    input[1] = 12;
    input[2] = 2;

    match vec_inp {
        Some(inp) => input = inp,
        None => ()
    }

    match boot_param {
        Some((noun, verb)) => {
            input[1] = noun;
            input[2] = verb;
        }
        None => ()
    }

    for i in 0..input.len() {
        if i % 4 == 0 && input[i] == 99 {
            break;
        } else if i % 4 == 0 && input[i] == 1 {
            let fst = input[i + 1].clone();
            let snd = input[i + 2].clone();
            let thd = input[i + 3].clone();

            input[thd] = input[fst] + input[snd];
        } else if i % 4 == 0 && input[i] == 2 {
            let fst = input[i + 1].clone();
            let snd = input[i + 2].clone();
            let thd = input[i + 3].clone();

            input[thd] = input[fst] * input[snd];
        }

    }

    input[0]
}

pub fn soln2_iter(input0: usize, vec_inp: Option<Vec<usize>>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if input0 == soln2(vec_inp.clone(), Some((noun, verb))) {
                println!("noun: {}, verb: {}", noun, verb);
                return 100 * noun + verb;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn fule_adder_test() {
        let result = 50346;
        assert_eq!(result, fule_adder(33583));
    }

    #[test]
    fn soln2_test_1() {
        let v: Option<Vec<usize>> = Some(vec![1,0,0,0,99]);
        assert_eq!(2, soln2(v, None));
    }
    #[test]
    fn soln2_test_2() {
        let v: Option<Vec<usize>> = Some(vec![1,1,1,4,99,5,6,0,99]);
        assert_eq!(30, soln2(v, None));
    }
    #[test]
    fn soln2_iter_test_1() {
        let v: Option<Vec<usize>> = None;
        assert_eq!(1202, soln2_iter(5305097, v));
    }
}
