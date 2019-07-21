#![feature(custom_attribute)]

use mammoth_macro::mammoth_module;
use mammoth_setup::MammothInterface;

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
    /* implementation */
}

#[test]
fn test_constructor() {
    let t = r#"
    x = 73
    y = 121
    "#;
    let cfg = Some(toml::from_str(t).unwrap());
    let m = __construct(cfg);

    assert_eq!(m.x, 73);
    assert_eq!(m.y, 121);
}

#[test]
fn test_version() {
    let v = __version();

    assert_eq!(v, semver::Version::new(0,0,1));
}