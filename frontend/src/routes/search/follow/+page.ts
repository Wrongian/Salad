import { getFollowers, getFollowings } from "$lib/scripts/queries";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {

    const getFollowerResult = await getFollowers("", 1, fetch)
    const getFollowingResult = await getFollowings("", 1, fetch)

    const followers = (getFollowerResult)?.profiles ?? [];
    const followings = (getFollowingResult)?.profiles ?? [];

    const totalFollowers = getFollowerResult?.total_size ?? 1;
    const totalFollowing = getFollowerResult?.total_size ?? 1;

    return {
        followers,
        followings,
        totalFollowers,
        totalFollowing
    };
};