import { blackSwanError } from "../../stores/stores";
const BLACKSWAN_ERROR_STATUS_CODE = 500;
const BAD_REQUEST_STATUS = 400;
import { validatePayload, type Validator } from "./validator.js";
import { TErrorValidator, type TError } from "./validation/error.js";
import {
  TStandardPayloadValidator,
  type TStandardPayload,
} from "./validation/response.js";
import { addError } from "$lib/modules/Errors.svelte";

const MASKED_ERROR_MESSAGE =
  "Oh no! Looks like something went wrong. Please try again later.";

type HttpMethods =
  | "GET"
  | "POST"
  | "PUT"
  | "PATCH"
  | "UPDATE"
  | "DELETE"
  | "HEAD";
type fetch = typeof fetch;

export async function validateFetch<R, U extends object = {}>(
  endpoint: string,
  method: HttpMethods,
  payload: U,
  validator: Validator<R>,
  options?: {
    fetch?: fetch;
  },
): Promise<R | null> {
  const hasBody = !(method == "GET" || method == "DELETE" || method == "HEAD");
  const useFetch = options?.fetch ?? fetch;
  // check body
  const request = hasBody
    ? {
        method,
        body: JSON.stringify(payload),
      }
    : {
        method,
      };
  // get the response
  const response = await useFetch(endpoint, request);

  //
  const jsonBody = await response.json().catch((_) => {
    // black swan
    blackSwanError.set({
      status: BLACKSWAN_ERROR_STATUS_CODE,
      message: MASKED_ERROR_MESSAGE,
    });
    return null;
  });

  if (!response.ok) {
    return await validatePayload<TError>(jsonBody, TErrorValidator)
      .then(({ err }) => {
        if (response.status === 400) {
          addError(err, response.status);
        } else if (response.status === 403) {
          // forbidden page
        } else if (response.status === 404) {
          // not found
        } else {
          blackSwanError.set({
            status: BLACKSWAN_ERROR_STATUS_CODE,
            message: MASKED_ERROR_MESSAGE,
          });
        }
        return null;
      })
      .catch((_) => {
        blackSwanError.set({
          status: BLACKSWAN_ERROR_STATUS_CODE,
          message: MASKED_ERROR_MESSAGE,
        });
        return null;
      });
  }
  // { payload: T }
  if ("payload" in jsonBody) {
    // validate
    return await validatePayload(jsonBody.payload, validator).catch((_) => {
      blackSwanError.set({
        status: BLACKSWAN_ERROR_STATUS_CODE,
        message: MASKED_ERROR_MESSAGE,
      });
      return null;
    });
  } else {
    blackSwanError.set({
      status: BLACKSWAN_ERROR_STATUS_CODE,
      message: MASKED_ERROR_MESSAGE,
    });
    return null;
  }
}
