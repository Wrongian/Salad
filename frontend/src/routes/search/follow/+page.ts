import { getFollowers, getFollowings } from "$lib/scripts/queries";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ data, route, fetch, params }) => {

    const followers = (await getFollowers("", 1, fetch))?.profiles ?? [];
    const followings = (await getFollowings("", 1, fetch))?.profiles ?? [];
    return {
        followers,
        followings
    };
};