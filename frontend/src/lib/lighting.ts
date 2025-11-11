import { useFetch } from "./utils";

export async function runArtnetScene(sceneName: string): Promise<boolean> {
    const result = await useFetch(`/dmx/scene/${sceneName}`, "POST");

    return result.success;
}
