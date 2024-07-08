import type {
  TAuthResult,
  TResult,
  TUpdateProfileQuery,
  TUpdateProfile,
  TCreateLinkPayload,
  TUpdateLinkTitlePayload,
  TUpdateLinkBioPayload,
  TUpdateLinkHrefPayload,
  TReorderPayload,
} from "./query.d.ts";
import { blackSwanError } from "../../stores/stores.js";
import { goto, invalidateAll, replaceState } from "$app/navigation";
import {
  TLinkBodyValidator,
  TProfileBodyValidator,
  UpdateImageResponseBodyValidator,
  standardResponseValidator,
  type TLink,
  type TProfileBody,
  type TUpdateImageResponseBody,
} from "./response-validator.js";
import { addError } from "$lib/modules/Errors.svelte";
import type { NavigationEvent } from "@sveltejs/kit";
import { validatePayload, type Validator } from "./validator.js";
import { TErrorValidator, type TError } from "./validation/error.js";
import { TStandardPayloadValidator, type TStandardPayload } from "./validation/response.js";
import { validateFetch } from "./fetch.js";

const MASKED_ERROR_MESSAGE =
  "Oh no! Looks like something went wrong. Please try again later.";

const BASEURL = "/";
const PROFILES_PREFIX = "/api/profiles"
const UPDATE_PROFILE_IMAGE_ENDPOINT = PROFILES_PREFIX + "/image";
const UPDATE_DISPLAY_PROFILE_ENDPOINT = PROFILES_PREFIX + "/display";
const UPDATE_LINK_TITLE_ENDPOINT = "/api/links/title";
const UPDATE_LINK_BIO_ENDPOINT = "/api/links/bio";
const UPDATE_LINK_HREF_ENDPOINT = "/api/links/href";
const REORDER_LINK_ENDPOINT = "/api/links/reorder";
const DELETE_LINK_ENDPOINT = "/api/links";
const LOGIN_ENDPOINT = "/api/login";
const REGISTER_ENDPOINT = "/api/register";

const BLACKSWAN_ERROR_STATUS_CODE = 500;
const BAD_REQUEST_STATUS = 400;

type fetch = typeof fetch;
type HttpMethods = "GET" | "POST" | "PUT" | "PATCH" | "UPDATE" | "DELETE" | "HEAD";
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
  password: string,
  next: string,
): Promise<void> => {
  // validate request here
  await validateFetch<TStandardPayload, { username: string, password: string }>(
    LOGIN_ENDPOINT,
    "POST",
    { username, password },
    TStandardPayloadValidator
  )
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
  password: string,
  next: string,
): Promise<void> => {
  // validate request here
  await validateFetch<TStandardPayload, { username: string, password: string, email: string }>(
    REGISTER_ENDPOINT,
    "POST",
    { username, password, email },
    TStandardPayloadValidator
  )
};

export const updateProfile = async (updateQuery: TUpdateProfileQuery) => {
  await validateFetch<TStandardPayload, TUpdateProfileQuery>(
    UPDATE_DISPLAY_PROFILE_ENDPOINT,
    "PUT",
    updateQuery,
    TStandardPayloadValidator
  )
};


export const getProfile = async (
  username: string,
  fetch: fetch,
): Promise<TProfileBody | null> => {
  return await validateFetch<TProfileBody, { username: string }>(
    `${PROFILES_PREFIX}/${username}`,
    "GET",
    { username },
    TProfileBodyValidator
  )
};

export const getLinks = async (
  username: string,
  fetch: fetch,
): Promise<TLink[]> => {
  const result: TResult<TLink[]> = await fetch(`/api/links/${username}`, {
    method: "GET",
  })
    .then(async (success) => {
      const body = await success.json();
      if (success.ok) {
        const payload = await TLinkBodyValidator.validateAsync(body)
          .then((v) => v.links)
          .catch((e) => {
            console.log(e);
            // this is unexpected
            return Promise.reject(MASKED_ERROR_MESSAGE);
          });
        return {
          payload,
          success: true as const,
        };
      } else {
        if (!standardResponseValidator(body)) {
          // this is unexpected
          return Promise.reject(MASKED_ERROR_MESSAGE);
        }
        return {
          success: false as const,
          status: success.status,
          err: body.err,
        };
      }
    })
    .catch((err) => {
      return {
        status: BLACKSWAN_ERROR_STATUS_CODE,
        err: JSON.stringify(err),
        success: false as const,
      };
    });

  if (result.success) {
    return result.payload;
  } else if (result.status < BLACKSWAN_ERROR_STATUS_CODE) {
    addError(result.err, result.status);
    return [];
  } else {
    // something really bad happened here
    blackSwanError.set({ status: result.status, message: result.err });
    return [];
  }
};

