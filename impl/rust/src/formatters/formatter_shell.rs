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
// Equivalent of the second "Write" function in the FormatterShell.cs file in the mono impl. In
// rust, however, functions cannot be overloaded.  
fn section_to_string(slice_section:Section, string:String) -> String {
   if slice_section.kind == (Some(Kind::OS)) {
        for i in slice_section.items {
           string =  format!("# {}", i);
        }
   }
   if slice_section.kind == (Some(Kind::RUN)) {
            for i in slice_section.items {
                string = format!("{}{}\n", string, i);
            }
 
   }
    return string;
}
