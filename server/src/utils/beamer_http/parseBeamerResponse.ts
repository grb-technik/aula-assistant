import { CRLF, LFLF } from "./consts.js";

export type ParsedBeamerReponse = {
    httpVersion: string;
    statusCode: number;
    statusMessage: string;
    header: Record<string, string>;
    body: string | null;
};

export function parseBeamerReponse(buf: Buffer): ParsedBeamerReponse {
    const data = buf.toString();
    // the header and the body are seperated by a \x0a\x0a (<LF><LF>)
    const splitedData = data.split(LFLF);
    const header = splitedData[0].split(CRLF);
    const body = splitedData[1] || "";
    const parsedHeader = parseHeader(header);
    return {
        httpVersion: parsedHeader.httpVersion,
        statusCode: parsedHeader.statusCode,
        statusMessage: parsedHeader.statusMessage,
        header: parsedHeader.header,
        body: body,
    };
}

function parseHeader(header: string[]): {
    httpVersion: string;
    statusCode: number;
    statusMessage: string;
    header: Record<string, string>;
} {
    const parsedHeaders: {
        httpVersion: string;
        statusCode: number;
        statusMessage: string;
        header: Record<string, string>;
    } = {} as any;

    let statusLine = header[0].split(" ");
    header = header.slice(1);
    parsedHeaders.httpVersion = statusLine[0];
    parsedHeaders.statusCode = parseInt(statusLine[1]);
    parsedHeaders.statusMessage = statusLine[2];
    parsedHeaders.header = {};
    header.forEach((header) => {
        if (!header || header === "") return;
        let splitedHeader = header.split(": ");
        console.log(splitedHeader)
        parsedHeaders.header[splitedHeader[0]] = splitedHeader[1];
    });

    return parsedHeaders;
}
