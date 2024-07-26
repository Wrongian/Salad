import { getIsLoggedIn, getUsername } from "$lib/scripts/queries";

// disable server-side rendering
// export const ssr = false;
export const load = async ({ fetch }) => {
  const isLoggedIn = await getIsLoggedIn(fetch);
  var username = await getUsername(fetch);

  return {
    isLoggedIn: isLoggedIn,
    username: username,
  };
};
