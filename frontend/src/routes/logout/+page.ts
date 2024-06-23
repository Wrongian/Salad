import type { PageLoad } from './$types';
import { logout } from "$lib/scripts/queries";


/**
 * validates and prepares the corresponding page data
 * @param param0 
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({fetch}) => {
    await logout(fetch);
    
    return {

    };
};