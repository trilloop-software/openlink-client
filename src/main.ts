import { createApp } from 'vue'
import App from './App.vue'

import { createPinia } from 'pinia'
import { Quasar } from 'quasar'

import router from './services/router'

import '@quasar/extras/roboto-font/roboto-font.css'
import '@quasar/extras/material-icons/material-icons.css'

import 'quasar/src/css/index.sass'

createApp(App)
  .use(router)
  .use(createPinia())
  .use(Quasar, {
    plugins: {},
  })
  .mount('#app')
