use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub type TextureId = u32;

pub struct Assets {
    m_assets_root: String,
    m_texture_creator: Rc<TextureCreator<WindowContext>>,
    m_textures: HashMap<TextureId, Texture<'static>>,
    m_next_texture_id: TextureId,
}

impl Assets {
    pub fn new<P>(assets_root: P, texture_creator: Rc<TextureCreator<WindowContext>>) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            m_assets_root: assets_root.as_ref().to_string_lossy().into(),
            m_texture_creator: texture_creator,
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

    pub fn load_texture<P>(&mut self, path: P) -> Result<TextureId, String>
    where
        P: AsRef<Path>,
    {
        // SAFETY:
        //
        // We extend the lifetime of `Texture` to `'static` even though it actually
        // borrows from an SDL_Renderer created and owned by `Canvas<Window>`.
        //
        // This is sound ONLY because the following invariants are upheld by the
        // engine architecture:
        //
        // 1. All `Texture` objects stored in `Assets` are created exclusively from
        //    this `TextureCreator`, which in turn was created from the `Canvas`
        //    owned by `Sdl2Context`.
        //
        // 2. `Canvas<Window>` (and therefore the underlying SDL_Renderer) outlives
        //    the `Assets` instance and all `Texture`s stored within it. In practice,
        //    this means `Assets` is always dropped BEFORE `Sdl2Context`.
        //
        // 3. The `Canvas` is created once at startup and is never moved, replaced,
        //    or recreated while any `Texture` exists.
        //
        // 4. No `Texture` is ever accessed after SDL shutdown begins or after the
        //    `Canvas` has been dropped.
        //
        // Rust cannot express this cross-struct lifetime relationship, so we
        // manually uphold it here. Violating ANY of the above invariants will result
        // in undefined behavior (use-after-free of GPU resources).
        let texture: Texture<'_> = self.m_texture_creator.load_texture(path)?;
        let texture: Texture<'static> = unsafe { std::mem::transmute(texture) };

        let texture_id = self.m_next_texture_id;
        self.m_next_texture_id += 1;
        self.m_textures.insert(texture_id, texture);

        Ok(texture_id)
    }

    pub fn get_texture(&self, id: TextureId) -> Option<&Texture<'static>> {
        self.m_textures.get(&id)
    }
}
