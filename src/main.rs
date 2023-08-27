use serde_json::Value;
use tokio::{ io::AsyncWriteExt, fs::File };
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::PathBuf;
use std::path::Path;
use mslnk::ShellLink;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Fetching latest release from github...");
  let client = reqwest::Client::new();

  let resp = client.get("https://api.github.com/repos/phaze-the-dumb/VRChat-Photo-Manager/releases/latest")
    .header("user-agent", "VRChat Photo Manager Installer (0.1.0)")
    .send()
    .await?
    .text()
    .await?;

  let parsed: Value = serde_json::from_str(resp.as_str()).unwrap();

  if !Path::new("C:\\temp").exists() {
    std::fs::create_dir("C:\\temp").unwrap();
  }

  println!("Downloading version {}...", parsed.get("tag_name").unwrap().as_str().unwrap());

  let download_url = parsed.get("assets").unwrap().get(0).unwrap().get("browser_download_url").unwrap().as_str().unwrap();

  let download_resp = client.get(download_url)
    .send()
    .await?;

  let body = download_resp.bytes().await?;

  let mut out = File::create("C:\\temp\\VRChat-Photo-Manager.tar.gz").await?;

  out.write_all(&body).await?;
  println!("Finished downloading. Unpacking File...");

  let tar_gz = std::fs::File::open("C:\\temp\\VRChat-Photo-Manager.tar.gz").unwrap();
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);

  let prefix = "dist/win-unpacked";
  let path: PathBuf = PathBuf::from("C:\\Program Files\\Phaze\\VRChatPhotoManager\\");

  if !Path::new(&path).exists() {
    std::fs::create_dir(&PathBuf::from("C:\\Program Files\\Phaze")).unwrap();
    std::fs::create_dir(&path).unwrap();
  }

  println!("Extracting files...");
  archive
    .entries()?
    .filter_map(|e| e.ok())
    .map(| mut entry | -> Result<PathBuf, u8> {
      let path = path.join(entry.path().unwrap().strip_prefix(prefix).unwrap().to_owned());
      println!("{}", path.display());

      entry.unpack(&path).unwrap();
      return Ok(path);
    })
    .filter_map(|e: Result<PathBuf, _>| e.ok())
    .for_each(|x| println!("> {}", x.display()));

  println!("Finished extracting files...");
  std::fs::remove_file("C:\\temp\\VRChat-Photo-Manager.tar.gz").unwrap();

  let users = std::fs::read_dir("C:\\Users").unwrap();
  let target = r"C:\\Program Files\\Phaze\\VRChatPhotoManager\\VRChat Photo Manager.exe";

  println!("{:#?}", users);

  for user in users{
    let folder = user.unwrap();
    let user = folder.path();

    let lnk = user.to_str().unwrap().to_owned() + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs";

    if !Path::new(&lnk).exists() { continue; }
    println!("{}", lnk);

    let sl = ShellLink::new(target).unwrap();
    sl.create_lnk(lnk + "\\VRChat Photo Manager.lnk").unwrap();
  }

  println!("Done! See you later!");
  Ok(())
}