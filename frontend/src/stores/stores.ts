import type { TErrorContext } from "$lib/types/ErrorTypes";
import {get, writable, type Writable } from "svelte/store";


export const errorStore: Writable<Map<string, TErrorContext>> = writable(new Map<string, TErrorContext>())

export type TBlackSwanError = {
    status: number,
    message: string
}

export const blackSwanError = writable<TBlackSwanError | undefined>();



// export const routeStore = writable({
//     prevRoute: '/'
// });

// export const updatePageStore = () => {
//     afterNavigate(({ to, from }) => {
//         // console.log("to" +  to?.url.pathname)
//         // console.log("from" + from?.url.pathname)
//         let prevRoute = get(routeStore).prevRoute;
//         let fromRoute = from?.url.pathname || "";
//         if (prevRoute != fromRoute) {
//             routeStore.set({
//                 prevRoute : fromRoute,
//             })
//         } 
//     });

//     return routeStore;
// };
