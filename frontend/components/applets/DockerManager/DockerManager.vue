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
          {{ tr.public_port }}
        </vs-td>
      </vs-tr>
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
      containers: [],
    }
  },
  created() {
    this.get_container();
  },
  methods: {
    get_container() {
      axios({
        method: 'get',
        url: '/containers/api/running_list',
        headers: {
          'Content-Type': 'application/json'
        },
      }).then((response) => {
        if (response.data.length != 0){
          for (let i=0;i<response.data.length;i++) {
            this.containers.push({
              id: response.data[i].Id.slice(0, 11),
              name: response.data[i].Names[0].replace("/", ""),
              image: response.data[i].Image,
              state: response.data[i].State,
              status: response.data[i].Status,
              ip: response.data[i].Ports["Ip"] || "Host",
              public_port: response.data[i].Ports["PublicPort"] || "X"
            })
          }
        }
      });
    },
    get_status_theme(status){
      if (status.includes("Up")) {
        return "up-docker"
      } else {
        return "down-docker"
      }

    }
  }
}
</script>

<style lang="scss" scoped>
.docker-manager-containers{
  background: var(--darcula-bg);
  color: var(--darcula-fg);

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