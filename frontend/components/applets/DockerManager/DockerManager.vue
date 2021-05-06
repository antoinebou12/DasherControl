<template>
  <div class="center docker-manager-containers">
  <vs-table>
    <template #thead>
      <vs-tr>
        <vs-th class="quick-status-col">
        </vs-th>
        <vs-th>
          ID
        </vs-th>
        <vs-th>
          Name
        </vs-th>
        <vs-th>
          Image
        </vs-th>
        <vs-th>
          State
        </vs-th>
        <vs-th>
          Status
        </vs-th>
        <vs-th>
          IP
        </vs-th>
        <vs-th>
          Public Port
        </vs-th>
      </vs-tr>
    </template>
    <template #tbody>
      <vs-tr
          :key="i"
          v-for="(tr, i) in containers"
          :data="tr">
        <vs-td>
          <div class="quick-status">
            <i class="bx bxs-server"></i>
            <i :class="get_status_theme(tr.status)" class="bx bxs-circle"></i>
          </div>
          <div class="quick-control">
            <vs-button @click="stop_container(tr.id)" v-if="get_status(tr.status)">
              <i class="bx bx-stop"></i>
            </vs-button>
            <vs-button @click="restart_container(tr.id)" v-if="!get_status(tr.status)">
              <i class="bx bx-play"></i>
            </vs-button>
          </div>
        </vs-td>
        <vs-td>
          {{ tr.id }}
        </vs-td>
        <vs-td>
          {{ tr.name }}
        </vs-td>
        <vs-td>
          {{ tr.image }}
        </vs-td>
        <vs-td>
          {{ tr.state }}
        </vs-td>
        <vs-td>
          {{ tr.status }}
        </vs-td>
        <vs-td>
          {{ tr.ip }}
        </vs-td>
        <vs-td>
          <a @click="openContainer(tr)">{{ tr.public_port }}</a>
        </vs-td>
      </vs-tr>
    </template>
    <template #notFound>
      <span></span>
    </template>
  </vs-table>
  </div>
</template>

<script>
import axios from "axios";


export default {
  name: "DockerManager",
  data() {
    return {
      statusInterval: null,
      containers: [],
      token: '',
    }
  },
  created() {
    this.get_container();
    this.get_token();
    this.statusInterval = setInterval(() => {
      this.get_container();
    }, 1000)
  },
  updated() {
    this.get_container();
  },
  beforeDestroy() {
    clearInterval(this.statusInterval);
  },
  methods: {
    get_container() {
      axios({
        method: 'get',
        url: '/containers/api/docker_list',
        headers: {
          'Content-Type': 'application/json'
        },
      }).then((response) => {
        this.containers = []
        if (response.data.length != 0) {
          for (let i = 0; i < response.data.length; i++) {
            let ports = (response.data[i]["Ports"].length == 0 ? [{"Ip": "Host", "PublicPort": "X"}]: response.data[i]["Ports"]);
            this.containers.push({
              id: response.data[i].Id.slice(0, 11),
              name: response.data[i].Names[0].replace("/", ""),
              image: response.data[i].Image,
              state: response.data[i].State,
              status: response.data[i].Status,
              ip: ports[0]["Ip"] || window.location.hostname.replace("www.", ""),
              public_port: ports[0]["PublicPort"] || "X"
            })
          }
        }
      });
    },
    get_status(status) {
      if (status.includes("Up")) {
        return true
      } else {
        return false
      }
    },
    get_status_theme(status) {
      if (this.get_status(status)) {
        return "up-docker"
      } else {
        return "down-docker"
      }
    },
    get_token(){
      if (this.token == '') {
        axios({
          method: 'get',
          url: '/tenants/api/token',
        }).then((response) => {
          this.token = response.data
          return this.token
        })
      } else {
        return this.token
      }
    },
    stop_container(id) {
      this.get_container()
      axios({
        method: 'post',
        headers: {
          Authorization: `Bearer ${this.get_token()}`
        },
        url: '/containers/api/stop/' + id,
      }).then((response) => {
        this.get_container()
      })
    },
    restart_container(id) {
      this.get_container()
      axios({
        method: 'post',
        headers: {
          Authorization: `Bearer ${this.get_token()}`
        },
        url: '/containers/api/restart/' + id,
      }).then((response) => {
        this.get_container()
      })
    },
    openContainer(containerData) {
      this.$emit('openContainer', containerData)
    }
  }
}
</script>

<style lang="scss" scoped>
.docker-manager-containers{
  background: var(--bg);
  color: var(--fg);

  .quick-status-col {
    width: 10px
  }

  .tr-selected {
    background: rgba(var(--vs-color), 0.1);
    color: rgba(var(-vs-color), 0.1)
  }
  .quick-status {
    display:inline-block;
    position: relative;
    margin: 10px;
    .bxs-server {
      font-size: 30px
    }
    .bxs-circle {
      position:absolute;
      bottom: 0;
      right: 0;
      margin: -3px -6px;
    }
    .down-docker{
      color: red;
    }
    .up-docker {
      color: green;
    }
  }
}
</style>