import Vue from 'vue';
// vuex
import Vuex from 'vuex';
import createPersistedState from 'vuex-persistedstate'

import Vuesax from 'vuesax'
import 'vuesax/dist/vuesax.css' //Vuesax styles

import App from './App.vue';
import axios from "axios";
import Cookies from "js-cookie";

Vue.config.productionTip = false

// vuesax
Vue.use(Vuesax)


// vuex
Vue.use(Vuex)
export const store = new Vuex.Store({
  plugins: [
    createPersistedState({
      storage: {
        getItem: (key) => Cookies.get(key),
        setItem: (key, value) => Cookies.set(key, value, { expires: 3, secure:false , sameSite: 'strict' }),
        removeItem: (key) => Cookies.remove(key)
      }
    })
  ],
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
  actions: {
    async getToken({ commit }){
      if (this.state.user.token === '') {
        axios({
          method: 'get',
          url: '/tenants/api/token',
        }).then((response) => {
          if (response.data !== '') {
            commit('setToken', response.data)
          } else {
            commit('setToken', '')
          }
        })
      }
    }
  }
})



new Vue({
  el: '#app',
  store,
  render: h => h(App),
});