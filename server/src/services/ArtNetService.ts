import getLogger from "../utils/logger.js";
import dgram from "node:dgram";
import env from "../env.js";

class ArtNetService {
    private static _instance: ArtNetService | null = null;
    // map of universe to ip address
    private static nodes: { [universe: number]: string } = {};

    public static getInstance(
        sendBroadcast: boolean = env.SEND_ARTNET_AS_BROADCAST_ANYWAY === "true",
    ): ArtNetService {
        if (!ArtNetService._instance) ArtNetService._instance = new ArtNetService(sendBroadcast);
        return ArtNetService._instance;
    }

    /**
     * Links a universe to the desired destination ip address (of the artnet-node listening on that universe)
     * note: only one node/ ip address per universe is supported
     * @param universe the universe of the node
     * @param ip the ip address of the node
     */
    public static addNode(universe: number, ip: string) {
        ArtNetService.nodes[universe] = ip;
    }

    private socket: dgram.Socket;

    private constructor(useBroadcast: boolean = false) {
        this.socket = dgram.createSocket({ type: "udp4" });

        this.socket.bind(6454, () => {
            if (useBroadcast) this.socket.setBroadcast(true);
        });

        this.socket.on("error", (err) => {
            this.log("ERROR", `ArtNet-Service error: ${JSON.stringify(err)}`);
            this.socket.close();
        });

        this.log("INFO", "ArtNetService initialized");
    }

    /* 
    I need to experiment with this, but I think it's not necessary in the first place
    private onMessage: (msg: Buffer, rinfo: dgram.RemoteInfo) => void = () => {};
    public setIncommingHandler(onMessage: (msg: Buffer, rinfo: dgram.RemoteInfo) => void) {
        this.onMessage = onMessage;
        this.socket.on("message", onMessage);
    } 
    */

    /**
     * Builds an artnet package with all the required headers and data
     * @param universe the universe of the artnet package
     * @param data all data to be sent/ packed into the artnet package
     * @returns the artnet package as an array which just needs to be converted to a buffer before sending it
     */
    private buildArtNetPackage(universe: number, data: Array<number>): Array<number> {
        let lenght = data.length;

        if (lenght > 512) {
            // DMX512 = 512 channels = 512 bytes
            lenght = 512;
        }

        if (lenght % 2) {
            lenght += 1;
        }

        const hUni = (universe >> 8) & 0xff;
        const lUni = universe & 0xff;
        const hLen = (lenght >> 8) & 0xff;
        const lLen = lenght & 0xff;

        // Protocol Name, Version, Sequence, Universe, Data Length
        const artnetHeader = [65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, lUni, hUni, hLen, lLen];
        const artnetPackage = artnetHeader.concat(data.slice(0, hLen * 256 + lLen));

        return artnetPackage;
    }

    public sendData(
        universe: number,
        data: Uint8ClampedArray,
        cb: (err: Error | null) => void = (err) => {
            if (err) {
                this.log("ERROR", `ArtNet-Service error: ${JSON.stringify(err)}`);
            }
        },
    ) {
        const ip = ArtNetService.nodes[universe];
        if (!ip) {
            this.log("ERROR", `ArtNet-Node for universe ${universe} not found`);
            return;
        }

        const artnetPackage = this.buildArtNetPackage(universe, Array.from(data));
        const bufferedPackage = Buffer.from(artnetPackage);

        this.socket.send(bufferedPackage, 0, bufferedPackage.length, 6454, ip, cb);
    }

    public sendDataAsync(universe: number, data: Uint8ClampedArray): Promise<void> {
        return new Promise((resolve, reject) => {
            this.sendData(universe, data, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    public closeSocket() {
        this.log("INFO", "Closing ArtNet Socket");
        this.socket.close();
        ArtNetService._instance = null;
    }

    private log = getLogger("ArtNetService");
}

export default ArtNetService;
