use core::panic;
use std::collections::HashSet;
// use std::borrow::Borrow;
// use std::collections::HashSet;
use std::env;
use std::collections as coll;
// use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
// use clap::ValueEnum;
// use std::path;
// use std::path;
// use std::ffi;
// use std::str::FromStr;
// use serde_json;
use serde_json;
use clap::Parser;
use std::path::PathBuf;

#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
#[derive(Clone)]
// #[derive(strum_macros::Display)]
enum TerminalVersion {
    Stable,
    Preview,
    Unpackaged,
}

fn prep_version_path_struct() -> coll::BTreeMap<TerminalVersion, OsString> {
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

fn get_any_version_path_by_version (term_versions_paths : &coll::BTreeMap<TerminalVersion, OsString>) -> (TerminalVersion, OsString) {
    let mut current_term_cfg_name_path_result : Result<(TerminalVersion, OsString), &str> = Err("No windows terminal configuration found.");

    for file_path in term_versions_paths{
        // println!("{}", file_path.1.clone().into_string().unwrap());
        match fs::metadata(file_path.1) {
            Ok(_) => {
                println!("Config path for {:?} version found.", file_path.0);
                current_term_cfg_name_path_result = Ok( (file_path.0.clone(), file_path.1.clone()) );
                break;
            }
            Err(_) => {

            }
        }
    };


    let current_term_cfg_name_path = match current_term_cfg_name_path_result {
        Ok(v) => {
            println!("Config file for {:?} version will be used.", v.0);
            v.clone()
        }
        Err(v) => {
            panic!("{}", v);
        }
    };

    return current_term_cfg_name_path;
}

fn get_specific_version_path_by_version( version : &TerminalVersion, term_versions_paths : &coll::BTreeMap<TerminalVersion, OsString>) -> (TerminalVersion, OsString) {
    return (version.clone(), term_versions_paths.get(version).unwrap().clone() );

}

fn get_config_json( config_string_data : &String ) -> serde_json::Value {
    let config_json_data : serde_json::Value = serde_json::from_str(config_string_data).unwrap();

    return config_json_data;
}

fn update_config( config_json_data : &serde_json::Value, path_to_config : &OsString) {
    // TODO this looks ugly split it and make sensible error handling
    let _ = fs::write(path_to_config, & serde_json::to_string_pretty(&config_json_data).unwrap_or(serde_json::to_string(&config_json_data).unwrap()));
}

fn get_config_string_data( current_term_cfg_name_path : &(TerminalVersion, OsString) ) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(current_term_cfg_name_path.1.clone().into_string().unwrap());
        // .expect("Loading config file data failed.");
        
    // println!("JSON contents: {}", contents);

    return contents;
}

fn change_bg_image(path_to_img : &OsString, config_json_data : &mut serde_json::Value) -> Result<(), String> {
    // TODO: this is too strict, I think. Check if this works with URIs
    if fs::metadata(path_to_img).is_err() {
        return Err("Incorrect path. Image file not found.".to_string());
    }
        // .expect("Incorrect path. File not found.");

    config_json_data["profiles"]["defaults"]["backgroundImage"] = path_to_img.clone().into_string().unwrap().into();
    return Ok(());
}

fn change_bg_image_opacity(opacity_value : u8, config_json_data : &mut serde_json::Value) -> Result<(), String> {
    if opacity_value > 100 {
        return Err(format!("Incorrect image opacity value. Correct inputs in range 0-100"));
        // panic!("Incorrect image opacity value. Correct inputs in range 0.0-1.0")
    }


    config_json_data["profiles"]["defaults"]["backgroundImageOpacity"] = ((opacity_value as f64) / 100.0).into();
    return Ok(());
}

fn change_bg_image_alignment(aligment_type : &String, config_json_data : &mut serde_json::Value) -> Result<(), String> {
    let alignment_types = coll::HashSet::from(["center", "left", "top", "right", "bottom", "topLeft", "topRight", "bottomLeft", "bottomRight"]);   

    if !alignment_types.contains(aligment_type.as_str()) {
        return Err(format!("Incorrect aligment type. Possible types: {:#?}", alignment_types));
        // panic!("Incorrect aligment type. Possible types: {:#?}", alignment_types);
    }

    config_json_data["profiles"]["defaults"]["backgroundImageAlignment"] = aligment_type.clone().into();
    return Ok(());
}

