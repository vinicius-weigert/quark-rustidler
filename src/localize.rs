use log::*;

use std::{
    fs,
    error::Error,
    collections::HashMap,
    path::{
        Path,
        PathBuf
    }
};

use fluent::{
    FluentArgs,
    FluentBundle,
    FluentResource
};

use unic_langid::langid;


pub struct LangBundle {
    pub lang_name: String,
    pub bundle: FluentBundle<FluentResource>
}

impl LangBundle {
    pub fn from_file(file: PathBuf) -> Result<(String, LangBundle), Box<dyn Error>> {
        let filename = file.file_stem().unwrap();
        let filename = filename.to_str().unwrap();
        let filename = filename.to_string();
        info!("Loading locale: {}", filename);
        info!("Loading getting content");
        let content = fs::read_to_string(file.clone())?;

        //Try to create the resource from the file
        info!("Creating fluent resource");
        let fluent_resource = FluentResource::try_new(content)
            .map_err(|e| format!("Failed creating resource. {:?}", e))?;

        //Create bundle and add the resource
        info!("Creating bundle");
        let mut bundle = FluentBundle::new(vec![langid!("en-US")]);
        bundle.add_resource(fluent_resource)
            .map_err(|e| format!("Failed adding resource to bundle. {:?}", e))?;
        //Disable bidi isolation characters
        bundle.set_use_isolating(false);

        //Check if there's language-name set into the ftl file
        let pattern = bundle.get_message("language-name");
        if pattern.is_none() {
            Err(format!("Missign language-name identifier"))?
        }
        let pattern = pattern.unwrap().value();
        if pattern.is_none() {
            Err(format!("Missign language-name identifier"))?
        }

        //Pass it to a string
        let mut errors = vec![];
        let lang_name = bundle.format_pattern(&pattern.unwrap(), None, &mut errors).to_string();
        if errors.len() > 0 {
            Err(format!("Failed to format pattern for {:?}. {:?}", file, errors))?
        }

        info!("Done loading {}", filename);

        Ok((filename, LangBundle { lang_name, bundle }))
    }
}


pub struct Localizor {
    pub selected_lang: String,
    pub resources: HashMap<String, LangBundle>
}

impl Localizor {
    //Create a new instance and load the bundles
    pub fn new() -> Localizor {
        Localizor {
            resources: HashMap::new(),
            selected_lang: "".to_string()
        }
    }

    //Load folder
    pub fn load(&mut self, folder: &Path) -> Result<(), Box<dyn Error>> {
        self.selected_lang = "en-US".to_string();
        self.from_folder(folder)?;

        let resource = self.resources.get("en-US");
        if resource.is_none() {
            Err("Missign english base translation".to_string())?
        }

        Ok(())
    }

    //Select language
    pub fn select_lang(&mut self, lang: String) -> Result<(), Box<dyn Error>> {
        if !self.resources.contains_key(&*lang) {
            Err(format!("Cannot find resource lang {}", lang))?
        }

        self.selected_lang = lang.clone();
        info!("Selected lang {}", lang);

        Ok(())
    }

    //Get string
    pub fn get(&self, id: &'static str, args: Option<&FluentArgs>) -> Result<String, Box<dyn Error>> {
        let mut resource = self.resources.get(&self.selected_lang).unwrap();

        //If the requested message isn't present on the current bundle
        if !resource.bundle.has_message(id) {
            //Try to get it from en-US base bundle
            resource = self.resources.get("en-US").unwrap();

            //RIP, the message doesn't exists
            if !resource.bundle.has_message(id) {
                Err(format!("Message's id \"{}\" doesn't exist", id))?
            }
        }

        //Get the message
        let pattern = resource.bundle.get_message(id).unwrap().value();

        //Proccess it!
        let mut errors = vec![];
        let result = resource.bundle.format_pattern(&pattern.unwrap(), args, &mut errors);

        if errors.len() > 0 {
            /*No need to crash, change for warning*/
            Err(format!("Failed to format pattern for {:?} id. {:?}", id, errors))?
        }

        Ok(result.to_string())
    }

    //Load locale folder
    fn from_folder(&mut self, folder: &Path) -> Result<(), Box<dyn Error>> {
        if !folder.is_dir() {
            error!("Cannot find locale folder.");
        }

        //Get every file in the locale folder
        info!("Reading locale dir");
        for entry in fs::read_dir(folder)? {
            let file = entry?.path();

            //No dirs allowed
            if file.is_dir() {
                continue;
            }

            //Parse file to resource
            let (id, resource) = match LangBundle::from_file(file.clone()) {
                Ok(result) => result,
                Err(e) => {
                    warn!("Couldn't load resource from file {:#?}. Skipping it, {:#?}", file, e);
                    continue;
                }
            };
            //Insert resource on the hashmap
            info!("Inserting {} into locale resources", id);
            self.resources.insert(id, resource);
        }

        Ok(())
    }
}
