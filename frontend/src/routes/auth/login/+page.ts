import type { PageLoad } from './$types';


/**
 * validates and prepares the corresponding page data
 * @param param0 
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({url, data, route, fetch, params}) => {
    // let searchParams = url.searchParams;
    // let nextRoute = searchParams.get("next");
    // url.searchParams.set("next", next)
    // goto(`?${url.searchParams.toString()}`);
    return {
        reset: false
    };
};