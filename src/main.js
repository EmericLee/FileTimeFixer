import { createApp } from 'vue'
import { Quasar, Notify } from 'quasar'
// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'
import '@quasar/extras/fontawesome-v6/fontawesome-v6.css'

import App from './App.vue'
// Import Quasar css
import 'quasar/dist/quasar.css'
import './styles/global.scss'

const app = createApp(App)

app.use(Quasar, {
  plugins: { Notify }, // import Quasar plugins and add here
})

// 在开发环境中启用Vue DevTools
if (import.meta.env.DEV) {
  // 确保Vue DevTools已启用
  app.config.devtools = true

  // 提示用户安装浏览器扩展
  console.log('Vue DevTools is enabled. Consider installing the Vue DevTools browser extension for better debugging experience.')
}

app.mount('#app')