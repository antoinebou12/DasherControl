<template>
  <div v-if="token !== ''" class="center workspace-selector-container">
    <span class="toggle-hide" >
      <i :class="{'bx-down-arrow': !isVisible, 'bx-up-arrow':isVisible}"  class="bx " @click="isVisible = !isVisible"/>
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
            :is-selected="selected == workspace"
            :class="{'selected': selected.id === workspace.id}">
          <vs-td>
            <span>{{ workspace.name }}</span>
            <span @click="deleteWorkspace(workspace.id)"><i class="bx bx-x delete"></i></span>
          </vs-td>
        </vs-tr>
      </template>
      <template #notFound>
        <span v-if="isVisible">Try creating a new workspace with the Save icon</span>
        <span v-else></span>
      </template>
    </vs-table>
    <vs-input v-if="isVisible"
              class="workspace-name-input"
              v-on:keyup.enter="editWorkspaceName()"
              ref="workspaceNameInput"
              type="text"
              placeholder="Press Enter to Save Name"
              v-model="workspaceName"/>
  </div>
</template>

<script>
import axios from "axios";
export default {
  name: "WorkspaceSelector",
  data() {
    return {
      selected: {"name": "", "id": -1},
      workspaces: [],
      isVisible: true,
      workspaceName: "",
      currentWorkspaceName: "",
      clickCount: 0,
      clickTimer: null
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
    },
    selected: function(newVal, oldVal) {
      if (newVal === null) {
        this.workspaceName = "";
      }
      this.workspaceName = newVal.name;
      this.$emit('selected')
    }
  },
  created() {
    if (this.token !== '') {
      this.get_workspaces();
    }

  },
  mounted() {
    if (this.workspaces.length && this.token !== '') {
      this.selected = this.workspaces[0];
    }
  },
  updated() {
    this.get_workspaces();
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
    editWorkspaceName() {
      this.$emit("updateName")
    },
    deleteWorkspace(id) {
      axios({
        method: 'post',
        url: '/workspaces/api/delete/' + id,
        headers: {
          'Authorization': `Bearer ${this.token}`
        },
      }).then((response) => {
        createNotification(
            self,
            name + " Workspace Deleted",
            "",
            'primary')
        this.get_workspaces()
      }).catch((error) => {
        createNotification(
            self,
            name + "Error While Deleting The Workspace",
            "",
            'danger')
        this.get_workspaces()
      });
    }
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
    .delete {
      color: rgba(var(--vs-danger));
      position: absolute;
      right: 23px;
      font-size: 18px;
    }
    .workspace-name-input {
      margin-top: 8px;
    }
  }
</style>