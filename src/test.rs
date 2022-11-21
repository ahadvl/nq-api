/// generate the test value of sturct
pub trait Test {
    fn test() -> Self 
    where 
        Self: Sized;
}