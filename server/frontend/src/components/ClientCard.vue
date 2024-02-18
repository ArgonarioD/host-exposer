<script setup lang="ts">
import { PropType, ref } from 'vue'
import { ClientInformation, editClientName } from '@/services/client'
import useClipboard from 'vue-clipboard3'
import { Ref } from 'vue'

const props = defineProps({
    client: {
        type: Object as PropType<ClientInformation>,
        required: true,
    },
})

const { toClipboard } = useClipboard()
const showSnackbar = ref(false)
const snackbarText = ref('')

const newName = ref(props.client.entity.name)
const newNameRules = [
    (value: string) => !!value.trim() || 'Name cannot be empty or whitespace',
]
const submitNewNameLoading = ref(false)

async function submitNewName(dialogIsActive: Ref<boolean>) {
    submitNewNameLoading.value = true
    try {
        await editClientName(props.client.entity.id, newName.value)
        props.client.entity.name = newName.value
    } catch (e) {
        snackbar(`${e}`)
    }
    dialogIsActive.value = false
    newName.value = ''
    submitNewNameLoading.value = false
}

async function cancelEditName(dialogIsActive: Ref<boolean>) {
    dialogIsActive.value = false
    newName.value = ''
}

async function copyToClipboard(text: string) {
    await toClipboard(text)
    snackbar(`Copied ${text} to clipboard.`)
}

function snackbar(text: string) {
    showSnackbar.value = true
    snackbarText.value = text
    setTimeout(() => {
        showSnackbar.value = false
    }, 3000)
}
</script>
<template>
    <v-card :subtitle="props.client.entity.id" elevation="3">
        <template #title>
            {{ props.client.entity.name }}
            <v-dialog width="500">
                <template #activator="{ props }">
                    <v-btn
                        v-bind="props"
                        icon="mdi-pencil"
                        variant="text"
                        color="primary"
                    />
                </template>
                <template #default="{ isActive }">
                    <v-card :title="`Rename ${props.client.entity.name}`">
                        <template #text>
                            <v-text-field
                                v-model="newName"
                                :rules="newNameRules"
                                :label="`New Name of ${props.client.entity.name}`"
                            />
                        </template>
                        <template #actions>
                            <v-btn
                                text="submit"
                                color="primary"
                                :loading="submitNewNameLoading"
                                @click="async () => submitNewName(isActive)"
                            />
                            <v-btn
                                text="cancel"
                                color="grey"
                                @click="async () => cancelEditName(isActive)"
                            />
                        </template>
                    </v-card>
                </template>
            </v-dialog>
        </template>
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
                {{ snackbarText }}
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
