<template>
    <v-app>
        <v-main>
            <div class="text-h2 ma-5 text-center">
                Host Exposer
                <a
                    href="https://github.com/ArgonarioD/host-exposer"
                    target="_blank"
                    ><v-btn icon="mdi-github" color="primary" variant="text"
                /></a>
            </div>
            <ClientList v-if="authenticated" />
            <LoginForm v-else @login-successfully="authenticated = true" />
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import ClientList from '@/components/ClientList.vue'
import LoginForm from './components/LoginForm.vue'
import { ref } from 'vue'

const authenticated = ref(false)
</script>
