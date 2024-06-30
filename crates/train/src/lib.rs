pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn test() {
    let hello = vec![0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let hello = Vec::new();
        assert_eq!(result, 4);
    }
}
