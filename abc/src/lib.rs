#[cfg(test)]
mod tests {
    #[cfg(feature = "ghi")]
    #[test]
    fn fail() {
        panic!("ghi feature is switched on");
    }

    #[cfg(not(feature = "def"))]
    #[test]
    fn it_works() {
        panic!("def feature is not switched on");
    }
}
