use gfx;
use image;
use find_folder::Search;
use gfx::handle::{Buffer, Sampler, ShaderResourceView, RenderTargetView, DepthStencilView};
use gfx::format::Formatted;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Srgba8;
pub type ShaderType = <ColorFormat as Formatted>::View;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
        normal: [f32; 3] = "aNormal",
        uv: [f32; 2] = "aTexCoord",
    }

    constant Transform {
        model: [[f32; 4]; 4] = "model",
        view: [[f32; 4]; 4] = "view",
        projection: [[f32; 4]; 4] = "projection",
    }

    constant Light {
        ambient: [f32; 3] = "light_ambient",
        padding1: f32 = "pad1", // prevents the shader/code offset mismatcherror
        diffuse: [f32; 3] = "light_diffuse",
        padding2: f32 = "pad2", // prevents the shader/code offset mismatch error
        specular: [f32; 3] = "light_specular",
        padding3: f32 = "pad3", // prevents the shader/code offset mismatch error
        pos: [f32; 3] = "light_pos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        light: gfx::ConstantBuffer<Light> = "Light",
        // TextureSampler cannot reside in constants? 'Copy trait not implemented'
        shininess: gfx::Global<f32> = "material_shininess",
        diffuse: gfx::TextureSampler<ShaderType> = "material_diffuse",
        specular: gfx::TextureSampler<ShaderType> = "material_specular",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline light_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex { pos, normal, uv }
    }
}

impl Light {
    pub fn new(ambient: [f32; 3], diffuse: [f32; 3], specular: [f32; 3], pos: [f32; 3]) -> Light {
        Light {
            ambient,
            padding1: 0.0,
            diffuse,
            padding2: 0.0,
            specular,
            padding3: 0.0,
            pos,
        }
    }
}

pub fn load_texture<F, R>(
    factory: &mut F,
    path: &str,
) -> ShaderResourceView<R, ShaderType>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    let path = Search::ParentsThenKids(4, 4).for_folder(path).unwrap();
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<ColorFormat>(kind, &[&img])
        .unwrap();
    view
}

pub struct ObjectBrush<R: gfx::Resources> {
    transform: Buffer<R, Transform>,
    light: Buffer<R, Light>,
    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    sampler: Sampler<R>,
}

impl<R: gfx::Resources> ObjectBrush<R> {
    pub fn new<F>(factory: &mut F) -> ObjectBrush<R>
    where
        F: gfx::Factory<R>,
    {
        let transform = factory.create_constant_buffer(1);
        let light = factory.create_constant_buffer(1);
        let pso = factory
            .create_pipeline_simple(
                include_bytes!("shader/obj_vertex.glsl"),
                include_bytes!("shader/obj_fragment.glsl"),
                pipe::new(),
            )
            .expect("Cannot create PSO for object");
        let sampler = factory.create_sampler_linear();
        ObjectBrush {
            transform,
            light,
            pso,
            sampler,
        }
    }

    pub fn draw<C>(
        &self,
        render_target: &RenderTargetView<R, ColorFormat>,
        depth: &DepthStencilView<R, DepthFormat>,
        encoder: &mut gfx::Encoder<R, C>,
    ) where
        C: gfx::CommandBuffer<R>,
    {

    }
}

pub struct Material<R: gfx::Resources> {
    pub diffuse: ShaderResourceView<R, ShaderType>,
    pub specular: ShaderResourceView<R, ShaderType>,
    pub shininess: f32,
}

impl<R: gfx::Resources> Material<R> {
    pub fn new<F>(
        factory: &mut F,
        diffuse_texture_path: &str,
        specular_texture_path: &str,
        shininess: f32,
    ) -> Material<R> where F: gfx::Factory<R> {
        let diffuse = load_texture(factory, diffuse_texture_path);
        let specular = load_texture(factory, specular_texture_path);
        Material {
            diffuse,
            specular,
            shininess,
        }
    }
}

pub struct Object<R: gfx::Resources> {
    vertex_buffer: Buffer<R, Vertex>,
    slice: gfx::Slice<R>,
    pub materail: Material<R>,
}
