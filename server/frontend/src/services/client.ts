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

export async function getAllClientsInformation(): Promise<ClientInformation[]> {
    const resp = await fetch('/api/client')
    const json = (await resp.json()) as ClientInformation[]
    return json
}