fn change_bg_image_stretch_mode(stretch_mode : & String,  config_json_data : & mut serde_json::Value) -> Result<(), String> {
    let stretch_modes = coll::HashSet::from(["none", "fill", "uniform", "uniformToFill"]);

    if !stretch_modes.contains(stretch_mode.as_str()) {
        return Err(format!("Incorrect stretch mode. Possible types: {:#?}", stretch_modes));
        // panic!("Incorrect stretch mode. Possible types: {:#?}", stretch_modes);
    }
    
    config_json_data["profiles"]["defaults"]["backgroundImageStretchMode"] = stretch_mode.clone().into();
    return Ok(());
}

fn change_term_opacity(opacity_value : u8,  config_json_data : &mut serde_json::Value) -> Result<(), String> {
    if opacity_value > 100 {
        return Err("Incorrect terminal opacity value. Correct inputs in range 0-100".to_string());
        // panic!("Incorrect terminal opacity value. Correct inputs in range 0-100")
    }

    config_json_data["profiles"]["defaults"]["opacity"] = opacity_value.into();
    return Ok(());
}

// Arg parser struct
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
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

fn main() -> Result<(), String> {
    // let args : Vec<String> = env::args().collect();
    // dbg!(args);

    let cli = Cli::parse();

    // panic!("poggers");

    // Setting info level of messages
    match cli.message_level {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Wrong info level. Using normal mode."),
    }

    // Choosing terminal version
    let string_terminal_version : coll::HashMap< &str, TerminalVersion> = coll::HashMap::from([
        ("stable", TerminalVersion::Stable),
        ("preview", TerminalVersion::Preview),
        ("unpackaged", TerminalVersion::Unpackaged),
        ("s", TerminalVersion::Stable),
        ("p", TerminalVersion::Preview),
        ("u", TerminalVersion::Unpackaged),
    ]);

    let terminal_version_arguments : HashSet<&str> = FromIterator::from_iter(string_terminal_version.keys().cloned());

    let versions_to_paths_mapping: coll::BTreeMap<TerminalVersion, OsString> = prep_version_path_struct();

    let current_version_and_path: (TerminalVersion, OsString) = match cli.terminal_version {
        Some(v) => {
            get_specific_version_path_by_version(&string_terminal_version.get(v.as_str())
            .expect(format!("Wrong version. Possible arguments: {:#?}", terminal_version_arguments).as_str()),
            &versions_to_paths_mapping)
        },
        None => {
           get_any_version_path_by_version(&versions_to_paths_mapping)
        }
    };



    let config_string_data: String = get_config_string_data(&current_version_and_path).expect("Loading config file data failed.");

    let mut config_json_data = get_config_json(&config_string_data);


    // Tests
    // TODO

    // println!("{:#?}", config_json_data);

    // change_bg_image_opacity(0.9, &mut config_json_data);
 
    // change_bg_image(&OsString::from("C:/Users/spete/Downloads/forest3.jpg"), &mut config_json_data);

    // change_bg_image_alignment(&"top".to_string(), &mut config_json_data);

    // change_bg_image_stretch_mode(&"fill".to_string(), &mut config_json_data);

    // change_term_opacity(20, &mut config_json_data);

    // println!("{:#?}", config_json_data);


    // Executing features

    if cli.path.is_some() {
        change_bg_image(&cli.path.unwrap().unwrap().into_os_string(), &mut config_json_data).unwrap();
    };

    if cli.align.is_some() {
        change_bg_image_alignment(&cli.align.unwrap(), &mut config_json_data).unwrap();
    }

    if cli.image_opacity.is_some() {
        // TODO test this, it could introduce bugs, 
        change_bg_image_opacity(cli.image_opacity.unwrap(), &mut config_json_data).unwrap();
    }

    if cli.stretch.is_some() {
        change_bg_image_stretch_mode(&cli.stretch.unwrap(), &mut config_json_data).unwrap();
    }

    if cli.terminal_opacity.is_some() {
        change_term_opacity(cli.terminal_opacity.unwrap(), &mut config_json_data).unwrap();
    }


    // Saving prettified string to JSON file
    
    update_config(&config_json_data, &current_version_and_path.1);

    Ok(())

    //TODO use expects in debug mode, use custom function that only returns the expect in normal mode


}
