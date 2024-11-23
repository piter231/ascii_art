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

Command:
```bash
cargo run high_seas.webp 500 1 80 32  
```

Black and white image:
![bw-image](output_example.jpg)

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


## Requirements

- Rust (version 1.70 or later)
- Rust libraries: `image`

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/piter231/ascii_art.git
   cd ascii_art
   ```
2. Run the program:
   ```bash
   cargo run <input-file> <black_and_white_image_bias> <ascii_art_bias> <x_dimension_of_output> <y_dimension_of_output>
   ```


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contribution

Feel free to contribute by submitting issues or creating pull requests. Any feedback is appreciated!

