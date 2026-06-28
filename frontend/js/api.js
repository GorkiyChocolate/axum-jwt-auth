const API_URL = "/api";

let accessToken = localStorage.getItem("access_token") || "";

async function api(endpoint, options = {}) {

    const headers = options.headers || {};

    headers["Content-Type"] = "application/json";

    if (accessToken !== "") {
        headers["Authorization"] = `Bearer ${accessToken}`;
    }

    const response = await fetch(API_URL + endpoint, {
        ...options,
        headers,
        credentials: "include"
    });

    let data;

    try {
        data = await response.json();
    } catch {
        data = await response.text();
    }

    return {
        ok: response.ok,
        status: response.status,
        headers: response.headers,
        data
    };
}