<script setup lang="ts">
import { ref } from 'vue'
import { setCookie } from '@/utils/cookie'
import { useRouter } from 'vue-router'

const username = ref('')
const password = ref('')

const router = useRouter()

const handleSubmit = async (e: Event) => {
  e.preventDefault()

  if (username.value && password.value) {
    try {
      const response = await fetch(`${import.meta.env.VITE_AUTH_URL}/auth`, {
        body: JSON.stringify({
          username: username.value,
          password: password.value,
        }),
        headers: {
          'Content-Type': 'application/json',
        },
        method: 'POST',
      })

      const data = await response.json()

      if (data?.token) {
        setCookie('token', data.token)
        await router.push('/dashboard')
      }
    } catch (e) {
      console.error(e)
    }
  }
}
</script>

<template>
  <form @submit="handleSubmit">
    <h1>Register</h1>
    <label>
      <span>Username</span>
      <input v-model="username" name="username" required />
    </label>
    <label>
      <span>Password</span>
      <input v-model="password" name="password" type="password" required />
    </label>
    <button type="submit">Submit</button>
    <a href="/login">Login</a>
  </form>
</template>
