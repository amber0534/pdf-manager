#!/bin/bash

#
# Usage: ./launch.sh
#
# Mbv rofi zal er een pdf mbv zathura worden geopend.
#

# Rofi theme file
ROFI_THEME="${HOME}/docs/scripts/rofi/pdf_opener/theme.rasi"
# Database file met pdfs
DATABASE_FILE="${HOME}/docs/scripts/rofi/pdf_opener/pdf_manager/pdfs.txt"
# Rofi commando voor in pipeline
PROMPT_STR="Open"
ROFI_COMMAND="rofi -dmenu -i -theme $ROFI_THEME -p $PROMPT_STR"

# Roep rofi aan
CHOICE=$(
  sed 's/.* @ //g' ${DATABASE_FILE} |
  $ROFI_COMMAND
)
EXIT_CODE=$?

# Exit code van rofi is 0 als de gebruiker een keuze heeft gemaakt,
# 1 als de gebruiker escapet, en iets van 10-28 bij gebruik van
# keybindings (niet van toepassing hier)
if [[ $EXIT_CODE != 0 ]]; then
  echo "Exit status not equal to 0"
  exit 1
fi

PDF_FILE=$(
  cat $DATABASE_FILE | \
  grep " @ $CHOICE$" | \
  sed 's/ @ .*//g'
)

# Check of file bestaat
if [[ -f "$PDF_FILE" ]]; then
  zathura $PDF_FILE
else
  echo "File '$PDF_FILE' bestaat niet."
fi


