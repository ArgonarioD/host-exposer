<template>
    <v-app>
        <v-main>
            <div class="text-h2 ma-5 text-center">Host Exposer</div>
            <ClientList v-if="authenticated"/>
            <LoginForm v-else @login-successfully="authenticated = true"/>
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import ClientList from '@/components/ClientList.vue'
import LoginForm from './components/LoginForm.vue';
import { ref } from 'vue';

const authenticated = ref(false)

</script>
