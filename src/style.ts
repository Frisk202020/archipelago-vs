namespace App {
    type Range = GoogleAppsScript.Spreadsheet.Range;
    const PADDING = 20;

    export function shared_style(sheet: Sheet) {
        const colors = get_colors();
        const range = sheet.getDataRange();

        center(range);
        resize(sheet, range);
        sheet.getRange(1,1).setBackground(colors._void);
        sheet.getRange(1,2,1,range.getLastColumn()-1).setBackground(colors.label);
        sheet.getRange(2,1,range.getLastRow()-1,1).setBackground(colors.label);
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