#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

pub fn load_png(path: &str) -> anyhow::Result<Texture> {
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
