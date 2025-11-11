import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

type Result<T, E = Error> =
    | {
        data: T;
        error: null;
    }
    | {
        data: null;
        error: E;
    };

export async function tryCatch<T, E = Error>(promise: Promise<T>): Promise<Result<T, E>> {
    try {
        const data = await promise;
        return { data, error: null };
    } catch (error) {
        return { data: null, error: error as E };
    }
}

export async function useFetch(
    path: string,
    method: string = "GET",
    urlParams: Record<string, string> = {},
    queryParams: Record<string, string> = {},
    body: Record<string, any> = {},
): Promise<{
    success: boolean;
} | {
    success: false;
    error: string;
    deError: string;
}> {
    for (const [key, value] of Object.entries(urlParams)) {
        path = path.replace(`:${key}`, value);
    }

    const fetchRequest = await tryCatch(fetch(
        `${import.meta.env.VITE_BACKEND_URL}${path}?${new URLSearchParams(queryParams).toString()}`,
        {
            method,
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        },
    ));

    if (fetchRequest.error) {
        return {
            success: false,
            error: "Server connection failed",
            deError: "Es konnte keine Verbindung zum Server aufgebaut werden.",
        };
    }

    return {
        success: true,
    };
}

