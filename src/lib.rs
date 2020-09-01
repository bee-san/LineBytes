/// LineBytes
/// You want to read a file line by line.
/// That file isn't UTF8, so you cannot read into a string or use `.lines()`. 
// This little library solves that problem.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
