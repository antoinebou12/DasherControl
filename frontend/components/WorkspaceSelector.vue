<template>
  <div v-if="token !== ''" class="center workspace-selector-container">
    <span class="toggle-hide" >
      <i class="bx bx-down-arrow" @click="isVisible = !isVisible; get_workspaces()"/>
    </span>
    <vs-table class="workspace-selector" v-model="selected">
      <template #thead>
        <vs-tr>
          <vs-th>
            Workspaces
          </vs-th>
        </vs-tr>
      </template>
      <template #tbody v-if="isVisible">
        <vs-tr
            :key="i"
            v-for="(workspace, i) in workspaces"
            :data="workspace"
            :is-selected="selected == workspace">
          <vs-td>{{ workspace.name }}</vs-td>
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
  name: "WorkspaceSelector",
  data() {
    return {
      selected: null,
      workspaces: [],
      isVisible: true,
      token: ''
    }
  },
  beforeMount() {
    this.get_token()
  },
  methods: {
    get_workspaces(){
      let self = this;
      axios({
        method: 'get',
        url: '/workspaces/api/list',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.get_token()}`
        },
      }).then(function(response){
        self.workspaces = response.data
      }).catch(function(error){
        self.workspaces = []
      });
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
    .hidden {
      display: none;
    }
    .toggle-hide {
      position:absolute;
      top: 56px;
      right: 24px;
    }
  }
</style>