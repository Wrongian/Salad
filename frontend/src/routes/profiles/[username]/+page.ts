import { getProfile, getLinks } from "$lib/scripts/queries";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ data, route, fetch, params }) => {
  const profileData = await getProfile(params.username, fetch);
  if (!profileData) {
    error(404, {
      message: "Profile not found",
    });
  }
  let links = await getLinks(params.username, fetch);

  return {
    ...profileData,
    links,
  };
};
