use shiplift::
{Docker, LogsOptions, ContainerOptions, PullOptions, Error, Container, Image};
use futures::StreamExt;
use tokio;
use futures::executor::block_on;
use std::sync::{Once, Mutex};
use std::mem::MaybeUninit;


pub fn get_docker_interface() ->  &'static Mutex<DockerInterface> {
    static mut DOCKERINTERFACE: MaybeUninit<Mutex<DockerInterface>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    let mut docker_interface = DockerInterface::new();
    docker_interface.init();


    ONCE.call_once(|| unsafe {
        DOCKERINTERFACE
            .as_mut_ptr()
            .write(Mutex::new(
                docker_interface
            ))
    });

    return unsafe { &*DOCKERINTERFACE.as_ptr()};
}

pub struct DockerInterface{
    docker: Docker,
    containers: Vec<shiplift::rep::Container>,
    images: Vec<shiplift::rep::Image>
}

impl DockerInterface {
    pub fn new() -> DockerInterface{
        return DockerInterface {
            docker: Docker::new(),
            containers: vec![],
            images: vec![]
        };
    }
    pub fn init(&mut self) {
        self.containers = self.get_containers() ;
        self.images = self.get_images();
    }

    #[tokio::main]
    pub async fn get_containers(&self) -> Vec<shiplift::rep::Container> {
        let mut containers_vec = vec![];
        match self.docker.containers().list(&Default::default()).await {
            Ok(containers) => {
                for c in containers {
                    containers_vec.push(c)
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                return vec![];
            },
        }
        return containers_vec;
    }

    #[tokio::main]
    pub async fn get_images(&self) -> Vec<shiplift::rep::Image> {
        match self.docker.images().list(&Default::default()).await {
            Ok(images) => return images,
            Err(e) => {
                eprintln!("{}", e);
                return vec![];
            },
        }
    }

    #[tokio::main]
    pub async fn docker_info(&self) {
        match self.docker.info().await {
            Ok(info) => println!("Info:{:?}", info),
            Err(e) => eprintln!("Error: {}", e),
        }
    }


    #[tokio::main]
    pub async fn log_container(&self, id: String) {
        let mut logs_stream = self.docker
            .containers()
            .get(&id)
            .logs(&LogsOptions::builder().stdout(true).stderr(true).build());
    }


    #[tokio::main]
    pub async fn pull_image(&self, image: &str) {
        let mut stream = self.docker
            .images()
            .pull(&PullOptions::builder().image(image).build());

        while let Some(pull_result) = stream.next().await {
            match pull_result {
                Ok(output) => println!("{:?}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    #[tokio::main]
    pub async fn check_image_exist(&self, image: &str) -> bool {
        match self.docker.images().search(image).await{
            Ok(images) => {
                return images
                    .iter()
                    .any(|image_search| image_search.name == image.to_string())
            },
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        }

    }

    #[tokio::main]
    pub async fn create_docker_container(&self, image: &str) -> Result<String, Error> {
        match self.docker
            .containers()
            .create(&ContainerOptions::builder(image).build())
            .await
        {
            Ok(info) => return Ok(info.id),
            Err(e) => return Err(e)
        }
    }

    #[tokio::main]
    pub(crate) async fn delete_container(&self, id: &str) {
        if let Err(e) = self.docker.containers().get(id).delete().await {
            eprintln!("Error: {}", e)
        }
    }

    #[tokio::main]
    pub async fn restart_container(&self, id: &str) -> Result<String, Error> {
        match self.docker
            .containers()
            .get(id)
            .restart(None)
            .await
        {
            Ok(info) => return Ok("container restart".to_string()),
            Err(e) => return Err(e)
        }
    }

    #[tokio::main]
    pub async fn stop_container(&self, id: &str) -> Result<String, Error> {
        match self.docker
            .containers()
            .get(id)
            .stop(None)
            .await
        {
            Ok(info) => return Ok("container restart".to_string()),
            Err(e) => return Err(e)
        }
    }
}