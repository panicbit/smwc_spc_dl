extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate base64;

use failure::Error;
use std::{
    path::Path,
    fs,
    env::args,
};

fn main() {
    let target_folder = args().nth(1).expect("arg 1 must be the target folder");
    let max_id = args().nth(2).expect("arg 2 must be the maximum id").parse().expect("invalid maximum id");

    for id in (0..=max_id).rev() {
        println!("### {} ###", id);
        for index in 0.. {
            let url = format!("https://www.smwcentral.net/ajax.php?a=getfilepreview&s=smwmusic&id={}&index={}", id, index);
            let preview = reqwest::get(&url)
                .unwrap()
                .error_for_status()
                .unwrap()
                .json::<Option<SPCPreview>>()
                .unwrap();
            let preview = match preview {
                Some(preview) => preview,
                None => break,
            };

            println!("#{:05}[{}]: {}", id, index, preview.filename);

            preview.save_spc(&target_folder).unwrap();

            if index + 1 >= preview.files.len() {
                break;
            }
        }
    }

}

#[derive(Deserialize, Debug)]
struct SPCPreview {
    file: u64,
    files: Vec<String>,
    filename: String,
    data: String,
}

impl SPCPreview {
    fn save_spc(&self, dir: impl AsRef<Path>) -> Result<(), Error> {
        let path = dir.as_ref()
            .join(&self.file.to_string())
            .join(&self.filename);
        let dir = path.parent().expect("path has no parents");
        let data = base64::decode(&self.data)?;

        fs::create_dir_all(dir)?;
        fs::write(&path, data)?;

        Ok(())
    }
}
