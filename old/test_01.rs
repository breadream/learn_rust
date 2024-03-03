fn return_two() -> i8 {
    2
}

fn return_six() -> i8 {
    4 + return_two()
}

#[cfg(test)]
mod tests {
    use super::*; // tes module needs to use the functions above it
    
    #[test]
    fn it_returns_six() {
        assert_eq!(return_six(), 6)
    }
    #[test]
    fn it_returns_two() {
        assert_eq!(return_two(), 2);
    }
}
