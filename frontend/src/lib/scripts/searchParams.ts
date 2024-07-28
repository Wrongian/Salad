

/**
 * 
 * @param params any object with the shape { k: v }, where k <: string, and typeof v <: object | number | string
 * 
 * For example, the object: `{key1: value1, key2: value2, ...}`
 * @returns a searchparam string in the form `key1=value1&key2=value2...` 
 */
export function getAsSearchParamString(params: {[key: string]: object | number | string}) {
    const entries = Object.entries(params).map(entry => [entry[0], entry[1].toString()])
    return new URLSearchParams(entries).toString()
}