import Vue from 'vue';
import Vuesax from 'vuesax'
import App from './App.vue';
import mitt from 'mitt'

export const emitter = mitt()

Vue.config.productionTip = false

import 'vuesax/dist/vuesax.css' //Vuesax styles

Vue.use(Vuesax)

new Vue({
  el: '#app',
  render: h => h(App),
});