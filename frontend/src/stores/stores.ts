import type { TErrorContext } from "$lib/types/ErrorTypes";
import { writable, type Writable } from "svelte/store";


export const errorStore: Writable<TErrorContext[]> = writable([])

export type TBlackSwanError = {
    status: number,
    message: string
}

export const blackSwanError = writable<TBlackSwanError | undefined>();


