pub fn lines_as_ints(input: &str) -> Vec<u64> {
    return input
        .lines()
        .map(|item| item.to_string().parse().unwrap())
        .collect();
}

pub fn values_as_ints(values: Vec<&str>) -> Vec<u64> {
    return values
        .iter()
        .map(|item| item.to_string().trim().parse().unwrap())
        .collect();
}
