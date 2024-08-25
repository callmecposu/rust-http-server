use std::{any::Any, collections::HashMap};

// resource hashmap struct
#[derive(Debug)]
pub struct ResourceMap {
    resources: HashMap<String, Box<dyn Any>>,
}

impl ResourceMap {
    pub fn new() -> Self {
        ResourceMap { resources: HashMap::<String, Box<dyn Any>>::new() }
    }

    pub fn add_resource<T: 'static>(&mut self, name: String, value: T) -> &mut Self {
        self.resources.insert(name, Box::new(value));
        return self;
    }

    pub fn get_resource<T: 'static>(&self, name: String) -> Option<&T> {
        let res = match self.resources.get(&name) {
            Some(v) => v,
            None => {
                return None;
            }
        };
        return res.downcast_ref::<T>();
    }
}   