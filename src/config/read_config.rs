use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::config::config_structures::{Brahmaputra};
use crate::config::configuration::GLOBAL;

pub async fn read_config(file_path: String) -> bool{

    // Create a buffer to hold the file contents
    let mut contents = String::new();

    // Open the file asynchronously
    return match File::open(file_path).await {
        Ok(mut file) => {
            // Read the file's contents into the buffer
            match file.read_to_string(&mut contents).await {
                Ok(data) => {
                    GLOBAL::set_app_config(contents).await;
                    true
                }
                Err(err) => {
                    println!("{}", err);
                    false
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}