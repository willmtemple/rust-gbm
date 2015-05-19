#![allow(dead_code)]
mod gbm;

#[macro_use]
pub mod formats;
pub use self::gbm::*;

#[cfg(test)]
mod tests {
    #[test]
    fn macro_test() {
        assert!(__gbm_fourcc_code!('X', 'R', '2', '4') == 0x34325258);
        assert!(__gbm_fourcc_code!('\0', '\0', '\0', '\0') == 0);
        assert!(__gbm_fourcc_code!(0xFF, 0x00, 0xFF, 0x00) == 0x00FF00FF);
    }
}
