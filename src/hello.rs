pub fn say_hi(_name: String) -> String {
    let first = "Hello ".to_string();
    return first + &_name;
}

 #[cfg(test)]
mod test {

    #[test]
    fn hello_world_with_rust() {
        assert_eq!("Hello somkiat", crate::hello::say_hi("somkiat".to_string()));
    }
}