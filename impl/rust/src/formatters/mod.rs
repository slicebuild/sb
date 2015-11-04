use slice::Slice;
use options_parse::Format;

mod docker_formatter;
mod shell_formatter;

/// Generates code for sections
/// # Arguments
/// * `slice` slice
/// * `format` code format
pub fn generate_code(slice: &Slice, format: &Format) -> String {
    match *format {
		Format::Docker => docker_formatter::generate_code(slice),
		Format::Shell => shell_formatter::generate_code(slice),
	}
}