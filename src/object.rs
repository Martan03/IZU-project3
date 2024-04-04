#[derive(Debug)]
pub struct Object {
    pub id: usize,
    pub class: String,
    pub attr: Vec<String>,
}

impl Object {
    /// Creates new object
    pub fn new<T>(id: usize, class: T, attr: Vec<String>) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            id,
            class: class.as_ref().to_string(),
            attr,
        }
    }
}
