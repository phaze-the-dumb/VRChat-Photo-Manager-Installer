use std::{ fs, time::Duration, process::Command };

fn main() {
  let client = reqwest::blocking::Client::new();

  let container_folder = dirs::config_dir().unwrap().join("PhazeDev/VRChatPhotoManager");
  match fs::metadata(&container_folder){
    Ok(meta) => {
      if meta.is_file(){
        panic!("Cannot launch app as the container path is a file not a directory");
      }
    },
    Err(_) => {
      fs::create_dir(&container_folder).unwrap();
    }
  }

  let latest_version = client.get("https://cdn.phaz.uk/vrcpm/latest")
    .send().unwrap().text().unwrap();

  println!("Downloading VRChat Photo Manager version: {}", latest_version);

  #[cfg(target_os = "windows")]
  let latest_file = client.get(format!("https://cdn.phaz.uk/vrcpm/builds/vrcpm-{}.exe", latest_version))
    .timeout(Duration::from_secs(120))
    .send().unwrap().bytes().unwrap();

  #[cfg(target_os = "linux")]
  let latest_file = client.get(format!("https://cdn.phaz.uk/vrcpm/builds/vrcpm-{}", latest_version))
    .timeout(Duration::from_secs(120))
    .send().unwrap().bytes().unwrap();

  #[cfg(target_os = "windows")]
  fs::write(container_folder.join("vrchat-photo-manager.exe"), latest_file).unwrap();

  #[cfg(target_os = "linux")]
  fs::write(container_folder.join("vrchat-photo-manager"), latest_file).unwrap();

  println!("File downloaded...");

  #[cfg(target_os = "windows")]
  let mut cmd = Command::new(container_folder.join("vrchat-photo-manager.exe"));

  #[cfg(target_os = "linux")]
  let mut cmd = Command::new(container_folder.join("vrchat-photo-manager"));

  cmd.current_dir(container_folder);
  cmd.spawn().expect("Cannot run VRChat Photo Manager");
}