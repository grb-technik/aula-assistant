import { Socket, createSocket } from "node:dgram";
import { fromBuffer, OscArgInput, toBuffer as toOSCBuffer } from "osc-min";
import { Result } from "../../utils/trycatch.js";

export function createOSCClient({ target, bind }: {
    target: {
        host: string;
        port: number;
    }, bind: {
        host: string;
        port: number;
    }
}): Result<Socket, Error> {
    try {
        const socket = createSocket("udp4");

        socket.bind(bind.port, bind.host);
        socket.on("error", () => {
            socket.close();
        });
        socket.connect(target.port, target.host);

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

export function sendOSC(socket: Socket, address: string, ...args: (string | number)[]): Result<undefined, Error> {
    try {
        const buffer = toOSCBuffer({
            address,
            args: args.map<OscArgInput>((arg) =>
                typeof arg === "number"
                    ? { type: "float", value: arg }
                    : { type: "string", value: arg }
            ),
        });

        const { port: socketPort, address: socketAddress } = socket.address();
        socket.send(buffer, socketPort, socketAddress);

        return {
            data: undefined,
            error: null
        };
    } catch (err) {
        return {
            data: null,
            error: err as Error
        };
    }
};

export function registerOSCListener<ArgsType = unknown>(socket: Socket, onMessage: (address: string, args: ArgsType) => void) {
    socket.on("message", (buffer, _) => {
        const data = fromBuffer(buffer);
        if (data.oscType === "message") {
            const msg = data;
            let args: ArgsType;
            if (Array.isArray(msg.args)) {
                args = msg.args.map(arg => arg.value) as unknown as ArgsType;
            } else {
                args = (msg.args as any).value as ArgsType;
            }
            onMessage(msg.address, args);
        }
    });
}
