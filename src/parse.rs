
use std::fs::{
    self, File,
};
use std::io::{
    prelude::*, BufReader,
};

pub fn parseFile(sections: &mut super::twineSections) {
    enum state {
        init,
    
    }
    let parseState = state::init;
    let raw = fs::read_to_string("index.html").unwrap();
    let tag_storydata = "<tw-storydata";
    let tag_firstPassage = "<tw-passage";
    let tag_endStorydata = "</tw-storydata";
    
    let storydata_index = raw.find(tag_storydata).expect("no tw-storydata found");
    let firstPassage_index = &raw[storydata_index..raw.len()].find(tag_firstPassage).expect("no tw-passage found") + storydata_index;
    let endStorydata_index = raw.find(tag_endStorydata).expect("no /tw-storydata found");
    
    fs::write("output/header.txt", &raw[0..storydata_index]);
    fs::write("output/footer.txt", &raw[endStorydata_index..raw.len()]);
    
    sections.storydata_index = storydata_index;
    sections.firstPassage_index = firstPassage_index;
    sections.endStorydata_index = endStorydata_index;
}
