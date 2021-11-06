const Applet = imports.ui.applet;
const Util = imports.misc.util;

class Kilotime extends Applet.TextApplet {
    constructor(orientation, panel_height, instance_id) {
        super(orientation, panel_height, instance_id);

        this.setAllowedLayout(Applet.AllowedLayout.BOTH);

        try {
            this.orientation = orientation;
            this.interval_id = 0;
        } catch (e) {
            global.logError(e);
        }
    }
    _updateClock() {
        let now = new Date(),
            then = new Date(
                now.getFullYear(),
                now.getMonth(),
                now.getDate(),
                0, 0, 0),
            diff = now.getTime() - then.getTime(),
            kiloseconds = Math.floor(diff / 1000000),
            seconds = Math.floor((diff / 1000)) % 1000;
        //true to get the decimal format
        if (false) {
            let kiloseconds = Math.floor(diff / 1000) / 1000;
            this.set_applet_label(("o" + kiloseconds.toFixed(3)).slice(-6) + "k");
        } else {
            this.set_applet_label(kiloseconds + "K " + ("000" + seconds).slice(-3) + "S");
        }
    }
    on_applet_added_to_panel() {
        this._updateClock();

        if (this.interval_id == 0) {
            this.interval_id = setInterval(() => {
                this._updateClock();
            }, 1000);
        }

    }
    on_applet_removed_from_panel() {
        if (this.interval_id > 0) {
            clearInterval(this.interval_id);
            this.interval_id = 0;
        }
    }
}

function main(metadata, orientation, panel_height, instance_id) {
    return new Kilotime(orientation, panel_height, instance_id);
}
