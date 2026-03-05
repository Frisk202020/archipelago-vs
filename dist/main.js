function main() {
    App.main();
}

function onOpen() {
  const ui = SpreadsheetApp.getUi();
  
  ui.createMenu('Script de Frisk') // The name on the toolbar
    .addItem('Mettre à jour les données', 'main')
    .addItem('Palette de couleurs', 'init_html')
    .addToUi();
}

function get_colors() {
  return App.fetch_current_colors();
} function set_colors() {
  App.set_colors()
} function init_html() {
  const html = HtmlService.createHtmlOutputFromFile("index");
  SpreadsheetApp.getUi()
    .showModalDialog(html, "Palette de couleur")
}