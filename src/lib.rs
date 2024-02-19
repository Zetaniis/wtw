// use core::panic;
use std::collections::HashSet;
use std::env;
use std::collections as coll;
// use std::f32::consts::E;
use std::ffi::OsString;
use std::fs;
use anyhow::Ok;
// use std::ops::ControlFlow;
use serde_json;
use clap::Parser;
use std::path::PathBuf;
use omnipath;
use anyhow::anyhow;

#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
#[derive(Clone)]
// #[derive(strum_macros::Display)]
pub enum TerminalVersion {
    Stable,
    Preview,
    Unpackaged,
}


pub struct ConfigManager {
    json_data : Option<serde_json::Value>,
    terminal_version : Option<TerminalVersion>,
    config_path : Option<OsString>,
}

impl ConfigManager {

    pub fn new() -> ConfigManager {
        ConfigManager{json_data: None, config_path: None, terminal_version: None}
    }

    pub fn exec(&mut self) -> anyhow::Result<()> {

        // println!("Args:");
        // for argument in env::args_os() {
        //     println!("{argument:?}");
        // }

        let cli = Cli::parse();


        // Setting info level of messages
        match cli.message_level {
            0 => println!("Debug mode is off"),
            1 => println!("Debug mode is kind of on"),
            2 => println!("Debug mode is on"),
            _ => println!("Wrong info level. Using normal mode."),
        }

        // Choosing terminal version
        // TODO would be good to somehow merge this with aliases for arg options to have one source of truth

        // let current_version_and_path = parse_terminal_version_and_get_config_version_tuple(&cli)
        
        let string_terminal_version : coll::HashMap< &str, TerminalVersion> = coll::HashMap::from([
            ("stable", TerminalVersion::Stable),
            ("preview", TerminalVersion::Preview),
            ("unpackaged", TerminalVersion::Unpackaged),
            ("s", TerminalVersion::Stable),
            ("p", TerminalVersion::Preview),
            ("u", TerminalVersion::Unpackaged),
        ]);

        let terminal_version_arguments : HashSet<&str> = FromIterator::from_iter(string_terminal_version.keys().cloned());

        let versions_to_paths_mapping: coll::BTreeMap<TerminalVersion, OsString> = self.prep_version_path_struct();


        match cli.terminal_version {
            Some(v) => {
                self.assign_path_and_version_for_specific_version(&string_terminal_version.get(v.as_str())
                        .expect(format!("This version doesn't exist. Possible arguments: {:#?}", terminal_version_arguments).as_str()),
                    &versions_to_paths_mapping)?
            },
            None => {
                self.assign_path_and_version_for_any_version(&versions_to_paths_mapping)?
            }
        };



        // let config_string_data: String = self.create_config_from_path(&current_version_and_path).expect("Loading config file data failed.");

        // let mut config_json_data = self.create_config_from_string_data(&config_string_data);

        self.create_config_from_path(&self.config_path.clone().expect("config_path is None. It should have already data here."))?;


        // Executing features

        if cli.path.is_some() {
            // println!("{:?}", &cli.path.clone().unwrap().unwrap());
            self.change_bg_image(&cli.path.unwrap().unwrap().into_os_string())?;
        };

        if cli.align.is_some() {
            self.change_bg_image_alignment(&cli.align.unwrap())?;
        }

        if cli.image_opacity.is_some() {
            // TODO test this, it could introduce bugs, 
            self.change_bg_image_opacity(cli.image_opacity.unwrap())?;
        }

        if cli.stretch.is_some() {
            self.change_bg_image_stretch_mode(&cli.stretch.unwrap())?;
        }

        if cli.terminal_opacity.is_some() {
            self.change_term_opacity(cli.terminal_opacity.unwrap())?;
        }


        // Saving prettified string to JSON file
        
        self.update_config()?;

        return Ok(());
    }
        
    fn create_config_from_string_data(&mut self, config_string_data : &String ) -> anyhow::Result<()> {
        self.json_data = serde_json::from_str(config_string_data)?;

        return Ok(());
    }

    fn create_config_from_path(&mut self, current_term_cfg_path : &OsString ) ->  anyhow::Result<()> {
        let string_data = fs::read_to_string(current_term_cfg_path.clone().into_string().unwrap());
            // .expect("Loading config file data failed.");
            
        // println!("JSON contents: {}", contents);

        self.create_config_from_string_data(&string_data.unwrap())?;
        return Ok(());
    }
    
