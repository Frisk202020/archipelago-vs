namespace App {
    export function set_colors(
        win: string, lose: string,
        c_void: string, labels: string,
        empty: string
    ) {
        console.log(win)
    }

    export function fetch_current_colors() {
        return {
            win: COLORS.win.to_hex(),
            lose: COLORS.lose.to_hex(),
            empty: COLORS.empty, labels: COLORS.labels, void: COLORS.void
        };
    }
}