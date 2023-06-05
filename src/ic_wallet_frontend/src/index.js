import {get_actor, identify, is_logged, logout} from "./utils/agent";

const actor = await get_actor();

async function login() {

    let is_logged_in = await is_logged();


    let loginButton = document.getElementById("login");
    if (is_logged_in) {
        loginButton.innerText = "logout";
    }
    loginButton.addEventListener("click", async () => {
        if (is_logged_in) {
            loginButton.classList.add("loader");
            await logout();
            loginButton.classList.remove("loader");
        } else {
            loginButton.classList.add("loader");
            await identify();
            loginButton.classList.remove("loader");
        }

    })

}

async function check_connection() {
    const is_connected = await actor.test_connection();
    if (is_connected === true) {
        document.querySelector(".dot").style.backgroundColor = "lightgreen"
    }
}

async function check_balance() {
    const balance = await actor.getBalance();
    console.log(balance)
    let balance_button = document.querySelector("#balance");
    balance_button.innerText = "0 ICP"
    balance_button.style.color = "green"
}

async function main() {
    await login();
    await check_connection();
    await check_balance();
    let button = document.getElementById("wallet_public_address")
    button.classList.add("loader");
    const wallet_public_address = await actor.getDepositAddress();
    button.innerText = wallet_public_address;
    button.classList.remove("loader");

}

await main();