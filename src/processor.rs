use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{env, process, io, fs};
extern crate rocket;
// use rocket::response::status::NotFound;
use rocket::{Rocket, Build};
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16},
};

use crate::routes::get_routes::{get_all, get_many, get_one, get_custom_filter};
use crate::input_filter_engine::query_filter;

pub fn processor() -> Result<Rocket<Build>,Box<dyn Error>>{

let directory = String::from("C:/Users/Acer/OneDrive/Documents/Everything_rust/hadron/.data/configure");

    let files = match fs::read_dir(&directory){
        Ok(val)=> {val},
      _=> {
        eprint!("{} is nowhere to be found",&directory);    
        process::exit(0)
    }  
    };

    // println!("{:#?}",files);

    for file in files {
        if let Ok(val) = file{
            // println!("{:#?}",val);
             if val.file_type()?.is_file() {
                // Process the file
                println!("File Name: {:?}", val.file_name());
                
                let mut opened = File::open(val.path())?;

                let mut contents = Vec::new();

                if let Ok(len) =  opened.read_to_end(&mut contents){

                    println!("length of data {}",len);
                    // println!("{:#?}",contents);

                    // let key = GenericArray::from([0u8; 16]);
                    // let key = GenericArray::from_slice();

                    let key_val = "Thats my Kung Fu".as_bytes();
                    let mut vec_key = Vec::new();
                    for &byte in key_val.iter(){
                        vec_key.push(byte);
                    } 

                   let key = GenericArray::from_slice(&vec_key);

                    // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);

                    let mut blocks:Vec<GenericArray<u8, U16>> = Vec::new();


                    let mut counter = 0;

                    loop{    
                    
                        println!("counterrr:{}, {}",counter,contents.len());

                       blocks.push( GenericArray::from_slice(&contents[counter..(counter+16)]).clone());

                       println!("{}",&counter);

                       counter = counter + 16; 


                        if counter == contents.len(){

                            println!("{counter}");
                            
                            break;
                        }
                        
                    }                     


                   // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
                    //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

                    let cipher = Aes128::new(&key);


                    // cipher.decrypt_block(&mut block);

                    cipher.decrypt_blocks(&mut blocks);

                    for block in blocks{

                        let plaintext = String::from_utf8(block.to_vec())?;

                        println!("{:#?}",&plaintext);

                    }



                    // let p_file = String::new();
                    //////////////////////////////////////
                    //write bloc

                    // let mut new_file = OpenOptions::new().append(true).open(directory.clone()+"/configure.dat")?;

                    // if let Err(err) =  new_file.write_all(&block){
                    //     eprintln!("{:?}",err);
                    //     process::exit(0);
                    // }



                }

            
            }

        };


    }

    let args:Vec<String> = env::args().collect();

    // dbg!(&args);
    // print!("{}",args.iter().count());

    if args.iter().count() == 1{

        println!("                                                        ..                    
        ./((###########(/                   /(###############(/            
     /(#####((////////((###(/            (####((/////////((######(*        
  /(##########################(       ,(###########################((      
*(((############################(    (###############################(/    
/((((/***/(###(((((((###########%%(  %####(///(#####((((((###########(((/   
/((((*,,,*(###(((///((###(/*/(#%%%%%%%%##(*,,,/(####(((//((###(/***/***/((/  
.((((/,,,,/####((////((###(/**(#%%%%%%%%##/,,,*(######(///((###(/****/***((/. 
/((((*,,,*(####((///(((###(/*/#%%%%%%%%%##(,,,*(######((/(((###(/****/***/((* 
,((((/**//((##(((((((((###((/(#%%%%%%%%%##(**//((####(((((((###((///((((((((. 
//(((((((((###((((((((####((##%%%%%%%%%###((((((####(((((((#####(((######(/  
/(((((((######((((((##########%%%%(  %####(((#######((((###############(/   
*/((((((((######################(    (##############################((/    
  /(((((((####################(       .(##########################((/      
     /(((###################(            (#######################(*        
        .((#############(                   *#################(   \n\n");


    println!("Hadron Query Mode::::::::::::\n");


    
    loop {
        let mut input = String::new();

    // Read user input and handle errors
    match io::stdin().read_line(&mut input) {
        Ok(_) => {

            if input == String::from("exit\r\n") || input == String::from("exit\n"){
                println!("Hadron exiting.........");
                process::exit(0);
            }else{
                // println!("{input}");
                query_filter::filter(&input);
            }
        }
        Err(_) => {
            // eprintln!("Error: {}", error);
            // println!("Hadron exiting.........");
            process::exit(0);
        }
    }
}
    }

    if args[1].to_string() == String::from("powerup") {
        Ok(rocket::build().mount("/get", routes![get_one,get_all,get_many,get_custom_filter]))
    }
    else{
            process::exit(0);

    }

}