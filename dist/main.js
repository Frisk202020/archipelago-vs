function main() {
    App.main();
}

function onOpen() {
  const ui = SpreadsheetApp.getUi();
  
  ui.createMenu('Script de Frisk') // The name on the toolbar
    .addItem('Mettre à jour les données', 'main')
    .addToUi();
}