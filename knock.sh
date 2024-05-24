#!/bin/bash

alphabet="abcdefghijklmnopqrstuvwxyz"

length=${#alphabet}

max_num=1000

cpp_flag=0
py_flag=0

cpp_dir="cpp"
py_dir="python"

cpp_tmp_file="./${cpp_dir}/tmp.cpp"
py_tmp_file="./${py_dir}/tmp.py"

for ((i = 1; i < $max_num; i++)); do
	for ((j = 0; j < $length; j++)); do
		char=${alphabet:j:1}
		connect_cpp_file="./${cpp_dir}/${char}-${i}.cpp"
		if [ ! -f "$connect_cpp_file" ]; then
			cp -p $cpp_tmp_file $connect_cpp_file
			echo "cp -p $cpp_tmp_file $connect_cpp_file"
			cpp_flag=1
			break
		fi
	done
	if [ $cpp_flag -eq 1 ]; then
		break
	fi
done

#for ((i = 1; i < $max_num; i++)); do
#	for ((j = 0; j < $length; j++)); do
#		char=${alphabet:j:1}
#		connect_py_file="./${py_dir}/${char}-${i}.py"
#		if [ ! -f "$connect_py_file" ]; then
#			cp -p $py_tmp_file $connect_py_file
#			echo "cp -p $py_tmp_file $connect_py_file"
#			py_flag=1
#			break
#		fi
#	done
#	if [ $py_flag -eq 1 ]; then
#		break
#	fi
#done

echo "copy done!"

