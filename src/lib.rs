
pub struct Stack0 {}

impl Stack0 {
    pub fn new() -> Stack0 {
        Stack0 {}
    }

    pub fn push(self: & mut Stack0, value: bool) {
    }

    pub fn top(self: & Stack0) -> bool {
        true
    }

    pub fn pop(self: & mut Stack0) -> bool {
        true
    }

    pub fn size(self: & Stack0) -> u32 {
        1
    }
}

pub struct Stack1 {
    stack: usize
}

impl Stack1 {
    pub fn new() -> Stack1 {
        Stack1 { stack: 1 }
    }

    pub fn push(& mut self, value: bool) -> Result<bool, String>{
        Ok(value)
    }

    pub fn top(& self) -> Result<bool, String> {
        Ok(true)
    }

    pub fn pop(& mut self) -> Result<bool, String> {
        Ok(true)
    }

    pub fn size(& self) -> u32 {
        1
    }
}
// impl BStack0{
//     pub fn new() -> BStack0 {
//         BStack0 { stack: 1 }
//     }

//     pub fn pop(self : & mut BStack0) -> Result<usize, str> {
//         Ok(0)
//     }

//     fn size(self : & BStack0) -> u32 {
//         //what is the first
//         usize::BITS - usize::leading_zeros(self.stack) - 1
//     }

//     fn is_empty(self : & BStack0) -> bool {
//         self.stack == 1
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut stack = Stack1::new();
        stack.push(false);
        stack.push(true);
        let result1 = stack.pop();
        let result2 = stack.pop();
        assert_eq!(result1, Ok(true));
        assert_eq!(result2, Ok(false));
    }

    #[test]
    fn size_increase_when_push() {
        let mut stack = Stack1::new();
        stack.push(false);
        let result1 = stack.size();
        stack.push(true);
        let result2 = stack.size();
        assert_eq!(result2, result1 + 1);
    }

    #[test]
    fn push_and_top() {
        let mut stack = Stack1::new();
        stack.push(false);
        let result1 = stack.top();
        stack.push(true);
        let result2 = stack.top();
        assert_eq!(result1, Ok(false));
        assert_eq!(result2, Ok(true));
    }

    #[test]
    fn size_decreases_when_pop() {
        let mut stack = Stack1::new();
        stack.push(false);
        stack.push(true);
        let result1 = stack.size();
        stack.pop();
        let result2 = stack.size();
        assert_eq!(result1, result2 - 1);
    }

    #[test]
    fn empty_when_created() {
        let result = Stack1::new().size();
        assert_eq!(result, 0);
    }

    #[test]
    fn empty_does_not_pop() {
        let mut stack = Stack1::new();
        let result = stack.pop();
        assert_eq!(result, Err("Empty stack".into()));
    }

}
