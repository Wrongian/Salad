import { error, redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { getUserInsights } from "$lib/scripts/queries";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ fetch, parent, url }) => {
  const { isLoggedIn } = await parent();
  if (!isLoggedIn) return redirect(302, "/auth/login");

  const intervalType = url.searchParams.get("interval");
  const userInsights = (await getUserInsights(fetch)) ?? {
    total_profile_views: 0,
    interval_views: [],
    interval_unfollows: [],
    interval_shares: [],
    interval_follows: [],
    interval_follow_requests: [],
  };
  return { intervalType, ...userInsights };
};
