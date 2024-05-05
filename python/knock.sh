#!/bin/bash

alphabet="abcdefghijklmnopqrstuvwxyz"

length=${#alphabet}

max_num=1000

flag=0

for ((i = 1; i < $max_num; i++)); do
	for ((j = 0; j < $length; j++)); do
		char=${alphabet:j:1}
		connect_char_file="${char}-${i}.py"
		if [ ! -f "$connect_char_file" ]; then
			cp -p tmp.py $connect_char_file
			echo "cp -p tmp.py $connect_char_file"
			flag=1
			break
		fi
	done
	if [ $flag -eq 1 ]; then
		break
	fi
done

echo "copy done!"