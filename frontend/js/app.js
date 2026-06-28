loginForm.addEventListener("submit",async(e)=>{

    e.preventDefault();

    const email=document.getElementById("loginEmail").value;

    const password=document.getElementById("loginPassword").value;

    const res=await login(email,password);

    printResponse(res.data);

    if(!res.ok){

        showToast("Login failed","#dc2626");

        return;

    }

    token.value=res.data.access_token;

    dashboard.classList.remove("hidden");

    profile.innerHTML=`

        <b>${res.data.name}</b><br><br>

        Created:

        ${res.data.created_at}

    `;

    showToast("Login successful");

});

registerForm.addEventListener("submit",async(e)=>{

    e.preventDefault();

    const name=document.getElementById("registerName").value;

    const email=document.getElementById("registerEmail").value;

    const password=document.getElementById("registerPassword").value;

    const res=await register(name,email,password);

    printResponse(res.data);

    if(res.ok){

        showToast("Account created");

    }

    else{

        showToast("Registration failed","#dc2626");

    }

});

document.getElementById("currentBtn").onclick=async()=>{

    const res=await currentUser();

    printResponse(res.data);

};

document.getElementById("helloBtn").onclick=async()=>{

    const res=await hello();

    printResponse(res.data);

};

document.getElementById("logoutBtn").onclick=async()=>{

    const res=await logout();

    dashboard.classList.add("hidden");

    token.value="";

    profile.innerHTML="Not logged in";

    printResponse(res.data);

    showToast("Logged out");

};

document.getElementById("copyToken").onclick=()=>{

    navigator.clipboard.writeText(token.value);

    showToast("Token copied");

};