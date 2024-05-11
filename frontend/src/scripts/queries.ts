const SERVER_IP_ADDR = import.meta.env.VITE_BACKEND_IP_ADDR

export const login = async (username: string, password: string): Promise<void> => {
    console.log(username, password);
    const response = await fetch(`${SERVER_IP_ADDR}/login`, {
        method: "POST",
        mode: "no-cors",
        body: JSON.stringify({
            username: username,
            password: password
        })
    });
    console.log(response)
}