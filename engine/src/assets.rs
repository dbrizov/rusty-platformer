use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub type TextureId = u32;

pub struct Assets<'a> {
    m_assets_root: String,
    m_textures: HashMap<TextureId, Box<Texture<'a>>>,
    m_next_texture_id: TextureId,
}

impl<'a> Assets<'a> {
    pub fn new<P>(assets_root: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            m_assets_root: String::from(assets_root.as_ref().to_str().unwrap()),
            m_textures: HashMap::new(),
            m_next_texture_id: 0,
        }
    }

    pub fn asset_path<I, P>(&self, parts: I) -> PathBuf
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut path = PathBuf::from(&self.m_assets_root);
        path.extend(parts);

        if !path.exists() {
            panic!("Asset not found: {}", path.display());
        }

        path
    }

    pub fn load_texture<P>(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        path: P,
    ) -> Result<TextureId, String>
    where
        P: AsRef<Path>,
    {
        let texture = texture_creator.load_texture(path)?;
        let texture_id = self.m_next_texture_id;
        self.m_textures.insert(texture_id, Box::new(texture));
        self.m_next_texture_id += 1;

        Ok(texture_id)
    }

    pub fn get_texture(&self, id: TextureId) -> Option<&Texture<'a>> {
        self.m_textures.get(&id).map(Box::as_ref)
    }
}
