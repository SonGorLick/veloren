use super::{
    super::{util::arr_to_mat, Pipeline, TgtColorFmt, TgtDepthStencilFmt},
    Globals, Light, Shadow,
};
use gfx::{
    self, gfx_defines, gfx_impl_struct_meta, gfx_pipeline, gfx_pipeline_inner,
    gfx_vertex_struct_meta,
    state::{ColorMask, Comparison, Stencil, StencilOp},
};
use vek::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "v_pos",
        norm: [f32; 3] = "v_norm",
        col: [f32; 3] = "v_col",
    }

    vertex Instance {
        inst_mat0: [f32; 4] = "inst_mat0",
        inst_mat1: [f32; 4] = "inst_mat1",
        inst_mat2: [f32; 4] = "inst_mat2",
        inst_mat3: [f32; 4] = "inst_mat3",
        inst_col: [f32; 3] = "inst_col",
        inst_wind_sway: f32 = "inst_wind_sway",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        ibuf: gfx::InstanceBuffer<Instance> = (),

        globals: gfx::ConstantBuffer<Globals> = "u_globals",
        lights: gfx::ConstantBuffer<Light> = "u_lights",
        shadows: gfx::ConstantBuffer<Shadow> = "u_shadows",

        noise: gfx::TextureSampler<f32> = "t_noise",

        tgt_color: gfx::BlendTarget<TgtColorFmt> = ("tgt_color", ColorMask::all(), gfx::preset::blend::ALPHA),
        tgt_depth_stencil: gfx::DepthStencilTarget<TgtDepthStencilFmt> = (gfx::preset::depth::LESS_EQUAL_WRITE,Stencil::new(Comparison::Always,0xff,(StencilOp::Keep,StencilOp::Keep,StencilOp::Keep))),
    }
}

impl Vertex {
    pub fn new(pos: Vec3<f32>, norm: Vec3<f32>, col: Rgb<f32>) -> Self {
        Self {
            pos: pos.into_array(),
            col: col.into_array(),
            norm: norm.into_array(),
        }
    }
}

impl Instance {
    pub fn new(mat: Mat4<f32>, col: Rgb<f32>, wind_sway: f32) -> Self {
        let mat_arr = arr_to_mat(mat.into_col_array());
        Self {
            inst_mat0: mat_arr[0],
            inst_mat1: mat_arr[1],
            inst_mat2: mat_arr[2],
            inst_mat3: mat_arr[3],
            inst_col: col.into_array(),
            inst_wind_sway: wind_sway,
        }
    }
}

impl Default for Instance {
    fn default() -> Self { Self::new(Mat4::identity(), Rgb::broadcast(1.0), 0.0) }
}

pub struct SpritePipeline;

impl Pipeline for SpritePipeline {
    type Vertex = Vertex;
}
