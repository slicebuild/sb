//FormatterDocker Mod
use super::slice::{item, section};
// When this function is called, it will have to be changed to handle a return value, instead.
pub fn Write(slice: Slice) -> String { 
    let mut string = "";
    for i in slice.Section {
        if i.0 != None {
           string = format!("{}{}", string, section_to_string(i.0));
        }
    }
        string;
}
// Equivalent of the second "write" function in the FormatterDocker.cs file in the mono impl. In
// rust, however, functions cannot be overloaded.  
fn section_to_string(slice_section:Section, string:String) -> String {
   if slice_section.kind == (Some(Kind::From)) {
        for i in slice_section.items {
           string =  format!("FROM {}", i);
        }
   }
   if slice_section.kind == (Some(Kind::RUN)) {
        let mut text = "";
            for i in slice_section.items {
                text = format!("{}  &&  \\\\ \n{}", text, i);
            }
            // Above code is supposed to emulate the below line. I am unsure about it, though.
            // var text = string.Join($" && {"\\"} {Environment.NewLine}", section.Lines);

        string = format!("{}RUN \n{}\n", string, text);
 
   }
    return string;
}
