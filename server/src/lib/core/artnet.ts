import { Socket, createSocket } from "node:dgram";
import { Result } from "../../utils/trycatch.js";

export function createArtNetClient({ bind, broadcast }: {
    bind: {
        host: string;
        port: number;
    };
    broadcast: boolean;
}): Result<Socket, Error> {
    try {
        const socket = createSocket("udp4");
        socket.bind(bind.port, bind.host);
        socket.on("error", () => {
            socket.close();
        });
        if (broadcast) socket.setBroadcast(true);

        return {
            data: socket,
            error: null
        };
    } catch (err) {
        return {
            data: null,
            error: err as Error
        };
    }
}

export function buildArtNetPackage(universe: number, data: Array<number>): Buffer<ArrayBuffer> {
    let lenght = data.length;

    if (lenght > 512) {
        lenght = 512;
    }

    if (lenght % 2) {
        lenght += 1;
    }

    const hUni = (universe >> 8) & 0xff;
    const lUni = universe & 0xff;
    const hLen = (lenght >> 8) & 0xff;
    const lLen = lenght & 0xff;

    const artnetHeader = [65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, lUni, hUni, hLen, lLen];
    const artnetPackage = artnetHeader.concat(data.slice(0, hLen * 256 + lLen));

    const buffer = Buffer.from(artnetPackage);

    return buffer;
}

export function sendArtNet(
    socket: Socket,
    buffer: Buffer<ArrayBuffer>,
    host: string,
    port: number,
): Promise<Result<null, Error>> {
    return new Promise((resolve) => {
        socket.send(buffer, 0, buffer.length, port, host, (err, bytes) => {
            if (err) {
                return resolve({
                    data: null,
                    error: err as Error
                });
            }

            resolve({
                data: null,
                error: null
            });
        });
    });
}
