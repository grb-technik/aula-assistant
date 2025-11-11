import dgram from "dgram";
import { OscArgInput, toBuffer as toOSCBuffer } from "osc-min";
import env from "../env.js";
import getLogger from "../utils/logger.js";

type X32Info = {
    host: string;
    port: number;
};

class OSCx32Service {
    private static _instance: OSCx32Service | null = null;
    public static getInstance(host: string = env.X32_IP): OSCx32Service {
        if (!OSCx32Service._instance) OSCx32Service._instance = new OSCx32Service(host);
        return OSCx32Service._instance;
    }

    public socket: dgram.Socket;
    private x32_info: X32Info;
    private constructor(host: string, port: number = 10023) {
        this.x32_info = { host, port };

        this.socket = dgram.createSocket("udp4");
        this.socket.bind(0, "0.0.0.0");
        this.socket.on("error", (err) => {
            this.log("ERROR", err.message);
            this.socket.close();
        });

        setInterval(() => {
            this.sendOSC("/status");
            this.sendOSC("/xremote");
        }, 1000);

        this.log("INFO", "OSCx32Service initialized");
    }

    public sendOSC(address: string, ...args: (string | number)[]) {
        const buffer = toOSCBuffer({
            address,
            args: args.map<OscArgInput>((arg) => {
                if (typeof arg === "number") {
                    return { type: "float", value: arg };
                } else if (typeof arg === "string") {
                    return { type: "string", value: arg };
                } else {
                    throw new Error();
                }
            }),
        });
        this.socket.send(buffer, this.x32_info.port, this.x32_info.host);
    }

    public closeUDPSocket() {
        this.log("INFO", "Closing OSC Socket");
        this.socket.close();
        OSCx32Service._instance = null;
    }

    private log = getLogger("OSCx32Service");
}

export default OSCx32Service;
