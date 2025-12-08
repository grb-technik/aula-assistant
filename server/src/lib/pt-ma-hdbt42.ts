import { createTCPClient, sendTCPRequest } from "./core/tcp.js";
import { Result } from "../utils/trycatch.js";

const SEND_RS232_CMD_PREFIX = "Send_H_4_4:";
const SEND_RS232_HTTP1_METHOD = "POST";
const SEND_RS232_HTTP1_PATH = "/cgi-bin/MMX32_Keyvalue.cgi";
const SEND_RS232_HTTP1_HEADER = {};
const SEND_RES232_HTTP1_BODY_PREFIX = "{CMD=>";
const CRLF = "\x0d\x0a" as const;
const LFLF = "\x0a\x0a" as const;

export async function sendRS232(host: string, port: number, command: string): Promise<Result<ParsedResponse, Error>> {
    const reqBuffer = buildRequestPackage({
        method: SEND_RS232_HTTP1_METHOD,
        host,
        port,
        body: `${SEND_RES232_HTTP1_BODY_PREFIX}${SEND_RS232_CMD_PREFIX}${command}}`,
        headers: SEND_RS232_HTTP1_HEADER,
        path: SEND_RS232_HTTP1_PATH,
    });

    const client = createTCPClient(host, port);
    if (client.error) {
        return Promise.resolve({
            data: null,
            error: client.error,
        });
    }

    const res = await sendTCPRequest(client.data, reqBuffer);
    if (res.error) {
        return {
            data: null,
            error: res.error,
        };
    }

    const parsedRes = parseReponse(res.data);
    if (parsedRes.error) {
        return {
            data: null,
            error: parsedRes.error,
        };
    }

    return {
        data: parsedRes.data,
        error: null,
    }
}

function buildRequestPackage(req: {
    method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS" | "CONNECT" | "TRACE";
    host: string;
    port: number;
    path: string;
    headers: Record<string, string>;
    body: string | Uint8Array | Buffer | null;
}): Buffer<ArrayBuffer> {
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

    req.headers["Connection"] = "close";

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

export type ParsedResponse = {
    httpVersion: string;
    statusCode: number;
    statusMessage: string;
    header: Record<string, string>;
    body: string | null;
};

function parseReponse(buf: Buffer): Result<ParsedResponse, Error> {
    const data = buf.toString();
    const splittedData = data.split(LFLF);
    if (splittedData.length < 1) {
        return {
            data: null,
            error: new Error("Invalid HTTP response: missing header section"),
        };
    }

    let headerLines = splittedData[0].split(CRLF);
    if (headerLines.length === 0) {
        return {
            data: null,
            error: new Error("Invalid HTTP response: empty header"),
        };
    }

    const statusLine = headerLines[0].split(" ");
    if (statusLine.length < 3) {
        return {
            data: null,
            error: new Error("Invalid HTTP status line"),
        };
    }

    const httpVersion = statusLine[0];
    const statusCode = parseInt(statusLine[1]);
    const statusMessage = statusLine.slice(2).join(" ");
    const header: Record<string, string> = {};
    headerLines = headerLines.slice(1);
    headerLines.forEach((line) => {
        if (!line) return;
        const [key, value] = line.split(": ");
        if (key && value) header[key] = value;
    });

    const body = splittedData[1] || null;

    return {
        data: {
            httpVersion,
            statusCode,
            statusMessage,
            header,
            body,
        },
        error: null,
    };
}
