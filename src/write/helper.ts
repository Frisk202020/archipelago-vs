namespace App.Write.Helper {
    const DEFAULT_BG = "#ffffff";

    let players_cache: string[][] | null = null;
    export function players(x: Global.GlobalData) {
        if (players_cache) { return players_cache; }

        players_cache = x.players.map((x)=>[x]);
        return players_cache;
    }

    export function get_first_row(sheet: Sheet, colums: number) {
        return sheet.getRange(1, 2, 1, colums);
    } export function get_first_column(sheet: Sheet, rows: number) {
        return sheet.getRange(2, 1, rows, 1);
    }

    function get_clear_range(sheet: Sheet): {rows: number, cols: number} {
        let cols: number | null = null;
        for (let i = 1; true; i++) {
        const cell = sheet.getRange(1,i);
        if (cell.getValue() === "" && cell.getBackground() === DEFAULT_BG) {
            cols = i-1; break;
        }
        }

        for (let i = 2; true; i++) {
        const cell = sheet.getRange(i,1);
        if (cell.getValue() === "" && cell.getBackground() === DEFAULT_BG) {
            return {rows: i-1, cols};
        }
        }
    } function clear(sheet: Sheet) {
        const size = get_clear_range(sheet);
        if (size.rows === 0 || size.cols === 0) { return; }

        sheet.getRange(1, 1, size.rows, size.cols)
        .clearContent()
        .setBackgrounds(Array(size.rows).fill(Array(size.cols).fill("#ffffff")));  
    }

    interface Writter {
        fn: (s: Sheet, d: Global.GlobalData)=>void,
        id: number
    } export function write_sheet(
        ss: Spread,
        data: Global.GlobalData,
        writter: Writter
    ) {
        const sheet = ss.getSheetById(writter.id);
        if (!sheet) { throw new Error(`Sheet ${writter.id} not found`); }
        clear(sheet);
        
        writter.fn(sheet, data);
        Style.shared_style(sheet);
    }
}