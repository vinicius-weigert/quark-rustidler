use rand::Rng;

use std::fs;
use std::error::Error;
use std::collections::HashMap;
use std::path::{
    Path, 
    PathBuf
};


pub struct Resource {
    pub ascii: String,
    pub path: PathBuf
}

impl Resource {
    pub fn from(file: PathBuf) -> Result<Resource, Box<dyn Error>> {
        let content = fs::read_to_string(&file)?;

        Ok(Resource {
            path: file,
            ascii: content
        })
    }
}


pub struct ResourceHandler {
    pub resources: HashMap<String, Resource>
}

impl Default for ResourceHandler {
    fn default() -> ResourceHandler {
        ResourceHandler {
            resources: HashMap::new()
        }
    }
}

impl ResourceHandler {
    pub fn new() -> ResourceHandler {
        ResourceHandler::default()
    }

    pub fn load(&mut self, folder: &Path) -> Result<(), Box<dyn Error>> {
        let entries = fs::read_dir(folder)?;

        for entry in entries {
            let entry = entry?.path().canonicalize()?;

            if entry.is_dir() {
                self.load(&entry)?;
            } else {
                //Name generator, for example: logos.0 should return the first logo
                //Strip the path base out
                let identifier = entry
                    .strip_prefix(Path::new("./res/ascii").canonicalize()?)?
                    //Remove file's extension
                    .with_extension("")
                    //Put the shit on an vector, and then join it
                    .iter()
                    .map(|s| s.to_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(".");                    

                self.resources.insert(identifier, Resource::from(entry)?);
            }
        }

        Ok(())
    }

    pub fn get(&self, id: &'static str) -> Option<&Resource> {
        //Yes, that's all
        //Proxy ftw
        self.resources.get(id)
    }
    
    pub fn random(&self, id: &'static str) -> Option<&Resource> {
        let mut keys: Vec<&String> = self.resources
            .keys()
            .filter(|key| key.starts_with(id))
            .collect();
        
        keys.sort();

        
        //Well, there's no shit in here
        if keys.is_empty() {
            return None;
        }

        //Why the fuck would you randomize one thing out of a set that contains only one thing?
        if keys.len() == 1 {
            return self.resources.get(keys[0]);
        }


        let mut rng = rand::thread_rng();
        let number = rng.gen_range(0..=(keys.len()-1));

        self.resources.get(keys[number])
    }

    pub fn random_range(&self, id: &'static str, dbounds: Option<(usize, usize)>) -> Option<&Resource> {
        let mut keys: Vec<&String> = self.resources
            .keys()
            .filter(|key| key.starts_with(id))
            .collect();
        
        keys.sort();

        
        //Well, there's no shit in here
        if keys.is_empty() {
            return None;
        }

        //Why the fuck would you randomize one thing out of a set that contains only one thing?
        if keys.len() == 1 {
            return self.resources.get(keys[0]);
        }

        
        //Poka yoke user bounds for no reason
        let mut bounds = (0, keys.len()-1);

        if dbounds.is_some() {
            if let Some((a, b)) = dbounds {
                if a < b {
                    bounds.0 = a;
                }

                if b < bounds.1 {
                    bounds.1 = b;
                }
            }
        }


        let mut rng = rand::thread_rng();
        let number = rng.gen_range(bounds.0..=bounds.1);


        self.resources.get(keys[number])
    }
}
