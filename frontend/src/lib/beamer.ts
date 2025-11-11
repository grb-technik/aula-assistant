import { useFetch } from "./utils";

export async function setBeamerPower(on: boolean): Promise<boolean> {
    const result = await useFetch(`/beamer/power/${on ? "on" : "off"}`, "POST");

    return result.success;
}
