<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { ClientInformation, getAllClientsInformation } from '@/services/client'
import ClientCard from '@/components/ClientCard.vue'

const clients = ref<ClientInformation[]>()
const loading = ref(false)

async function refreshClientsInformation() {
    loading.value = true
    clients.value = await getAllClientsInformation()
    loading.value = false
}

onMounted(async () => {
    await refreshClientsInformation()
})
</script>
<template>
    <div class="text-center ma-3">
        <v-btn color="primary" :loading="loading" variant="outlined" text="refresh clients' information" prepend-icon="mdi-refresh" @click="refreshClientsInformation"/>
    </div>
    <div v-if="loading" class="ma-5 pa-5">
        <v-skeleton-loader type="card, list-item-three-line, list-item-three-line" elevation="3"/>
    </div>
    <div v-else class="d-flex">
        <ClientCard
            class="ma-2 pa-2"
            v-for="client in clients"
            :client="client"
        />
    </div>
</template>
