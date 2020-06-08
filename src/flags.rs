// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx_sys;

bitflags! {
    pub struct StateFlags: u64 {
        const STATE_RGB_WRITE = bgfx_sys::BGFX_STATE_RGB_WRITE;
        const STATE_ALPHA_WRITE = bgfx_sys::BGFX_STATE_ALPHA_WRITE;
        const STATE_DEPTH_WRITE = bgfx_sys::BGFX_STATE_DEPTH_WRITE;
        const STATE_DEPTH_TEST_LESS = bgfx_sys::BGFX_STATE_DEPTH_TEST_LESS;
        const STATE_DEPTH_TEST_LEQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_LEQUAL;
        const STATE_DEPTH_TEST_EQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_EQUAL;
        const STATE_DEPTH_TEST_GEQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_GEQUAL;
        const STATE_DEPTH_TEST_GREATER = bgfx_sys::BGFX_STATE_DEPTH_TEST_GREATER;
        const STATE_DEPTH_TEST_NOTEQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_NOTEQUAL;
        const STATE_DEPTH_TEST_NEVER = bgfx_sys::BGFX_STATE_DEPTH_TEST_NEVER;
        const STATE_DEPTH_TEST_ALWAYS = bgfx_sys::BGFX_STATE_DEPTH_TEST_ALWAYS;
        const STATE_DEPTH_TEST_SHIFT = bgfx_sys::BGFX_STATE_DEPTH_TEST_SHIFT as u64;
        const STATE_DEPTH_TEST_MASK = bgfx_sys::BGFX_STATE_DEPTH_TEST_MASK;
        const STATE_BLEND_ZERO = bgfx_sys::BGFX_STATE_BLEND_ZERO;
        const STATE_BLEND_ONE = bgfx_sys::BGFX_STATE_BLEND_ONE;
        const STATE_BLEND_SRC_COLOR = bgfx_sys::BGFX_STATE_BLEND_SRC_COLOR;
        const STATE_BLEND_INV_SRC_COLOR = bgfx_sys::BGFX_STATE_BLEND_INV_SRC_COLOR;
        const STATE_BLEND_SRC_ALPHA = bgfx_sys::BGFX_STATE_BLEND_SRC_ALPHA;
        const STATE_BLEND_INV_SRC_ALPHA = bgfx_sys::BGFX_STATE_BLEND_INV_SRC_ALPHA;
        const STATE_BLEND_DST_ALPHA = bgfx_sys::BGFX_STATE_BLEND_DST_ALPHA;
        const STATE_BLEND_INV_DST_ALPHA = bgfx_sys::BGFX_STATE_BLEND_INV_DST_ALPHA;
        const STATE_BLEND_DST_COLOR = bgfx_sys::BGFX_STATE_BLEND_DST_COLOR;
        const STATE_BLEND_INV_DST_COLOR = bgfx_sys::BGFX_STATE_BLEND_INV_DST_COLOR;
        const STATE_BLEND_SRC_ALPHA_SAT = bgfx_sys::BGFX_STATE_BLEND_SRC_ALPHA_SAT;
        const STATE_BLEND_FACTOR = bgfx_sys::BGFX_STATE_BLEND_FACTOR;
        const STATE_BLEND_INV_FACTOR = bgfx_sys::BGFX_STATE_BLEND_INV_FACTOR;
        const STATE_BLEND_SHIFT = bgfx_sys::BGFX_STATE_BLEND_SHIFT as u64;
        const STATE_BLEND_MASK = bgfx_sys::BGFX_STATE_BLEND_MASK;
        const STATE_BLEND_EQUATION_ADD = bgfx_sys::BGFX_STATE_BLEND_EQUATION_ADD;
        const STATE_BLEND_EQUATION_SUB = bgfx_sys::BGFX_STATE_BLEND_EQUATION_SUB;
        const STATE_BLEND_EQUATION_REVSUB = bgfx_sys::BGFX_STATE_BLEND_EQUATION_REVSUB;
        const STATE_BLEND_EQUATION_MIN = bgfx_sys::BGFX_STATE_BLEND_EQUATION_MIN;
        const STATE_BLEND_EQUATION_MAX = bgfx_sys::BGFX_STATE_BLEND_EQUATION_MAX;
        const STATE_BLEND_EQUATION_SHIFT = bgfx_sys::BGFX_STATE_BLEND_EQUATION_SHIFT as u64;
        const STATE_BLEND_EQUATION_MASK = bgfx_sys::BGFX_STATE_BLEND_EQUATION_MASK;
        const STATE_BLEND_INDEPENDENT = bgfx_sys::BGFX_STATE_BLEND_INDEPENDENT;
        const STATE_CULL_CW = bgfx_sys::BGFX_STATE_CULL_CW;
        const STATE_CULL_CCW = bgfx_sys::BGFX_STATE_CULL_CCW;
        const STATE_CULL_SHIFT = bgfx_sys::BGFX_STATE_CULL_SHIFT as u64;
        const STATE_CULL_MASK = bgfx_sys::BGFX_STATE_CULL_MASK;
        const STATE_ALPHA_REF_SHIFT = bgfx_sys::BGFX_STATE_ALPHA_REF_SHIFT as u64;
        const STATE_ALPHA_REF_MASK = bgfx_sys::BGFX_STATE_ALPHA_REF_MASK;
        const STATE_PT_TRISTRIP = bgfx_sys::BGFX_STATE_PT_TRISTRIP;
        const STATE_PT_LINES = bgfx_sys::BGFX_STATE_PT_LINES;
        const STATE_PT_LINESTRIP = bgfx_sys::BGFX_STATE_PT_LINESTRIP;
        const STATE_PT_POINTS = bgfx_sys::BGFX_STATE_PT_POINTS;
        const STATE_PT_SHIFT = bgfx_sys::BGFX_STATE_PT_SHIFT as u64;
        const STATE_PT_MASK = bgfx_sys::BGFX_STATE_PT_MASK;
        const STATE_POINT_SIZE_SHIFT = bgfx_sys::BGFX_STATE_POINT_SIZE_SHIFT as u64;
        const STATE_POINT_SIZE_MASK = bgfx_sys::BGFX_STATE_POINT_SIZE_MASK;
        const STATE_MSAA = bgfx_sys::BGFX_STATE_MSAA;
        const STATE_RESERVED_MASK = bgfx_sys::BGFX_STATE_RESERVED_MASK;
        const STATE_NONE = bgfx_sys::BGFX_STATE_NONE;
        const STATE_MASK = bgfx_sys::BGFX_STATE_MASK;
        const STATE_DEFAULT = bgfx_sys::BGFX_STATE_DEFAULT;
        const STATE_BLEND_ADD = bgfx_sys::BGFX_STATE_BLEND_ADD;
        const STATE_BLEND_ALPHA = bgfx_sys::BGFX_STATE_BLEND_ALPHA;
        const STATE_BLEND_DARKEN = bgfx_sys::BGFX_STATE_BLEND_DARKEN;
        const STATE_BLEND_LIGHTEN = bgfx_sys::BGFX_STATE_BLEND_LIGHTEN;
        const STATE_BLEND_MULTIPLY = bgfx_sys::BGFX_STATE_BLEND_MULTIPLY;
        const STATE_BLEND_NORMAL = bgfx_sys::BGFX_STATE_BLEND_NORMAL;
        const STATE_BLEND_SCREEN = bgfx_sys::BGFX_STATE_BLEND_SCREEN;
        const STATE_BLEND_LINEAR_BURN = bgfx_sys::BGFX_STATE_BLEND_LINEAR_BURN;
    }
}

