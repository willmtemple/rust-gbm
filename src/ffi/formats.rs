//Rust macro equivalent to C macro included in mesa project
macro_rules! __gbm_fourcc_code {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        (($a as u32) | (($b as u32) << 8) |
        (($c as u32) << 16) | (($d as u32) << 24))
    }
}

//This file is taken directly from mesa/src/gbm/main/gbm.h and rustified.

const GBM_FORMAT_BIG_ENDIAN: u32 = (1<<31);

/* color index */
const GBM_FORMAT_C8: u32 = __gbm_fourcc_code!('C', '8', ' ', ' ');

/* 8 bpp RGB */
const GBM_FORMAT_RGB332: u32 = __gbm_fourcc_code!('R', 'G', 'B', '8');
const GBM_FORMAT_BGR233: u32 = __gbm_fourcc_code!('B', 'G', 'R', '8');

/* 16 bpp RGB */
const GBM_FORMAT_XRGB4444: u32 = __gbm_fourcc_code!('X', 'R', '1', '2');
const GBM_FORMAT_XBGR4444: u32 = __gbm_fourcc_code!('X', 'B', '1', '2');
const GBM_FORMAT_RGBX4444: u32 = __gbm_fourcc_code!('R', 'X', '1', '2');
const GBM_FORMAT_BGRX4444: u32 = __gbm_fourcc_code!('B', 'X', '1', '2');

const GBM_FORMAT_ARGB4444: u32 = __gbm_fourcc_code!('A', 'R', '1', '2');
const GBM_FORMAT_ABGR4444: u32 = __gbm_fourcc_code!('A', 'B', '1', '2');
const GBM_FORMAT_RGBA4444: u32 = __gbm_fourcc_code!('R', 'A', '1', '2');
const GBM_FORMAT_BGRA4444: u32 = __gbm_fourcc_code!('B', 'A', '1', '2');

const GBM_FORMAT_XRGB1555: u32 = __gbm_fourcc_code!('X', 'R', '1', '5');
const GBM_FORMAT_XBGR1555: u32 = __gbm_fourcc_code!('X', 'B', '1', '5');
const GBM_FORMAT_RGBX5551: u32 = __gbm_fourcc_code!('R', 'X', '1', '5');
const GBM_FORMAT_BGRX5551: u32 = __gbm_fourcc_code!('B', 'X', '1', '5');

const GBM_FORMAT_ARGB1555: u32 = __gbm_fourcc_code!('A', 'R', '1', '5');
const GBM_FORMAT_ABGR1555: u32 = __gbm_fourcc_code!('A', 'B', '1', '5');
const GBM_FORMAT_RGBA5551: u32 = __gbm_fourcc_code!('R', 'A', '1', '5');
const GBM_FORMAT_BGRA5551: u32 = __gbm_fourcc_code!('B', 'A', '1', '5');

const GBM_FORMAT_RGB565: u32 = __gbm_fourcc_code!('R', 'G', '1', '6');
const GBM_FORMAT_BGR565: u32 = __gbm_fourcc_code!('B', 'G', '1', '6');

/* 24 bpp RGB */
const GBM_FORMAT_RGB888: u32 = __gbm_fourcc_code!('R', 'G', '2', '4');
const GBM_FORMAT_BGR888: u32 = __gbm_fourcc_code!('B', 'G', '2', '4');

/* 32 bpp RGB */
const GBM_FORMAT_XRGB8888: u32 = __gbm_fourcc_code!('X', 'R', '2', '4');
const GBM_FORMAT_XBGR8888: u32 = __gbm_fourcc_code!('X', 'B', '2', '4');
const GBM_FORMAT_RGBX8888: u32 = __gbm_fourcc_code!('R', 'X', '2', '4');
const GBM_FORMAT_BGRX8888: u32 = __gbm_fourcc_code!('B', 'X', '2', '4');

const GBM_FORMAT_ARGB8888: u32 = __gbm_fourcc_code!('A', 'R', '2', '4');
const GBM_FORMAT_ABGR8888: u32 = __gbm_fourcc_code!('A', 'B', '2', '4');
const GBM_FORMAT_RGBA8888: u32 = __gbm_fourcc_code!('R', 'A', '2', '4');
const GBM_FORMAT_BGRA8888: u32 = __gbm_fourcc_code!('B', 'A', '2', '4');

