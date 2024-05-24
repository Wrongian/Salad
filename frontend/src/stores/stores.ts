import { writable, type Writable } from "svelte/store";

export const authStore = writable(null)

export const errorStore: Writable<TErrorContext> = writable()

export type TErrorContext = {
    addError: (message: string, statusCode: number) => void,
    removeAt: (index: number) => void
}