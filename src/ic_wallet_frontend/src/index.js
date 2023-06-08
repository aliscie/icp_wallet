import {get_actor, identify, is_logged, logout} from "./utils/agent";
import {log} from "util";

const actor = await get_actor();

async function login() {
    let is_logged_in = await is_logged();

    let loginButton = document.getElementById("login");
    if (is_logged_in) {
        loginButton.innerText = "logout";
        let auth = await identify();
        console.log(auth)
        document.getElementById("user_principal").innerText = auth._principal.toText();
    }
    loginButton.addEventListener("click", async () => {
        console.log("click")
        if (is_logged_in) {
            console.log("logout")
            loginButton.classList.add("loader");
            await logout();
            window.location.reload();
            loginButton.classList.remove("loader");
        } else {
            loginButton.classList.add("loader");
            let x = await identify();
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
    const balances = await actor.getBalances();
    console.log("balances", balances)
    let balance_button = document.querySelector("#balance");
    balance_button.innerText = "0 ICP"
    balance_button.style.color = "green"
}

async function main() {
    // await logout();
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