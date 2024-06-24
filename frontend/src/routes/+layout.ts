import { getIsLoggedIn } from "$lib/scripts/queries";

// disable server-side rendering
// export const ssr = false;
export const load = async ({fetch}) => {

   const isLoggedIn = await getIsLoggedIn(fetch);
   return {
      "isLoggedIn": isLoggedIn,
   }
}