export const getIsLoggedIn = async (fetch: fetch): Promise<boolean> => {
  let isLoggedIn: boolean = false;
  const response = await fetch("/api/logged-in", {
    method: "GET",
  })
    .then((success: any) => {
      if (success.status === 200) {
        isLoggedIn = true;
      } else {
        isLoggedIn = false;
      }
    })
    .catch((err: any) => {
      // something something error
      // placeholder
      console.log("error checking logged in");
      isLoggedIn = false;
    });
  return isLoggedIn;
};

// logout route, doesnt do anything if not logged in
// cant really get an error logging out since its a get request
export const logout = async (fetch: fetch, next: string): Promise<void> => {
  await fetch("/api/logout").catch((err) => {
    console.log("Error logging out");
    console.log(err);
  });
  await invalidateAll();
  if (next != null) {
    goto(next);
  } else {
    goto(BASEURL);
  }
};

// get username route
export const getUsername = async (fetch: fetch): Promise<string> => {
  let response = await fetch("/api/get-username")
    .then(async (success) => {
      const payload = await success.json();
      return {
        payload: payload,
        status: success.status,
      };
    })
    .catch((error) => {
      return { status: 400, payload: error };
    });
  if (response.status === 200) {
    return response.payload.username;
  }
  return "";
};

export const updateTextProfile = async (
  query: TUpdateProfile,
): Promise<void> => {
  let response = await fetch("/api/profiles/display", {
    method: "PUT",
    body: JSON.stringify(query),
  })
    .then(async (res) => {
      let body = await res.json();
      return {
        status: res.status,
        err: body.err,
      };
    })
    .catch((err) => {
      return { status: 400, err };
    });

  if (response.status === 200) {
    // code to redirect client to GET profile/:username
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred updating profile",
    });
  }
};

export const addLinks = async (query: TCreateLinkPayload): Promise<void> => {
  let response = await fetch("/api/links", {
    method: "POST",
    body: JSON.stringify(query),
  })
    .then(async (success) => {
      const body = await success.json();
      if (!standardResponseValidator(body)) {
        return Promise.reject(MASKED_ERROR_MESSAGE);
      }

      return { status: success.status, err: body.err };
    })
    .catch((err) => {
      return { status: 400, err };
    });

  if (response.status === 200) {
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in updating links",
    });
  }
};

export const updateLinkTitle = async (
  query: TUpdateLinkTitlePayload,
  link_id: number,
) => {
  let response = await fetch(`${UPDATE_LINK_TITLE_ENDPOINT}/${link_id}`, {
    method: "PUT",
    body: JSON.stringify(query),
  })
    .then(async (success) => {
      const body = await success.json();
      if (!standardResponseValidator(body)) {
        return Promise.reject(MASKED_ERROR_MESSAGE);
      }
      return { status: success.status, err: body.err };
    })
    .catch((err) => {
      return { status: 400, err };
    });

  if (response.status === 200) {
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in updating link title",
    });
  }
};

export const updateLinkBio = async (
  query: TUpdateLinkBioPayload,
  link_id: number,
) => {
  const response = await fetch(`${UPDATE_LINK_BIO_ENDPOINT}/${link_id}`, {
    method: "PUT",
    body: JSON.stringify(query),
  })
    .then(async (success) => {
      const body = await success.json();
      if (!standardResponseValidator(body)) {
        return Promise.reject(MASKED_ERROR_MESSAGE);
      }
      return { status: success.status, err: body.err };
    })
    .catch((err) => {
      return { status: 400, err };
    });

  if (response.status === 200) {
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in updating link bio",
    });
  }
};

