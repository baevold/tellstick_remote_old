#[cfg(test)]
mod main_test {
    use main;
   
    #[test]
    fn test_retvalue() {
        assert_eq!(2, main::retvalue());
    }
}
