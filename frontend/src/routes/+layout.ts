import {
  getIsLoggedIn,
  getNotifications,
  getUsername,
} from "$lib/scripts/queries";
import type { TNotificationsPayload } from "$lib/scripts/validation/response";

// disable server-side rendering
// export const ssr = false;
export const load = async ({ fetch }) => {
  const isLoggedIn = await getIsLoggedIn(fetch);
  var username = await getUsername(fetch);
  var notifications: TNotificationsPayload = { notifications: [] };
  if (username) {
    notifications = await getNotifications(fetch);
  }

  return {
    isLoggedIn: isLoggedIn,
    username: username,
    notifications: notifications,
  };
};
