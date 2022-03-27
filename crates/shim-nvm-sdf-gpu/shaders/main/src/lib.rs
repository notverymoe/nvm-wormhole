#![cfg_attr(
    target_arch = "spirv",
    feature(register_attr),
    register_attr(spirv),
    no_std
)]

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use core::panic::PanicInfo;
use glam::{vec2, Vec2, UVec3};
use nvm_sdf::dim2::tree::evaluate_tree;

#[spirv(compute(threads(64)))]
pub fn main_vs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] tmp: &[u32],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] tmp2: &[Vec2],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 2)] out: &mut [f32],
) {
    out[id.x as usize] = evaluate_tree::<8>(tmp, tmp2, 0, 8, vec2(0.0,0.0));
}

#[panic_handler]
fn a(_: &PanicInfo) -> ! {
    loop {}
}