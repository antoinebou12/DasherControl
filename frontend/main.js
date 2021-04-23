import Vue from 'vue';
import Vuex from 'vuex';
import Vuesax from 'vuesax'
import 'vuesax/dist/vuesax.css' //Vuesax styles

import App from './App.vue';
import axios from "axios";

Vue.config.productionTip = false

// vuesax
Vue.use(Vuesax)


// vuex
Vue.use(Vuex)
export const store = new Vuex.Store({
  state: {
    user: {
      username: '',
      email: '',
      token: ''
    },
  },
  mutations: {
    setUser(state, user) {
      state.user.username = user.username
      state.user.email = user.email
    },
    setToken(state, token) {
      state.user.token = token
    }
  },
  getters: {
    getUser: state => {
      if (state.user.token !== '') {
        axios({
          method: 'get',
          url: '/tenants/api/token',
        }).then((response) => {
          state.user.token = response.data
        })
      }
      if (state.user.token !== '') {
        return state.user
      }
      return false
    }
  }
})



new Vue({
  el: '#app',
  store,
  render: h => h(App),
});