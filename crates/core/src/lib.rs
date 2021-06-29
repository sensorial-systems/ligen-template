//! Generator definition.

#[derive(Clone, Copy, Debug, Default)]
pub struct Generator;

impl ligen::generator::Generator for Generator {
    fn new(_context: &Context, _attributes: &Attributes) -> Self {
        Default::default()
    }
}

impl ligen::generator::FileGenerator for Generator {
    fn generate_files(&self, _context: &Context, _file_set: &mut FileSet, _implementation: Option<&ImplementationVisitor>) {

    }
}

impl FFIGenerator for Generator {
    fn generate_ffi(&self, _context: &Context, _implementation: Option<&ImplementationVisitor>) -> TokenStream {
        Default::default()
    }
}
