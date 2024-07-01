import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { getIsLoggedIn, getUsername } from "$lib/scripts/queries";
import { getProfile, getLinks } from "$lib/scripts/queries";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ data, route, fetch, params }: any) => {
  // change later
  // probably use svelte store for both isloggedin and the prev routing
  const isLoggedIn = await getIsLoggedIn(fetch);
  if (!isLoggedIn) {
    redirect(302, "/auth/login");
  }
  const username = await getUsername(fetch);
  if (!username || username == "") {
    redirect(302, "/auth/login");
  }
  const profileData = await getProfile(username, fetch);
  const links = await getLinks(username, fetch);
  // placeholder
  return {
    ...profileData,
    links,
  };
};
