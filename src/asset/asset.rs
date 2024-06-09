use std::sync::{Arc, RwLock};
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use thiserror::Error;
use crate::asset::asset::YarnSpinnerDialogLoaderError::Io;
use crate::parsing::components::YarnSpinnerNode;
use crate::parsing::yarn_spinner_parsing;

#[derive(Asset, TypePath, Debug)]
pub struct YarnSpinnerDialog {
    pub nodes: Vec<Arc<RwLock<YarnSpinnerNode>>>,
}

#[derive(Default)]
pub struct YarnSpinnerDialogLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum YarnSpinnerDialogLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parsing error")]
    ParsingError,
    #[error("Unkown node in jump_line: {0}")]
    UnknownNode(String)
}

impl AssetLoader for YarnSpinnerDialogLoader {
    type Asset = YarnSpinnerDialog;
    type Settings = ();
    type Error = YarnSpinnerDialogLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async {
            let mut file_content = String::new();
            reader.read_to_string(&mut file_content).await?;
            let nodes = yarn_spinner_parsing::load_from_file(file_content.as_str());
            nodes.map(|nodes| YarnSpinnerDialog { nodes })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yarn"]
    }
}
