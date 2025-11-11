import { tryCatch } from "./utils";

export async function checkAdvancedPin(_pin: string): Promise<boolean | null> {
    const result = await tryCatch(
        new Promise<boolean>((resolve, _reject) => {
            resolve(true); // TODO
        }),
    );

    if (result.error) {
        return null;
    }

    return result.data;
}
