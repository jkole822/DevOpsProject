<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getCookie } from '@/utils/cookie'

const tasks = ref([])

const fetchTasks = async () => {
  const token = getCookie('token')
  if (token) {
    try {
      const response = await fetch(`${import.meta.env.VITE_TASK_URL}/tasks`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })

      const data = await response.json()

      if (data) {
        tasks.value = data
        console.log('All Tasks:\n', data)
      }
    } catch (e) {
      console.error(e)
    }
  }
}

onMounted(() => {
  fetchTasks()
})
</script>

<template>
  <a href="/create-task">Create Task</a>
  <a href="/account">Account</a>
  <h1>Tasks</h1>
  <ul>
    <li v-for="{ id, title } in tasks" :key="id">
      <span>{{ title }}</span>
      <a :href="`/task/${id}`">Edit</a>
    </li>
  </ul>
</template>
