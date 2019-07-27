pub trait Id {
    type Identifier: Eq;

    fn id(&self) -> Self::Identifier;
    fn description(&self) -> &str {
        "item"
    }
}