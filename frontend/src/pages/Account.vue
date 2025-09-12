<script setup lang="ts">
import { ref } from 'vue'
import { deleteCookie, getCookie } from '@/utils/cookie'
import { useRouter } from 'vue-router'

const confirmDelete = ref('')
const confirmPassword = ref('')
const errorMessage = ref('')
const newPassword = ref('')
const password = ref('')
const username = ref('')

const router = useRouter()

const handleClick = async () => {
  deleteCookie('token')
  await router.push('/login')
}

const updateUser = async (e: Event) => {
  e.preventDefault()

  if (newPassword.value !== confirmPassword.value) {
    return (errorMessage.value = 'Passwords must match')
  }

  errorMessage.value = ''

  const token = getCookie('token')
  if (token && username.value && newPassword.value) {
    try {
      const response = await fetch(`${import.meta.env.VITE_AUTH_URL}/auth`, {
        method: 'PUT',
        body: JSON.stringify({
          password: newPassword.value,
          username: username.value,
        }),
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
      })

      if (response.ok) {
        deleteCookie('token')
        await router.push('/login')
      }
    } catch (e) {
      console.error(e)
    }
  }
}

const deleteUser = async (e: Event) => {
  e.preventDefault()

  const token = getCookie('token')
  if (token && confirmDelete.value && password.value) {
    try {
      const response = await fetch(`${import.meta.env.VITE_AUTH_URL}/auth`, {
        method: 'DELETE',
        body: JSON.stringify({
          confirmation: confirmDelete.value,
          password: password.value,
        }),
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
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
  <h1>Account</h1>
  <p v-if="errorMessage">{{ errorMessage }}</p>
  <button @click="handleClick">Log Out</button>
  <h2>Update Information</h2>
  <form @submit="updateUser">
    <label>
      <span>Username</span>
      <input v-model="username" name="username" required />
    </label>
    <label>
      <span>New Password</span>
      <input v-model="newPassword" name="newPassword" type="password" required />
    </label>
    <label>
      <span>Confirm New Password</span>
      <input v-model="confirmPassword" name="confirmPassword" type="password" required />
    </label>
    <button type="submit">Submit</button>
  </form>
  <h2>Delete Account</h2>
  <form @submit="deleteUser">
    <label>
      <span>Password</span>
      <input v-model="password" name="password" type="password" required />
    </label>
    <label>
      <span>Confirmation</span>
      <input
        v-model="confirmDelete"
        aria-describedby="delete-instructions"
        name="confirmDelete"
        required
      />
      <span id="delete-instructions"
        >To delete your account, please enter the confirmation text &quot;delete my account&quot;
        into the above text field. This action is permanent and cannot be undone.</span
      >
    </label>
    <button type="submit">Submit</button>
  </form>
</template>
