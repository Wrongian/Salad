import { getProfile } from "../../../scripts/queries";
import type { PageLoad } from "./$types";

/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ data, route, fetch, params }) => {
  const profileData = await getProfile(params.username);
  return {
    ...profileData,
    links: [
      {
        link_id: 0,
        title: "link 0 title",
        href: "/",
        description: "link description 0",
        picture: "this is a link picture",
      },
      {
        link_id: 1,
        title: "link 1 title",
        href: "/",
        description: "link description 1",
        picture: "this is a link picture",
      },
      {
        link_id: 2,
        title: "link 2 title",
        href: "/",
        description: "link description 2",
        picture: "this is a link picture",
      },
    ],
  };
};
