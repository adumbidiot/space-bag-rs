# space-bag-rs
Too much stuff, not enough space.

Did you know that your files have space in them? 
Thats right! Every file is made of binary, which are ones and zeroes. 
Imagine how much space is wasted by all those empty zeroes!
Have you ever wished you could remove all that pesky space from your files? 
Well, you're in luck!
This program vacuums out all the empty space so that you can fit more files on your computer! Wow!

## How it works
All files are binary and binary is composed of ones and zeroes. 
This program converts files from binary (base 2) into base 1. 
Yes, this is a joke. 
Yes, this is horrible.
I also used a naiive implementation since I found it unlikely anyone would use this on medium-to-large files.

## Usage
```
space-bag pack <uncompressed input file> <compressed output file> 
space-bag unpack <compressed input file> <uncompressed output file>
```

## Obligatory Warning
If you didn't understand the ramifactions of the "How it Works" section here's an explicit warning: Please don't use this on a file larger than 4 bytes. 

## Inspiration
 * https://devblogs.microsoft.com/oldnewthing/20180515-00/?p=98755
 * ```
   Jimbo: what if people just stopped using .zip tho? and started using something like a vacuum bag. those are really good at getting puffy stuff into dense stuff
   Me: oh like space bags? yeah i bet theres a lot of air in my files i could take out tbh
   Me: Wait what if I removed all the 0s from my binary files since zero is nothing. I could take out all the space and leave all the important ones.
   ```
