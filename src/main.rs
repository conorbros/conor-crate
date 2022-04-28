fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        let x = true;
        assert!(x);
    }
}
