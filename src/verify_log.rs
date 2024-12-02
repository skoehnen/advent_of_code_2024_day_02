pub fn verify(data: String) -> i32 {
    validate_report(parse_data(data));
    return 0;
}

fn count_safe_reports(report_summary: Vec<i32>) -> i32 {
    return 0;
}

fn parse_data(data: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        println!("{}", line);
        let mut inner_vec: Vec<i32> = Vec::new();

        let split_data = line.split(" ");

        for number in split_data {
            println!("{}", number);
            inner_vec.push(number.parse::<i32>().unwrap());
        }

        result.push(inner_vec);
    }

    return result;
}

fn check_levels(data: Vec<i32>) -> i32 {
    return 0;
}

fn validate_report(data: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for line in data {
        result.push(check_levels(line));
    }

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
        assert_eq!(data[3], 0);
        assert_eq!(data[4], 1);
    }
}