import {AuthClient} from "@dfinity/auth-client";
import {Actor, HttpAgent} from "@dfinity/agent";
import {idlFactory} from "../../../declarations/ic_wallet_backend";


// CANISTER_ID is replaced by webpack based on node environment
export const canisterId = "be2us-64aaa-aaaaa-qaabq-cai"

export const createActor = (canisterId, options = {}) => {
    const agent = options.agent || new HttpAgent({...options.agentOptions});

    if (options.agent && options.agentOptions) {
        console.warn(
            "Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent."
        );
    }

    agent.fetchRootKey().catch((err) => {
        console.warn(
            "Unable to fetch root key. Check to ensure that your local replica is running"
        );
        console.error(err);
    });
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...options.actorOptions,
    });
};


export const get_actor = async () => {
    const authClient = await AuthClient.create();
    const identity = await authClient.getIdentity();
    return createActor(canisterId, {
        agentOptions: {
            identity,
            host: window.location.href,
        }
    });
}

export async function identify() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        return authClient.getIdentity();
    }
    let identityProvider = "https://identity.ic0.app/#authorize";
    // if (import.meta.env.VITE_DFX_NETWORK != "ic") {
    //     identityProvider = `http://${import.meta.env.VITE_IDENTITY_PROVIDER_ID}.localhost:8510/#authorize`
    // }

    //     internet_identity: http://127.0.0.1:8000/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bw4dl-smaaa-aaaaa-qaacq-cai
    // identityProvider = `http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943/#authorize`
    return await authClient.login({
        identityProvider,
        onSuccess: () => {
            window.location.reload()
        }
    });
}

export async function is_logged() {
    const authClient = await AuthClient.create();
    return await authClient.isAuthenticated()
}

export async function logout() {
    const authClient = await AuthClient.create();
    await authClient.logout()
}