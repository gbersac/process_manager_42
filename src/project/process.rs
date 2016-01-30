use std::collections::BTreeMap;
use super::{ResourcePtr};

/// Created by parsing. To be transformed to a process.
#[derive(Debug)]
pub struct TokenProcess {
    pub name: String,
    pub prerequisites: BTreeMap<String, usize>,
    pub products: BTreeMap<String, usize>,
    pub time: usize
}

impl TokenProcess {
    pub fn new(
    	name: String,
    	prerequisites: BTreeMap<String, usize>,
    	products: BTreeMap<String, usize>,
    	time: usize
    ) -> TokenProcess {
    	TokenProcess {
    		name: name,
    		prerequisites: prerequisites,
    		products: products,
    		time: time
    	}
    }
}

#[derive(Debug)]
struct ResourceQt
{
    res: ResourcePtr,
    quantities: usize
}

impl ResourceQt
{
    pub fn new(res: ResourcePtr, quantities: usize ) -> ResourceQt
    {
        ResourceQt {
            res: res,
            quantities: quantities
        }
    }
}

#[derive(Debug)]
pub struct Process {
    name: String,
    prerequisites: Vec<ResourceQt>,
    products: Vec<ResourceQt>,
    time: usize
}

impl Process {
    fn map_to_resqt(map: Vec<(ResourcePtr, usize)>) -> Vec<ResourceQt> {
        let mut to_return = Vec::new();
        for (key, value) in map {
            to_return.push(ResourceQt::new(key, value));
        }
        to_return
    }

    pub fn new(
        name: String,
        prerequisites: Vec<(ResourcePtr, usize)>,
        products: Vec<(ResourcePtr, usize)>,
        time: usize
    ) -> Process {
        Process {
            name: name,
            prerequisites: Process::map_to_resqt(prerequisites),
            products: Process::map_to_resqt(products),
            time: time
        }
    }
}
