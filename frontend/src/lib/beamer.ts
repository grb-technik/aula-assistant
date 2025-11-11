import { tryCatch } from "./utils";

export async function setBeamerPower(_on: boolean): Promise<boolean> {
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
