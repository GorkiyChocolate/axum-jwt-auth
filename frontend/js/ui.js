const loginTab=document.getElementById("loginTab");
const registerTab=document.getElementById("registerTab");

const loginForm=document.getElementById("loginForm");
const registerForm=document.getElementById("registerForm");

const dashboard=document.getElementById("dashboard");

const responseBox=document.getElementById("response");

const profile=document.getElementById("profile");

const token=document.getElementById("token");

const toast=document.getElementById("toast");

function showToast(message,color="#16a34a"){

    toast.textContent=message;

    toast.style.background=color;

    toast.classList.add("show");

    setTimeout(()=>{

        toast.classList.remove("show");

    },3000);

}
function printResponse(obj){

    responseBox.textContent=JSON.stringify(obj,null,4);

}
loginTab.onclick=()=>{

    loginTab.classList.add("active");

    registerTab.classList.remove("active");

    loginForm.classList.remove("hidden");

    registerForm.classList.add("hidden");

};

registerTab.onclick=()=>{

    registerTab.classList.add("active");

    loginTab.classList.remove("active");

    registerForm.classList.remove("hidden");

    loginForm.classList.add("hidden");

};