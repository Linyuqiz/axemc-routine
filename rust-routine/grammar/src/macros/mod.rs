#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[macros::axemc_macro]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
