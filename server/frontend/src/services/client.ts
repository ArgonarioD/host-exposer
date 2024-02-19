export interface ClientInformation {
    adapter_addresses: AdapterAddress[]
    entity: Entity
}

export interface AdapterAddress {
    name: string
    v4?: string
    v6?: string
}

export interface Entity {
    create_time: string
    id: string
    last_fetch_time: string
    name: string
}

export async function testPasswordAuthenticatable(): Promise<boolean> {
    try {
        const resp = await fetch('/api/client/auth', publicRequestConfig())
        return resp.ok
    } catch (e) {
        return false
    }
}

export async function editClientName(targetClientId: string, newName: string) {
    const bodyObj = {
        new_name: newName
    }
    await fetch(`/api/client/${targetClientId}`, {
        method: 'put',
        body: JSON.stringify(bodyObj),
        ...publicRequestConfig()
    })
}

export async function getAllClientsInformation(): Promise<ClientInformation[]> {
    const resp = await fetch('/api/client', publicRequestConfig())
    return (await resp.json()) as ClientInformation[]
}

function publicRequestConfig(): RequestInit {
    const password = sessionStorage.getItem("password")!!
    return {
        headers: {
            'Content-Type': 'application/json;charset=utf-8',
            Authorization: `Basic ${password}`
        }
    }
}
