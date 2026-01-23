use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

pub type TextureId = u32;

pub struct Assets<'a> {
    m_assets_root: Option<String>,
    m_textures: HashMap<TextureId, Rc<Texture<'a>>>,
    m_next_texture_id: TextureId,
}

impl<'a> Assets<'a> {
    pub fn new() -> Self {
        Self {
            m_assets_root: None,
            m_textures: HashMap::new(),
            m_next_texture_id: 0,
        }
    }

    pub fn get_assets_root(&self) -> Option<String> {
        match &self.m_assets_root {
            Some(root) => Some(root.clone()),
            _ => None,
        }
    }

    pub fn set_assets_root<P>(&mut self, assets_root: P)
    where
        P: AsRef<Path>,
    {
        self.m_assets_root = Some(String::from(assets_root.as_ref().to_str().unwrap()));
    }

    pub fn asset_path<I, P>(&self, parts: I) -> PathBuf
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut path = match &self.m_assets_root {
            Some(root) => PathBuf::from(root.clone()),
            None => {
                panic!("Asset root is None");
            }
        };

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
    ) -> Result<Rc<Texture<'a>>, String>
    where
        P: AsRef<Path>,
    {
        let texture = texture_creator.load_texture(path)?;
        let texture_rc = Rc::new(texture);

        self.m_textures
            .insert(self.m_next_texture_id, texture_rc.clone());
        self.m_next_texture_id += 1;

        Ok(texture_rc)
    }
}
