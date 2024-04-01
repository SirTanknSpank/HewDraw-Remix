use super::*;
use globals::*;
// status script import
 
mod attack_s4;
mod special_hi;

pub fn install(agent: &mut Agent) {
    attack_s4::install(agent);
    special_hi::install(agent);
}
