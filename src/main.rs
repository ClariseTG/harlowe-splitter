use std::fs::{
    self, File,
};
use std::{
    io::{
        prelude::*, BufReader,
    }, vec::Vec,
};

mod parse;
mod passage;

// TODO:
// - get passages parsed
// - get saving the fn passages to their own file working
// - get going through the other passages and creating a file,
//      writing the header and fn passages, writing the passage contents, and writing the footer
//      working
// - get editing the tw-storydata to include the pid + name working. maybe even hard code the template
//      since it doesn't change between harlowe versions, css, and js

pub struct twineSections {
    pub storydata_index: usize,
    pub firstPassage_index: usize,
    pub endStorydata_index: usize,
}

fn main() {
    // CONFIG
    //let targetDirectory = "remagica_site";

    // STORAGE
    let mut sections = twineSections {
        storydata_index: 0,
        // storydata header + css between these two
        firstPassage_index: 0,
        // passages between these two
        endStorydata_index: 0,
    };

    // INITIALIZATION
    // creating the directories
    // TODO: make this based off of the specified target directory?
    fs::create_dir_all("output").expect("main directory not created");
    fs::create_dir_all("output/content").expect("content directory not created");
    // create header.txt and footer.txt
    // .txt separates it from the passages, which will all
    //      lack file extensions
    let mut header = File::create("output/header.txt");
    let mut footer = File::create("output/footer.txt");
    let mut dataPassages = File::create("output/data.txt");

    // PARSING FILE - FIRST PASS
    // open the target file
    // TODO: make this be based off of a flag in the terminal
    parse::parseFile(&mut sections);
    
    let raw = fs::read_to_string("index.html").unwrap();
    
    // PASSAGES
    passage::handlePassages(&sections);
    
    // CLEANUP, IF NECESSARY
}
