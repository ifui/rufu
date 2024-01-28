// 三元运算函数实现
pub fn ternary_operation<T>(condition: bool, when_true: T, when_false: T) -> T {
    if condition {
        when_true
    } else {
        when_false
    }
}
