use std::borrow::Borrow;
use std::collections::HashSet;
use std::env;
use std::collections as coll;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
// use std::path;
// use std::path;
// use std::ffi;
// use std::str::FromStr;
// use serde_json;
use serde_json;

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
    println!("{}", local_app_data_path.to_str().expect("Didn't parse to &str"));

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

    for path in &term_versions_paths {
        println!("{:?}, {}", path.0, path.1.clone().into_string().unwrap());
    }

    return term_versions_paths;
}

fn get_any_version_path (term_versions_paths : &coll::BTreeMap<TerminalVersion, OsString>) -> (TerminalVersion, OsString) {
    let mut current_term_cfg_name_path_result : Result<(TerminalVersion, OsString), &str> = Err("No windows terminal configuration found.");

    for file_path in term_versions_paths{
        println!("{}", file_path.1.clone().into_string().unwrap());
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
    let _ = fs::write(path_to_config, config_json_data.to_string());
}

fn get_config_string_data( current_term_cfg_name_path : &(TerminalVersion, OsString) ) -> String {
    let contents = fs::read_to_string(current_term_cfg_name_path.1.clone().into_string().unwrap())
        .expect("Loading config file data failed.");
        
    // println!("JSON contents: {}", contents);

    return contents;
}

fn change_bg_image(path_to_img : &OsString, config_json_data : &mut serde_json::Value) {
    fs::metadata(path_to_img)
        .expect("Incorrect path. File not found.");

    config_json_data["profiles"]["defaults"]["backgroundImage"] = path_to_img.clone().into_string().unwrap().into();
}

fn change_bg_image_opacity(opacity_value : f64, config_json_data : &mut serde_json::Value) {
    if opacity_value < 0.0 || opacity_value > 1.0 {
        panic!("Incorrect image opacity value. Correct inputs in range 0.0-1.0")
    }

    config_json_data["profiles"]["defaults"]["backgroundImageOpacity"] = opacity_value.into();
}

fn change_bg_image_alignment(aligment_type : &String, config_json_data : &mut serde_json::Value) {
    let alignment_types = coll::HashSet::from(["center", "left", "top", "right", "bottom", "topLeft", "topRight", "bottomLeft", "bottomRight"]);   

    if !alignment_types.contains(aligment_type.as_str()) {
        panic!("Incorrect aligment type. Possible types: {:#?}", alignment_types);
    }

    config_json_data["profiles"]["defaults"]["backgroundImageAlignment"] = aligment_type.clone().into();
}

fn change_bg_image_stretch_mode(stretch_mode : &String,  config_json_data : &mut serde_json::Value) {
    let stretch_modes = coll::HashSet::from(["none", "fill", "uniform", "uniformToFill"]);

    if !stretch_modes.contains(stretch_mode.as_str()) {
        panic!("Incorrect stretch mode. Possible types: {:#?}", stretch_modes);
    }
    
    config_json_data["profiles"]["defaults"]["backgroundImageStretchMode"] = stretch_mode.clone().into();
}

fn change_term_opacity(opacity_value : i8,  config_json_data : &mut serde_json::Value) {
    if opacity_value < 0 || opacity_value > 100 {
        panic!("Incorrect terminal opacity value. Correct inputs in range 0-100")
    }

    config_json_data["profiles"]["defaults"]["opacity"] = opacity_value.into();
}

fn main() -> Result<(), String> {
    let args : Vec<String> = env::args().collect();
    dbg!(args);

    let versions_to_paths_mapping: coll::BTreeMap<TerminalVersion, OsString> = prep_version_path_struct();

    let current_version_and_path= get_any_version_path(&versions_to_paths_mapping);

    // for specific version
    // let current_version_and_path: (TerminalVersion, OsString) = get_specific_version_path_by_version(&current_term_cfg_name_path_result);

    let config_string_data: String = get_config_string_data(&current_version_and_path);

    let mut config_json_data = get_config_json(&config_string_data);


    println!("{:#?}", config_json_data);

    // change_bg_image_opacity(0.9, &mut config_json_data);
 
    // change_bg_image(&OsString::from("C:/Users/spete/Downloads/forest3.jpg"), &mut config_json_data);

    // change_bg_image_alignment(&"top".to_string(), &mut config_json_data);

    // change_bg_image_stretch_mode(&"fill".to_string(), &mut config_json_data);

    // change_term_opacity(20, &mut config_json_data);


    println!("{:#?}", config_json_data);



    
    // update_config(&config_json_data, &current_version_and_path.1);

    Ok(())
}
