use std::{ fs, time::Duration, process::Command };

fn main() {
  let client = reqwest::blocking::Client::new();

  let container_folder = dirs::home_dir().unwrap().join("AppData\\Roaming\\PhazeDev\\VRChatPhotoManager");
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

  let latest_file = client.get(format!("https://cdn.phaz.uk/vrcpm/builds/vrcpm-{}.exe", latest_version))
    .timeout(Duration::from_secs(120))
    .send().unwrap().bytes().unwrap();

  fs::write(container_folder.join("vrchat-photo-manager.exe"), latest_file).unwrap();
  println!("File downloaded...");

  let mut cmd = Command::new(container_folder.join("vrchat-photo-manager.exe"));
  cmd.current_dir(container_folder);
  cmd.spawn().expect("Cannot run VRChat Photo Manager");
}