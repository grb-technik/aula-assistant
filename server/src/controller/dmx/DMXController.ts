import ArtNetService from "../../services/ArtNetService.js";
import PATCH from "./patch.js";

class DMXController {
    private data: Uint8ClampedArray = new Uint8ClampedArray(512).fill(0);

    constructor() {
        ArtNetService.addNode(0, "255.255.255.255");
    }

    clearScenes(): Promise<void> {
        this.data = new Uint8ClampedArray(512).fill(0);
        for (let i = 0; i < 24; i++) {
            // make single channel fixtures on
            this.data[i] = 255;
        }
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    sceneAnlageAn(): Promise<void> {
        this.data = new Uint8ClampedArray(512).fill(0);
        for (let i = 0; i < 24; i++) {
            // make single channel fixtures on
            this.data[i] = 255;
        }
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    sceneAnlageAus(): Promise<void> {
        this.data = new Uint8ClampedArray(512).fill(0);
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageWW(): Promise<void> {
        this.data[PATCH.CWWW_L_DIMMER] = 255;
        this.data[PATCH.CWWW_R_DIMMER] = 255;
        this.data[PATCH.CWWW_L_WW] = 255;
        this.data[PATCH.CWWW_R_WW] = 255;
        this.data[PATCH.CWWW_L_STROBE] = 0;
        this.data[PATCH.CWWW_R_STROBE] = 0;
        this.data[PATCH.CWWW_L_COLOR_TEMP] = 0;
        this.data[PATCH.CWWW_R_COLOR_TEMP] = 0;
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }
}

export default DMXController;
