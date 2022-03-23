# shred-rs
A CLI tool to shred a file or directory full of files.
Bash representation of the code would be:
```
#!/bin/bash
# Function to get the filesize and create a dummy file of the exact filesize.
forFile () {
FILESIZE=$(wc -c $1 | awk '{print $1}')
dd if=/dev/zero of=$1 bs=1 count=$FILESIZE >> /dev/null 2>&1
}

# Check if the provided argument is a file and not a directory
if [ ! -d $1 ]
then
	forFile $1
else
	echo "Please pass a file to be over-written by 0s!"
fi

```
