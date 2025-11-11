import BeamerService from "../services/BeamerService.js";

class BeamerController {

    public static async powerOn() {
        return await BeamerService.getInstance().powerOn();
    }

    public static async powerOff() {
        return await BeamerService.getInstance().powerOff();
    }
    
}

export default BeamerController;
