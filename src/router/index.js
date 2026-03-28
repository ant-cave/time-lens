import { createRouter, createWebHashHistory } from 'vue-router'
import Today from '../views/Today.vue'
import Calendar from '../views/Calendar.vue'
import Analysis from '../views/Analysis.vue'
import Settings from '../views/Settings.vue'
import About from '../views/About.vue'



// 路由配置
const routes = [
  { path: '/', redirect: '/today' },
  { path: '/today', component: Today },
  { path: '/calendar', component: Calendar },
  { path: '/analysis', component: Analysis },
  { path: '/settings', component: Settings },
  { path: '/about', component: About },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