impl Default for StateFlags {
    #[inline]
    fn default() -> StateFlags {
        StateFlags::STATE_DEFAULT
    }
}

#[inline]
pub fn state_alpha_ref(aref: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_ALPHA_REF!(aref)).unwrap()
}

#[inline]
pub fn state_point_size(size: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_POINT_SIZE!(size)).unwrap()
}

#[inline]
pub fn state_blend_func_separate(srcrgb: u32, dstrgb: u32, srca: u8, dsta: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_SEPARATE!(srcrgb, dstrgb, srca, dsta)).unwrap()
}

#[inline]
pub fn state_blend_equation_separate(rgb: u32, a: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_EQUATION_SEPARATE!(rgb, a)).unwrap()
}

#[inline]
pub fn state_blend_func(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_SEPARATE!(src, dst, src, dst)).unwrap()
}

#[inline]
pub fn state_blend_equation(equation: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_EQUATION_SEPARATE!(equation, equation)).unwrap()
}

#[inline]
pub fn state_blend_func_rt_x(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_x!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_xe(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_xE!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_1(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_1!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_2(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_2!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_3(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_3!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_1e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_1E!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_2e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_2E!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_3e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_3E!(src, dst, equation) as u64).unwrap()
}

bitflags! {
    pub struct BufferFlags: u16 {
        const BUFFER_NONE = bgfx_sys::BGFX_BUFFER_NONE;
        const BUFFER_COMPUTE_FORMAT_8X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X1;
        const BUFFER_COMPUTE_FORMAT_8X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X2;
        const BUFFER_COMPUTE_FORMAT_8X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X4;
        const BUFFER_COMPUTE_FORMAT_16X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X1;
        const BUFFER_COMPUTE_FORMAT_16X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X2;
        const BUFFER_COMPUTE_FORMAT_16X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X4;
        const BUFFER_COMPUTE_FORMAT_32X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X1;
        const BUFFER_COMPUTE_FORMAT_32X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X2;
        const BUFFER_COMPUTE_FORMAT_32X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X4;
        const BUFFER_COMPUTE_FORMAT_SHIFT = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_SHIFT as u16;
        const BUFFER_COMPUTE_FORMAT_MASK = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_MASK;
        const BUFFER_COMPUTE_TYPE_UINT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_UINT;
        const BUFFER_COMPUTE_TYPE_INT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_INT;
        const BUFFER_COMPUTE_TYPE_FLOAT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_FLOAT;
        const BUFFER_COMPUTE_TYPE_SHIFT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_SHIFT as u16;
        const BUFFER_COMPUTE_TYPE_MASK = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_MASK;
        const BUFFER_COMPUTE_READ = bgfx_sys::BGFX_BUFFER_COMPUTE_READ;
        const BUFFER_COMPUTE_WRITE = bgfx_sys::BGFX_BUFFER_COMPUTE_WRITE;
        const BUFFER_DRAW_INDIRECT = bgfx_sys::BGFX_BUFFER_DRAW_INDIRECT;
        const BUFFER_ALLOW_RESIZE = bgfx_sys::BGFX_BUFFER_ALLOW_RESIZE;
        const BUFFER_INDEX32 = bgfx_sys::BGFX_BUFFER_INDEX32;
        const BUFFER_COMPUTE_READ_WRITE = bgfx_sys::BGFX_BUFFER_COMPUTE_READ_WRITE;
    }
}

impl Default for BufferFlags {
    #[inline]
    fn default() -> BufferFlags {
        BufferFlags::BUFFER_NONE
    }
}

bitflags! {
    pub struct ClearFlags: u16 {
        const CLEAR_NONE = bgfx_sys::BGFX_CLEAR_NONE;
        const CLEAR_COLOR = bgfx_sys::BGFX_CLEAR_COLOR;
        const CLEAR_DEPTH = bgfx_sys::BGFX_CLEAR_DEPTH;
        const CLEAR_STENCIL = bgfx_sys::BGFX_CLEAR_STENCIL;
        const CLEAR_DISCARD_COLOR_0 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_0;
        const CLEAR_DISCARD_COLOR_1 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_1;
        const CLEAR_DISCARD_COLOR_2 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_2;
        const CLEAR_DISCARD_COLOR_3 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_3;
        const CLEAR_DISCARD_COLOR_4 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_4;
        const CLEAR_DISCARD_COLOR_5 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_5;
        const CLEAR_DISCARD_COLOR_6 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_6;
        const CLEAR_DISCARD_COLOR_7 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_7;
        const CLEAR_DISCARD_DEPTH = bgfx_sys::BGFX_CLEAR_DISCARD_DEPTH;
        const CLEAR_DISCARD_STENCIL = bgfx_sys::BGFX_CLEAR_DISCARD_STENCIL;
        const CLEAR_DISCARD_COLOR_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_MASK;
        const CLEAR_DISCARD_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_MASK;
    }
}

impl Default for ClearFlags {
    #[inline]
    fn default() -> ClearFlags {
        ClearFlags::CLEAR_NONE
    }
}

bitflags! {
    pub struct DebugFlags: u32 {
        const DEBUG_NONE = bgfx_sys::BGFX_DEBUG_NONE;
        const DEBUG_WIREFRAME = bgfx_sys::BGFX_DEBUG_WIREFRAME;
        const DEBUG_IFH = bgfx_sys::BGFX_DEBUG_IFH;
        const DEBUG_STATS = bgfx_sys::BGFX_DEBUG_STATS;
        const DEBUG_TEXT = bgfx_sys::BGFX_DEBUG_TEXT;
    }
}

impl Default for DebugFlags {
    #[inline]
    fn default() -> DebugFlags {
        DebugFlags::DEBUG_NONE
    }
}

bitflags! {
    pub struct ResetFlags: u32 {
        const RESET_NONE = bgfx_sys::BGFX_RESET_NONE;
        const RESET_FULLSCREEN = bgfx_sys::BGFX_RESET_FULLSCREEN;
        const RESET_FULLSCREEN_SHIFT = bgfx_sys::BGFX_RESET_FULLSCREEN_SHIFT;
        const RESET_FULLSCREEN_MASK = bgfx_sys::BGFX_RESET_FULLSCREEN_MASK;
        const RESET_MSAA_X2 = bgfx_sys::BGFX_RESET_MSAA_X2;
        const RESET_MSAA_X4 = bgfx_sys::BGFX_RESET_MSAA_X4;
        const RESET_MSAA_X8 = bgfx_sys::BGFX_RESET_MSAA_X8;
        const RESET_MSAA_X16 = bgfx_sys::BGFX_RESET_MSAA_X16;
        const RESET_MSAA_SHIFT = bgfx_sys::BGFX_RESET_MSAA_SHIFT;
        const RESET_MSAA_MASK = bgfx_sys::BGFX_RESET_MSAA_MASK;
        const RESET_VSYNC = bgfx_sys::BGFX_RESET_VSYNC;
        const RESET_MAXANISOTROPY = bgfx_sys::BGFX_RESET_MAXANISOTROPY;
        const RESET_CAPTURE = bgfx_sys::BGFX_RESET_CAPTURE;
        const RESET_HMD = bgfx_sys::BGFX_RESET_HMD;
        const RESET_HMD_DEBUG = bgfx_sys::BGFX_RESET_HMD_DEBUG;
        const RESET_HMD_RECENTER = bgfx_sys::BGFX_RESET_HMD_RECENTER;
        const RESET_FLUSH_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLUSH_AFTER_RENDER;
        const RESET_FLIP_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLIP_AFTER_RENDER;
        const RESET_SRGB_BACKBUFFER = bgfx_sys::BGFX_RESET_SRGB_BACKBUFFER;
        const RESET_HIDPI = bgfx_sys::BGFX_RESET_HIDPI;
    }
}

impl Default for ResetFlags {
    #[inline]
    fn default() -> ResetFlags {
        ResetFlags::RESET_NONE
    }
}
