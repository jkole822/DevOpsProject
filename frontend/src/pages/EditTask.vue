<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getCookie } from '@/utils/cookie'
import { useRoute, useRouter } from 'vue-router'

const done = ref(false)
const title = ref('')

const route = useRoute()
const router = useRouter()

const handleSubmit = async (e: Event) => {
  e.preventDefault()

  const token = getCookie('token')
  if (title.value && token) {
    try {
      const response = await fetch(`${import.meta.env.VITE_TASK_URL}/tasks/${route.params.id}`, {
        body: JSON.stringify({
          done: done.value,
          title: title.value,
        }),
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        method: 'PUT',
      })

      if (response.ok) await router.push('/dashboard')
    } catch (e) {
      console.error(e)
    }
  }
}

const deleteTask = async () => {
  const token = getCookie('token')
  if (token) {
    try {
      const response = await fetch(`${import.meta.env.VITE_TASK_URL}/tasks/${route.params.id}`, {
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        method: 'DELETE',
      })

      if (response.ok) await router.push('/dashboard')
    } catch (e) {
      console.error(e)
    }
  }
}

const fetchTask = async () => {
  try {
    const token = getCookie('token')

    if (token) {
      const response = await fetch(`${import.meta.env.VITE_TASK_URL}/tasks/${route.params.id}`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })

      const data = await response.json()

      if (data) {
        done.value = data.done
        title.value = data.title
      }
    }
  } catch (e) {
    console.error(e)
  }
}

onMounted(() => {
  if (typeof route.params.id === 'string') fetchTask()
})
</script>

<template>
  <a href="/dashboard">Dashboard</a>
  <form @submit="handleSubmit">
    <h1>Edit Task</h1>
    <label>
      <span>Title</span>
      <input v-model="title" name="title" required />
    </label>
    <label>
      <span>Done</span>
      <input v-model="done" name="done" type="checkbox" required />
    </label>
    <button type="submit">Submit</button>
  </form>
  <button @click="deleteTask">Delete</button>
</template>
