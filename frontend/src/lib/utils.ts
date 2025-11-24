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

export async function useFetch<T = unknown>(
    path: string,
    req: {
        method: "GET" | "POST" | "PUT" | "DELETE";
        urlParams: Record<string, string>;
        queryParams: Record<string, string>;
        body: any;
    },
): Promise<Result<T, Error>> {
    for (const [key, value] of Object.entries(req.urlParams)) {
        path = path.replace(`:${key}`, value);
    }

    const fetchRequest = await tryCatch(
        fetch(`${import.meta.env.VITE_BACKEND_URL}${path}?${new URLSearchParams(req.queryParams).toString()}`, {
            method: req.method,
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req.body),
        }),
    );

    if (fetchRequest.error) {
        return {
            data: null,
            error: fetchRequest.error,
        };
    }

    if (fetchRequest.data.status !== 200) {
        return {
            data: null,
            error: new Error(`Request failed with status ${fetchRequest.data.status}`),
        };
    }

    const json = await tryCatch(fetchRequest.data.json() as Promise<T>);

    if (json.error) {
        return {
            data: null,
            error: json.error,
        };
    }

    return {
        data: json.data,
        error: null,
    };
}
