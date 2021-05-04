<template>
  <div v-if="token !== ''" class="center workspace-selector-container">
    <span class="toggle-hide" >
      <i class="bx bx-down-arrow" @click="isVisible = !isVisible"/>
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
        <vs-tr>
          <vs-td><vs-input ref="workspaceNameInput" type="text" placeholder="workspace" v-model="workspaceName"/></vs-td>
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
      workspaceName: "Workspace"
    }
  },
  computed: {
    token: {
      get: function (){
        return this.$store.state.user.token
      },
      set: function (newVal) {
        this.token = newVal;
      }
    },
  },
  watch: {
    isVisible: function(newVal, oldVal) {
      this.get_workspaces()
    }
  },
  created() {
    this.get_workspaces()
  },
  methods: {
    get_workspaces(){
        axios({
          method: 'get',
          url: '/workspaces/api/list',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${this.token}`
          },
        }).then((response) => {
          this.workspaces = response.data
        }).catch((error) => {
          this.workspaces = []
        });
    },
  },
}
</script>

<style lang="scss" scoped>
  .workspace-selector-container{
    color: var(--fg);

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