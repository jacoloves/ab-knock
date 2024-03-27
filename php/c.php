<?php

$line = fgets(STDIN);

$pieaces = explode('.', $line);

echo end($pieaces);
