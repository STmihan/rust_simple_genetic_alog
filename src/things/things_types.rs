use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Thing {
    pub name: Cow<'static, str>,
    pub value: i32,
    pub weight: i32,
}

impl Thing {
    pub fn new(name: String, value: i32, weight: i32) -> Thing {
        return Thing {
            name: Cow::Owned(name),
            value,
            weight,
        };
    }

    pub const fn new_const(name: &'static str, value: i32, weight: i32) -> Thing {
        return Thing {
            name: Cow::Borrowed(name),
            value,
            weight,
        };
    }
}
