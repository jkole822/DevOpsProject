<script setup lang="ts">
import { ref } from 'vue'
import { getCookie } from '@/utils/cookie'
import { useRouter } from 'vue-router'

const title = ref('')

const router = useRouter()

const handleSubmit = async (e: Event) => {
  e.preventDefault()

  const token = getCookie('token')
  if (title.value && token) {
    try {
      const response = await fetch(`${import.meta.env.VITE_TASK_URL}/tasks`, {
        body: JSON.stringify({
          title: title.value,
        }),
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        method: 'POST',
      })

      if (response.ok) await router.push('/dashboard')
    } catch (e) {
      console.error(e)
    }
  }
}
</script>

<template>
  <a href="/dashboard">Dashboard</a>
  <form @submit="handleSubmit">
    <h1>Create Task</h1>
    <label>
      <span>Title</span>
      <input v-model="title" name="title" required />
    </label>
    <button type="submit">Submit</button>
  </form>
</template>
