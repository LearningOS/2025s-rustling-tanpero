// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 让测试通过：使用相等的值
        assert_eq!(1, 1);
        
        // 让测试失败：使用不相等的值
        // 取消下面这行的注释将导致测试失败
        // assert_eq!(1, 2);
    }
}
