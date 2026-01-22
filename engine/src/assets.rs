use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::path::{Path, PathBuf};

pub struct AssetDatabase {
    m_assets_root: Option<String>,
}

impl AssetDatabase {
    pub fn new() -> Self {
        Self {
            m_assets_root: None,
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
        let mut path = PathBuf::from(self.m_assets_root.as_ref().unwrap().clone());
        path.extend(parts);

        if !path.exists() {
            panic!("Asset not found: {}", path.display());
        }

        path
    }

    pub fn load_texture<'a, P>(
        &self,
        texture_creater: &'a TextureCreator<WindowContext>,
        path: P,
    ) -> Result<Texture<'a>, String>
    where
        P: AsRef<Path>,
    {
        println!("{}", path.as_ref().display());
        let image = texture_creater.load_texture(path)?;
        Ok(image)
    }
}
