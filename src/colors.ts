namespace App {
    class RGB {
        r: number; b: number; g: number;
        constructor(r: number, g: number, b: number) {
            this.r = r; this.g = g; this.b = b;
        }

        to_hex(): string {
            return "#"+[
                this.r, this.g, this.b
            ].map((x)=>x.toString(16).padStart(2, "0")).join("");
        } static from_hex(x: string): RGB {
            const bigint = parseInt(x.substring(1), 16);
            const r = (bigint >> 16) & 255;
            const g = (bigint >> 8) & 255;
            const b = bigint & 255;

            return new RGB(r,g,b);
        } 
    }
    export function interpolate(from: RGB, to: RGB, pRatio: number): string {
        const out = new RGB(
            from.r + pRatio * (to.r - from.r),
            from.g + pRatio * (to.g - from.g),
            from.b + pRatio * (to.b - from.b)
        );

        return out.to_hex();
    };

    class ColorLibrary {
        victory: RGB; defeat: RGB; _void: string;
        label: string; empty: string;
        static service = PropertiesService.getScriptProperties();

        constructor(
            victory: RGB, defeat: RGB, _void: string,
            label: string, empty: string
        ) {
            this.victory = victory; this.defeat = defeat; this._void = _void;
            this.label = label; this.empty = empty;
        }

        static hex_from_storage(key: "empty" | "void" | "label", fallback: string) {
            const x = this.service.getProperty(key);
            return x ? x : fallback;
        } static rgb_from_storage(key: "victory" | "defeat", fallback: RGB) {
            const x = this.service.getProperty(key);
            return x ? RGB.from_hex(x) : fallback;
        }
        to_stored_value() {
            return new ColorLibrary(
                ColorLibrary.rgb_from_storage("victory", this.victory),
                ColorLibrary.rgb_from_storage("defeat", this.defeat),
                ColorLibrary.hex_from_storage("void", this._void),
                ColorLibrary.hex_from_storage("label", this.label),
                ColorLibrary.hex_from_storage("empty", this.empty)
            );
        }
        to_hex() {
            return {
                victory: this.victory.to_hex(),
                defeat: this.defeat.to_hex(),
                empty: this.empty,
                _void: this._void,
                label: this.label 
            };
        }

        update(colors: string[]) {
            if (colors.length < 5) { throw new Error("Color array is unexpectedly short !"); }

            this.victory = RGB.from_hex(colors[0]);
            this.defeat = RGB.from_hex(colors[1]);
            this._void = colors[2];
            this.label = colors[3];
            this.empty = colors[4];
        }
        update_from(model: ColorLibrary) {
            this.victory = model.victory;
            this.defeat = model.defeat;
            this._void = model._void;
            this.label = model.label;
            this.empty = model.empty;
        }
    }

    const DEFAULT_COLORS = new ColorLibrary(
        new RGB(255, 215, 0),new RGB(248, 125, 133),
        "#494848","#989898","#7e7d7d"
    ); const COLORS = DEFAULT_COLORS.to_stored_value();

    export function get_colors() { return COLORS; }
    export function get_hex_colors() {
        return COLORS.to_hex();
    }

    export function save_colors(raw: string) {
        const colors = raw.split(";");
        for (const x of [
            ["victory", colors[0]],
            ["defeat", colors[1]],
            ["void", colors[2]],
            ["label", colors[3]],
            ["empty", colors[4]]
        ]) {
            ColorLibrary.service.setProperty(x[0], x[1]);
            COLORS.update(colors);
        }
    } export function reset_colors() {
        for (const x of [
            ["victory", DEFAULT_COLORS.victory.to_hex()],
            ["defeat", DEFAULT_COLORS.defeat.to_hex()],
            ["void", DEFAULT_COLORS._void],
            ["label", DEFAULT_COLORS.label],
            ["empty", DEFAULT_COLORS.empty]
        ]) {
            ColorLibrary.service.setProperty(x[0], x[1]);
        }

        COLORS.update_from(DEFAULT_COLORS);
    }
}