import type { TAuthResult, TResult, TUpdateProfileQuery } from "./query.d.ts";
import { blackSwanError } from "../stores/stores.js";
import { goto, invalidateAll, replaceState } from "$app/navigation";
import {
  TLinkBodyValidator,
  TProfileBodyValidator,
  authResponseValidator,
  type TLinkBody,
  type TProfileBody,
} from "./response-validator.js";
import { addError } from "$lib/modules/Errors.svelte";
import type { NavigationEvent } from "@sveltejs/kit";

/**
 * forms a POST query to the /login endpoint to validate and log in user
 * expects: status 400 with message on error and 200 on successful login
 * expects: response with a body of type { result: boolean, token: string }
 * expects: if fetch promise is rejected, then response has the type { status: 400, message: error_message }
 * TODO: bearer/cookie based token + boolean result
 * @param username
 * @param password
 */
export const login = async (
  username: string,
  password: string
): Promise<void> => {
  const response: TAuthResult = await fetch(`/api/login`, {
    method: "POST",
    body: JSON.stringify({
      username: username,
      password: password,
    }),
    // redirect: "manual",
  })
    .then(async (success) => {
      // check and handle redirects
      if (success.redirected) {
        await goto(success.url);
        return { status: 302, err: "" };
      }

      const resBody = await success.json();
      if (!authResponseValidator(resBody))
        return Promise.reject("Obtained an invalid response body.");
      return { status: success.status, err: resBody.err };
    })
    .catch((err) => {
      return { status: 500, err: JSON.stringify(err) };
    });

  if (response.status === 200) {
    // code to redirect client to GET profile/:username
    goto(`/profiles/${username}`);
  } else if (response.status === 400) {
    // TODO: type validation and integration tests
    addError(response.err, response.status);
  } else if (response.status > 400 && response.status <= 500) {
    // render error page on other error status codes
    blackSwanError.set({ status: response.status, message: response.err });
  }
};

/**
 * forms a POST query to the /register endpoint to create a new user if it doesn't exist in the database.
 * expects: status 400 with message on error and 200
 * TODO: bearer/cookie based token + boolean result
 *
 * @param email
 * @param username
 * @param password
 */
export const register = async (
  email: string,
  username: string,
  password: string
): Promise<void> => {
  const response: TAuthResult = await fetch(`/api/register`, {
    method: "POST",
    body: JSON.stringify({
      email: email,
      username: username,
      password: password,
    }),
  })
    .then(async (success) => {
      // TODO: set cookie
      return success;
    })
    .then(async (success) => {
      // check and handle redirects
      if (success.redirected) {
        await goto(success.url);
        return { status: 302, err: "" };
      }

      const resBody = await success.json();
      if (!authResponseValidator(resBody))
        return Promise.reject("Obtained an invalid response body.");
      return { status: success.status, err: resBody.err };
    })
    .catch((err) => {
      return { status: 400, err: JSON.stringify(err) };
    });

  if (response.status === 200) {
    // code to redirect client to GET profile/:username
    goto(`/profiles/${username}`);
  } else if (response.status === 400) {
    // TODO: flash svelte error
    addError(response.err, response.status);
  } else {
    blackSwanError.set({ status: response.status, message: response.err });
  }
};

// TODO: server-side CORS
export const resetPassword = async (email: string) => {
  const response = await fetch(`/api/users`, {
    method: "PUT",
    body: JSON.stringify({
      email: email,
    }),
  }).catch((err) => {
    return { status: 400, message: err };
  });

  if (response.status === 200) {
    // reloads the current login page
    invalidateAll();
  } else {
    // TODO: flash svelte error
  }
  // uncomment to test for reset routing
  // invalidateAll()
};

// TODO: server-side CORS
export const updateProfile = async (updateQuery: TUpdateProfileQuery) => {
  const response = await fetch(`/api/profiles`, {
    method: "PUT",
    body: JSON.stringify(updateQuery),
  });

  if (response.status === 200) {
    // code to redirect client to GET profile/:username
    await invalidateAll();
  } else {
    // TODO: flash svelte error
  }
};

type fetch = typeof fetch;

export const getProfile = async (
  username: string,
  fetch: fetch
): Promise<TProfileBody> => {
  const result: TResult<TProfileBody> = await fetch(
    `/api/profiles/${username}`,
    { method: "GET" }
  )
    .then(async (success) => {
      return {
        payload: await TProfileBodyValidator.validateAsync(
          await success.json()
        ),
        success: true as const,
      };
    })
    .catch((err) => {
      return { status: 400, err: JSON.stringify(err), success: false as const };
    });
  if (result.success) {
    return result.payload;
  } else {
    // something really bad happened here
    blackSwanError.set({ status: result.status, message: result.err });
    return result as never;
  }
};

export const getLinks = async (username: string): Promise<TLinkBody> => {
  const result: TResult<TLinkBody> = await fetch(`/api/links/${username}`, {
    method: "GET",
  })
    .then(async (success) => {
      return {
        payload: await TLinkBodyValidator.validateAsync(await success.json()),
        success: true as const,
      };
    })
    .catch((err) => {
      return { status: 400, err: JSON.stringify(err), success: false as const };
    });

  if (result.success) {
    return result.payload;
  } else {
    // something really bad happened here
    blackSwanError.set({ status: result.status, message: result.err });
    return [] as never;
  }
};

export const getIsLoggedIn = async (fetch: fetch) : Promise<boolean> => {
  let isLoggedIn: boolean = false;
  const response = await fetch("/api/logged-in",{
    method: "GET",
  }).then((success: any) => {
    // console.log(success);
    if (success.status === 200) {
      isLoggedIn = true;
    }
    else {
      isLoggedIn = false;
    }
  }).catch((err: any) => {
    // something something error
    // placeholder
    console.log("error checking logged in");
    isLoggedIn = false;
  }
  )
  return isLoggedIn;
}
