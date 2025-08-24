use crate::{
    RendiumDrawHandle, RendiumInstance,
    types::{Color, Vector2},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

fn load_texture(path: &str) -> anyhow::Result<Texture> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path)?;
    let decoder = png::Decoder::new(BufReader::new(file));
    let mut reader = decoder.read_info()?;

    let mut data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut data)?;
    data.truncate(info.buffer_size());

    Ok(Texture {
        data,
        width: info.width,
        height: info.height,
    })
}

#[derive(Clone)]
pub struct GPUTexture {
    pub bind_group: wgpu::BindGroup,
    sampler: wgpu::Sampler,
    view: wgpu::TextureView,
}

fn create_gpu_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bind_group_layout: &wgpu::BindGroupLayout,
    texture: &Texture,
) -> GPUTexture {
    let size = wgpu::Extent3d {
        width: texture.width,
        height: texture.height,
        depth_or_array_layers: 1,
    };

    let wgpu_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &wgpu_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &texture.data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * texture.width),
            rows_per_image: Some(texture.height),
        },
        size,
    );

    let view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    GPUTexture {
        bind_group,
        sampler,
        view,
    }
}

pub trait TextureLoad {
    fn load_texture(&mut self, filename: &str, tex_name: &str) -> anyhow::Result<()>;
}

impl TextureLoad for RendiumInstance {
    fn load_texture(&mut self, filename: &str, tex_name: &str) -> anyhow::Result<()> {
        if let Some(state) = &self.state {
            let texture = load_texture(filename)?;
            let bind_group_layout = state.render_pipeline.get_bind_group_layout(0);
            let gpu_texture =
                create_gpu_texture(&state.device, &state.queue, &bind_group_layout, &texture);
            self.texture_storage
                .insert(tex_name.to_string(), gpu_texture);
        }

        Ok(())
    }
}

pub trait DrawTexture {
    fn draw_texture(&mut self, name: &str, pos: Vector2, size: Vector2, col: Color);
}

impl DrawTexture for RendiumDrawHandle {
    fn draw_texture(&mut self, name: &str, pos: Vector2, size: Vector2, col: Color) {
        let tex_index = match get_texture_index(self, name) {
            Some(i) => i,
            None => {
                return;
            }
        };

        let Vector2(x, y) = pos;
        let Vector2(w, h) = size;

        let base = self.vertices.len() as u32;

        self.add_vertex([x, y, 0.0], col, [0.0, 0.0], tex_index);
        self.add_vertex([x + w, y, 0.0], col, [1.0, 0.0], tex_index);
        self.add_vertex([x + w, y + h, 0.0], col, [1.0, 1.0], tex_index);
        self.add_vertex([x, y + h, 0.0], col, [0.0, 1.0], tex_index);

        self.add_index(base);
        self.add_index(base + 1);
        self.add_index(base + 2);

        self.add_index(base);
        self.add_index(base + 2);
        self.add_index(base + 3);
    }
}

fn get_texture_index(d: &RendiumDrawHandle, name: &str) -> Option<u32> {
    d.textures
        .keys()
        .enumerate()
        .find(|(_, key)| key == &name)
        .map(|(i, _)| i as u32)
}
