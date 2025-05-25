// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn calculate_gpus(gpus:&str,intelutilitylevel:&str, generatorlevel:&str) -> String {
    if gpus != "" {
    let newgpus = gpus.parse::<f32>().unwrap();
    let newintelutilitylevel = intelutilitylevel.parse::<f32>().unwrap();
    let new_generatorlevel = generatorlevel.parse::<f32>().unwrap();
    let tkperminute = newgpus  * (1.0+(0.5 * newintelutilitylevel));
    let gashours = 6.0 + new_generatorlevel * 6.0;
    return format!("{0}///{1}///{2}///{3}///{4}", tkperminute/60.0 ,  tkperminute ,  tkperminute *60.0 ,  tkperminute*1440.0 ,  tkperminute*gashours*60.0).to_string();
    }
    else {return gpus.to_string();}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calculate_gpus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
