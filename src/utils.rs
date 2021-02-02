use std::convert::TryInto;
type ConvertErr = <RGB as std::convert::TryFrom<std::vec::Vec<u8>>>::Error;
fn vec_to_RGB(vec: Vec<u8>) -> Result<RGB, ConvertErr> {
    vec.try_into()
}
