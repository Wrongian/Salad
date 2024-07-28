import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { getProfile, getLinks } from "$lib/scripts/queries";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ fetch, parent }) => {
  // change later
  // probably use svelte store for both isloggedin and the prev routing

  const { isLoggedIn, username } = await parent();
  if (!isLoggedIn || !username) return redirect(302, "/auth/login");

  const profileData = await getProfile(username, fetch);
  const links = await getLinks(username, fetch);
  // placeholder
  return {
    ...profileData,
    links,
  };
};
