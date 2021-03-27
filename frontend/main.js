import Vue from 'vue';
import Vuesax from 'vuesax'
import App from './App.vue';

Vue.config.productionTip = false

import 'vuesax/dist/vuesax.css' //Vuesax styles
Vue.use({
  // options here
})

new Vue({
  el: '#app',
  render: h => h(App),
});