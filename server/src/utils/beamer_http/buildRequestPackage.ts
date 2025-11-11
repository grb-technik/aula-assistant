import { CRLF } from "./consts.js";

type HTTPMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS" | "CONNECT" | "TRACE";

export type BeamerRequest = {
    method: HTTPMethod;
    host: string;
    port: number;
    path: string;
    headers: Record<string, string>;
    body: string | Uint8Array | Buffer | null;
};

export function buildRequestPackage(req: BeamerRequest): Buffer {
    let body = null as string | Uint8Array | null;

    if (typeof req.body === "string" || req.body === null || req.body instanceof Uint8Array) {
        body = req.body;
    }
    if (req.body instanceof Buffer) {
        body = new Uint8Array(req.body);
    }

    let data = `${req.method} ${req.path} HTTP/1.0${CRLF}`;
    if (req.port === 80) {
        data += `Host: ${req.host}${CRLF}`;
    } else {
        data += `Host: ${req.host}:${req.port}${CRLF}`;
    }

    Object.entries(req.headers).forEach(([key, value]) => {
        data += `${key}: ${value}${CRLF}`;
    });
    if (body !== null) {
        data += `Content-Length: ${body.length}${CRLF}`;
    }
    data += CRLF;

    let encoder = new TextEncoder();
    let dataUint8Array = encoder.encode(data);
    if (body !== null) {
        if (typeof body === "string") {
            dataUint8Array = new Uint8Array([...dataUint8Array, ...encoder.encode(body)]);
        } else {
            dataUint8Array = new Uint8Array([...dataUint8Array, ...body]);
        }
    }

    return Buffer.from(dataUint8Array.buffer);
}
