namespace App {
  const SESSION_SHEET_ID = 1178533548;
  
  export type Sheet = GoogleAppsScript.Spreadsheet.Sheet;
  export type Spread = GoogleAppsScript.Spreadsheet.Spreadsheet;

  export function main() {
    const ss = SpreadsheetApp.getActiveSpreadsheet();
    const session_sheet = ss.getSheetById(SESSION_SHEET_ID);

    if (!session_sheet) { throw new Error("Session Sheet not found"); }
    const session_data = Session.parse_sheet(session_sheet);
    const data = Global.GlobalData.build_from_sessions(session_data);

    Write.write_sheets(ss, data);
  }
}