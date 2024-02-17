<script setup lang="ts">
import { PropType, ref } from 'vue'
import { ClientInformation } from '@/services/client'
import useClipboard from 'vue-clipboard3'

const props = defineProps({
    client: {
        type: Object as PropType<ClientInformation>,
        required: true,
    },
})

const { toClipboard } = useClipboard()
const showSnackbar = ref(false)
const copiedText = ref('')

async function copyToClipboard(text: string) {
    await toClipboard(text)
    showSnackbar.value = true
    copiedText.value = text
    setTimeout(() => {
        showSnackbar.value = false
    }, 3000)
}
</script>
<template>
    <v-card
        :title="props.client.entity.name"
        :subtitle="props.client.entity.id"
        elevation="3"
    >
        <template #text>
            <p>
                <span class="font-weight-bold">Last Fetched Time:</span>
                {{ props.client.entity.last_fetch_time }}
            </p>
            <p>
                <span class="font-weight-bold">Create Time:</span>
                {{ props.client.entity.create_time }}
            </p>
            <v-list>
                <div
                    v-for="addresses in props.client.adapter_addresses"
                    class="ma-2"
                >
                    <v-divider />
                    <v-list-subheader>{{ addresses.name }}</v-list-subheader>
                    <v-list-item v-if="addresses.v4 !== null">
                        <template #title>IPv4</template>
                        <template #subtitle>{{ addresses.v4 }}</template>
                        <template #append>
                            <v-btn
                                color="primary"
                                icon="mdi-content-copy"
                                variant="text"
                                @click="
                                    async () => copyToClipboard(addresses.v4!)
                                "
                            />
                        </template>
                    </v-list-item>
                    <v-list-item v-if="addresses.v6 !== null">
                        <template #title>IPv6</template>
                        <template #subtitle>{{ addresses.v6 }}</template
                        ><template #append>
                            <v-btn
                                color="primary"
                                icon="mdi-content-copy"
                                variant="text"
                                @click="
                                    async () => copyToClipboard(addresses.v6!)
                                "
                            />
                        </template>
                    </v-list-item>
                </div>
            </v-list>
            <v-snackbar v-model="showSnackbar">
                Copied {{ copiedText }} to clipboard.
                <template #actions>
                    <v-btn
                        variant="text"
                        @click="showSnackbar = false"
                        color="pink"
                    >
                        Close
                    </v-btn>
                </template>
            </v-snackbar>
        </template>
    </v-card>
</template>
