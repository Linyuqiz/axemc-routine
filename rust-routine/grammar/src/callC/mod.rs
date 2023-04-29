pub extern crate libc;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
extern "C" {
    pub fn add(x: libc::c_int, y: libc::c_int) -> libc::c_int;
}
