Some brainfuck interpreter thing, nothing fancy...

### Usage
1. Clone
2. Compile
3. Run

### Characters
| Char 	| Use                                                                       	|
|------	|---------------------------------------------------------------------------	|
| +    	| Increment the current cell by one                                         	|
| -    	| Decrement the current cell by one                                         	|
| >    	| Move the pointer one cell to the left                                     	|
| <    	| Move the pointer once cell to the right                                   	|
| .    	| Print the current cell to stdout                                          	|
| ,    	| Read one byte of input to the current cell                                	|
| [    	| Start a loop                                                              	|
| ]    	| End a loop                                                                	|
| ?    	| Use in place of `.`. Pushes to debug vector instead of printing to stdout 	|

