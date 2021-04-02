<template>
  <div class="center workspace-selector-container">
    <vs-table class="workspace-selector" v-model="selected">
      <template #thead>
        <vs-tr>
          <vs-th>
            Workspaces
          </vs-th>
        </vs-tr>
      </template>
      <template #tbody>
        <vs-tr
            :key="i"
            v-for="(workspace, i) in workspaces"
            :data="workspace"
            :is-selected="selected == workspace">
          <vs-td>{{ workspace.name }}</vs-td>
        </vs-tr>
      </template>
    </vs-table>
  </div>
</template>

<script>
import axios from "axios";
export default {
  name: "WorkspaceSelector",
  data() {
    return {
      selected: null,
      workspaces: []
    }
  },
  beforeMount() {
    this.get_workspaces();
  },
  methods: {
    get_workspaces(){
      let self = this;
      axios({
        method: 'get',
        url: '/workspaces/api/list',
        headers: {
          'Content-Type': 'application/json'
        },
      }).then(function(response){
        self.workspaces = response.data
      });
    }
  }
}
</script>

<style lang="scss" scoped>
  .workspace-selector-container{
    color: var(--darcula-fg);

    .tr-selected {
      background: rgba(var(--vs-color),0.1);
      color: rgba(var(-vs-color), 0.1)
    }
  }
</style>