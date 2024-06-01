import type { TAuthResult, TUpdateProfileQuery } from "./query.d.ts";
import { blackSwanError } from "../stores/stores.js";
import { get } from "svelte/store";
import { goto, invalidateAll } from "$app/navigation";
import { authResponseValidator } from "./response-validator.js";
import { addError } from "$lib/modules/Errors.svelte";
const SERVER_IP_ADDR = import.meta.env.VITE_BACKEND_IP_ADDR



/**
 * forms a POST query to the /login endpoint to validate and log in user
 * expects: status 400 with message on error and 200 on successful login
 * expects: response with a body of type { result: boolean, token: string } 
 * expects: if fetch promise is rejected, then response has the type { status: 400, message: error_message }
 * TODO: bearer/cookie based token + boolean result 
 * @param username 
 * @param password 
 */
export const login = async (username: string, password: string): Promise<void> => {
    const response: TAuthResult = await fetch(`${SERVER_IP_ADDR}/login`, {
        method: "POST",
        mode: "cors",
        body: JSON.stringify({
            username: username,
            password: password
        })
    }).then(async success => {
        const resBody = await success.json()
        if (!authResponseValidator(resBody)) return Promise.reject("Obtained an invalid response body.")
        return {status: success.status, err: resBody.err }
    }).catch(err => {
        return {status: 500, err: JSON.stringify(err)}
    });

    if (response.status === 200) {
        // code to redirect client to GET profile/:userId
        goto('/profiles')
    } else if (response.status === 400) {
        // TODO: type validation and integration tests 
        addError(response.err, response.status)
    } else {
        // render error page on other error status codes
        blackSwanError.set({status: response.status, message: response.err})
    }
}

/**
 * forms a POST query to the /register endpoint to create a new user if it doesn't exist in the database. 
 * expects: status 400 with message on error and 200 
 * TODO: bearer/cookie based token + boolean result
 * 
 * @param email 
 * @param username 
 * @param password 
 */
export const register = async (email: string, username: string, password: string): Promise<void> => {

    const response: TAuthResult = await fetch(`${SERVER_IP_ADDR}/register`, {
        method: "POST",
        mode: "cors",
        body: JSON.stringify({
            email: email,
            username: username,
            password: password
        })
    }).then(async success => {
        const resBody = await success.json()
        if (!authResponseValidator(resBody)) return Promise.reject("Obtained an invalid response body.")
        return {status: success.status, err: resBody.err }
    }).catch(err => {
        return {status: 400, err: JSON.stringify(err)}
    });

    if (response.status === 200) {
        // code to redirect client to GET profile/:userId
        goto('/profiles')
    } else if (response.status === 400) {
        // TODO: flash svelte error
        addError(response.err, response.status);
    } else {
        blackSwanError.set({status: response.status, message: response.err})
    }

}

// TODO: server-side CORS 
export const resetPassword = async (email: string) => {
    console.log(SERVER_IP_ADDR)
    const response = await fetch(`${SERVER_IP_ADDR}/users`, {
        method: "PUT",
        body: JSON.stringify({
            email: email    
        })
    }).catch(err => {
        return {status: 400, message: err}
    });


    if (response.status === 200) {
        // reloads the current login page
        invalidateAll()
    } else {
        // TODO: flash svelte error
    }
    // uncomment to test for reset routing 
    // invalidateAll()
}

// TODO: server-side CORS 
export const updateProfile = async (updateQuery: TUpdateProfileQuery) => {
    const response = await fetch(`${SERVER_IP_ADDR}/profiles`, {
        method: "PUT",
        body: JSON.stringify(updateQuery)
    })

    if (response.status === 200) {
        // code to redirect client to GET profile/:userId
        goto('/profiles')
    } else {
        // TODO: flash svelte error
    }

}