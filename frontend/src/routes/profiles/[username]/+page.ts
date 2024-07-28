import { getProfile, getLinks, getFollowStatus } from "$lib/scripts/queries";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import type { TFollowStatus } from "$lib/scripts/validation/response";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ fetch, params, parent }) => {
  const { isLoggedIn } = await parent();
  const profileData = await getProfile(params.username, fetch);
  if (!profileData) {
    error(404, {
      message: "Profile not found",
    });
  }
  let followStatus: TFollowStatus | undefined = isLoggedIn
    ? await getFollowStatus(profileData.id, fetch)
    : undefined;

  let links = await getLinks(params.username, fetch);

  return {
    ...profileData,
    followStatus,
    links,
  };
};