export const updateLinkHref = async (
  query: TUpdateLinkHrefPayload,
  link_id: number,
) => {
  const response = await fetch(`${UPDATE_LINK_HREF_ENDPOINT}/${link_id}`, {
    method: "PUT",
    body: JSON.stringify(query),
  })
    .then(async (success) => {
      const body = await success.json();
      if (!standardResponseValidator(body)) {
        return Promise.reject(MASKED_ERROR_MESSAGE);
      }
      return { status: success.status, err: body.err };
    })
    .catch((err) => {
      return { status: 400, err };
    });

  if (response.status === 200) {
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in updating link href",
    });
  }
};




export const deleteLink = async (link_id: number) => {
  const response = await fetch(`${DELETE_LINK_ENDPOINT}/${link_id}`, {
    method: "DELETE",
  })
    .then(async (response) => {
      const body = await response.json();
      if (!standardResponseValidator(body)) {
        return Promise.reject(MASKED_ERROR_MESSAGE);
      }
      return { status: response.status, err: body.err };
    })
    .catch((e) => {
      return { status: 400, err: JSON.stringify(e) };
    });

  if (response.status === 200) {
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in deleting link href",
    });
  }
};

export const reorderLink = async (query: TReorderPayload) => {
  const response = await fetch(`${REORDER_LINK_ENDPOINT}`, {
    method: "POST",
    body: JSON.stringify(query),
  })
    .then(async (response) => {
      const body = await response.json();
      if (!standardResponseValidator(body)) {
        return Promise.reject(MASKED_ERROR_MESSAGE);
      }
      return { status: response.status, err: body.err };
    })
    .catch((e) => {
      return { status: 400, err: JSON.stringify(e) };
    });

  if (response.status === 200) {
    await invalidateAll();
  } else if (response.status === 400) {
    addError(response.err, response.status);
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in reordering link.",
    });
  }
};

export const updateLinkPicture = async (
  image: Blob,
  filetype: String,
  id: number,
): Promise<TUpdateImageResponseBody> => {
  let response: TResult<TUpdateImageResponseBody> = await fetch(
    "/api/links/" + id.toString() + "/image/" + filetype,
    {
      method: "PUT",
      body: image,
    },
  )
    .then(async (success) => {
      const body = await success.json();
      const validatedBody =
        await UpdateImageResponseBodyValidator.validateAsync(body).catch(
          (e) => {
            console.error(e);
            throw new Error(MASKED_ERROR_MESSAGE);
          },
        );

      return {
        payload: validatedBody,
        success: true as const,
      };
    })
    .catch((err) => {
      return {
        success: false as const,
        err: JSON.stringify(err),
        status: 400,
      };
    });

  if (response.success) {
    await invalidateAll();
    return response.payload;
  } else if (response.status === 400) {
    addError(response.err, response.status);
    return { result: false, err: response.err, href: "" };
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in updating link picture",
    });
    return {} as never;
  }
};

export const updateProfilePicture = async (
  image: Blob,
  filetype: String,
): Promise<TUpdateImageResponseBody> => {
  let response: TResult<TUpdateImageResponseBody> = await fetch(
    `${UPDATE_PROFILE_IMAGE_ENDPOINT}/${filetype}`,
    {
      method: "PUT",
      body: image,
    },
  )
    .then(async (success) => {
      const body = await success.json();
      const validatedBody =
        await UpdateImageResponseBodyValidator.validateAsync(body).catch(
          (e) => {
            console.error(e);
            throw new Error(MASKED_ERROR_MESSAGE);
          },
        );

      return {
        payload: validatedBody,
        success: true as const,
      };
    })
    .catch((err) => {
      return {
        success: false as const,
        err: JSON.stringify(err),
        status: 400,
      };
    });

  if (response.success) {
    await invalidateAll();
    return response.payload;
  } else if (response.status === 400) {
    addError(response.err, response.status);
    return { result: false, err: response.err, href: "" };
  } else {
    blackSwanError.set({
      status: response.status,
      message: "Error occurred in updating profile picture",
    });
    return {} as never;
  }
};
