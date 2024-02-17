<script setup lang="ts">
import { ref } from 'vue'

const password = ref('')
const rules = [
    (value: string) =>
        (!!value && value.trim() !== '') || 'Password is required',
]

async function performLogin(password: string) {

}
</script>
<template>
    <div class="d-flex justify-center w-100">
        <v-card min-width="300" max-width="500" class="flex-grow-1" elevation="3">
            <!-- <template #title> Password </template> -->
            <template #text>
                <v-text-field
                    v-model="password"
                    label="Server Password"
                    type="password"
                    placeholder="Enter your password"
                    :rules="rules"
                />
            </template>
            <template #actions>
                <v-btn color="primary" @click="() => console.log(password)">
                    Login
                </v-btn>
            </template>
        </v-card>
    </div>
</template>
