import type { TErrorState } from "$lib/types/ErrorTypes";
import { writable, type Writable } from "svelte/store";

export const authStore = writable(null)

export const errorStore: Writable<TErrorState> = writable()

export const err = writable();

