pub struct BStack0 {
    stack: usize
}

impl BStack0{
    pub fn new() -> BStack0 {
        BStack0 { stack: 1 }
    }

    pub fn pop(self : & mut BStack0) -> Result<usize, str> {
        Ok(0)
    }

    fn size(self : & BStack0) -> u32 {
        //what is the first
        usize::BITS - usize::leading_zeros(self.stack) - 1
    }

    fn is_empty(self : & BStack0) -> bool {
        self.stack == 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_when_created() {
        let result = BStack0::new().is_empty();
        assert_eq!(result, true);
    }

    #[test]
    fn size_0_when_created() {
        let result = BStack0::new().size();
        assert_eq!(result, 0);
    }
}
