import { tryCatch } from "./utils";

export async function loadArtnetScenes(): Promise<string[] | null> {
    const result = await tryCatch(
        new Promise<string[]>((resolve, _reject) => {
            resolve([]); // TODO
        }),
    );

    if (result.error) {
        return null;
    }

    return result.data || [];
}

export async function runArtnetScene(_sceneName: string): Promise<boolean> {
    const result = await tryCatch(
        new Promise<boolean>((resolve, _reject) => {
            resolve(true); // TODO
        }),
    );

    if (result.error) {
        return false;
    }

    return true;
}
