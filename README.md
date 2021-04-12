<p>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

<h1 align="center">DasherControl</h1>
<p>Another Interactive Configurable Dashboard with Customisable GridItem with IFrame and Bookmark and other cool feature with basic Container Controller for Docker
  made with Vuejs and Rust (rocket)
</p>

<h2>Why ...</h2>
<p>
  Everything is a web app that can be install with a docker in container. I want to manage all my web application on one dashboard like Sonarr and jellyfin without opening 
  like 10 tabs in chrome (rip my ram). When using service like portainer of the docker cli it's long to setup a reverse proxy with SSL to be secure your homelab. So i want to     write widget (Applet) that can do all my task that i do on the daily when managing my homelab. 
  Also i want to make a simple dashboard with widget (vuejs compoment) like windows vista, but on the web and save in db.
</p>

Preview Look
![preview look](https://raw.githubusercontent.com/antoinebou13/DasherControl/main/images/DasherControl.png)

## Finish

- [x] Applets with IFrame
- [x] Save Workspace and switch between workspace
- [X] Applets Management 
- [X] Simple Start and Manage Docker Containers

# In Progress

- [ ] User Auth (Half done W.I.P)
- [ ] Terminal ssh web
- [ ] Install App with Docker/Docker-Compose
- [ ] CI/CD
- [ ] Canvas applets

## TODO
- [ ] Documentation
- [ ] RTMP Client
- [ ] Customise Theme and Change Background
- [ ] Save docker-compose/container info in the database
- [ ] Nginx Config Generator for reverse Proxy and SSL (maybe trafik)
- [ ] Tests


# Issues
I use Iframe to display the other website some the login of the website will not work because of the csrf token or other restriction of iframe.

## Install

```sh
cargo install diesel_cli --no-default-features --features postgres
// go in Rocket.toml and .env and change DATABASE_URL to your postgresql server
diesel migration run
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
# not implemented yet
```sh
cargo test
cd frontend && npm test
```
