use std::fs::{
    self, File,
};
use std::{
    io::{
        prelude::*, BufReader,
    }, vec::Vec,
};

struct Passage {
    pid: u32,
    // indices for the name, relative to the start of the passage
    name_st: u8,
    name_en: u8,
    //indices for content
    cont_st: u32,
    cont_en: u32,
    fn_pass: bool,
}

pub fn handlePassages(sections: &super::twineSections){
    let raw = fs::read_to_string("index.html").unwrap();
    let storydataRaw = String::from(
        &raw[sections.storydata_index..sections.firstPassage_index]
        );
    let passagesRaw = String::from(
        &raw[sections.firstPassage_index..sections.endStorydata_index]
        );
    
    // the passage id. essentially the index of a passage.
    let mut pid: u32;
    // pid of the passage identified in the host as the starting passage.
    // this passage will be copied into index.html instead of a
    // content file.
    let mut startPassage: u32;
    // vector of which passages need to be included in every single content file
    let mut functionPassages: Vec<u32> = Vec::new();
    
    fn sortFunctionPassages(&raw, &mut functionPassages);
}

fn sortFunctionPassages(raw: &String, functionPassages: &mut Vec<u32>){
    let passageIndices: Vec<_> = raw.match_indices("<tw-passagedata").collect();
    // this will miss the final passage!!
    for i in 0..passageIndices.len() - 1 {
        let passage = &raw[passageIndices[i].0..passageIndices[i+1].0];
    }
    // again for passageindices.len() to eof
    // TODO
    
    // write the function passages to their own file 
}

fn parsePassage(raw: &String) -> Passage {
    let passage = Passage {};
    // get the
    // pid,
    let pidSt = raw.find("pid=\"");
    let pidEn = &raw[pidSt..raw.len()].find("\"");
    Passage.pid = &raw[pidSt..pidEn].parse().expect("Invalid PID on passage");
    // name,
    // and indices for content
    passage
}
