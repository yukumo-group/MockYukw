mod synth;

use crate::synth::params;
use crate::synth::errors;

#[unsafe(no_mangle)]
pub extern "C" fn aq_error_message(version: i32, code: i32) -> u32 {
    let text = match errors::host_error_message(code){
        Some(real_error_message) => {
            real_error_message
        }
        None => {
            ""
        }
    };
    0
}
