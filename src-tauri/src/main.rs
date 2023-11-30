// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



fn main() {
    let app = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("error while building tauri application");
  
  let app_window = tauri::WindowBuilder::new(
    &app,
    "spotify_window", /* the unique window label */
    tauri::WindowUrl::External("https://open.spotify.com/?".parse().unwrap())
  ).title("Spotify").build().expect("failed to build window");
  
  app_window.eval(r#"
  document.addEventListener('DOMContentLoaded', () => {
    const style = document.createElement('style')
    style.innerText = '.ButtonInner-sc-14ud5tc-0.hBkswG.encore-over-media-set { display: none; }'
    
    document.body.appendChild(style)
  })
  "#).expect("Inject Failed");

  app.run(|_, _| {});
}
