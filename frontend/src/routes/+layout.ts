import { getIsLoggedIn, getUsername } from "$lib/scripts/queries";

// disable server-side rendering
// export const ssr = false;
export const load = async ({fetch}) => {

   const isLoggedIn = await getIsLoggedIn(fetch);
   const username = await getUsername(fetch);
   console.log(username)

   return {
      "isLoggedIn": isLoggedIn,
      "username": username,
   }
}