import { getProfile, getUsername } from "$lib/scripts/queries";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { type TProfileBody } from '../../lib/scripts/validation/response';
import { goto } from "$app/navigation";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ data, route, fetch, params }) => {
  var username = await getUsername(fetch);
  var profileData: TProfileBody | null;
  if (username) {
    profileData = await getProfile(username, fetch);
  }
  else {
    goto("/auth/login")
    profileData = null;
  }
  return {
    profileData,
  };
};
