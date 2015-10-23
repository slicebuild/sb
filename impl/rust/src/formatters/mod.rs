use slice::item::Slice;
use options_parse::Format;

mod docker_formatter;
mod shell_formatter;

pub fn code_for_slice(slice: &Slice, format: Format) -> String {
    match format {
		Format::Docker => docker_formatter::code_for_slice(slice),
		Format::Shell => shell_formatter::code_for_slice(slice),
	}
}