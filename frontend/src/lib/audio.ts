import { useFetch } from "./utils";

export const CHANNELS = [
    "m1",
    "m2",
    "m3",
    "h1",
    "bluetooth",
    "beamer",
    "sub",
] as const;

export type ChannelType = typeof CHANNELS[number];

export async function loadSceneDefault() {
    return (await useFetch("/x32/load-scene/default", "POST")).success;
}

export async function muteToggle(channel: ChannelType) {
    const result = await useFetch(`/x32/channel/${channel}/mute-toggle`, "POST");
    return result.success;
}

export async function faderPlus(channel: ChannelType) {
    const result = await useFetch(`/x32/channel/${channel}/fader/add`, "POST");
    return result.success;
}

export async function faderMinus(channel: ChannelType) {
    const result = await useFetch(`/x32/channel/${channel}/fader/min`, "POST");
    return result.success;
}
