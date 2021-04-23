<h1 align="center">DasherControl</h1>
<p>Another Interactive Configurable Dashboard with Customisable GridItem with IFrame and Bookmark and other cool feature with basic Container Controller for Docker
  made with Vuejs and Rust (rocket)
</p>

<p>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/antoinebou13/DasherControl/CI">
    <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/antoinebou13/DasherControl">
  </a>
</p>

<h2>Why ...</h2>
<p>
  Everything is a web app that can be install with a docker in container. I want to manage all my web application on one dashboard like Sonarr and jellyfin without opening 
  like 10 tabs in chrome (rip my ram). When using service like portainer of the docker cli it's long to setup a reverse proxy with SSL to be secure your homelab. So i want to     write widget (Applet) that can do all my task that i do on the daily when managing my homelab. 
  Also i want to make a simple dashboard with widget (vuejs compoment) like windows vista, but on the web and save in db.
</p>

Preview Look
![preview look](https://raw.githubusercontent.com/antoinebou13/DasherControl/main/images/DasherControl.png)


# Roadmap
![DasherControlv1](https://github.com/antoinebou13/DasherControl/projects/1)

## Finish

- [x] Applets with IFrame
- [x] Save Workspace and switch between workspace
- [X] Applets Management 
- [X] Simple Start and Manage Docker Containers
- [X] CI/CD
- [X] User Auth

# In Progress
- [ ] Customise Theme and Change Background
- [ ] Logging
- [ ] Canvas applets
- [ ] Terminal ssh web
- [ ] Install App with Docker/Docker-Compose
- [ ] Tests

## TODO
- [ ] Documentation
- [ ] RTMP IP Camera Client
- [ ] User Auth (OAUTH@ Github)
- [ ] Save docker-compose/container info in the database
- [ ] Nginx Config Generator for reverse Proxy and SSL (maybe trafik)
- [ ] Export and import of containers and workspaces

# Issues
I use Iframe to display the other website some the login of the website will not work because of the csrf token or other restriction of iframe.

## Install (Tested only on Ubuntu 20.04)

```sh
// bash scripts/rust-setup-dev.sh
// bash scripts/js-dev-setup.sh

cd frontend && npm install && npm run build && cd ..

cargo install diesel_cli --no-default-features --features postgres
// go in Rocket.toml and .env and change DATABASE_URL to your postgresql server
diesel migration run

// create admin user
cargo run --bin create_admin

// run web app
cargo run
```

### Docker  (W.I.P)
```
docker build -t antoinebou13/DasherControl .
docker run antoinebou13/DasherControl
```


### Docker-compose (W.I.P)
```
docker-compose up -d
```

## Run tests
```sh
cargo test
cd frontend && npm test // no test yet on frontend
```
