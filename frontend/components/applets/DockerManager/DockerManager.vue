<template>
  <vs-table>
    <template #thead>
      <vs-tr>
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
          Ip
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
          {{ tr.id }}
        </vs-td>
        <vs-td>
          {{ tr.name }}
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
              id: response.data[i].Id,
              name: response.data[i].Names[0],
              state: response.data[i].Image[0],
              status: response.data[i].Status[0],
              ip: response.data[i].Ports["Ip"],
              public_port: response.data[i].Ports["PublicPort"]
            })
          }
        }
      });
    }
  }
}
</script>

<style scoped>

</style>