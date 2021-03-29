import Vue from 'vue';
import {vsButton, vsCard, vsInput, vsDialog, vsNavbar} from 'vuesax'
import Vuesax from 'vuesax'
import App from './App.vue';

Vue.config.productionTip = false

import 'vuesax/dist/vuesax.css' //Vuesax styles

// Vue.use(vsButton)
// Vue.use(vsCard)
// Vue.use(vsInput)
// Vue.use(vsDialog)
// Vue.use(vsNavbar)
Vue.use(Vuesax)

new Vue({
  el: '#app',
  render: h => h(App),
});