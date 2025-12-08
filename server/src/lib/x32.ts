import { Socket } from "node:dgram";
import { registerOSCListener, sendOSC } from "./core/osc.js";

export function x32StartHeartbeat(socket: Socket) {
    const id = setInterval(() => {
        sendOSC(socket, "/status");
        sendOSC(socket, "/xremote");
    }, 1000);
    return () => clearInterval(id);
}

export function x32LoadScene(socket: Socket, scene: number) {
    sendOSC(socket, "/-action/goscene", scene);
}

export type X32ChannelType = "main/m" | `ch/${number}`;

export const X32FaderDiff = 0.15;

export function x32ChannelFaderLevel(socket: Socket, channel: X32ChannelType, level: number) {
    if (level < 0) level = 0;
    if (level > 1) level = 1;
    sendOSC(socket, `/${channel}/mix/fader`, level);
}

export function x32ChannelMute(socket: Socket, channel: X32ChannelType, muted: boolean) {
    sendOSC(socket, `/${channel}/mix/on`, muted ? 0 : 1);
}

export function x32ChannelListen(socket: Socket, channel: X32ChannelType, listeners: {
    onMuteChange: (muted: boolean) => void;
    onFaderChange: (level: number) => void;
}) {
    registerOSCListener<number>(socket, (address, value) => {
        if (address.startsWith(`/${channel}`)) {
            switch (address) {
                case `/${channel}/mix/on`:
                    const muteValue = value === 0;
                    listeners.onMuteChange(muteValue);
                    return;
                case `/${channel}/mix/fader`:
                    const faderValue = value as number;
                    listeners.onFaderChange(faderValue);
                    return;
            }
        }
    });
}
