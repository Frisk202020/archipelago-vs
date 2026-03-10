namespace App {
  const GAME_SHEET_ID = 1484584255;
  const SESSION_SHEET_ID = 1178533548;
  const PLAYER_SHEET_ID = 1081434893;
  const GENERAL_ID = 0;
  const DEFAULT_BG = "#ffffff";
  let players_cache: string[][] | null = null;
  function players(x: GlobalData) {
    if (players_cache) { return players_cache; }

    players_cache = x.players.map((x)=>[x]);
    return players_cache;
  }

  export type Sheet = GoogleAppsScript.Spreadsheet.Sheet;

  function get_first_row(sheet: Sheet, colums: number) {
    return sheet.getRange(1, 2, 1, colums);
  } function get_first_column(sheet: Sheet, rows: number) {
    return sheet.getRange(2, 1, rows, 1);
  }

  function write_players_sheet(sheet: Sheet, data: GlobalData) {
    const n = data.players.length;
    get_first_row(sheet, n).setValues([data.players]);
    get_first_column(sheet, n).setValues(players(data));

    const display = data.get_player_display();
    sheet.getRange(2, 2, n, n)
      .setValues(display.data)
      .setBackgrounds(display.colors);
  }

  function write_games_sheet(sheet: Sheet, data: GlobalData) {
    get_first_row(sheet, data.games.length).setValues([data.games]);
    get_first_column(sheet, data.players.length).setValues(players(data));

    const display = data.get_game_display();
    sheet.getRange(2, 2, data.players.length, data.games.length)
      .setValues(display.data)
      .setBackgrounds(display.colors);
  }

  function write_general_sheet(sheet: Sheet, data: GlobalData) {
    const colors = get_colors();

    get_first_row(sheet, 3).setValues([["Nombre de VS", "Victoires", "Score (%)"]]);
    get_first_column(sheet, data.players.length).setValues(players(data));
    sheet.getRange(2,2,data.players.length,3).setValues(data.get_general_display());
    sheet.getRange(2,2,data.players.length,2).setBackgrounds(Array(data.players.length).fill(Array(2).fill(colors.empty)));
    const rule = SpreadsheetApp.newConditionalFormatRule()
      .setGradientMaxpointWithValue(
        colors.victory.to_hex(),
        SpreadsheetApp.InterpolationType.NUMBER,
        '100',
      )
      .setGradientMinpointWithValue(
        colors.defeat.to_hex(),
        SpreadsheetApp.InterpolationType.NUMBER,
        '0',
      )
      .setRanges([sheet.getRange(2,4,data.players.length,1)])
      .build();
    sheet.setConditionalFormatRules([rule]);
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

  function write_sheet(
    ss: GoogleAppsScript.Spreadsheet.Spreadsheet, 
    id: number,
    writter: (x: Sheet, y: GlobalData)=>void,
    data: GlobalData
  ) {
    const sheet = ss.getSheetById(id);
    if (!sheet) { throw new Error(`Sheet ${id} not found`); }
    clear(sheet);
    
    writter(sheet, data);
    shared_style(sheet);
  } export function main() {
    const ss = SpreadsheetApp.getActiveSpreadsheet();
    const session_sheet = ss.getSheetById(SESSION_SHEET_ID);

    if (!session_sheet) { throw new Error("Session Sheet not found"); }
    const session_data = parse_sheet(session_sheet);
    const data = GlobalData.build_from_sessions(session_data);

    write_sheet(ss, GAME_SHEET_ID, write_games_sheet, data);
    write_sheet(ss, PLAYER_SHEET_ID, write_players_sheet, data);
    write_sheet(ss, GENERAL_ID, write_general_sheet, data);
  }
}