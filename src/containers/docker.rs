use shiplift::Docker;
use tokio;

#[tokio::main]
pub async fn containers_list() {
    let docker = Docker::new();
    match docker.containers().list(&Default::default()).await {
        Ok(containers) => {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        }
        Err(e) => println!("Error: {}", e)
    }
}

#[tokio::main]
pub async fn images_list() {
    let docker = Docker::new();
    match docker.images().list(&Default::default()).await {
        Ok(images) => {
            for image in images {
                println!("{:?}", image.repo_tags);
            }
        },
        Err(e) => println!("Something bad happened! {}", e),
    }
}