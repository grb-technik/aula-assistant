import { Socket } from "node:net";
import { Result } from "../../utils/trycatch.js";

export function createTCPClient(host: string, port: number): Result<Socket, Error> {
    try {
        const client = new Socket();
        client.connect(port, host);
        return {
            data: client,
            error: null
        };
    } catch (err) {
        return {
            data: null,
            error: err as Error
        };
    }
}

export function sendTCPRequest(
    client: Socket,
    buffer: Buffer
): Promise<Result<Buffer, Error>> {
    if (!isTCPClientReady(client)) {
        return Promise.resolve({
            data: null,
            error: new Error("TCP client is not ready")
        });
    }

    return new Promise((resolve) => {
        try {
            let data: Buffer[] = [];

            const onData = (chunk: Buffer) => {
                data.push(chunk);
            }

            const onEnd = () => {
                cleanup();
                resolve({
                    data: Buffer.concat(data),
                    error: null
                });
            }

            const cleanup = () => {
                clearTimeout(timeout);
                client.off("data", onData);
                client.off("end", onEnd);
                client.end();
                client.destroy();
            }

            const timeout = setTimeout(() => {
                cleanup();
                resolve({
                    data: null,
                    error: new Error("TCP request timed out")
                });
            }, 1000);

            client.on("data", onData);
            client.once("end", onEnd);

            client.write(buffer);
        } catch (err) {
            return resolve({
                data: null,
                error: err as Error
            });
        }
    });

}

function isTCPClientReady(client: Socket): boolean {
    return (
        !client.closed &&
        !client.destroyed &&
        !client.errored &&
        client.readable &&
        client.writable &&
        !client.connecting
    );
}
