import Vue from 'vue';
import App from './App.vue';

Vue.config.productionTip = false

// (function() {
//   if('serviceWorker' in navigator) {
//     navigator.serviceWorker.register('/service-worker-cache.js');
//   }
// })();

new Vue({
  el: '#app',
  render: h => h(App),
});