async function register(name, email, password) {

    return api("/auth/register", {

        method: "POST",

        body: JSON.stringify({

            name,

            email,

            password

        })

    });

}

async function login(email, password) {

    const result = await api("/auth/login", {

        method: "POST",

        body: JSON.stringify({

            email,

            password

        })

    });

    if(result.ok){

        accessToken = result.data.access_token;

        localStorage.setItem("access_token", accessToken);

    }

    return result;

}

async function currentUser(){

    return api("/auth/current");

}

async function logout(){

    localStorage.removeItem("access_token");

    accessToken = "";

    return api("/auth/logout",{

        method:"POST"

    });

}

async function hello(){

    return api("/hello");

}