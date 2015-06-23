use main;

#[cfg(test)]
mod tests {
    #[test]
    fn test_retvalue() {
        assert_eq!(2, main::retvalue());
    }
}


