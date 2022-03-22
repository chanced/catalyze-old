#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_new_standalone_generator() {
        let kitchen = fs::File::open("../proto_op/kitchen.bin").unwrap();
        let mut gen =
            catalyze::Generator::new_standalone(&kitchen, &["kitchen.proto"], "").unwrap();
        let res = gen.render().unwrap();
    }
}
