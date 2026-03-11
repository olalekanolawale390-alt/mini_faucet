use crate::blockchain::imports::*;

pub fn allowance() -> Cors{
    return Cors::default().allow_any_header().allow_any_method().allow_any_origin();
}