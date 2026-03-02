namespace App {
    type Range = GoogleAppsScript.Spreadsheet.Range;
    export class RGB {
        r: number; b: number; g: number;
        constructor(r: number, g: number, b: number) {
            this.r = r; this.g = g; this.b = b;
        }

        to_hex(): string {
            return "#"+[
                this.r, this.g, this.b
            ].map((x)=>x.toString(16).padStart(2, "0")).join("");
        }
    }
    const PADDING = 20; 
    export const COLORS = {
        win: new RGB(255, 215, 0),
        lose: new RGB(248, 125, 133),
        void: "#494848",
        labels: "#989898",
        empty: "#7e7d7d"
    }

    export function shared_style(sheet: Sheet) {
        const range = sheet.getDataRange();
        center(range);
        resize(sheet, range);
        sheet.getRange(1,1).setBackground(COLORS.void);
        sheet.getRange(1,2,1,range.getLastColumn()-1).setBackground(COLORS.labels);
        sheet.getRange(2,1,range.getLastRow()-1,1).setBackground(COLORS.labels);
    }

    function center(range: Range) {
        range.setHorizontalAlignment("center");
        range.setVerticalAlignment("middle");
    } function resize(sheet: Sheet, range: Range) { 
        const max = range.getLastColumn();
        sheet.autoResizeColumns(1, max);
        for (let i = 1; i <= max; i++) {
            const currentWidth = sheet.getColumnWidth(i);
            sheet.setColumnWidth(i, currentWidth + PADDING);
        }
    }
}