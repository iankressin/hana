#!/bin/bash
HANA_DIR="$HOME/.hana"
BIN_DIR="$HANA_DIR/bin"
RECORDS_DIR="$HANA_DIR/records"

function install {
  echo "Installing Hana CLI ..."

  LATEST=$(curl -qs https://api.github.com/repos/iankressin/hana/releases/latest | grep tag_name | head -n 1 | cut -d '"' -f 4);
  URL="https://github.com/iankressin/hana/releases/download/${LATEST}/hana-gui"

  echo $URL

  if [ ! -d $HANA_DIR ]; then
    echo "--> Configuring directories ..."
    mkdir $HANA_DIR
    cd $HANA_DIR
    if [ ! -d $BIN_DIR ]; then
      mkdir $BIN_DIR 
      cd $BIN_DIR
    fi
  fi

  mkdir $RECORDS_DIR
  echo "{}" >> "$RECORDS_DIR/folders.json"
  
  cd $BIN_DIR

  echo "--> Downloading ..."
  bash -c "ls"
  bash -c "curl --fail -# -L $URL > hana-gui"
  BIN="hana-gui"
	  chmod +x $BIN || fail "chmod +x failed"

  echo "*** DONE ***"
  echo "Now, please add $BIN_DIR to your PATH"
}

install
