use std::collections::BTreeMap;

/// Created by parsing. To be transformed to a process
#[derive(Debug)]
pub struct TokenProcess {
    name: String,
    needs: BTreeMap<String, usize>,
    results: BTreeMap<String, usize>,
    time: usize
}

impl TokenProcess {
    pub fn new(
    	name: String,
    	needs: BTreeMap<String, usize>,
    	results: BTreeMap<String, usize>,
    	time: usize
    ) -> TokenProcess {
    	TokenProcess {
    		name: name,
    		needs: needs,
    		results: results,
    		time: time
    	}
    }
}

pub struct Process ;
