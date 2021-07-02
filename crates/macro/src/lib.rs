//! Generator macro.

ligen::define_binding_generator!(name = "{{crate_name}}", generator = "{{crate_name}}_core::Generator");
// Or if you want to create a project generator:
// ligen::define_project_generator!(name = "{{crate_name}}", generator = "{{crate_name}}_core::Generator");