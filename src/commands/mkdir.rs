use crate::errors::ShellError;
use crate::parser::hir::SyntaxType;
use crate::parser::registry::{CommandConfig, NamedType, PositionalType};
use crate::prelude::*;
use indexmap::IndexMap;
use std::path::{Path, PathBuf};

pub struct Mkdir;

impl Command for Mkdir {
    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        mkdir(args)
    }

    fn name(&self) -> &str {
        "mkdir"
    }

    fn config(&self) -> CommandConfig {
        let named: IndexMap<String, NamedType> = IndexMap::new();

        CommandConfig {
            name: self.name().to_string(),
            positional: vec![PositionalType::mandatory("file", SyntaxType::Path)],
            rest_positional: false,
            named,
            is_sink: false,
            is_filter: false,
        }
    }
}

pub fn mkdir(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let mut full_path = PathBuf::from(args.shell_manager.path());

    match &args.nth(0) {
        Some(Tagged { item: value, .. }) => full_path.push(Path::new(&value.as_string()?)),
        _ => {}
    }

    match std::fs::create_dir_all(full_path) {
        Err(reason) => Err(ShellError::labeled_error(
            reason.to_string(),
            reason.to_string(),
            args.nth(0).unwrap().span(),
        )),
        Ok(_) => Ok(OutputStream::empty()),
    }
}