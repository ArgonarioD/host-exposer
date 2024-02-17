<template>
    <v-app>
        <v-main>
            <div class="text-h2 ma-5 text-center">Host Exposer</div>
            <LoginForm />
            <ClientList />
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import ClientList from '@/components/ClientList.vue'
import LoginForm from './components/LoginForm.vue';

//
</script>
