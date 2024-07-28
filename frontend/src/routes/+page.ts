import { getIsLoggedIn, getUsername } from "$lib/scripts/queries";
import { redirect } from "@sveltejs/kit";

// disable server-side rendering
// export const ssr = false;
export const load = async ({ fetch, parent }) => {
  const { isLoggedIn, username } = await parent();
  if (!isLoggedIn || !username) return redirect(302, "/auth/login");
  return redirect(302, `/profiles/${username}`);
};
