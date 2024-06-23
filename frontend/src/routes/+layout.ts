import { getIsLoggedIn } from "../scripts/queries"

// disable server-side rendering
// export const ssr = false;
export const load = async ({fetch}) => {

 const isLoggedIn = await getIsLoggedIn(fetch);

 // testing
//  console.log(isLoggedIn)

 return {
    "isLoggedIn": isLoggedIn,
 }
}