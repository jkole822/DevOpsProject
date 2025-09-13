import { createRouter, createWebHistory } from 'vue-router'
import { deleteCookie, getCookie } from '@/utils/cookie'
import { Account, CreateTask, CreateUser, Dashboard, EditTask, Login } from '@/pages'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: CreateUser, name: 'Register', meta: { requiresAuth: false } },
    { path: '/login', component: Login, name: 'Login', meta: { requiresAuth: false } },
    { path: '/account', component: Account, name: 'Account', meta: { requiresAuth: true } },
    { path: '/dashboard', component: Dashboard, name: 'Dashboard', meta: { requiresAuth: true } },
    {
      path: '/create-task',
      component: CreateTask,
      name: 'CreateTask',
      meta: { requiresAuth: true },
    },
    { path: '/task/:id', component: EditTask, name: 'EditTask', meta: { requiresAuth: true } },
  ],
})

router.beforeEach(async (to, from, next) => {
  const token = getCookie('token')

  if (to.meta.requiresAuth && !token) {
    next({ name: 'Login' })
  } else if (to.meta.requiresAuth && token) {
    try {
      const response = await fetch(`${import.meta.env.VITE_AUTH_URL}/auth`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })
      const data = await response.json()

      if (data.user_id) {
        next()
      } else {
        next({ name: 'Login' })
      }
    } catch (e) {
      console.error(e)
      deleteCookie('token')
      next({ name: 'Login' })
    }
  } else if (!to.meta.requiresAuth && token) {
    try {
      const response = await fetch(`${import.meta.env.VITE_AUTH_URL}/auth`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })

      const data = await response.json()

      if (data.user_id) {
        next({ name: 'Dashboard' })
      } else {
        next()
      }
    } catch (e) {
      console.error(e)
      deleteCookie('token')
      next({ name: 'Login' })
    }
  } else {
    next()
  }
})

export default router
