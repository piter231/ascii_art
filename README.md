### README.md

# ASCII Art Generator

`ascii_art` is a simple Rust program that transforms images into ASCII art, using hashtags (`#`) and spaces (` `) as characters. The program also provides the option to generate a black-and-white version of the input image.

## Features

- Converts an image to ASCII art using `#` and ` ` characters.
- Generates a black-and-white version of the input image (output.jpg).
- Supports popular image formats (e.g., PNG, JPEG, WEBP).

## Example

Original image:  
![input-image](high_seas.webp)

Generated ASCII art:  
```
                                              ############## ############## ##  
                                              ################# ################
                                              ######## ############### #####  ##
                                              ########################## # #####
                                                     ############ ########### ##
                                                     ###########################
                                                     ################## ## #####
                                         ###         #################### ######
                                        ###             ######### ######        
                        ##              ##        ########################      
             ##        ##          #   ##     # ########################        
            ##                #####   ##   ###  #######################         
           ###        ##  #########  ###   ##   ################# #        ##   
           ### ####  ###  ###  #### ####  ###  ##########           ######      
          ########   ##  ###  ####  ###  ###  ###          ######  ####         
         #### ###   ##   ###  ###  ###   #         ###  ######### #########     
        ###   ##   ###  ########               ######        ###  ### #####     
        ##   ##   ###  #### ###         ##   #    ##   ########       ####      
       ###  ##   ##         ##   ######     ## ####   ##    ##   ######         
      ###             #######   ####       ##        ########                   
              ###### ###       #########  ### ####  ###                         
               #####       ## #### ##### ########              #                
           ##      #   ######      ####  ###                                    
             ##        #   #  ######                                            
                ## #  #   #            ##                                       
                         #      # ######  #                                     
   #                     #    #########                                         
                               ######  #                                             
```

Generated black-and-white image:  
![output-bw](output-bw.jpg)

## Requirements

- Rust (version 1.70 or later)
- Rust libraries: `image`, `termion`

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/ascii_art.git
   cd ascii_art
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the program:
   ```bash
   cargo run --release -- <input-file> --ascii --output <output-file>
   ```

## Usage

### Command-Line Options
- `--ascii`: Generate ASCII art from the input image.
- `--bw`: Generate a black-and-white version of the input image.
- `--output <output-file>`: Specify the output file for the generated result.

### Example Commands
Generate ASCII art and save it to a file:
```bash
cargo run --release -- input.jpg --ascii --output output.txt
```

Generate a black-and-white version of an image:
```bash
cargo run --release -- input.jpg --bw --output output-bw.jpg
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contribution

Feel free to contribute by submitting issues or creating pull requests. Any feedback is appreciated!

