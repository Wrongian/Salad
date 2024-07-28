import { searchUsers } from "$lib/scripts/queries";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, url}) => {
    const searchQuery = url.searchParams.get('query') ?? ""
    const pageIndex = parseInt(url.searchParams.get('index') ?? "1")
    const userResult = await searchUsers(searchQuery, pageIndex, {}, fetch)
    const users = userResult?.profiles ?? []
    const totalSize = userResult?.total_size ?? 0;
    return {
        searchQuery,
        users,
        pageIndex,
        totalSize,
    };
};