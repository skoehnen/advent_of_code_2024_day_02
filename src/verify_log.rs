pub fn verify(data: String) -> i32 {
    return count_safe_reports(validate_report(parse_data(data)));
}

fn count_safe_reports(report_summary: Vec<i32>) -> i32 {
    return report_summary.iter().sum();
}

fn parse_data(data: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        //println!("{}", line);
        let mut inner_vec: Vec<i32> = Vec::new();

        let split_data = line.split(" ");

        for number in split_data {
            //println!("{}", number);
            inner_vec.push(number.parse::<i32>().unwrap());
        }

        result.push(inner_vec);
    }

    return result;
}

fn problem_dampener(data: Vec<i32>, index: usize) -> Vec<i32> {
    println!("problem_dampener remove item {} with index {}", data[index], index);
    let mut result = data.clone();
    result.remove(index);
    return result;
}

fn problem_dampener_detection(data: Vec<i32>) -> Vec<i32> {
    // Check if the values consistently grow/decrease
    if data[0] > data[1] {
        for i in 1..data.len() {
            if data[i] > data[i - 1] {
                return problem_dampener(data, i - 1);
            }
        }
    } else {
        for i in 1..data.len() {
            if data[i] < data[i-1] {
                return problem_dampener(data, i - 1);
            }
        }
    }

    for i in 1..data.len() {
        let difference = data[i] - data[i-1];
        let abs_diff = difference.abs();

        if abs_diff == 0 {
            return problem_dampener(data, i - 1);
        }

        if abs_diff > 3 {
            return problem_dampener(data, i - 1);
        }
    }
    return data;
}

fn check_levels(data: Vec<i32>) -> i32 {
    let clean_data = problem_dampener_detection(data);

    // Check if the values consistently grow/decrease
    if clean_data[0] > clean_data[1] {
        for i in 1..clean_data.len() {
            if clean_data[i] > clean_data[i - 1] {
                return 0;
            }
        }
    } else {
        for i in 1..clean_data.len() {
            if clean_data[i] < clean_data[i-1] {
                return 0;
            }
        }
    }

    for i in 1..clean_data.len() {
        let difference = clean_data[i] - clean_data[i-1];
        let abs_diff = difference.abs();

        if abs_diff == 0 {
            return 0;
        }

        if abs_diff > 3 {
            return 0;
        }
    }

    return 1;
}

fn validate_report(data: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut line_number = 0;

    for line in data {
        println!("Line-number: {:?}", line_number);
        println!("{:?}", line);
        result.push(check_levels(line));
        line_number = line_number + 1;
    }

    println!("{:?}", result);

    return result;
}

#[cfg(test)]
mod tests {
    use std::fs;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_data() {
        let file_path = "data/test.data";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let data = parse_data(contents);

        assert_eq!(data.len(), 6);

        for i in data.iter() {
            assert_eq!(i.len(), 5);
        }

        let line00 = &data[0];
        let line01 = &data[1];
        let line02 = &data[2];
        let line03 = &data[3];
        let line04 = &data[4];
        let line05 = &data[5];

        assert_eq!(line00[0], 7);
        assert_eq!(line00[1], 6);
        assert_eq!(line00[2], 4);
        assert_eq!(line00[3], 2);
        assert_eq!(line00[4], 1);

        assert_eq!(line01[0], 1);
        assert_eq!(line01[1], 2);
        assert_eq!(line01[2], 7);
        assert_eq!(line01[3], 8);
        assert_eq!(line01[4], 9);

        assert_eq!(line02[0], 9);
        assert_eq!(line02[1], 7);
        assert_eq!(line02[2], 6);
        assert_eq!(line02[3], 2);
        assert_eq!(line02[4], 1);

        assert_eq!(line05[0], 1);
        assert_eq!(line05[1], 3);
        assert_eq!(line05[2], 6);
        assert_eq!(line05[3], 7);
        assert_eq!(line05[4], 9);
    }

    #[test]
    fn test_validate_report() {
        let file_path = "data/test.data";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let data = validate_report(parse_data(contents));

        println!("{:?}", data);

        assert_eq!(data.len(), 6);

        assert_eq!(data[0], 1);
        assert_eq!(data[1], 0);
        assert_eq!(data[2], 0);
        assert_eq!(data[3], 1);
        assert_eq!(data[4], 1);
        assert_eq!(data[5], 1);
    }

    #[test]
    fn test_check_levels() {
        let file_path = "data/test.data";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let test_pass_1 = vec![1,2,3,4,5];
        let test_pass_2 = vec![5, 4, 3, 2, 1];
        let test_pass_3 = vec![1, 3, 6, 7, 9];

        let test_fail_1 = vec![1,5,8,12,16];
        let test_fail_2 = vec![16,12,8,5,1];
        let test_fail_3 = vec![1,2,7,8,9];

        let data_pass_1 = check_levels(test_pass_1);
        let data_pass_2 = check_levels(test_pass_2);
        let data_pass_3 = check_levels(test_pass_3);

        let data_fail_1 = check_levels(test_fail_1);
        let data_fail_2 = check_levels(test_fail_2);
        let data_fail_3 = check_levels(test_fail_3);

        assert_eq!(data_pass_1, 1);
        assert_eq!(data_pass_2, 1);
        assert_eq!(data_pass_3, 1);

        assert_eq!(data_fail_1, 0);
        assert_eq!(data_fail_2, 0);
        assert_eq!(data_fail_3, 0);
    }

    #[test]
    fn test_count_safe_reports() {
        let file_path = "data/test.data";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let data = count_safe_reports(validate_report(parse_data(contents)));

        assert_eq!(data, 4);
    }

    #[test]
    fn test_verify() {
        let file_path = "data/test.data";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let data = verify(contents);

        assert_eq!(data, 4);
    }
}