use mammoth_macro::mammoth_module;
use mammoth_setup::prelude::*;

fn constructor(cfg: Option<toml::Value>) -> Module {
    let cfg = cfg.unwrap();
    let m = cfg.as_table().unwrap();
    let x = m.get("x").unwrap().as_integer().unwrap();
    let y = m.get("y").unwrap().as_integer().unwrap();

    Module {x, y}
}

#[mammoth_module(constructor)]
pub struct Module {
    pub x: i64,
    pub y: i64
}

impl MammothInterface for Module {
    fn on_validation(&self, _: &mut Logger) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Log for Module {
    fn register_logger(&mut self, _: AsyncLoggerReference) {
        unimplemented!()
    }

    fn retrieve_logger(&self) -> Option<AsyncLoggerReference> {
        unimplemented!()
    }
}

#[test]
fn test_constructor() {
    let t = r#"
    x = 73
    y = 121
    "#;
    let cfg = Some(toml::from_str(t).unwrap());
    let _ = __construct(cfg);
}

#[test]
fn test_version() {
    let v = __version();

    assert!(mammoth_setup::version::compatible(&v));
}