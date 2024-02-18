<script setup lang="ts">
import { ref } from 'vue'
import { Base64 } from 'js-base64'
import { testPasswordAuthenticatable } from '@/services/client'

const emit = defineEmits(['login-successfully'])

const password = ref('')
const showPassword = ref(false)
const rules = [
    (value: string) =>
        (!!value && value.trim() !== '') || 'Password is required',
]
const loginLoading = ref(false)
const showSnackbar = ref(false)

async function performLogin() {
    loginLoading.value = true
    sessionStorage.setItem('password', Base64.encode(password.value))
    if (await testPasswordAuthenticatable()) {
        emit('login-successfully')
    } else {
        showSnackbar.value = true
        setTimeout(() => {
            showSnackbar.value = false
        }, 3000)
    }
    loginLoading.value = false
}
</script>
<template>
    <div class="d-flex justify-center w-100">
        <v-card
            min-width="300"
            max-width="500"
            class="flex-grow-1"
            elevation="3"
        >
            <template #text>
                <v-text-field
                    v-model="password"
                    label="Server Password"
                    :type="showPassword ? 'text' : 'password'"
                    placeholder="Enter your password"
                    :rules="rules"
                    :append-inner-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
                    @click:append-inner="showPassword = !showPassword"
                />
            </template>
            <template #actions>
                <v-btn
                    color="primary"
                    @click="performLogin"
                    :loading="loginLoading"
                >
                    Login
                </v-btn>
            </template>
        </v-card>
    </div>
    <v-snackbar v-model="showSnackbar">
        Login failed
        <template #actions>
            <v-btn variant="text" @click="showSnackbar = false" color="pink">
                Close
            </v-btn>
        </template>
    </v-snackbar>
</template>