const GBM_FORMAT_XRGB2101010: u32 = __gbm_fourcc_code!('X', 'R', '3', '0');
const GBM_FORMAT_XBGR2101010: u32 = __gbm_fourcc_code!('X', 'B', '3', '0');
const GBM_FORMAT_RGBX1010102: u32 = __gbm_fourcc_code!('R', 'X', '3', '0');
const GBM_FORMAT_BGRX1010102: u32 = __gbm_fourcc_code!('B', 'X', '3', '0');

const GBM_FORMAT_ARGB2101010: u32 = __gbm_fourcc_code!('A', 'R', '3', '0');
const GBM_FORMAT_ABGR2101010: u32 = __gbm_fourcc_code!('A', 'B', '3', '0');
const GBM_FORMAT_RGBA1010102: u32 = __gbm_fourcc_code!('R', 'A', '3', '0');
const GBM_FORMAT_BGRA1010102: u32 = __gbm_fourcc_code!('B', 'A', '3', '0');

/* packed YCbCr */
const GBM_FORMAT_YUYV: u32 = __gbm_fourcc_code!('Y', 'U', 'Y', 'V');
const GBM_FORMAT_YVYU: u32 = __gbm_fourcc_code!('Y', 'V', 'Y', 'U');
const GBM_FORMAT_UYVY: u32 = __gbm_fourcc_code!('U', 'Y', 'V', 'Y');
const GBM_FORMAT_VYUY: u32 = __gbm_fourcc_code!('V', 'Y', 'U', 'Y');

const GBM_FORMAT_AYUV: u32 = __gbm_fourcc_code!('A', 'Y', 'U', 'V');

/*
 *  * 2 plane YCbCr
 *   * index 0 = Y plane, [7:0] Y
 *    * index 1 = Cr:Cb plane, [15:0] Cr:Cb little endian
 *     * or
 *      * index 1 = Cb:Cr plane, [15:0] Cb:Cr little endian
 *       */
const GBM_FORMAT_NV12: u32 = __gbm_fourcc_code!('N', 'V', '1', '2');
const GBM_FORMAT_NV21: u32 = __gbm_fourcc_code!('N', 'V', '2', '1');
const GBM_FORMAT_NV16: u32 = __gbm_fourcc_code!('N', 'V', '1', '6');
const GBM_FORMAT_NV61: u32 = __gbm_fourcc_code!('N', 'V', '6', '1');

/*
 *  * 3 plane YCbCr
 *   * index 0: Y plane, [7:0] Y
 *    * index 1: Cb plane, [7:0] Cb
 *     * index 2: Cr plane, [7:0] Cr
 *      * or
 *       * index 1: Cr plane, [7:0] Cr
 *        * index 2: Cb plane, [7:0] Cb
 *         */
const GBM_FORMAT_YUV410: u32 = __gbm_fourcc_code!('Y', 'U', 'V', '9');
const GBM_FORMAT_YVU410: u32 = __gbm_fourcc_code!('Y', 'V', 'U', '9');
const GBM_FORMAT_YUV411: u32 = __gbm_fourcc_code!('Y', 'U', '1', '1');
const GBM_FORMAT_YVU411: u32 = __gbm_fourcc_code!('Y', 'V', '1', '1');
const GBM_FORMAT_YUV420: u32 = __gbm_fourcc_code!('Y', 'U', '1', '2');
const GBM_FORMAT_YVU420: u32 = __gbm_fourcc_code!('Y', 'V', '1', '2');
const GBM_FORMAT_YUV422: u32 = __gbm_fourcc_code!('Y', 'U', '1', '6');
const GBM_FORMAT_YVU422: u32 = __gbm_fourcc_code!('Y', 'V', '1', '6');
const GBM_FORMAT_YUV444: u32 = __gbm_fourcc_code!('Y', 'U', '2', '4');
const GBM_FORMAT_YVU444: u32 = __gbm_fourcc_code!('Y', 'V', '2', '4');
