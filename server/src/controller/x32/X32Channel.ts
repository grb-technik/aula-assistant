import { fromBuffer, OscArgOutput, OscMessageOutput } from "osc-min";
import OSCx32Service from "../../services/OSCx32Service.js";

/**
 * M1 = 01
 * M2 = 02
 * M3 = 03
 * H1 = 04
 * DN300Z = 09
 * BEAMER = 15
 */
export default class X32Channel {
    private channel: string;
    private mix_on: "ON" | "OFF";
    private mix_fader: number;

    constructor(channel: string) {
        this.channel = channel;
        this.mix_on = "OFF";
        this.mix_fader = 0.75;
        OSCx32Service.getInstance().socket.on("message", (buffer, _) => {
            const data = fromBuffer(buffer);
            if (data.oscType === "message") {
                const msg = data;
                if (msg.address.startsWith(`/ch/${this.channel}`)) {
                    switch (msg.address) {
                        case `/ch/${this.channel}/mix/on`:
                            this.mix_on = this.getValue(msg) === 1 ? "ON" : "OFF";
                            return;
                        case `/ch/${this.channel}/mix/fader`:
                            this.mix_fader = this.getValue(msg) as number;
                            return;
                    }
                }
            }
        });
    }

    toggleMute() {
        this.mix_on = this.mix_on === "ON" ? "OFF" : "ON";
        OSCx32Service.getInstance().sendOSC(`/ch/${this.channel}/mix/on`, this.mix_on);
    }

    faderPlus() {
        this.mix_fader = this.mix_fader + 0.15;
        OSCx32Service.getInstance().sendOSC(`/ch/${this.channel}/mix/fader`, this.mix_fader);
    }

    faderMinus() {
        this.mix_fader = this.mix_fader - 0.15;
        OSCx32Service.getInstance().sendOSC(`/ch/${this.channel}/mix/fader`, this.mix_fader);
    }

    private getValue(msg: OscMessageOutput): any {
        if (Array.isArray(msg.args)) {
            return msg.args[0].value;
        } else {
            return (msg.args as OscArgOutput).value;
        }
    }
}
