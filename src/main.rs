fn main() {
    println!("Hello, world!");
}


#[cfg(tests)]
mod tests {

    #[test]
    fn simple_test() {
        assert_eq!(0, 0);
    }


}