    fn update_config(&self) -> anyhow::Result<()> {
        // Save prettified string data. If that can't happen, save raw string data.
        let mut contents = serde_json::to_string_pretty(&self.json_data);
        // .unwrap_or(serde_json::to_string(&self.json_data).)?;

        if contents.is_err() {
            contents = serde_json::to_string(&self.json_data);
        }

        fs::write(&self.config_path.clone().expect("Config_path is None. It should already have data here."), &contents.unwrap())?;

        return Ok(());
    }

    fn prep_version_path_struct(&self) -> coll::BTreeMap<TerminalVersion, OsString> {
        let local_app_data_path = env::var_os("LOCALAPPDATA")
            .expect("%LOCALAPPDATA% enviornmental variable didn't parse.");
        // println!("{}", local_app_data_path.to_str().expect("Didn't parse to &str"));

        // let local_app_data_path_obj = path::Path::new(&local_app_data_path);

        let mut term_versions_paths : coll::BTreeMap<TerminalVersion, OsString> = coll::BTreeMap::from([
            (TerminalVersion::Stable, OsString::from(r#"\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json"#)),
            (TerminalVersion::Preview, OsString::from(r#"\Packages\Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe\LocalState\settings.json"#)),
            (TerminalVersion::Unpackaged, OsString::from(r#"\Microsoft\Windows Terminal\settings.json"#)),
        ]);

        for (_term_ver, path_str) in term_versions_paths.iter_mut() {
                // *path_str = path::Path::new("..")
                //     .join(local_app_data_path.clone())
                //     .join(&path_str).into_os_string();

                let mut build_path = local_app_data_path.clone();
                build_path.push(path_str.clone());
                *path_str = build_path;
        }

        // for path in &term_versions_paths {
        //     println!("{:?}, {}", path.0, path.1.clone().into_string().unwrap());
        // }

        return term_versions_paths;
    }

    fn assign_path_and_version_for_any_version(&mut self, term_versions_paths : &coll::BTreeMap<TerminalVersion, OsString>) -> anyhow::Result<()> {
        let mut current_term_cfg_name_path_result = Err(anyhow!("No windows terminal config file found."));

        for file_path in term_versions_paths{
            // println!("{}", file_path.1.clone().into_string().unwrap());
            match fs::metadata(file_path.1) {
                std::result::Result::Ok(_) => {
                    println!("Config path for {:?} version found.", file_path.0);
                    current_term_cfg_name_path_result = Ok(());
                    self.terminal_version = Some(file_path.0.clone());
                    self.config_path = Some(file_path.1.clone());
                    break;
                }
                Err(_) => {

                }
            }
        };


        let current_term_cfg_name_path = current_term_cfg_name_path_result?;
        println!("Config file for {:?} version will be used.", self.terminal_version.clone().expect("terminal_version is None. It should already have data here."));

        //TODO change names
        return Ok(current_term_cfg_name_path);
    }

    fn assign_path_and_version_for_specific_version(&mut self, version : &TerminalVersion, term_versions_paths : &coll::BTreeMap<TerminalVersion, OsString>) -> anyhow::Result<()> {
        let path = term_versions_paths.get(version);

        if path.is_none() {
            return Err(anyhow!("No such version found."));
        }

        // let specific_path = specific_path.expect("specific_path should not be None here.");

        fs::metadata(&path.expect("specific_path should not be None here."))
            .expect("Versions to paths mapping error. A path should have been found.");

        self.terminal_version = Some(version.clone());
        self.config_path = path.cloned();
            
        return Ok(());
        // return Ok((version.clone(), specific_path.clone()));
    }


    fn change_bg_image(&mut self, path_to_img : &OsString) -> anyhow::Result<()> {
        // TODO: this is too strict, I think. Check if this works with URIs
        // Not sure what signs this allows right now. Research TODO

        // println!("{:?}", path_to_img);
        // Getting absolute path
        let abs_path_result = omnipath::sys_absolute(path_to_img.as_ref());

        // Outputs UNC path on Windows, so discarding this
        // let abs_path_result = std::fs::canonicalize(path_to_img);
        // println!("{:?}", abs_path_result);

        if abs_path_result.is_err(){
            return Err(anyhow!("Incorrect path. Path doesn't exist."));
        }
        
        // fs::metadata(path_to_img).expect("Incorrect path. Image file not found.");
            // return Err("Incorrect path. Image file not found.".to_string());

        self.json_data.as_mut().expect("json_data is None. It should have already data here.")["profiles"]["defaults"]["backgroundImage"] = abs_path_result.unwrap().to_string_lossy().into();
        return Ok(());
    }

    fn change_bg_image_opacity(&mut self, opacity_value : u8) -> anyhow::Result<()> {
        if opacity_value > 100 {
            return Err(anyhow!("Incorrect image opacity value. Correct inputs in range 0-100"));
            // panic!("Incorrect image opacity value. Correct inputs in range 0.0-1.0")
        }


        self.json_data.as_mut().expect("json_data is None. It should have already data here.")["profiles"]["defaults"]["backgroundImageOpacity"] = ((opacity_value as f64) / 100.0).into();
        return Ok(());
    }

    fn change_bg_image_alignment(&mut self, aligment_type : &String) -> anyhow::Result<()> {
        let alignment_types = coll::HashSet::from(["center", "left", "top", "right", "bottom", "topLeft", "topRight", "bottomLeft", "bottomRight"]);   

        if !alignment_types.contains(aligment_type.as_str()) {
            return Err(anyhow!("Incorrect aligment type. Possible types: {:#?}", alignment_types));
            // panic!("Incorrect aligment type. Possible types: {:#?}", alignment_types);
        }

        self.json_data.as_mut().expect("json_data is None. It should have already data here.")["profiles"]["defaults"]["backgroundImageAlignment"] = aligment_type.clone().into();
        return Ok(());
    }

    fn change_bg_image_stretch_mode(&mut self, stretch_mode : & String) -> anyhow::Result<()> {
        let stretch_modes = coll::HashSet::from(["none", "fill", "uniform", "uniformToFill"]);

        if !stretch_modes.contains(stretch_mode.as_str()) {
            return Err(anyhow!("Incorrect stretch mode. Possible types: {:#?}", stretch_modes));
            // panic!("Incorrect stretch mode. Possible types: {:#?}", stretch_modes);
        }
        
        self.json_data.as_mut().expect("json_data is None. It should have already data here.")["profiles"]["defaults"]["backgroundImageStretchMode"] = stretch_mode.clone().into();
        return Ok(());
    }

    fn change_term_opacity(&mut self, opacity_value : u8) -> anyhow::Result<()> {
        if opacity_value > 100 {
            return Err(anyhow!("Incorrect terminal opacity value. Correct inputs in range 0-100"));
            // panic!("Incorrect terminal opacity value. Correct inputs in range 0-100")
        }

        self.json_data.as_mut().expect("json_data is None. It should have already data here.")["profiles"]["defaults"]["opacity"] = opacity_value.into();
        return Ok(());
    }




}


// Arg parser struct
#[derive(Parser)]
#[command(version, about = "Tool for setting background image properties and terminal opacity for Windows Terminal. Updates 'default' property in configuration JSON.", long_about = None, arg_required_else_help = true)]
pub struct Cli {
    // /// Optional name to operate on
    // name: Option<String>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    // #[command(subcommand)]
    // command: Option<Commands>,

    /// Choose terminal version. Default will act on the first found. 
    #[arg(short, long, action = clap::ArgAction::Append)]
    pub terminal_version: Option<String>,

    /// Use image as background
    #[arg(short, long, value_name = "PATH_TO_IMAGE")]
    pub path: Option<Option<PathBuf>>,

    // /// Choose a random image from paths inputted or a folder
    // doesnt work for mutliple args - parses first arg and throws error
    // #[arg(short = 'r', long, action = clap::ArgAction::Append)]
    // random_image: Option<Vec<String>>,

    /// Change opacity of the image (% value)
    #[arg(short = 'o', long, action = clap::ArgAction::Append)]
    pub image_opacity: Option<u8>,

    /// Change alignment type of background image (% value)
    #[arg(short, long, value_name = "ALIGNMENT_TYPE")]
    pub align: Option<String>,

    /// Change stretch mode of background image
    #[arg(short, long, value_name = "STRETCH_MODE")]
    pub stretch: Option<String>,

    /// Change opacity of the terminal
    #[arg(short = 'O', long, action = clap::ArgAction::Append)]
    pub terminal_opacity: Option<u8>,

    /// Set message level
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub message_level: u8,
}

// #[derive(Clone)]
// struct VecStringOrString {
//     v: Vec<String>,
//     s: String
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// does testing things
//     Test {
//         /// lists test values
//         #[arg(short, long)]
//         list: bool,
//     },
// }

// fn prepare_arg_parser() -> Command {

//     return matches;
// }